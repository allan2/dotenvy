//! Wrappers for the `dotenvy` API.
//!
//! If the `dotenvy` API changes, only this module needs to be updated.
//!
//! ## Example
//!
//! ```no_run
//! use dotenvy_test_util::wrap;
//! wrap::dotenv().expect("Failed to load .env file");
//! ```

use dotenvy::{self, Iter, Result};
use std::env::Vars;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[inline(always)]
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    dotenvy::var(key)
}

#[inline(always)]
pub fn vars() -> Vars {
    dotenvy::vars()
}

#[inline(always)]
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<()> {
    dotenvy::from_path(path)
}

#[inline(always)]
pub fn from_path_override<P: AsRef<Path>>(path: P) -> Result<()> {
    dotenvy::from_path_override(path)
}

#[inline(always)]
pub fn from_path_iter<P: AsRef<Path>>(path: P) -> Result<Iter<File>> {
    dotenvy::from_path_iter(path)
}

#[inline(always)]
pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    dotenvy::from_filename(filename)
}

#[inline(always)]
pub fn from_filename_override<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    dotenvy::from_filename_override(filename)
}

#[inline(always)]
pub fn from_filename_iter<P: AsRef<Path>>(filename: P) -> Result<Iter<File>> {
    dotenvy::from_filename_iter(filename)
}

#[inline(always)]
pub fn from_read<R: io::Read>(reader: R) -> Result<()> {
    dotenvy::from_read(reader)
}

#[inline(always)]
pub fn from_read_override<R: io::Read>(reader: R) -> Result<()> {
    dotenvy::from_read_override(reader)
}

#[inline(always)]
pub fn from_read_iter<R: io::Read>(reader: R) -> Iter<R> {
    dotenvy::from_read_iter(reader)
}

#[inline(always)]
pub fn dotenv() -> Result<PathBuf> {
    dotenvy::dotenv()
}

#[inline(always)]
pub fn dotenv_override() -> Result<PathBuf> {
    dotenvy::dotenv_override()
}

#[inline(always)]
pub fn dotenv_iter() -> Result<Iter<File>> {
    dotenvy::dotenv_iter()
}
