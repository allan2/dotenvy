use std::path::Path;

use super::*;

mod default_env;
mod env_file;
mod env_file_builder;
mod testenv;

const CUSTOM_VARS: &[(&str, &str)] = &[
    ("CUSTOM_KEY_1", "CUSTOM_VALUE_1"),
    ("CUSTOM_KEY_2", "CUSTOM_VALUE_2"),
];

const DOTENV_EXPECT: &str = "TestEnv should have .env file";

fn assert_env_files_in_testenv(testenv: &TestEnv) {
    let files = testenv.env_files();

    test_in_env(testenv, || {
        for EnvFile { path, contents } in files {
            assert_env_file(path, contents);
        }
    });
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

fn assert_default_keys_not_set_in_testenv(testenv: &TestEnv) {
    test_in_env(testenv, assert_default_keys_unset);
}

fn assert_env_vars_in_testenv(testenv: &TestEnv, vars: &[(&str, &str)]) {
    test_in_env(testenv, || assert_env_vars(vars));
}

fn assert_path_exists_in_testenv(testenv: &TestEnv, path: impl AsRef<Path>) {
    let path = testenv.temp_path().join(path.as_ref());
    assert!(path.exists(), "{} should exist in testenv", path.display());
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
