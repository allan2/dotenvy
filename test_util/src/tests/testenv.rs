use super::*;
use dotenvy::dotenv;

mod init {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn vars_state() {
        let testenv = TestEnv::init();
        let existing_vars: HashMap<String, String> = std::env::vars().collect();
        dbg!("existing vars: {:?}", &existing_vars);
        test_in_env(&testenv, || {
            for (key, value) in &existing_vars {
                assert_env_var(key, value);
            }
        });
    }

    #[test]
    fn no_env_file() {
        let testenv = TestEnv::init();
        let env_file_path = testenv.temp_path().join(".env");

        test_in_env(&testenv, || {
            assert!(!env_file_path.exists());
            assert!(dotenv().is_err());
        });
    }

    #[test]
    fn work_dir_is_temp() {
        let testenv = TestEnv::init();
        assert_eq!(testenv.work_dir(), testenv.temp_path());
    }

    #[test]
    fn env_vars_are_empty() {
        let testenv = TestEnv::init();
        assert!(testenv.env_vars().is_empty());
    }

    #[test]
    fn env_files_are_empty() {
        let testenv = TestEnv::init();
        assert!(testenv.env_files().is_empty());
    }
}

mod init_with_env_file {
    use super::*;

    #[test]
    fn env_file_vars_state() {
        let testenv = init_test_env_file_testenv();
        test_keys_not_set(&testenv);
    }

    #[test]
    fn env_file_exists() {
        let testenv = init_test_env_file_testenv();
        test_env_files(&testenv);
    }

    #[test]
    fn env_file_loaded_vars_state() {
        let testenv = init_test_env_file_testenv();
        test_in_env(&testenv, || {
            dotenv().expect(DOTENV_EXPECT);
            // dotenv() does not override existing var
            // but existing key is not set in this testenv
            assert_env_var(EXISTING_KEY, OVERRIDING_VALUE);
            assert_env_var(TEST_KEY, TEST_VALUE);
        });
    }

    #[test]
    fn custom_env_file_vars_state() {
        let testenv = init_custom_env_file_testenv();
        test_in_env(&testenv, || {
            assert_test_keys_unset();
            for (key, _) in CUSTOM_VARS {
                assert_env_var_unset(key);
            }
        });
    }

    #[test]
    fn custom_env_file_exists() {
        let testenv = init_custom_env_file_testenv();
        test_env_files(&testenv);
    }

    #[test]
    fn custom_env_file_loaded_vars_state() {
        let testenv = init_custom_env_file_testenv();
        test_in_env(&testenv, || {
            dotenv().expect(DOTENV_EXPECT);
            assert_test_keys_unset();
            assert_env_vars(CUSTOM_VARS);
        });
    }

    #[test]
    fn empty_env_file_exists() {
        let testenv = init_empty_env_file_testenv();
        test_env_files(&testenv);
    }

    #[test]
    fn empty_env_file_loaded_vars_state() {
        let testenv = init_empty_env_file_testenv();
        test_in_env(&testenv, || {
            dotenv().expect(DOTENV_EXPECT);
            assert_test_keys_unset();
        });
    }

    #[test]
    fn custom_bom_env_file_exists() {
        let testenv = init_custom_bom_env_file_testenv();
        test_env_files(&testenv);
    }

    #[test]
    fn custom_bom_env_file_loaded_vars_state() {
        let testenv = init_custom_bom_env_file_testenv();
        test_in_env(&testenv, || {
            dotenv().expect(DOTENV_EXPECT);
            assert_env_var(TEST_KEY, TEST_VALUE);
        });
    }

    fn init_test_env_file_testenv() -> TestEnv {
        let env_file = create_test_env_file();
        TestEnv::init_with_env_file(env_file)
    }

    fn init_custom_env_file_testenv() -> TestEnv {
        let env_file = create_custom_env_file();
        TestEnv::init_with_env_file(env_file)
    }

    fn init_empty_env_file_testenv() -> TestEnv {
        TestEnv::init_with_env_file([])
    }

    fn init_custom_bom_env_file_testenv() -> TestEnv {
        let mut efb = EnvFileContents::new();
        efb.add_key_value(TEST_KEY, TEST_VALUE);
        efb.insert_utf8_bom();
        let env_file = efb.into_owned_string();
        TestEnv::init_with_env_file(env_file)
    }
}

mod add_env_file {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_add_twice() {
        let mut testenv = TestEnv::init();
        testenv.add_env_file(".env", create_test_env_file());
        testenv.add_env_file(".env", create_custom_env_file());
    }

    #[test]
    #[should_panic]
    fn panics_same_path_as_init() {
        let mut testenv = TestEnv::init_with_env_file(create_test_env_file());
        testenv.add_env_file(".env", create_test_env_file());
    }

    #[test]
    #[should_panic]
    fn panics_path() {
        let mut testenv = TestEnv::init();
        testenv.add_env_file("", create_test_env_file());
    }

