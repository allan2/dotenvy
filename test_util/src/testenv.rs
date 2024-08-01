use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, PoisonError},
};
use tempfile::{tempdir, TempDir};

use crate::{EnvFile, Error};

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
/// - [`TestEnv::new`]: blank environment (no env file)
/// - [`TestEnv::new_with_env_file`]: blank environment with a custom `.env`
#[derive(Debug)]
pub struct TestEnv {
    // Temporary directory that will be deleted on drop
    _temp_dir: TempDir,
    dir_path: PathBuf,
    work_dir: PathBuf,
    env_vars: EnvMap,
    env_files: Vec<EnvFile>,
}

/// Run a test closure within a test environment.
///
/// Resets the environment variables, loads the [`TestEnv`], then runs the test
/// closure. Ensures only one thread has access to the process environment.
///
/// ## Errors
///
/// - if the test fails
pub fn test_in_env<F>(testenv: &TestEnv, test: F) -> Result<(), Error>
where
    F: FnOnce(),
{
    let locker = get_env_locker();
    // ignore a poisoned mutex
    // we expect some tests may panic to indicate a failure
    // we reset the environment anyway upon acquiring the lock
    let original_env = locker.lock().unwrap_or_else(PoisonError::into_inner);
    // Safety: we hold the lock so no other thread can access the environment
    unsafe { reset_env(&original_env) };
    setup_files(testenv)?;
    unsafe { setup_env(testenv.env_vars()) };
    test();
    unsafe { reset_env(&original_env) };
    Ok(())
    // drop the lock
}

impl TestEnv {
    /// Blank testing environment in a new temporary directory.
    ///
    /// No env file or pre-existing variables set. The working directory is the
    /// created temporary directory.
    ///
    /// ## Errors
    ///
    /// - if creating the temporary directory fails
    /// - if canonicalizing the temporary directory's path fails
    pub fn new() -> Result<Self, Error> {
        let tempdir = tempdir().map_err(Error::CreatingTempDir)?;
        let dir_path = canonicalize_path(tempdir.path())?;
        let testenv = Self {
            _temp_dir: tempdir,
            work_dir: dir_path.clone(),
            dir_path,
            env_vars: HashMap::default(),
            env_files: vec![],
        };
        Ok(testenv)
    }

    /// Testing environment with custom env file contents.
    ///
    /// No pre-existing env vars set. The env file path is set to `.env`. The
    /// working directory is the created temporary directory.
    ///
    /// ## Errors
    ///
    /// - if creating the temporary directory fails
    /// - if canonicalizing the temporary directory's path fails
    pub fn new_with_env_file(contents: impl Into<Vec<u8>>) -> Result<Self, Error> {
        let mut testenv = Self::new()?;
        testenv.add_env_file(".env", contents)?;
        Ok(testenv)
    }

    /// Add an individual env file.
    ///
    /// ## Arguments
    ///
    /// - `path`: relative from the temporary directory
    /// - `contents`: bytes or string
    ///
    /// ## Errors
    ///
    /// - if the path is empty or the same as the temporary directory
    /// - if the env file already exists
    pub fn add_env_file<P, C>(&mut self, path: P, contents: C) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
        C: Into<Vec<u8>>,
    {
        let path = self.dir_path.join(path);
        self.check_env_file_path_is_valid(&path)?;
        self.add_env_file_assume_valid(path, contents.into());
        Ok(self)
    }

