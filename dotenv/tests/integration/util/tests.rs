use super::*;

const CUSTOM_VARS: &[(&str, &str)] = &[
    ("CUSTOM_KEY_1", "CUSTOM_VALUE_1"),
    ("CUSTOM_KEY_2", "CUSTOM_VALUE_2"),
];

const DOTENV_EXPECT: &str = "TestEnv should have .env file";

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
            dotenv_wrap().expect(DOTENV_EXPECT);
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
            assert!(dotenv_wrap().is_err());
        });
    }
}

mod testenv_init_with_envfile {
    use super::*;

    #[test]
    fn default_envfile_vars_state() {
        let testenv = init_default_envfile_testenv();
        assert_default_keys_not_set_in_testenv(testenv);
    }

    #[test]
    fn default_envfile_exists() {
        let testenv = init_default_envfile_testenv();
        assert_envfile_exists_in_testenv(testenv);
    }

    #[test]
    fn default_envfile_loaded_vars_state() {
        let testenv = init_default_envfile_testenv();
        test_in_env(testenv, || {
            dotenv_wrap().expect(DOTENV_EXPECT);
            // dotenv() does not override existing var
            // but existing key is not set in this testenv
            assert_env_var(TEST_EXISTING_KEY, TEST_OVERRIDING_VALUE);
            assert_env_var(TEST_KEY, TEST_VALUE);
        });
    }

    #[test]
    fn custom_envfile_vars_state() {
        let testenv = init_custom_envfile_testenv();
        test_in_env(testenv, || {
            assert_default_keys_not_set();
            for (key, _) in CUSTOM_VARS {
                assert_env_var_unset(key);
            }
        });
    }

    #[test]
    fn custom_envfile_exists() {
        let testenv = init_custom_envfile_testenv();
        assert_envfile_exists_in_testenv(testenv);
    }

    #[test]
    fn custom_envfile_loaded_vars_state() {
        let testenv = init_custom_envfile_testenv();
        test_in_env(testenv, || {
            dotenv_wrap().expect(DOTENV_EXPECT);
            assert_default_keys_not_set();
            assert_env_vars(CUSTOM_VARS);
        });
    }

    #[test]
    fn empty_envfile_exists() {
        let testenv = init_empty_envfile_testenv();
        assert_envfile_exists_in_testenv(testenv);
    }

    #[test]
    fn empty_envfile_loaded_vars_state() {
        let testenv = init_empty_envfile_testenv();
        test_in_env(testenv, || {
            dotenv_wrap().expect(DOTENV_EXPECT);
            assert_default_keys_not_set();
        });
    }

    fn init_default_envfile_testenv() -> TestEnv {
        let envfile = create_default_envfile();
        TestEnv::init_with_envfile(envfile)
    }

    fn init_custom_envfile_testenv() -> TestEnv {
        let envfile = create_custom_envfile(CUSTOM_VARS);
        TestEnv::init_with_envfile(envfile)
    }

    fn init_empty_envfile_testenv() -> TestEnv {
        TestEnv::init_with_envfile([])
    }
}

fn assert_envfile_exists_in_testenv(testenv: TestEnv) {
    let envfile_path = testenv.envfile_path().to_owned();

    test_in_env(testenv, || {
        assert!(envfile_path.exists());
        assert!(dotenv_wrap().is_ok());
    });
}

fn assert_default_keys_not_set_in_testenv(testenv: TestEnv) {
    test_in_env(testenv, assert_default_keys_not_set);
}

fn assert_default_keys_not_set() {
    assert_env_var_unset(TEST_EXISTING_KEY);
    assert_env_var_unset(TEST_KEY);
}
