use super::*;

mod default_env;
mod envfile_builder;
mod testenv;

const CUSTOM_VARS: &[(&str, &str)] = &[
    ("CUSTOM_KEY_1", "CUSTOM_VALUE_1"),
    ("CUSTOM_KEY_2", "CUSTOM_VALUE_2"),
];

const DOTENV_EXPECT: &str = "TestEnv should have .env file";

fn assert_envfile_exists_in_testenv(testenv: TestEnv) {
    let envfile_path = testenv.envfile_path().to_owned();

    test_in_env(testenv, || {
        assert!(envfile_path.exists());
        assert!(dotenv_wrap().is_ok());
    });
}

fn assert_default_keys_not_set_in_testenv(testenv: TestEnv) {
    test_in_env(testenv, assert_default_keys_unset);
}
