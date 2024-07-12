use std::env::{self, VarError};

mod envfile;
mod testenv;
mod tests;
mod wrapper;

pub use envfile::EnvFileBuilder;
pub use testenv::{test_in_default_env, test_in_env, KeyVal, TestEnv};
pub use wrapper::*;

/// Default key used in envfile
pub const TEST_KEY: &str = "TESTKEY";
/// Default value used in envfile
pub const TEST_VALUE: &str = "test_val";

/// Default existing key set before test is run
pub const TEST_EXISTING_KEY: &str = "TEST_EXISTING_KEY";
/// Default existing value set before test is run
pub const TEST_EXISTING_VALUE: &str = "from_env";
/// Default overriding value in envfile
pub const TEST_OVERRIDING_VALUE: &str = "from_file";

#[inline(always)]
pub fn create_default_envfile() -> String {
    format!(
        "{}={}\n{}={}",
        TEST_KEY, TEST_VALUE, TEST_EXISTING_KEY, TEST_OVERRIDING_VALUE
    )
}

/// missing equals
#[inline(always)]
pub fn create_invalid_envfile() -> String {
    format!(
        "{}{}\n{}{}",
        TEST_KEY, TEST_VALUE, TEST_EXISTING_KEY, TEST_OVERRIDING_VALUE
    )
}

pub fn create_custom_envfile(env_vars: &[(&str, &str)]) -> String {
    let mut efb = EnvFileBuilder::new();
    efb.add_vars(env_vars);
    efb.into_owned_string()
}

/// Assert that a slice of environment variables are set and have the expected
/// values.
///
/// ## Arguments
///
/// * `vars` - A slice of key-expected value tuples
///
/// ## Examples
///
/// ```rust
/// assert_env_vars(&[("TESTKEY", "test_val"), ("TEST_EXISTING_KEY", "from_env")]);
/// ```
pub fn assert_env_vars(vars: &[(&str, &str)]) {
    for (key, expected) in vars {
        assert_env_var(key, expected);
    }
}

/// Assert that an environment variable is set and has the expected value.
pub fn assert_env_var(key: &str, expected: &str) {
    match env::var(key) {
        Ok(actual) => assert_eq!(
            expected, actual,
            "\n\nFor Environment Variable `{}`:\n  EXPECTED: `{}`\n    ACTUAL: `{}`\n",
            key, expected, actual
        ),
        Err(VarError::NotPresent) => panic!("env var `{}` not found", key),
        Err(VarError::NotUnicode(val)) => panic!(
            "env var `{}` currently has invalid unicode: `{}`",
            key,
            val.to_string_lossy()
        ),
    }
}

/// Assert that an environment variable is not currently set.
pub fn assert_env_var_unset(key: &str) {
    match env::var(key) {
        Ok(actual) => panic!(
            "env var `{}` should not be set, currently it is: `{}`",
            key, actual
        ),
        Err(VarError::NotUnicode(val)) => panic!(
            "env var `{}` should not be set, currently has invalid unicode: `{}`",
            key,
            val.to_string_lossy()
        ),
        _ => (),
    }
}
