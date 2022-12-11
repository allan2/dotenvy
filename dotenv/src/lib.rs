//! [`dotenv`]: https://crates.io/crates/dotenv
//! A well-maintained fork of the [`dotenv`] crate
//!
//! This library loads environment variables from a *.env* file. This is convenient for dev environments.

mod errors;
mod find;
mod iter;
mod parse;

use std::env::{self, Vars};
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Once;

pub use crate::errors::*;
use crate::find::Finder;
use crate::iter::Iter;

static START: Once = Once::new();

/// Gets the value for an environment variable.
///
/// # Discovery
///
/// Environment variables are loaded from the *first .env file*
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then load environment variables with [`from_path`] instead, then invoke [`env::var`] manually.
///
/// # Sources
///
/// This function returns values from any environment variable key present in the process,
/// even if no *.env* file is discovered, the discovered *.env* file is not readable, or it contains invalid declarations.
///
/// # Repeated calls
///
/// *.env* auto-discovery and loading into the process environment is performed at the first call to [`var`] or [`vars`].
///
/// Any repeated calls skip auto-discovery and return variables as found in the environment.
///
/// # Errors
///
/// An [`Error::EnvVar`] is returned if the environment variable is missing or not valid unicode.
///
/// Errors while attempting to auto-discover and load an *.env* file are silenced.
///
/// If you need explicit error handling, then consider first loading enironment variables
/// with [`from_filename`] or [`from_path`], then manually invoking [`env::var`] instead.
///
/// # Examples:
///
/// ```no_run
/// let value = dotenvy::var("HOME").unwrap();
/// println!("{}", value);  // prints `/home/foo`
/// ```
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    START.call_once(|| {
        dotenv().ok();
    });
    env::var(key).map_err(Error::EnvVar)
}

/// Returns an iterator of `(key, value)` pairs for all environment variables of the current process.
/// The returned iterator contains a snapshot of the process's environment variables at the time of invocation.
/// Modifications to environment variables afterwards are not reflected.
///
/// # Discovery
///
/// Environment variables are loaded from the *first .env file*
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then load environment variables with [`from_path`] instead, then invoke [`env::vars`] manually.
///
/// # Sources
///
/// This function returns values from any environment variable key present in the process,
/// even if no *.env* file is discovered, the discovered *.env* file is not readable, or it contains invalid declarations.
///
/// # Repeated calls
///
/// *.env* auto-discovery and loading into the process environment is performed at the first call to [`var`] or [`vars`].
///
/// Any repeated calls skip auto-discovery and return variables as found in the environment.
///
/// # Errors
///
/// Errors while attempting to auto-discover and load an *.env* file are silenced.
///
/// If you need explicit error handling, then consider first loading enironment variables
/// with [`from_filename`] or [`from_path`], then manually invoking [`env::vars`] instead.
///
/// # Examples:
///
/// ```no_run
/// use std::io;
///
/// let result: Vec<(String, String)> = dotenvy::vars().collect();
/// ```
pub fn vars() -> Vars {
    START.call_once(|| {
        dotenv().ok();
    });
    env::vars()
}

/// Loads environment variables from the specified file.
///
/// # Discovery
///
/// Environment variables are loaded *exclusively* from the file specified as `path`.
///
/// If you prefer `dotenvy` to auto-discover your *.env* file in parent directories,
/// then use [`from_filename`] instead.
///
/// # Overriding behavior
///
/// If variables with the same names already exist in the environment, then their values are
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your *.env*
/// file, the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your *.env* file, ignoring variables
/// already existing in the environment, then use [`from_path_override`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if the specified file is not found or not readable.
///
/// An [`Error::LineParse`] is returned if the discovered environment file contains invalid declarations.
///
/// # Examples
///
/// ```no_run
/// use dirs::home_dir;
///
/// let my_path = home_dir().map(|a| a.join("/absolute/path/.env")).unwrap();
/// dotenvy::from_path(my_path.as_path());
/// ```
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let iter = Iter::new(File::open(path).map_err(Error::Io)?);
    iter.load()?;
    Ok(())
}

/// Loads environment variables from the specified file,
/// overriding existing environment variables.
///
/// # Discovery
///
/// Environment variables are loaded *exclusively* from the file specified as `path`.
///
/// If you prefer `dotenvy` to auto-discover your *.env* file in parent directories,
/// then use [`from_filename_override`] instead.
///
/// # Overriding behavior
///
/// Where multiple declarations for the same environment variable exist in your *.env* file, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`from_path`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if the specified file is not found or not readable.
///
/// An [`Error::LineParse`] is returned if the discovered environment file contains invalid declarations.
///
/// # Examples
///
/// ```no_run
/// use dirs::home_dir;
///
/// let my_path = home_dir().map(|a| a.join("/absolute/path/.env")).unwrap();
/// dotenvy::from_path_override(my_path.as_path());
/// ```
pub fn from_path_override<P: AsRef<Path>>(path: P) -> Result<()> {
    let iter = Iter::new(File::open(path).map_err(Error::Io)?);
    iter.load_override()?;
    Ok(())
}

