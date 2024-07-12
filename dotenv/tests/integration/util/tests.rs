use super::*;

mod default_env {
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
            dotenvy::dotenv().expect("Default TestEnv should have .env file");
            // dotenv() does not override existing var
            assert_env_var(TEST_EXISTING_KEY, TEST_EXISTING_VALUE);
            assert_env_var(TEST_KEY, TEST_VALUE);
        });
    }
}

mod testenv_init {
    use super::*;

    #[test]
    fn vars_state() {
        let init_testenv = TestEnv::init();
        assert_default_keys_not_set_in_testenv(init_testenv);
    }

    #[test]
    fn no_envfile() {
        let init_testenv = TestEnv::init();
        let envfile_path = init_testenv.envfile_path().to_owned();

        test_in_env(init_testenv, || {
            assert!(!envfile_path.exists());
            assert!(dotenvy::dotenv().is_err());
        });
    }
}

mod testenv_init_with_envfile {
    use super::*;

    #[test]
    fn default_envfile_vars_state() {
        let testenv = TestEnv::init_with_envfile(create_default_envfile());
        assert_default_keys_not_set_in_testenv(testenv);
    }

    #[test]
    fn default_envfile_exists() {
        let testenv = TestEnv::init_with_envfile(create_default_envfile());
        assert_envfile_exists_in_testenv(testenv);
    }

    #[test]
    fn default_envfile_loaded_vars_state() {
        let init_testenv = TestEnv::init_with_envfile(create_default_envfile());
        test_in_env(init_testenv, || {
            dotenvy::dotenv().expect("Default TestEnv should have .env file");
            // dotenv() does not override existing var
            // but existing key is not set in this testenv
            assert_env_var(TEST_EXISTING_KEY, TEST_OVERRIDING_VALUE);
            assert_env_var(TEST_KEY, TEST_VALUE);
        });
    }
}

fn assert_envfile_exists_in_testenv(testenv: TestEnv) {
    let envfile_path = testenv.envfile_path().to_owned();

    test_in_env(testenv, || {
        assert!(envfile_path.exists());
        assert!(dotenvy::dotenv().is_ok());
    });
}

fn assert_default_keys_not_set_in_testenv(testenv: TestEnv) {
    test_in_env(testenv, || {
        assert_env_var_unset(TEST_EXISTING_KEY);
        assert_env_var_unset(TEST_KEY);
    });
}
