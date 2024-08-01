use super::{create_default_env_file, DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE};
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
/// the directory structure, preset variables, env file name and contents, and
/// the working directory to run the test from.
///
/// Creation methods:
/// - [`TestEnv::init`]: blank environment (no env file)
/// - [`TestEnv::init_with_env_file`]: blank environment with a custom `.env`
/// - [`TestEnv::default`]: default testing environment (1 existing var and 2
///       set in a `.env` file)
#[derive(Debug)]
pub struct TestEnv {
    // Temporary directory that will be deleted on drop
    _temp_dir: TempDir,
    dir_path: PathBuf,
    work_dir: PathBuf,
    env_vars: EnvMap,
    env_files: Vec<EnvFile>,
}

#[derive(Debug, Clone)]
/// Simple path and byte contents representing a `.env` file
pub struct EnvFile {
    pub path: PathBuf,
    pub contents: Vec<u8>,
}

/// Run a test closure within a test environment.
///
/// Resets the environment variables, loads the [`TestEnv`], then runs the test
/// closure. Ensures only one thread has access to the process environment.
pub fn test_in_env<F>(testenv: &TestEnv, test: F)
where
    F: FnOnce(),
{
    let locker = get_env_locker();
    // ignore a poisoned mutex
    // we expect some tests may panic to indicate a failure
    let original_env = locker.lock().unwrap_or_else(PoisonError::into_inner);
    // we reset the environment anyway upon acquiring the lock
    reset_env(&original_env);
    create_env(testenv);
    test();
    reset_env(&original_env);
    // drop the lock
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
    let testenv = TestEnv::default();
    test_in_env(&testenv, test);
}

/// Create a [`TestEnv`] without an env file, but with the
/// default existing environment variable.
pub fn create_testenv_with_default_var() -> TestEnv {
    let mut testenv = TestEnv::init();
    testenv.add_env_var(DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE);
    testenv
}

impl TestEnv {
    /// Blank testing environment in a new temporary directory.
    ///
    /// No env file or pre-existing variables set. The working directory is the
    /// created temporary directory.
    pub fn init() -> Self {
        let tempdir = tempdir().expect("create tempdir");
        let dir_path = tempdir
            .path()
            .canonicalize()
            .expect("canonicalize dir_path");
        Self {
            _temp_dir: tempdir,
            work_dir: dir_path.clone(),
            dir_path,
            env_vars: HashMap::default(),
            env_files: vec![],
        }
    }

    /// Testing environment with custom env file contents.
    ///
    /// No pre-existing env vars set. The env file path is set to `.env`. The
    /// working directory is the created temporary directory.
    pub fn init_with_env_file(contents: impl Into<Vec<u8>>) -> Self {
        let mut testenv = Self::init();
        testenv.add_env_file(".env", contents);
        testenv
    }

    /// Add an individual env file.
    ///
    /// ## Arguments
    ///
    /// - `path`: relative from the temporary directory
    /// - `contents`: bytes or string
    ///
    /// ## Panics
    ///
    /// - if the path is empty or the same as the temporary directory
    /// - if the env file already exists
    pub fn add_env_file<P, C>(&mut self, path: P, contents: C) -> &mut Self
    where
        P: AsRef<Path>,
        C: Into<Vec<u8>>,
    {
        let path = self.dir_path.join(path);
        self.assert_env_file_path_is_valid(&path);
        self.add_env_file_assume_valid(path, contents.into())
    }