    #[test]
    fn allow_empty_contents() {
        let mut testenv = TestEnv::init();
        testenv.add_env_file(".env", []);
        test_env_files(&testenv);
    }

    #[test]
    fn allow_absolute_path() {
        let mut testenv = TestEnv::init();
        let path = testenv.temp_path().join(".env");
        assert!(path.is_absolute());
        testenv.add_env_file(&path, create_test_env_file());
        test_env_files(&testenv);
    }

    #[test]
    fn two_files() {
        let mut testenv = TestEnv::init();
        testenv.add_env_file(".env", create_test_env_file());
        testenv.add_env_file(".env.local", create_custom_env_file());
        test_env_files(&testenv);
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
    fn panics_emtpy_key() {
        let mut testenv = TestEnv::init();
        testenv.add_env_var("", "value");
    }

    #[test]
    fn allow_empty_value() {
        let mut testenv = TestEnv::init();
        testenv.add_env_var("TEST_KEY", "");
        test_env_vars(&testenv, &[("TEST_KEY", "")]);
    }

    #[test]
    fn two_vars() {
        let mut testenv = TestEnv::init();
        let vars = [("TEST_KEY", "one_value"), ("TEST_KEY_2", "two_value")];
        testenv.add_env_var(vars[0].0, vars[0].1);
        testenv.add_env_var(vars[1].0, vars[1].1);
        test_env_vars(&testenv, &vars);
    }

    #[test]
    fn owned_strings() {
        let mut testenv = TestEnv::init();
        testenv.add_env_var("TEST_KEY".to_string(), "test_val".to_string());
        test_env_vars(&testenv, &[("TEST_KEY", "test_val")]);
    }
}

mod set_env_vars {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_double_key() {
        let mut testenv = TestEnv::init();
        let mut vars = VARS.to_vec();
        vars.push(VARS[0]);
        testenv.set_env_vars(&vars);
    }

    #[test]
    #[should_panic]
    fn panics_empty_key() {
        let mut testenv = TestEnv::init();
        testenv.set_env_vars(&[("", "value")]);
    }

    #[test]
    fn from_tuples_slice() {
        let mut testenv = TestEnv::init();
        testenv.set_env_vars(VARS.as_slice());
        assert_vars_in_testenv(&testenv);
    }

    #[test]
    fn from_tuples_ref() {
        let mut testenv = TestEnv::init();
        testenv.set_env_vars(&VARS);
        assert_vars_in_testenv(&testenv);
    }

    #[test]
    fn from_vec_slice() {
        let mut testenv = TestEnv::init();
        let vec = VARS.to_vec();
        testenv.set_env_vars(vec.as_slice());
        assert_vars_in_testenv(&testenv);
    }

    const VARS: [(&str, &str); 2] = [("TEST_KEY", "one_value"), ("TEST_KEY_2", "two_value")];

    fn assert_vars_in_testenv(testenv: &TestEnv) {
        test_env_vars(testenv, &VARS);
    }
}

mod set_work_dir {
    use super::*;

    #[test]
    #[should_panic]
    fn panics_non_existing() {
        let mut testenv = TestEnv::init();
        testenv.set_work_dir("subdir");
    }

    #[test]
    fn allow_absolute_path() {
        let mut testenv = TestEnv::init();
        let path = testenv.temp_path().join("subdir");
        assert!(path.is_absolute());
        std::fs::create_dir_all(&path).expect("failed to create subdir");
        testenv.set_work_dir(&path);
        test_path_exists(&testenv, "subdir");
    }

    #[test]
    fn relative_path() {
        let mut testenv = TestEnv::init();
        std::fs::create_dir_all(testenv.temp_path().join("subdir"))
            .expect("failed to create subdir");
        testenv.set_work_dir("subdir");
        test_path_exists(&testenv, "subdir");
    }

    #[test]
    fn in_testenv() {
        let mut testenv = TestEnv::init();
        std::fs::create_dir_all(testenv.temp_path().join("subdir"))
            .expect("failed to create subdir");
        testenv.set_work_dir("subdir");
        test_in_env(&testenv, || {
            let current_dir = std::env::current_dir().expect("failed to get current dir");
            assert_eq!(current_dir, testenv.work_dir());
        });
    }
}

mod add_child_dir {
    use super::*;

    #[test]
    fn subdir() {
        let mut testenv = TestEnv::init();
        testenv.add_child_dir("subdir");
        test_path_exists(&testenv, "subdir");
    }

    #[test]
    fn allow_absolute_path() {
        let mut testenv = TestEnv::init();
        let path = testenv.temp_path().join("subdir");
        assert!(path.is_absolute());
        testenv.add_child_dir(&path);
        test_path_exists(&testenv, "subdir");
    }

    #[test]
    fn create_parents() {
        let mut testenv = TestEnv::init();
        testenv.add_child_dir("subdir/subsubdir");
        test_path_exists(&testenv, "subdir/subsubdir");
    }
}
