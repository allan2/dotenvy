#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_errors_doc,
    clippy::too_many_lines,
    clippy::missing_safety_doc
)]
#![deny(clippy::uninlined_format_args)]

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
pub use crate::iter::Iter;

static START: Once = Once::new();

/// Gets the value for an environment variable.
///
/// The value is `Ok(s)` if the environment variable is present and valid unicode.
///
/// Note: this function gets values from any visible environment variable key,
/// regardless of whether a *.env* file was loaded.
///
/// # Examples:
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let value = unsafe { dotenvy::var("HOME") }?;
/// println!("{}", value);  // prints `/home/foo`
/// #     Ok(())
/// # }
/// ```
pub unsafe fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    START.call_once(|| {
        unsafe { dotenv() }.ok();
    });
    env::var(key).map_err(Error::EnvVar)
}

/// Returns an iterator of `(key, value)` pairs for all environment variables of the current process.
/// The returned iterator contains a snapshot of the process's environment variables at the time of invocation. Modifications to environment variables afterwards will not be reflected.
///
/// # Examples:
///
/// ```no_run
/// use std::io;
///
/// let result: Vec<(String, String)> = unsafe { dotenvy::vars() }.collect();
/// ```
pub unsafe fn vars() -> Vars {
    START.call_once(|| {
        unsafe { dotenv() }.ok();
    });
    env::vars()
}

/// Loads environment variables from the specified path.
///
/// If variables with the same names already exist in the environment, then their values will be
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your *.env*
/// file, the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your *.env* file, ignoring variables
/// already existing in the environment, then use [`from_path_override`] instead.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///       let path = Path::new("path/to/.env");
///       unsafe { dotenvy::from_path(path) }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn from_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let iter = Iter::new(File::open(path).map_err(Error::Io)?);
    unsafe { iter.load() }
}

/// Loads environment variables from the specified path,
/// overriding existing environment variables.
///
/// Where multiple declarations for the same environment variable exist in your *.env* file, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`from_path`] instead.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let path = Path::new("path/to/.env");
/// unsafe { dotenvy::from_path_override(path) }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn from_path_override<P: AsRef<Path>>(path: P) -> Result<()> {
    let iter = Iter::new(File::open(path).map_err(Error::Io)?);
    unsafe { iter.load_override() }
}

/// Returns an iterator over environment variables from the specified path.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// for item in dotenvy::from_path_iter(Path::new("path/to/.env"))? {
///   let (key, val) = item?;
///   println!("{}={}", key, val);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn from_path_iter<P: AsRef<Path>>(path: P) -> Result<Iter<File>> {
    Ok(Iter::new(File::open(path).map_err(Error::Io)?))
}

/// Loads environment variables from the specified file.
///
/// If variables with the same names already exist in the environment, then their values will be
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your *.env*
/// file, the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your *.env* file, ignoring variables
/// already existing in the environment, then use [`from_filename_override`] instead.
///
/// # Examples
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// unsafe { dotenvy::from_filename("custom.env") }?;
/// #     Ok(())
/// # }
/// ```
///
/// It is also possible to load from a typical *.env* file like so. However, using [`dotenv`] is preferred.
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///       unsafe { dotenvy::from_filename(".env") }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn from_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let (path, iter) = Finder::new().filename(filename.as_ref()).find()?;
    unsafe { iter.load() }?;
    Ok(path)
}

/// Loads environment variables from the specified file,
/// overriding existing environment variables.
///
/// Where multiple declarations for the same environment variable exist in your *.env* file, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`from_filename`] instead.
///
/// # Examples
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// unsafe { dotenvy::from_filename_override("custom.env") }?;
/// #     Ok(())
/// # }
/// ```
///
/// It is also possible to load from a typical *.env* file like so. However, using [`dotenv_override`] is preferred.
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// unsafe { dotenvy::from_filename_override(".env")}?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn from_filename_override<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let (path, iter) = Finder::new().filename(filename.as_ref()).find()?;
    unsafe { iter.load_override() }?;
    Ok(path)
}

