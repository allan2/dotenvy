use std::path::PathBuf;

use dotenvy::Error;
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
