use super::*;

#[test]
fn vars_state() {
    test_in_default_env(|| {
        assert_env_var(TEST_EXISTING_KEY, TEST_EXISTING_VALUE);
        assert_env_var_unset(TEST_KEY);
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
        assert_env_var(TEST_EXISTING_KEY, TEST_EXISTING_VALUE);
        assert_env_var(TEST_KEY, TEST_VALUE);
    });
}