    /// Add an individual environment variable.
    ///
    /// This adds more pre-existing environment variables to the process before
    /// any tests are run.
    ///
    /// ## Errors
    ///
    /// - if the env var already exists in the testenv
    /// - if the key is empty
    pub fn add_env_var<K, V>(&mut self, key: K, value: V) -> Result<&mut Self, Error>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();
        self.check_env_var_is_valid(&key)?;
        self.env_vars.insert(key, value.into());
        Ok(self)
    }

    /// Set all the pre-existing environment variables.
    ///
    /// These variables will get added to the process' environment before the
    /// test is run. This overrides any previous env vars added to the
    /// [`TestEnv`].
    ///
    /// ## Errors
    ///
    /// - if an env var is set twice
    /// - if a key is empty
    pub fn set_env_vars(&mut self, env_vars: &[(&str, &str)]) -> Result<&mut Self, Error> {
        for &(key, value) in env_vars {
            self.add_env_var(key, value)?;
        }
        Ok(self)
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
    /// ## Errors
    ///
    /// - if the path does not exist
    /// - if canonicalizing the path fails
    pub fn set_work_dir(&mut self, path: impl AsRef<Path>) -> Result<&mut Self, Error> {
        let path = self.dir_path.join(path);
        if !path.exists() {
            return Err(Error::PathNotFound(path));
        }
        self.work_dir = canonicalize_path(path)?;
        Ok(self)
    }

    /// Create a child folder within the temporary directory.
    ///
    /// This will not change the working directory the test is run in, or where
    /// the env file is created.
    ///
    /// Will create parent directories if they are missing.
    ///
    /// ## Errors
    ///
    /// - if creating the directory fails
    pub fn add_child_dir(&mut self, path: impl AsRef<Path>) -> Result<&mut Self, Error> {
        let path = path.as_ref();
        let child_dir = self.temp_path().join(path);
        fs::create_dir_all(&child_dir).map_err(|err| Error::CreatingChildDir(child_dir, err))?;
        Ok(self)
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

    fn check_env_file_path_is_valid(&self, path: &Path) -> Result<(), Error> {
        if path == self.temp_path() {
            return Err(Error::EnvFilePathSameAsTempDir);
        }
        if self.env_files.iter().any(|f| f.path == path) {
            return Err(Error::EnvFileConflict(path.to_owned()));
        }
        Ok(())
    }

    fn check_env_var_is_valid(&self, key: &str) -> Result<(), Error> {
        if key.is_empty() {
            return Err(Error::KeyEmpty);
        }
        if self.env_vars.contains_key(key) {
            return Err(Error::KeyConflict(key.to_owned()));
        }
        Ok(())
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
///
/// ## Safety
///
/// This function should only be called in a single-threaded context.
unsafe fn reset_env(original_env: &EnvMap) {
    // remove keys if they weren't in the original environment
    env::vars()
        .filter(|(key, _)| !original_env.contains_key(key))
        .for_each(|(key, _)| env::remove_var(key));
    // ensure original keys have their original values
    original_env
        .iter()
        .for_each(|(key, value)| env::set_var(key, value));
}

/// ## Safety
///
/// This function should only be called in a single-threaded context.
unsafe fn setup_env(env_vars: &EnvMap) {
    for (key, value) in env_vars {
        env::set_var(key, value);
    }
}

/// Create an environment to run tests in.
///
/// Writes the env files, sets the working directory, and sets environment vars.
fn setup_files(testenv: &TestEnv) -> Result<(), Error> {
    env::set_current_dir(&testenv.work_dir)
        .map_err(|err| Error::SettingCurrentDir(testenv.work_dir.clone(), err))?;

    for EnvFile { path, contents } in &testenv.env_files {
        create_env_file(path, contents)?;
    }

    Ok(())
}

/// Create an env file for use in tests.
fn create_env_file(path: &Path, contents: &[u8]) -> Result<(), Error> {
    if path.exists() {
        return Err(Error::EnvFileConflict(path.to_owned()));
    }

    create_env_file_inner(path, contents)
        .map_err(|err| Error::CreatingEnvFile(path.to_owned(), err))?;

    Ok(())
}

// inner function to group together io::Results
fn create_env_file_inner(path: &Path, contents: &[u8]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(contents)?;
    file.sync_all()
}

fn canonicalize_path(path: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let path = path.as_ref();
    path.canonicalize()
        .map_err(|err| Error::CanonicalizingPath(path.to_owned(), err))
}