    /// Add an individual environment variable.
    ///
    /// This adds more pre-existing environment variables to the process before
    /// any tests are run.
    ///
    /// ## Panics
    ///
    /// - if the env var already exists in the testenv
    /// - if the key is empty
    pub fn add_env_var<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        self.assert_env_var_is_valid(&key);
        self.env_vars.insert(key, value.into());
        self
    }

    /// Set all the pre-existing environment variables.
    ///
    /// These variables will get added to the process' environment before the
    /// test is run. This overrides any previous env vars added to the
    /// [`TestEnv`].
    ///
    /// ## Panics
    ///
    /// - if an env var is set twice
    /// - if a key is empty
    pub fn set_env_vars(&mut self, env_vars: &[(&str, &str)]) -> &mut Self {
        for &(key, value) in env_vars {
            self.add_env_var(key, value);
        }
        self
    }

    /// Set the working directory the test will run from.
    ///
    /// The default is the created temporary directory. This method is useful if
    /// you wish to run a test from a subdirectory or somewhere else.
    ///
    /// ## Arguments
    ///
    /// - `path`: relative from the temporary directory
    ///
    /// ## Panics
    ///
    /// - if the path does not exist
    pub fn set_work_dir(&mut self, path: impl AsRef<Path>) -> &mut Self {
        self.work_dir = self
            .temp_path()
            .join(path.as_ref())
            .canonicalize()
            .expect("canonicalize work_dir");
        assert!(
            self.work_dir.exists(),
            "work_dir does not exist: {}",
            self.work_dir.display()
        );
        self
    }

    /// Create a child folder within the temporary directory.
    ///
    /// This will not change the working directory the test is run in, or where
    /// the env file is created.
    ///
    /// Will create parent directories if they are missing.
    pub fn add_child_dir(&mut self, path: impl AsRef<Path>) -> &mut Self {
        let path = path.as_ref();
        let child_dir = self.temp_path().join(path);
        if let Err(err) = fs::create_dir_all(child_dir) {
            panic!(
                "unable to create child directory: `{}` in `{}`: {err}",
                path.display(),
                self.temp_path().display()
            );
        }
        self
    }

    /// Reference to the path of the temporary directory.
    pub fn temp_path(&self) -> &Path {
        &self.dir_path
    }

    /// Reference to the working directory the test will be run from.
    pub fn work_dir(&self) -> &Path {
        &self.work_dir
    }

    /// Reference to environment variables that will be set **before** the test.
    pub const fn env_vars(&self) -> &EnvMap {
        &self.env_vars
    }

    /// Get a reference to the environment files that will created
    pub fn env_files(&self) -> &[EnvFile] {
        &self.env_files
    }

    fn add_env_file_assume_valid(&mut self, path: PathBuf, contents: Vec<u8>) -> &mut Self {
        let env_file = EnvFile { path, contents };
        self.env_files.push(env_file);
        self
    }

    fn assert_env_file_path_is_valid(&self, path: &Path) {
        assert!(
            path != self.temp_path(),
            "path cannot be empty or the same as the temporary directory"
        );
        assert!(
            !self.env_files.iter().any(|f| f.path == path),
            "env_file already in testenv: {}",
            path.display()
        );
    }

    fn assert_env_var_is_valid(&self, key: &str) {
        assert!(!key.is_empty(), "key cannot be empty");
        assert!(
            !self.env_vars.contains_key(key),
            "key already in testenv: {key}"
        );
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        let mut testenv = Self::init();
        testenv.add_env_var(DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE);
        testenv.add_env_file(".env", create_default_env_file());
        testenv
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
/// Writes the env files, sets the working directory, and sets environment vars.
fn create_env(testenv: &TestEnv) {
    env::set_current_dir(&testenv.work_dir).expect("setting working directory");

    for EnvFile { path, contents } in &testenv.env_files {
        create_env_file(path, contents);
    }

    for (key, value) in &testenv.env_vars {
        env::set_var(key, value);
    }
}

/// Create an env file for use in tests.
fn create_env_file(path: &Path, contents: &[u8]) {
    fn create_env_file_inner(path: &Path, contents: &[u8]) -> io::Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(contents)?;
        file.sync_all()
    }

    assert!(
        !path.exists(),
        "env_file `{}` already exists",
        path.display()
    );
    // inner function to group together io::Results

    // call inner function
    if let Err(err) = create_env_file_inner(path, contents) {
        // handle any io::Result::Err
        panic!("error creating env_file `{}`: {err}", path.display());
    }
}
