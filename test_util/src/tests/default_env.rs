use super::*;

#[test]
fn vars_state() {
    test_in_default_env(|| {
        assert_env_var_unset(DEFAULT_TEST_KEY);
        assert_default_existing_var();
    });
}

#[test]
fn envfile_exists() {
    let testenv = TestEnv::default();
    assert_envfiles_in_testenv(&testenv);
}

#[test]
fn envfile_loaded_vars_state() {
    test_in_default_env(|| {
        dotenvy::dotenv().expect(DOTENV_EXPECT);
        // dotenv() does not override existing var
        assert_default_keys();
    });
}

#[test]
fn only_default_existing() {
    let testenv = create_testenv_with_default_var();
    let envfile_path = testenv.temp_path().join(".env");
    test_in_env(&testenv, || {
        assert_default_existing_var();
        assert!(!envfile_path.exists());
    });
}