/// Returns an iterator over environment variables from the specified file.
///
/// # Discovery
///
/// Environment variables are loaded *exclusively* from the file specified as `path`.
///
/// If you prefer `dotenvy` to auto-discover your *.env* file in parent directories,
/// then use [`from_filename_iter`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if the specified file is not found or not readable.
///
/// # Examples
///
/// ```no_run
/// use dirs::home_dir;
///
/// let my_path = home_dir().map(|a| a.join("/absolute/path/.env")).unwrap();
///
/// for item in dotenvy::from_path_iter(my_path.as_path()).unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn from_path_iter<P: AsRef<Path>>(path: P) -> Result<Iter<File>> {
    Ok(Iter::new(File::open(path).map_err(Error::Io)?))
}

/// Loads environment variables from the first discovered file matching the specified name.
///
/// # Discovery
///
/// Environment variables are loaded from the *first file* matching `filename`
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then use [`from_path`] instead.
///
/// # Overriding behavior
///
/// If variables with the same names already exist in the environment, then their values are
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your *.env*
/// file, the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your *.env* file, ignoring variables
/// already existing in the environment, then use [`from_filename_override`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if auto-discovery fails, or the discovered file is not readable.
///
/// An [`Error::LineParse`] is returned if the discovered environment file contains invalid declarations.
///
/// # Examples
///
/// ```no_run
/// dotenvy::from_filename("custom.env").unwrap();
/// ```
///
/// It is also possible to load from a typical *.env* file like so. However, using [`dotenv`] is preferred.
///
/// ```
/// dotenvy::from_filename(".env").unwrap();
/// ```
pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let (path, iter) = Finder::new().filename(filename.as_ref()).find()?;
    iter.load()?;
    Ok(path)
}

/// Loads environment variables from the first discovered file matching the specified name,
/// overriding existing environment variables.
///
/// # Discovery
///
/// Environment variables are loaded from the *first file* matching `filename`
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then use [`from_path_override`] instead.
///
/// # Overriding behavior
///
/// Where multiple declarations for the same environment variable exist in your *.env* file, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`from_filename`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if auto-discovery fails, or the discovered file is not readable.
///
/// An [`Error::LineParse`] is returned if the discovered environment file contains invalid declarations.
///
/// # Examples
///
/// ```no_run
/// dotenvy::from_filename_override("custom.env").unwrap();
/// ```
///
/// It is also possible to load from a typical *.env* file like so. However, using [`dotenv_override`] is preferred.
///
/// ```
/// dotenvy::from_filename_override(".env").unwrap();
/// ```
pub fn from_filename_override<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let (path, iter) = Finder::new().filename(filename.as_ref()).find()?;
    iter.load_override()?;
    Ok(path)
}

