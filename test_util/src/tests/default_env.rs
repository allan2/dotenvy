use super::*;

#[test]
fn vars_state() {
    test_in_default_env(|| {
        assert_env_var_unset(DEFAULT_TEST_KEY);
        assert_default_existing_var();
    });
}

#[test]
fn env_file_exists() {
    let testenv = TestEnv::default();
    assert_env_files_in_testenv(&testenv);
}

#[test]
fn env_file_loaded_vars_state() {
    test_in_default_env(|| {
        dotenvy::dotenv().expect(DOTENV_EXPECT);
        // dotenv() does not override existing var
        assert_default_keys();
    });
}

#[test]
fn only_default_existing() {
    let testenv = create_testenv_with_default_var();
    let env_file_path = testenv.temp_path().join(".env");
    test_in_env(&testenv, || {
        assert_default_existing_var();
        assert!(!env_file_path.exists());
    });
}
