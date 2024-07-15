use super::{create_default_envfile, DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE};
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, PoisonError},
};
use tempfile::{tempdir, TempDir};

/// Env var convenience type.
type EnvMap = HashMap<String, String>;

/// Initialized in [`get_env_locker`]
static ENV_LOCKER: OnceCell<Arc<Mutex<EnvMap>>> = OnceCell::new();

/// A test environment.
///
/// Will create a new temporary directory. Use its builder methods to configure
/// the directory structure, preset variables, envfile name and contents, and
/// the working directory to run the test from.
///
/// Creation methods:
/// - [`TestEnv::init`]: blank environment (no envfile)
/// - [`TestEnv::init_with_envfile`]: blank environment with an envfile
/// - [`TestEnv::default`]: default testing environment (1 existing var and 2
///       set in a `.env` file)
#[derive(Debug)]
pub struct TestEnv {
    temp_dir: TempDir,
    work_dir: PathBuf,
    env_vars: Vec<KeyVal>,
    envfile_contents: Option<Vec<u8>>,
    envfile_path: PathBuf,
}

/// Simple key value struct for representing environment variables
#[derive(Debug, Clone)]
pub struct KeyVal {
    pub key: String,
    pub value: String,
}

/// Run a test closure within a test environment.
///
/// Resets the environment variables, loads the [`TestEnv`], then runs the test
/// closure. Ensures only one thread has access to the process environment.
pub fn test_in_env<F>(test_env: TestEnv, test: F)
where
    F: FnOnce(),
{
    let locker = get_env_locker();
    // ignore a poisoned mutex
    // we expect some tests may panic to indicate a failure
    let original_env = locker.lock().unwrap_or_else(PoisonError::into_inner);
    // we reset the environment anyway upon acquiring the lock
    reset_env(&original_env);
    create_env(&test_env);
    test();
    // drop the lock and the `TestEnv` - should delete the tempdir
}

/// Run a test closure within the default test environment.
///
/// Resets the environment variables, creates the default [`TestEnv`], then runs
/// the test closure. Ensures only one thread has access to the process
/// environment.
///
/// The default testing environment sets an existing environment variable
/// `DEFAULT_EXISTING_KEY`, which is set to `loaded_from_env`. It also creates a
/// `.env` file with the two lines:
///
/// ```ini
/// DEFAULT_TEST_KEY=default_test_val
/// DEFAULT_EXISTING_KEY=loaded_from_file
/// ```
///
/// Notice that file has the potential to override `DEFAULT_EXISTING_KEY` depending
/// on the what's being tested.
pub fn test_in_default_env<F>(test: F)
where
    F: FnOnce(),
{
    let test_env = TestEnv::default();
    test_in_env(test_env, test);
}

impl TestEnv {
    /// Blank testing environment in a new temporary directory.
    ///
    /// No envfile_contents or pre-existing variables to set. The envfile_name
    /// is set to `.env` but won't be written until its content is set. The
    /// working directory is the created temporary directory.
    pub fn init() -> Self {
        let tempdir = tempdir().expect("create tempdir");
        let work_dir = tempdir.path().to_owned();
        let envfile_path = work_dir.join(".env");
        Self {
            temp_dir: tempdir,
            work_dir,
            env_vars: Default::default(),
            envfile_contents: None,
            envfile_path,
        }
    }

    /// Testing environment with custom envfile_contents.
    ///
    /// No pre-existing env_vars set. The envfile_name is set to `.env`. The
    /// working directory is the created temporary directory.
    pub fn init_with_envfile(contents: impl Into<Vec<u8>>) -> Self {
        let mut test_env = Self::init();
        test_env.set_envfile_contents(contents);
        test_env
    }

    /// Change the absolute path to the envfile.
    pub fn set_envfile_path(&mut self, path: PathBuf) -> &mut Self {
        self.envfile_path = path;
        self
    }

    /// Specify the contents of the envfile.
    ///
    /// If this is the only change to the [`TestEnv`] being made, use
    /// [`init_with_envfile`](TestEnv::init_with_envfile).
    ///
    /// Setting it to an empty string will cause an empty envfile to be created
    pub fn set_envfile_contents(&mut self, contents: impl Into<Vec<u8>>) -> &mut Self {
        let contents = contents.into();
        self.envfile_contents = Some(contents);
        self
    }

    /// Set the working directory the test will run from.
    ///
    /// The default is the created temporary directory. This method is useful if
    /// you wish to run a test from a subdirectory or somewhere else.
    pub fn set_work_dir(&mut self, path: PathBuf) -> &mut Self {
        self.work_dir = path;
        self
    }

    /// Add an individual environment variable.
    ///
    /// This adds more pre-existing environment variables to the process before
    /// any tests are run.
    pub fn add_env_var(&mut self, key: impl ToString, value: impl ToString) -> &mut Self {
        self.env_vars.push(KeyVal {
            key: key.to_string(),
            value: value.to_string(),
        });
        self
    }

    /// Set the pre-existing environment variables.
    ///
    /// These variables will get added to the process' environment before the
    /// test is run. This overrides any previous env vars added to the
    /// [`TestEnv`].
    ///
    /// If you wish to just use a slice of tuples, use
    /// [`set_env_vars_tuple`](TestEnv::set_env_vars_tuple) instead.
    pub fn set_env_vars(&mut self, env_vars: Vec<KeyVal>) -> &mut Self {
        self.env_vars = env_vars;
        self
    }