/// Returns an iterator over environment variables from the first discovered file
/// matching the specified name.
///
/// # Discovery
///
/// Environment variables are loaded from the *first file* matching `filename`
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then use [`from_path_iter`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if auto-discovery fails, or the discovered file is not readable.
///
/// # Examples
///
/// ```no_run
/// for item in dotenvy::from_filename_iter("custom.env").unwrap() {
///     let (key, val) = item.unwrap();
///     println!("{}={}", key, val);
/// }
/// ```

pub fn from_filename_iter<P: AsRef<Path>>(filename: P) -> Result<Iter<File>> {
    let (_, iter) = Finder::new().filename(filename.as_ref()).find()?;
    Ok(iter)
}

/// Loads environment variables from [`io::Read`](std::io::Read).
///
/// This is useful for loading environment variables from from IPC or the network.
///
/// # Overriding behavior
///
/// If variables with the same names already exist in the environment, then their values are
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your `reader`,
/// the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your `reader`, ignoring variables
/// already existing in the environment, then use [`from_read_override`] instead.
///
/// # Regular *.env* files
///
/// For regular files, use [`from_path`] or [`from_filename`].
///
/// # Errors
///
/// An [`Error::Io`] is returned if the provided `reader` is not readable.
///
/// An [`Error::LineParse`] is returned if the provided `reader` produces invalid declarations.
///
/// # Examples
///
/// ```no_run
/// # #![cfg(unix)]
/// use std::io::Read;
/// use std::os::unix::net::UnixStream;
///
/// let mut stream = UnixStream::connect("/some/socket").unwrap();
/// dotenvy::from_read(stream).unwrap();
/// ```
pub fn from_read<R: io::Read>(reader: R) -> Result<()> {
    let iter = Iter::new(reader);
    iter.load()?;
    Ok(())
}

/// Loads environment variables from [`io::Read`](std::io::Read),
/// overriding existing environment variables.
///
/// This is useful for loading environment variables from from IPC or the network.
///
/// # Overriding behavior
///
/// Where multiple declarations for the same environment variable exist in your `reader`, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`from_read`] instead.
///
/// # Regular *.env* files
///
/// For regular files, use [`from_path_override`] or [`from_filename_override`].
///
/// # Errors
///
/// An [`Error::Io`] is returned if the provided `reader` is not readable.
///
/// An [`Error::LineParse`] is returned if the provided `reader` produces invalid declarations.
///
/// # Examples
///
/// ```no_run
/// # #![cfg(unix)]
/// use std::io::Read;
/// use std::os::unix::net::UnixStream;
///
/// let mut stream = UnixStream::connect("/some/socket").unwrap();
/// dotenvy::from_read_override(stream).unwrap();
/// ```
pub fn from_read_override<R: io::Read>(reader: R) -> Result<()> {
    let iter = Iter::new(reader);
    iter.load_override()?;
    Ok(())
}

/// Returns an iterator over environment variables from [`io::Read`](std::io::Read).
///
/// # Regular *.env* files
///
/// For regular files, use [`from_path_iter`] or [`from_filename_iter`].
///
/// # Examples
///
/// ```no_run
/// # #![cfg(unix)]
/// use std::io::Read;
/// use std::os::unix::net::UnixStream;
///
/// let mut stream = UnixStream::connect("/some/socket").unwrap();
///
/// for item in dotenvy::from_read_iter(stream) {
///     let (key, val) = item.unwrap();
///     println!("{}={}", key, val);
/// }
/// ```
pub fn from_read_iter<R: io::Read>(reader: R) -> Iter<R> {
    Iter::new(reader)
}

/// Loads the *.env* file from the current or any parent directory. This is typically what you want.
///
/// # Discovery
///
/// Environment variables are loaded from the *first .env file*
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then use [`from_path`] instead.
///
/// # Overriding behavior
///
/// If variables with the same names already exist in the environment, then their values are
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your *.env*
/// file, the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your *.env* file, ignoring variables
/// already existing in the environment, then use [`dotenv_override`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if *.env* auto-discovery fails, or the discovered *.env* file is not readable.
///
/// An [`Error::LineParse`] is returned if the discovered *.env* file contains invalid declarations.
///
/// # Examples
///
/// ```
/// dotenvy::dotenv().unwrap();
/// ```
pub fn dotenv() -> Result<PathBuf> {
    let (path, iter) = Finder::new().find()?;
    iter.load()?;
    Ok(path)
}

/// Loads the *.env* file from the current or any parent directory,
/// overriding any existing environment variables of the same name.
///
/// # Discovery
///
/// Environment variables are loaded from the *first .env file*
/// found in the *current directory* or any *parent directories*.
///
/// If you prefer `dotenvy` to load environment variables exclusively from a specific file,
/// then use [`from_path_override`] instead.
///
/// # Overriding behavior
///
/// Where multiple declarations for the same environment variable exist in your *.env* file, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`dotenv`] instead.
///
/// # Errors
///
/// An [`Error::Io`] is returned if *.env* auto-discovery fails, or the discovered *.env* file is not readable.
///
/// An [`Error::LineParse`] is returned if the discovered *.env* file contains invalid declarations.
///
/// # Examples
///
/// ```
/// use dotenvy::dotenv_override;
/// dotenv_override().ok();
/// ```
pub fn dotenv_override() -> Result<PathBuf> {
    let (path, iter) = Finder::new().find()?;
    iter.load_override()?;
    Ok(path)
}

/// Returns an iterator over environment variables.
///
/// # Errors
///
/// An [`Error::Io`] is returned if *.env* auto-discovery fails, or the discovered *.env* file is not readable.
///
/// # Examples
///
/// ```
/// for item in dotenvy::dotenv_iter().unwrap() {
///     let (key, val) = item.unwrap();
///     println!("{}={}", key, val);
/// }
/// ```
pub fn dotenv_iter() -> Result<iter::Iter<File>> {
    let (_, iter) = Finder::new().find()?;
    Ok(iter)
}
