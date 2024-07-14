//! Wrappers for the `dotenvy` API.
//!
//! If the `dotenvy` API changes, only this module needs to be updated.

use dotenvy::{self, Iter, Result};
use std::env::Vars;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

#[inline(always)]
pub fn var_wrap<K: AsRef<OsStr>>(key: K) -> Result<String> {
    dotenvy::var(key)
}

#[inline(always)]
pub fn vars_wrap() -> Vars {
    dotenvy::vars()
}

#[inline(always)]
pub fn from_path_wrap<P: AsRef<Path>>(path: P) -> Result<()> {
    dotenvy::from_path(path)
}

#[inline(always)]
pub fn from_path_override_wrap<P: AsRef<Path>>(path: P) -> Result<()> {
    dotenvy::from_path_override(path)
}

#[inline(always)]
pub fn from_path_iter_wrap<P: AsRef<Path>>(path: P) -> Result<Iter<File>> {
    dotenvy::from_path_iter(path)
}

#[inline(always)]
pub fn from_filename_wrap<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    dotenvy::from_filename(filename)
}

#[inline(always)]
pub fn from_filename_override_wrap<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    dotenvy::from_filename_override(filename)
}

#[inline(always)]
pub fn from_filename_iter_wrap<P: AsRef<Path>>(filename: P) -> Result<Iter<File>> {
    dotenvy::from_filename_iter(filename)
}

#[inline(always)]
pub fn from_read_wrap<R: io::Read>(reader: R) -> Result<()> {
    dotenvy::from_read(reader)
}

#[inline(always)]
pub fn from_read_override_wrap<R: io::Read>(reader: R) -> Result<()> {
    dotenvy::from_read_override(reader)
}

#[inline(always)]
pub fn from_read_iter_wrap<R: io::Read>(reader: R) -> Iter<R> {
    dotenvy::from_read_iter(reader)
}

#[inline(always)]
pub fn dotenv_wrap() -> Result<PathBuf> {
    dotenvy::dotenv()
}

#[inline(always)]
pub fn dotenv_override_wrap() -> Result<PathBuf> {
    dotenvy::dotenv_override()
}

#[inline(always)]
pub fn dotenv_iter_wrap() -> Result<Iter<File>> {
    dotenvy::dotenv_iter()
}
