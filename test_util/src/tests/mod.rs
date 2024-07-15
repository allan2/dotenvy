use super::*;

mod default_env;
mod envfile_builder;
mod testenv;

const CUSTOM_VARS: &[(&str, &str)] = &[
    ("CUSTOM_KEY_1", "CUSTOM_VALUE_1"),
    ("CUSTOM_KEY_2", "CUSTOM_VALUE_2"),
];

const DOTENV_EXPECT: &str = "TestEnv should have .env file";

fn assert_envfiles_exist_in_testenv(testenv: TestEnv) {
    let paths = testenv
        .envfiles()
        .iter()
        .map(|envfile| envfile.path.clone())
        .collect::<Vec<_>>();

    test_in_env(testenv, || {
        for path in paths {
            assert!(path.exists());
        }
    });
}

fn assert_default_keys_not_set_in_testenv(testenv: TestEnv) {
    test_in_env(testenv, assert_default_keys_unset);
}
