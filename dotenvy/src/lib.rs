#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::missing_errors_doc,
    clippy::too_many_lines,
    clippy::missing_safety_doc
)]
#![deny(clippy::uninlined_format_args, clippy::wildcard_imports)]

//! [`dotenv`]: https://crates.io/crates/dotenv
//! A well-maintained fork of the [`dotenv`] crate.
//!
//! This library allows for loading environment variables from an env file or a reader.
use crate::iter::Iter;
use std::{
    collections::HashMap,
    env::{self, VarError},
    fs::File,
    io::{BufReader, Read},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

mod err;
mod iter;
mod parse;

/// A map of environment variables.
///
/// This is a newtype around `HashMap<String, String>` with one additional function, `var`.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct EnvMap(HashMap<String, String>);

impl Deref for EnvMap {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EnvMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<(String, String)> for EnvMap {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl IntoIterator for EnvMap {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl EnvMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn var(&self, key: &str) -> Result<String> {
        self.get(key)
            .cloned()
            .ok_or_else(|| Error::NotPresent(key.to_owned()))
    }
}

pub use crate::err::{Error, Result};

#[cfg(feature = "macros")]
pub use dotenvy_macros::*;

/// Fetches the environment variable `key` from the current process.
///
/// This is `std_env_var` but with an error type of `dotenvy::Error`.
/// `dotenvy::Error` uses `NotPresent(String)` instead of `NotPresent`, reporting the name of the missing key.
///
/// # Errors
///
/// This function will return an error if the environment variable isn't set.
///
/// This function may return an error if the environment variable's name contains
/// the equal sign character (`=`) or the NUL character.
///
/// This function will return an error if the environment variable's value is
/// not valid Unicode.
///
/// # Examples
///
/// ```
/// let key = "HOME";
/// match dotenvy::var(key) {
///     Ok(val) => println!("{key}: {val:?}"),
///     Err(e) => println!("couldn't interpret {key}: {e}"),
/// }
/// ```
pub fn var(key: &str) -> Result<String> {
    env::var(key).map_err(|e| match e {
        VarError::NotPresent => Error::NotPresent(key.to_owned()),
        VarError::NotUnicode(os_str) => Error::NotUnicode(os_str, key.to_owned()),
    })
}

/// The sequence in which to load environment variables.
///
/// Values in the latter override values in the former.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum EnvSequence {
    /// Inherit the existing environment without loading from input.
    EnvOnly,
    /// Inherit the existing environment, and then load from input, overriding existing values.
    EnvThenInput,
    /// Load from input only.
    InputOnly,
    /// Load from input and then inherit the existing environment. Values in the existing environment are not overwritten.
    #[default]
    InputThenEnv,
}

#[derive(Default)]
pub struct EnvLoader<'a> {
    path: Option<PathBuf>,
    reader: Option<Box<dyn Read + 'a>>,
    sequence: EnvSequence,
}

impl<'a> EnvLoader<'a> {
    #[must_use]
    /// Creates a new `EnvLoader` with the path set to `env` in the current directory.
    pub fn new() -> Self {
        Self::with_path(".env")
    }

    /// Creates a new `EnvLoader` with the specified path.
    ///
    /// This operation is infallible. IO is deferred until `load` or `load_and_modify` is called.
    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: Some(path.as_ref().to_owned()),
            ..Default::default()
        }
    }

    /// Creates a new `EnvLoader` with the specified reader.
    ///
    /// This operation is infallible. IO is deferred until `load` or `load_and_modify` is called.
    pub fn with_reader<R: Read + 'a>(rdr: R) -> Self {
        Self {
            reader: Some(Box::new(rdr)),
            ..Default::default()
        }
    }

    /// Sets the sequence in which to load environment variables.
    #[must_use]
    pub const fn sequence(mut self, sequence: EnvSequence) -> Self {
        self.sequence = sequence;
        self
    }

    fn buf(self) -> Result<BufReader<Box<dyn Read + 'a>>> {
        let rdr = if let Some(path) = self.path {
            Box::new(File::open(path)?)
        } else if let Some(rdr) = self.reader {
            rdr
        } else {
            // only `EnvLoader::default` would have no path or reader
            return Err(Error::NoInput);
        };
        Ok(BufReader::new(rdr))
    }

    fn load_input(self) -> Result<EnvMap> {
        let iter = Iter::new(self.buf()?);
        iter.load()
    }

    unsafe fn load_input_and_modify(self) -> Result<EnvMap> {
        let iter = Iter::new(self.buf()?);
        unsafe { iter.load_and_modify() }
    }

    unsafe fn load_input_and_modify_override(self) -> Result<EnvMap> {
        let iter = Iter::new(self.buf()?);
        unsafe { iter.load_and_modify_override() }
    }

    /// Loads environment variables into a hash map.
    ///
    /// This is the primary method for loading environment variables.
    pub fn load(self) -> Result<EnvMap> {
        match self.sequence {
            EnvSequence::EnvOnly => Ok(env::vars().collect()),
            EnvSequence::EnvThenInput => {
                let mut existing: EnvMap = env::vars().collect();
                let input = self.load_input()?;
                existing.extend(input);
                Ok(existing)
            }
            EnvSequence::InputOnly => self.load_input(),
            EnvSequence::InputThenEnv => {
                let mut input = self.load_input()?;
                input.extend(env::vars());
                Ok(input)
            }
        }
    }

    /// Loads environment variables into a hash map, modifying the existing environment.
    ///
    /// This calls `std::env::set_var` internally and is not thread-safe.
    pub unsafe fn load_and_modify(self) -> Result<EnvMap> {
        match self.sequence {
            // nothing to modify
            EnvSequence::EnvOnly => Err(Error::InvalidOp),
            // override existing env with input, returning entire env
            EnvSequence::EnvThenInput => {
                let mut existing: EnvMap = env::vars().collect();
                let input = unsafe { self.load_input_and_modify_override() }?;
                existing.extend(input);
                Ok(existing)
            }
            // override existing env with input, returning input only
            EnvSequence::InputOnly => unsafe { self.load_input_and_modify_override() },
            // load input into env, but don't override existing
            EnvSequence::InputThenEnv => {
                let existing: EnvMap = env::vars().collect();

                let mut input = unsafe { self.load_input_and_modify() }?;

                for k in input.keys() {
                    if !existing.contains_key(k) {
                        unsafe { env::set_var(k, &input[k]) };
                    }
                }
                input.extend(existing);
                Ok(input)
            }
        }
    }
}
