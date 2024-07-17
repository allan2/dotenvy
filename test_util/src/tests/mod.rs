use std::path::Path;

use super::*;

mod default_env;
mod envfile;
mod envfile_builder;
mod testenv;

const CUSTOM_VARS: &[(&str, &str)] = &[
    ("CUSTOM_KEY_1", "CUSTOM_VALUE_1"),
    ("CUSTOM_KEY_2", "CUSTOM_VALUE_2"),
];

const DOTENV_EXPECT: &str = "TestEnv should have .env file";

fn assert_envfiles_in_testenv(testenv: &TestEnv) {
    let files = testenv.envfiles();

    test_in_env(testenv, || {
        for EnvFile { path, contents } in files {
            assert_envfile(path, contents);
        }
    });
}

fn assert_envfile(path: &Path, expected: &[u8]) {
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

fn expected_envfile(env_vars: &[(&str, &str)]) -> String {
    let mut envfile = String::new();
    for (key, value) in env_vars {
        envfile.push_str(key);
        envfile.push('=');
        envfile.push_str(value);
        envfile.push('\n');
    }
    envfile
}
