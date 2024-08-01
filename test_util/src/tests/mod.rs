use std::path::Path;

use super::*;

mod env_file;
mod testenv;

const TEST_KEY: &str = "TEST_KEY";
const TEST_VALUE: &str = "test_val";

const EXISTING_KEY: &str = "EXISTING_KEY";
const OVERRIDING_VALUE: &str = "loaded_from_file";

const CUSTOM_VARS: &[(&str, &str)] = &[
    ("CUSTOM_KEY_1", "CUSTOM_VALUE_1"),
    ("CUSTOM_KEY_2", "CUSTOM_VALUE_2"),
];

const DOTENV_EXPECT: &str = "TestEnv should have .env file";

fn test_env_files(testenv: &TestEnv) -> Result<(), Error> {
    let files = testenv.env_files();

    test_in_env(testenv, || {
        for EnvFile { path, contents } in files {
            assert_env_file(path, contents);
        }
    })
}

fn test_keys_not_set(testenv: &TestEnv) -> Result<(), Error> {
    test_in_env(testenv, assert_test_keys_unset)
}

fn test_env_vars(testenv: &TestEnv, vars: &[(&str, &str)]) -> Result<(), Error> {
    test_in_env(testenv, || assert_env_vars(vars))
}

fn assert_path_exists(testenv: &TestEnv, path: impl AsRef<Path>) {
    let path = testenv.temp_path().join(path.as_ref());
    assert!(path.exists(), "{} should exist in testenv", path.display());
}

fn assert_test_keys_unset() {
    assert_env_var_unset(TEST_KEY);
    assert_env_var_unset(EXISTING_KEY);
}

fn assert_env_file(path: &Path, expected: &[u8]) {
    assert!(path.exists(), "{} should exist in testenv", path.display());

    let actual = std::fs::read(path)
        .unwrap_or_else(|e| panic!("failed to read {} in testenv: {}", path.display(), e));

    assert_eq!(
        expected,
        &actual,
        "{} has incorrect contents",
        path.display()
    );
}

fn expected_env_file(env_vars: &[(&str, &str)]) -> String {
    let mut env_file = String::new();
    for (key, value) in env_vars {
        env_file.push_str(key);
        env_file.push('=');
        env_file.push_str(value);
        env_file.push('\n');
    }
    env_file
}

fn create_test_env_file() -> String {
    format!("{TEST_KEY}={TEST_VALUE}\n{EXISTING_KEY}={OVERRIDING_VALUE}")
}

fn create_custom_env_file() -> String {
    let mut env_file = String::new();
    for (key, value) in CUSTOM_VARS {
        env_file.push_str(key);
        env_file.push('=');
        env_file.push_str(value);
        env_file.push('\n');
    }
    env_file
}
