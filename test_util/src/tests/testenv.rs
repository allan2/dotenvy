use super::*;

mod init {
    use super::*;

    #[test]
    fn vars_state() {
        let init_testenv = TestEnv::init();
        assert_default_keys_not_set_in_testenv(init_testenv);
    }

    #[test]
    fn no_envfile() {
        let init_testenv = TestEnv::init();
        let envfile_path = init_testenv.temp_path().join(".env");

        test_in_env(init_testenv, || {
            assert!(!envfile_path.exists());
            assert!(wrap::dotenv().is_err());
        });
    }
}

mod init_with_envfile {
    use super::*;

    #[test]
    fn default_envfile_vars_state() {
        let testenv = init_default_envfile_testenv();
        assert_default_keys_not_set_in_testenv(testenv);
    }

    #[test]
    fn default_envfile_exists() {
        let testenv = init_default_envfile_testenv();
        assert_envfiles_exist_in_testenv(testenv);
    }

    #[test]
    fn default_envfile_loaded_vars_state() {
        let testenv = init_default_envfile_testenv();
        test_in_env(testenv, || {
            wrap::dotenv().expect(DOTENV_EXPECT);
            // dotenv() does not override existing var
            // but existing key is not set in this testenv
            assert_env_var(DEFAULT_EXISTING_KEY, DEFAULT_OVERRIDING_VALUE);
            assert_env_var(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
        });
    }

    #[test]
    fn custom_envfile_vars_state() {
        let testenv = init_custom_envfile_testenv();
        test_in_env(testenv, || {
            assert_default_keys_unset();
            for (key, _) in CUSTOM_VARS {
                assert_env_var_unset(key);
            }
        });
    }

    #[test]
    fn custom_envfile_exists() {
        let testenv = init_custom_envfile_testenv();
        assert_envfiles_exist_in_testenv(testenv);
    }

    #[test]
    fn custom_envfile_loaded_vars_state() {
        let testenv = init_custom_envfile_testenv();
        test_in_env(testenv, || {
            wrap::dotenv().expect(DOTENV_EXPECT);
            assert_default_keys_unset();
            assert_env_vars(CUSTOM_VARS);
        });
    }

    #[test]
    fn empty_envfile_exists() {
        let testenv = init_empty_envfile_testenv();
        assert_envfiles_exist_in_testenv(testenv);
    }

    #[test]
    fn empty_envfile_loaded_vars_state() {
        let testenv = init_empty_envfile_testenv();
        test_in_env(testenv, || {
            wrap::dotenv().expect(DOTENV_EXPECT);
            assert_default_keys_unset();
        });
    }

    #[test]
    fn custom_bom_envfile_exists() {
        let testenv = init_custom_bom_envfile_testenv();
        assert_envfiles_exist_in_testenv(testenv);
    }

    #[test]
    fn custom_bom_envfile_loaded_vars_state() {
        let testenv = init_custom_bom_envfile_testenv();
        test_in_env(testenv, || {
            wrap::dotenv().expect(DOTENV_EXPECT);
            assert_env_var(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
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

    fn init_custom_bom_envfile_testenv() -> TestEnv {
        let mut efb = EnvFileBuilder::new();
        efb.add_key_value(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
        efb.insert_utf8_bom();
        let envfile = efb.into_owned_string();
        TestEnv::init_with_envfile(envfile)
    }
}