///  Returns an iterator over environment variables from the specified file.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// for item in dotenvy::from_filename_iter("custom.env")? {
///     let (key, val) = item?;
///     println!("{}={}", key, val);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn from_filename_iter<P: AsRef<Path>>(filename: P) -> Result<Iter<File>> {
    let (_, iter) = Finder::new().filename(filename.as_ref()).find()?;
    Ok(iter)
}

/// Loads environment variables from [`io::Read`].
///
/// This is useful for loading environment variables from IPC or the network.
///
/// If variables with the same names already exist in the environment, then their values will be
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your `reader`,
/// the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your `reader`, ignoring variables
/// already existing in the environment, then use [`from_read_override`] instead.
///
/// For regular files, use [`from_path`] or [`from_filename`].
///
/// # Examples
///
/// ```no_run
/// use std::io::Read;
/// # #[cfg(unix)]
/// use std::os::unix::net::UnixStream;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # #[cfg(unix)]
/// let mut stream = UnixStream::connect("/some/socket")?;
/// # #[cfg(unix)]
/// unsafe { dotenvy::from_read(stream) }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn from_read<R: io::Read>(reader: R) -> Result<()> {
    let iter = Iter::new(reader);
    unsafe { iter.load() }?;
    Ok(())
}

/// Loads environment variables from [`io::Read`],
/// overriding existing environment variables.
///
/// This is useful for loading environment variables from IPC or the network.
///
/// Where multiple declarations for the same environment variable exist in your `reader`, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`from_read`] instead.
///
/// For regular files, use [`from_path_override`] or [`from_filename_override`].
///
/// # Examples
/// ```no_run
/// use std::io::Read;
/// # #[cfg(unix)]
/// use std::os::unix::net::UnixStream;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # #[cfg(unix)]
/// let mut stream = UnixStream::connect("/some/socket")?;
/// # #[cfg(unix)]
/// unsafe { dotenvy::from_read_override(stream) }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn from_read_override<R: io::Read>(reader: R) -> Result<()> {
    let iter = Iter::new(reader);
    unsafe { iter.load_override() }?;
    Ok(())
}

/// Returns an iterator over environment variables from [`io::Read`].
///
/// # Examples
///
/// ```no_run
/// use std::io::Read;
/// # #[cfg(unix)]
/// use std::os::unix::net::UnixStream;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # #[cfg(unix)]
/// let mut stream = UnixStream::connect("/some/socket")?;
///
/// # #[cfg(unix)]
/// for item in dotenvy::from_read_iter(stream) {
///     let (key, val) = item?;
///     println!("{}={}", key, val);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn from_read_iter<R: io::Read>(reader: R) -> Iter<R> {
    Iter::new(reader)
}

/// Loads the *.env* file from the current directory or parents. This is typically what you want.
///
/// If variables with the same names already exist in the environment, then their values will be
/// preserved.
///
/// Where multiple declarations for the same environment variable exist in your *.env*
/// file, the *first one* is applied.
///
/// If you wish to ensure all variables are loaded from your *.env* file, ignoring variables
/// already existing in the environment, then use [`dotenv_override`] instead.
///
/// An error will be returned if the file is not found.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///       unsafe { dotenvy::dotenv() }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn dotenv() -> Result<PathBuf> {
    let (path, iter) = Finder::new().find()?;
    unsafe { iter.load() }?;
    Ok(path)
}

/// Loads all variables found in the `reader` into the environment,
/// overriding any existing environment variables of the same name.
///
/// Where multiple declarations for the same environment variable exist in your *.env* file, the
/// *last one* is applied.
///
/// If you want the existing environment to take precedence,
/// or if you want to be able to override environment variables on the command line,
/// then use [`dotenv`] instead.
///
/// # Examples
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///       unsafe { dotenvy::dotenv_override() }?;
/// #     Ok(())
/// # }
/// ```
pub unsafe fn dotenv_override() -> Result<PathBuf> {
    let (path, iter) = Finder::new().find()?;
    unsafe { iter.load_override() }?;
    Ok(path)
}

/// Returns an iterator over environment variables.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// for item in dotenvy::dotenv_iter()? {
///     let (key, val) = item?;
///     println!("{}={}", key, val);
/// }
/// #     Ok(())
/// # }
/// ```
pub fn dotenv_iter() -> Result<iter::Iter<File>> {
    let (_, iter) = Finder::new().find()?;
    Ok(iter)
}