    /// Set the pre-existing environment variables using [`str`] tuples.
    ///
    /// These variables will get added to the process' environment before the
    /// test is run. This overrides any previous env vars added to the
    /// [`TestEnv`].
    ///
    /// If you wish to add an owned `Vec<KeyVal>` instead of `str` tuples, use
    /// [`set_env_vars`](TestEnv::set_env_vars) instead.
    pub fn set_env_vars_tuple(&mut self, env_vars: &[(&str, &str)]) -> &mut Self {
        self.env_vars = env_vars
            .iter()
            .map(|(key, value)| KeyVal {
                key: key.to_string(),
                value: value.to_string(),
            })
            .collect();

        self
    }

    /// Create a child folder within the temporary directory.
    ///
    /// This will not change the working directory the test is run in, or where
    /// the envfile is created.
    ///
    /// Will create parent directories if they are missing.
    pub fn add_child_dir_all(&self, rel_path: impl AsRef<Path>) -> PathBuf {
        let rel_path = rel_path.as_ref();
        let child_dir = self.temp_path().join(rel_path);
        if let Err(err) = fs::create_dir_all(&child_dir) {
            panic!(
                "unable to create child directory: `{}` in `{}`: {}",
                self.temp_path().display(),
                rel_path.display(),
                err
            );
        }
        child_dir
    }

    /// Get a reference to the path of the temporary directory.
    pub fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Get a reference to the working directory the test will be run from.
    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    /// Get a reference to environment variables that will be set **before**
    /// the test.
    pub fn env_vars(&self) -> &[KeyVal] {
        &self.env_vars
    }

    /// Get a reference to the bytes that will be placed in the envfile.
    ///
    /// If `None` is returned, an envfile will not be created
    pub fn envfile_contents_as_bytes(&self) -> Option<&[u8]> {
        self.envfile_contents.as_deref()
    }

    /// Get a reference to the string that will be placed in the envfile.
    ///
    /// If `None` is returned, an envfile will not be created
    ///
    /// ## Panics
    ///
    /// This will panic if the envfile contents are not valid UTF-8
    pub fn envfile_contents_as_str(&self) -> Option<&str> {
        self.envfile_contents_as_bytes().map(|bytes| {
            let out = std::str::from_utf8(bytes).expect("valid UTF-8");
            Some(out)
        })?
    }

    /// Get a reference to the path of the envfile.
    pub fn envfile_path(&self) -> &Path {
        &self.envfile_path
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        let temp_dir = tempdir().expect("create tempdir");
        let work_dir = temp_dir.path().to_owned();
        let env_vars = vec![KeyVal {
            key: DEFAULT_EXISTING_KEY.into(),
            value: DEFAULT_EXISTING_VALUE.into(),
        }];
        let envfile_contents = Some(create_default_envfile().into());
        let envfile_path = work_dir.join(".env");
        Self {
            temp_dir,
            work_dir,
            env_vars,
            envfile_contents,
            envfile_path,
        }
    }
}

impl From<(&str, &str)> for KeyVal {
    fn from(kv: (&str, &str)) -> Self {
        let (key, value) = kv;
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl From<(String, String)> for KeyVal {
    fn from(kv: (String, String)) -> Self {
        let (key, value) = kv;
        Self { key, value }
    }
}

/// Get a guarded copy of the original process' env vars.
fn get_env_locker() -> Arc<Mutex<EnvMap>> {
    Arc::clone(ENV_LOCKER.get_or_init(|| {
        let map: EnvMap = env::vars().collect();
        Arc::new(Mutex::new(map))
    }))
}

/// Reset the process' env vars back to what was in `original_env`.
fn reset_env(original_env: &EnvMap) {
    // remove keys if they weren't in the original environment
    env::vars()
        .filter(|(key, _)| !original_env.contains_key(key))
        .for_each(|(key, _)| env::remove_var(key));
    // ensure original keys have their original values
    original_env
        .iter()
        .for_each(|(key, value)| env::set_var(key, value));
}

/// Create an environment to run tests in.
///
/// Writes the envfile, sets the working directory, and sets environment vars.
fn create_env(test_env: &TestEnv) {
    // only create the envfile if its contents has been set
    if let Some(contents) = test_env.envfile_contents_as_bytes() {
        create_envfile(&test_env.envfile_path, contents);
    }

    env::set_current_dir(&test_env.work_dir).expect("setting working directory");

    for KeyVal { key, value } in &test_env.env_vars {
        env::set_var(key, value)
    }
}

/// Create an envfile for use in tests.
fn create_envfile(path: &Path, contents: &[u8]) {
    if path.exists() {
        panic!("envfile `{}` already exists", path.display())
    }
    // inner function to group together io::Results
    fn create_env_file_inner(path: &Path, contents: &[u8]) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(contents)?;
        file.sync_all()
    }
    // call inner function
    if let Err(err) = create_env_file_inner(path, contents) {
        // handle any io::Result::Err
        panic!("error creating envfile `{}`: {}", path.display(), err);
    }
}
