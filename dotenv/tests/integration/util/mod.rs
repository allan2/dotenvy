use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use dotenvy::{Error, Iter, Result};
use dotenvy_test_util::*;

/// common assertions
mod assert;
/// particular tests in a testenv
mod test;

pub use assert::*;
pub use test::*;

pub const KEY_1: &str = "FOOO";
pub const VAL_1: &str = "bar";
pub const KEYVAL_1: &str = "FOOO=bar";

pub const KEY_2: &str = "BARR";
pub const VAL_2: &str = "foo";
pub const KEYVAL_2: &str = "BARR=foo";

/// Call and unwrap the default api function
pub fn api_fn() {
    dotenvy::dotenv().unwrap();
}

/// Call, unwrap and return the `PathBuf` of the default api function
pub fn api_fn_path() -> PathBuf {
    dotenvy::dotenv().unwrap()
}

/// Call, unwrap and return the `Error` of the default api function
pub fn api_fn_err() -> Error {
    dotenvy::dotenv().unwrap_err()
}

pub fn check_iter_default_envfile_into_hash_map<F>(iter_fn: F)
where
    F: FnOnce() -> Result<Iter<File>>,
{
    let vars = [("FOOO", "bar"), ("BAZ", "qux")];
    let envfile = create_custom_envfile(&vars);
    let testenv = TestEnv::init_with_envfile(envfile);

    test_in_env(&testenv, || {
        let map: HashMap<String, String> = iter_fn()
            .expect("valid file")
            .map(|item| item.expect("valid item"))
            .collect();

        for (key, expected) in vars {
            let actual = map.get(key).expect("valid key");
            assert_eq!(expected, actual);
        }
    });
}

/// Relative to the temp dir
pub fn canonicalize_envfile_path(testenv: &TestEnv, envfile: impl AsRef<Path>) -> PathBuf {
    testenv
        .temp_path()
        .join(envfile.as_ref())
        .canonicalize()
        .expect("canonicalize envfile")
}
