//! This crate provides a configuration loader in the style of the [ruby dotenv
//! gem](https://github.com/bkeepers/dotenv). This library is meant to be used
//! on development or testing environments in which setting environment
//! variables is not practical. It loads environment variables from a .env
//! file, if available, and mashes those with the actual environment variables
//! provided by the operating system.

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
extern crate regex;

mod parse;
mod errors;
mod iter;
mod find;

use std::env::{self, Vars};
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Once, ONCE_INIT};

pub use errors::*;

static START: Once = ONCE_INIT;

/// After loading the dotenv file, fetches the environment variable key from the current process.
///
/// The returned result is Ok(s) if the environment variable is present and is valid unicode. If the
/// environment variable is not present, or it is not valid unicode, then Err will be returned.
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    START.call_once(|| { dotenv().ok(); });
    env::var(key).map_err(Error::from)
}

/// After loading the dotenv file, returns an iterator of (variable, value) pairs of strings,
/// for all the environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment variables at the
/// time of this invocation, modifications to environment variables afterwards will not be
/// reflected in the returned iterator.
pub fn vars() -> Vars {
    START.call_once(|| { dotenv().ok(); });
    env::vars()
}

/// Loads the file at the specified absolute path.
///
/// Examples
///
/// ```
/// use dotenv;
/// use std::env;
/// use std::path::{Path};
///
/// let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
/// dotenv::from_path(my_path.as_path());
/// ```
pub fn from_path(path: &Path) -> Result<()> {
    iter::Iter::new(File::open(path)?).load()
}

/// Loads the specified file from the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv;
/// dotenv::from_filename("custom.env").ok();
/// ```
///
/// It is also possible to do the following, but it is equivalent to using dotenv::dotenv(), which
/// is preferred.
///
/// ```
/// use dotenv;
/// dotenv::from_filename(".env").ok();
/// ```
pub fn from_filename(filename: &str) -> Result<PathBuf> {
    let directory = env::current_dir()?;

    let path = find::find(directory, filename)?;

    from_path(&path)?;

    Ok(path)
}

/// This is usually what you want.
/// It loads the .env file located in the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv;
/// dotenv::dotenv().ok();
/// ```
pub fn dotenv() -> Result<PathBuf> {
    from_filename(&".env")
}


/// Like `dotenv`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
/// ```no_run
/// use dotenv;
///
/// for item in dotenv::dotenv_iter().unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn dotenv_iter() -> Result<iter::Iter<File>> {
    let directory = env::current_dir()?;

    let path = find::find(directory, ".env")?;

    Ok(iter::Iter::new(File::open(path)?))
}
