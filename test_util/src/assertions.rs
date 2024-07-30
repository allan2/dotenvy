use super::*;
use std::env::{self, VarError};

/// Assert multiple environment variables are set and have the expected
/// values.
///
/// ## Arguments
///
/// * `vars` - A slice of `(key, expected_value)` tuples
///
/// ## Example
///
/// ```no_run
/// # use dotenvy_test_util::assert_env_vars;
/// assert_env_vars(&[
///     ("DEFAULT_TEST_KEY", "default_test_val"),
///     ("DEFAULT_EXISTING_KEY", "loaded_from_env"),
/// ]);
/// ```
pub fn assert_env_vars(vars: &[(&str, &str)]) {
    for (key, expected) in vars {
        assert_env_var(key, expected);
    }
}

/// Assert environment variable is set and has the expected value.
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

/// Assert environment variable is not currently set.
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

/// Assert default testing environment variables are not set.
pub fn assert_default_keys_unset() {
    assert_env_var_unset(DEFAULT_EXISTING_KEY);
    assert_env_var_unset(DEFAULT_TEST_KEY);
}

/// Assert default testing environment variables are set.
/// Assuming the default envfile is loaded.
pub fn assert_default_keys() {
    assert_env_var(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
    assert_default_existing_var();
}

/// Assert default existing environment variable is set.
pub fn assert_default_existing_var() {
    assert_env_var(DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE);
}
