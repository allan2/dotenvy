use super::*;

mod init {
    use super::*;

    #[test]
    fn vars_state() {
        let init_testenv = TestEnv::init();
        assert_default_keys_not_set_in_testenv(&init_testenv);
    }

    #[test]
    fn no_envfile() {
        let init_testenv = TestEnv::init();
        let envfile_path = init_testenv.temp_path().join(".env");

        test_in_env(&init_testenv, || {
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
        assert_default_keys_not_set_in_testenv(&testenv);
    }

    #[test]
    fn default_envfile_exists() {
        let testenv = init_default_envfile_testenv();
        assert_envfiles_in_testenv(&testenv);
    }

    #[test]
    fn default_envfile_loaded_vars_state() {
        let testenv = init_default_envfile_testenv();
        test_in_env(&testenv, || {
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
        test_in_env(&testenv, || {
            assert_default_keys_unset();
            for (key, _) in CUSTOM_VARS {
                assert_env_var_unset(key);
            }
        });
    }

    #[test]
    fn custom_envfile_exists() {
        let testenv = init_custom_envfile_testenv();
        assert_envfiles_in_testenv(&testenv);
    }

    #[test]
    fn custom_envfile_loaded_vars_state() {
        let testenv = init_custom_envfile_testenv();
        test_in_env(&testenv, || {
            wrap::dotenv().expect(DOTENV_EXPECT);
            assert_default_keys_unset();
            assert_env_vars(CUSTOM_VARS);
        });
    }

    #[test]
    fn empty_envfile_exists() {
        let testenv = init_empty_envfile_testenv();
        assert_envfiles_in_testenv(&testenv);
    }

    #[test]
    fn empty_envfile_loaded_vars_state() {
        let testenv = init_empty_envfile_testenv();
        test_in_env(&testenv, || {
            wrap::dotenv().expect(DOTENV_EXPECT);
            assert_default_keys_unset();
        });
    }

    #[test]
    fn custom_bom_envfile_exists() {
        let testenv = init_custom_bom_envfile_testenv();
        assert_envfiles_in_testenv(&testenv);
    }

    #[test]
    fn custom_bom_envfile_loaded_vars_state() {
        let testenv = init_custom_bom_envfile_testenv();
        test_in_env(&testenv, || {
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

mod add_envfile {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_add_twice() {
        let mut testenv = TestEnv::init();
        testenv.add_envfile(".env", create_default_envfile());
        testenv.add_envfile(".env", create_custom_envfile(CUSTOM_VARS));
    }

    #[test]
    #[should_panic]
    fn panics_same_path_as_init() {
        let mut testenv = TestEnv::init_with_envfile(create_default_envfile());
        testenv.add_envfile(".env", create_default_envfile());
    }

    #[test]
    #[should_panic]
    fn panics_same_path_as_default() {
        let mut testenv = TestEnv::default();
        testenv.add_envfile(".env", create_invalid_envfile());
    }

    #[test]
    #[should_panic]
    fn panics_path() {
        let mut testenv = TestEnv::init();
        testenv.add_envfile("", create_default_envfile());
    }

    #[test]
    fn allow_empty_contents() {
        let mut testenv = TestEnv::init();
        testenv.add_envfile(".env", []);
        assert_envfiles_in_testenv(&testenv);
    }

    #[test]
    fn allow_absolute_path() {
        let mut testenv = TestEnv::init();
        let path = testenv.temp_path().join(".env");
        assert!(path.is_absolute());
        testenv.add_envfile(&path, create_default_envfile());
        assert_envfiles_in_testenv(&testenv);
    }

    #[test]
    fn two_files() {
        let mut testenv = TestEnv::init();
        testenv.add_envfile(".env", create_default_envfile());
        testenv.add_envfile(".env.local", create_custom_envfile(CUSTOM_VARS));
        assert_envfiles_in_testenv(&testenv);
    }
}

mod add_env_var {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_add_twice() {
        let mut testenv = TestEnv::init();
        testenv.add_env_var("TEST_KEY", "one_value");
        testenv.add_env_var("TEST_KEY", "two_value");
    }

    #[test]
    #[should_panic]
    fn panics_same_var_as_default() {
        let mut testenv = TestEnv::default();
        testenv.add_env_var(DEFAULT_EXISTING_KEY, "value");
    }

    #[test]
    #[should_panic]
    fn panics_emtpy_key() {
        let mut testenv = TestEnv::init();
        testenv.add_env_var("", "value");
    }

    #[test]
    fn allow_empty_value() {
        let mut testenv = TestEnv::init();
        testenv.add_env_var("TEST_KEY", "");
        assert_env_vars_in_testenv(&testenv, &[("TEST_KEY", "")]);
    }

    #[test]
    fn two_vars() {
        let mut testenv = TestEnv::init();
        let vars = [("TEST_KEY", "one_value"), ("TEST_KEY_2", "two_value")];
        testenv.add_env_var(vars[0].0, vars[0].1);
        testenv.add_env_var(vars[1].0, vars[1].1);
        assert_env_vars_in_testenv(&testenv, &vars);
    }
}
