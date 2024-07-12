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

    #[test]
    fn custom_bom_envfile_exists() {
        let testenv = init_custom_bom_envfile_testenv();
        assert_envfile_exists_in_testenv(testenv);
    }

    #[test]
    fn custom_bom_envfile_loaded_vars_state() {
        let testenv = init_custom_bom_envfile_testenv();
        test_in_env(testenv, || {
            dotenv_wrap().expect(DOTENV_EXPECT);
            assert_env_var(TEST_KEY, TEST_VALUE);
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
        efb.add_key_value(TEST_KEY, TEST_VALUE);
        efb.insert_utf8_bom();
        let envfile = efb.into_owned_string();
        TestEnv::init_with_envfile(envfile)
    }
}

mod envfile_builder {
    use super::*;

    #[test]
    fn new_builds_empty() {
        let efb = EnvFileBuilder::new();
        assert_contents_empty(efb);
    }

    #[test]
    fn default_builds_empty() {
        let efb = EnvFileBuilder::default();
        assert_contents_empty(efb);
    }

    #[test]
    fn add_key_empty_value() {
        let mut efb = EnvFileBuilder::new();
        efb.add_key_value(TEST_KEY, "");
        let expected = format!("{}=\n", TEST_KEY);
        assert_contents_str(efb, &expected);
    }

    #[test]
    fn add_key_value() {
        let mut efb = EnvFileBuilder::new();
        efb.add_key_value(TEST_KEY, TEST_VALUE);
        let expected = format!("{}={}\n", TEST_KEY, TEST_VALUE);
        assert_contents_str(efb, &expected);
    }

    #[test]
    fn add_multiple_key_values() {
        let mut efb = EnvFileBuilder::new();
        efb.add_key_value(TEST_KEY, TEST_VALUE);
        efb.add_key_value(TEST_EXISTING_KEY, TEST_OVERRIDING_VALUE);
        let expected = expected_envfile(&[
            (TEST_KEY, TEST_VALUE),
            (TEST_EXISTING_KEY, TEST_OVERRIDING_VALUE),
        ]);
        assert_contents_str(efb, &expected);
    }

    #[test]
    fn add_vars() {
        let mut efb = EnvFileBuilder::new();
        efb.add_vars(CUSTOM_VARS);
        let expected = expected_envfile(CUSTOM_VARS);
        assert_contents_str(efb, &expected);
    }

    #[test]
    fn add_str() {
        let mut efb = EnvFileBuilder::new();
        efb.add_str("test");
        assert_contents_str(efb, "test");
    }

    #[test]
    fn add_bytes() {
        let mut efb = EnvFileBuilder::new();
        efb.add_bytes(b"test");
        assert_contents_str(efb, "test");
    }

    #[test]
    fn add_byte() {
        let mut efb = EnvFileBuilder::new();
        efb.add_byte(b't');
        assert_contents_str(efb, "t");
    }

    #[test]
    fn insert_utf8_bom() {
        let mut efb = EnvFileBuilder::new();
        efb.add_str("test");
        efb.insert_utf8_bom();
        assert_contents_str(efb, "\u{FEFF}test");
    }

    #[test]
    fn add_strln() {
        let mut efb = EnvFileBuilder::new();
        efb.add_strln("test");
        assert_contents_str(efb, "test\n");
    }

    fn assert_contents_empty(efb: EnvFileBuilder) {
        let contents = efb.into_owned_bytes();
        assert!(contents.is_empty());
    }

    fn assert_contents_str(efb: EnvFileBuilder, expected: &str) {
        let contents = efb.into_owned_string();
        assert_eq!(expected, contents,);
    }

    fn expected_envfile(env_vars: &[(&str, &str)]) -> String {
        let mut envfile = String::new();
        for (key, value) in env_vars {
            envfile.push_str(key);
            envfile.push('=');
            envfile.push_str(value);
            envfile.push('\n');
        }
        envfile
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
