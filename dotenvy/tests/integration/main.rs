use std::env::{self, VarError};

mod testenv;

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
    format!("{TEST_KEY}={TEST_VALUE}\n{TEST_EXISTING_KEY}={TEST_OVERRIDING_VALUE}",)
}

/// missing equals
#[inline(always)]
pub fn create_invalid_envfile() -> String {
    format!("{TEST_KEY}{TEST_VALUE}\n{TEST_EXISTING_KEY}{TEST_OVERRIDING_VALUE}",)
}

/// Assert that an environment variable is set and has the expected value.
pub fn assert_env_var(key: &str, expected: &str) {
    match env::var(key) {
        Ok(actual) => assert_eq!(
            expected, actual,
            "\n\nFor Environment Variable `{key}`:\n  EXPECTED: `{expected}`\n    ACTUAL: `{actual}`\n",
        ),
        Err(VarError::NotPresent) => panic!("env var `{key}` not found"),
        Err(VarError::NotUnicode(val)) => panic!(
            "env var `{key}` currently has invalid unicode: `{}`",
            val.to_string_lossy()
        ),
    }
}

/// Assert that an environment variable is not currently set.
pub fn assert_env_var_unset(key: &str) {
    match env::var(key) {
        Ok(actual) => panic!("env var `{key}` should not be set, currently it is: `{actual}`",),
        Err(VarError::NotUnicode(val)) => panic!(
            "env var `{key}` should not be set, currently has invalid unicode: `{}`",
            val.to_string_lossy()
        ),
        _ => (),
    }
}
