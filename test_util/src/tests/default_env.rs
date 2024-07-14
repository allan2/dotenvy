use super::*;

#[test]
fn vars_state() {
    test_in_default_env(|| {
        assert_env_var(DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE);
        assert_env_var_unset(DEFAULT_TEST_KEY);
    });
}

#[test]
fn envfile_exists() {
    let testenv = TestEnv::default();
    assert_envfile_exists_in_testenv(testenv);
}

#[test]
fn envfile_loaded_vars_state() {
    test_in_default_env(|| {
        dotenv_wrap().expect(DOTENV_EXPECT);
        // dotenv() does not override existing var
        assert_env_var(DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE);
        assert_env_var(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
    });
}
