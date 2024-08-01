use super::*;

mod new {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn vars_state() -> Result<(), Error> {
        let testenv = TestEnv::new()?;
        let mut vars: HashMap<String, String> = HashMap::new();
        test_in_env(&testenv, || {
            for (k, v) in std::env::vars() {
                vars.insert(k, v);
            }
        })?;
        for (k, v) in &vars {
            assert_env_var(k.as_str(), v.as_str());
        }
        Ok(())
    }

    #[test]
    fn no_env_file() -> Result<(), Error> {
        let testenv = TestEnv::new()?;
        let env_file_path = testenv.temp_path().join(".env");

        test_in_env(&testenv, || {
            assert!(!env_file_path.exists());
        })
    }

    #[test]
    fn work_dir_is_temp() -> Result<(), Error> {
        let testenv = TestEnv::new()?;
        assert_eq!(testenv.work_dir(), testenv.temp_path());
        Ok(())
    }

    #[test]
    fn env_vars_are_empty() -> Result<(), Error> {
        let testenv = TestEnv::new()?;
        assert!(testenv.env_vars().is_empty());
        Ok(())
    }

    #[test]
    fn env_files_are_empty() -> Result<(), Error> {
        let testenv = TestEnv::new()?;
        assert!(testenv.env_files().is_empty());
        Ok(())
    }
}

mod new_with_env_file {
    use super::*;

    #[test]
    fn env_file_vars_state() -> Result<(), Error> {
        let testenv = testenv_with_test_env_file()?;
        test_keys_not_set(&testenv)
    }

    #[test]
    fn env_file_exists() -> Result<(), Error> {
        let testenv = testenv_with_test_env_file()?;
        test_env_files(&testenv)
    }

    #[test]
    fn custom_env_file_vars_state() -> Result<(), Error> {
        let testenv = testenv_with_custom_env_file()?;
        test_in_env(&testenv, || {
            assert_test_keys_unset();
            for (key, _) in CUSTOM_VARS {
                assert_env_var_unset(key);
            }
        })
    }

    #[test]
    fn custom_env_file_exists() -> Result<(), Error> {
        let testenv = testenv_with_custom_env_file()?;
        test_env_files(&testenv)
    }

    #[test]
    fn empty_env_file_exists() -> Result<(), Error> {
        let testenv = testenv_with_empty_env_file()?;
        test_env_files(&testenv)
    }

    #[test]
    fn custom_bom_env_file_exists() -> Result<(), Error> {
        let testenv = testenv_with_custom_bom_env_file()?;
        test_env_files(&testenv)
    }

    fn testenv_with_test_env_file() -> Result<TestEnv, Error> {
        let env_file = create_test_env_file();
        TestEnv::new_with_env_file(env_file)
    }

    fn testenv_with_custom_env_file() -> Result<TestEnv, Error> {
        let env_file = create_custom_env_file();
        TestEnv::new_with_env_file(env_file)
    }

    fn testenv_with_empty_env_file() -> Result<TestEnv, Error> {
        TestEnv::new_with_env_file([])
    }

    fn testenv_with_custom_bom_env_file() -> Result<TestEnv, Error> {
        let mut efc = EnvFileContents::new();
        let bom = b"\xEF\xBB\xBF";
        efc.push_bytes(bom);
        efc.add_var(TEST_KEY, TEST_VALUE);
        let env_file = efc.into_owned_string()?;
        TestEnv::new_with_env_file(env_file)
    }
}

mod add_env_file {
    use super::*;

    #[test]
    fn errors_add_twice() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_env_file(".env", create_test_env_file())?;
        test_add_env_file_conflict(&mut testenv, ".env");
        Ok(())
    }

    #[test]
    fn errors_same_path_as_new_with_env_file() -> Result<(), Error> {
        let mut testenv = TestEnv::new_with_env_file(create_test_env_file())?;
        test_add_env_file_conflict(&mut testenv, ".env");
        Ok(())
    }

    #[test]
    fn errors_empty_path() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let err = testenv
            .add_env_file("", create_test_env_file())
            .unwrap_err();
        assert!(matches!(err, Error::EnvFilePathSameAsTempDir));
        Ok(())
    }

    #[test]
    fn allow_empty_contents() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_env_file(".env", [])?;
        test_env_files(&testenv)
    }

    #[test]
    fn allow_absolute_path() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let path = testenv.temp_path().join(".env");
        assert!(path.is_absolute());
        testenv.add_env_file(&path, create_test_env_file())?;
        test_env_files(&testenv)
    }

    #[test]
    fn two_files() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_env_file(".env", create_test_env_file())?;
        testenv.add_env_file(".env.local", create_custom_env_file())?;
        test_env_files(&testenv)
    }

    fn test_add_env_file_conflict(testenv: &mut TestEnv, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let expected_path = testenv.temp_path().join(path);
        let res = testenv.add_env_file(path, create_custom_env_file());
        assert_env_file_conflict(res, expected_path);
    }

    fn assert_env_file_conflict(
        result: Result<&mut TestEnv, Error>,
        expected_path: impl AsRef<Path>,
    ) {
        if let Err(err) = result {
            match err {
                Error::EnvFileConflict(path) => assert_eq!(expected_path.as_ref(), path),
                _ => panic!("unexpected error: {err}"),
            }
        } else {
            panic!("expected error");
        }
    }
}

mod add_env_var {
    use super::*;

    #[test]
    fn errors_add_twice() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_env_var("TEST_KEY", "one_value")?;

        let res = testenv.add_env_var("TEST_KEY", "two_value");

        assert_key_conflict(res, "TEST_KEY");
        Ok(())
    }

    #[test]
    fn errors_empty_key() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let err = testenv.add_env_var("", "value").unwrap_err();
        assert!(matches!(err, Error::KeyEmpty));
        Ok(())
    }

    #[test]
    fn allow_empty_value() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_env_var("TEST_KEY", "")?;
        test_env_vars(&testenv, &[("TEST_KEY", "")])
    }

    #[test]
    fn two_vars() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let vars = [("TEST_KEY", "one_value"), ("TEST_KEY_2", "two_value")];
        testenv.add_env_var(vars[0].0, vars[0].1)?;
        testenv.add_env_var(vars[1].0, vars[1].1)?;
        test_env_vars(&testenv, &vars)
    }

    #[test]
    fn owned_strings() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_env_var("TEST_KEY".to_string(), "test_val".to_string())?;
        test_env_vars(&testenv, &[("TEST_KEY", "test_val")])
    }
}

mod set_env_vars {
    use super::*;

    #[test]
    fn errors_double_key() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let mut vars = VARS.to_vec();
        vars.push(VARS[0]);

        let res = testenv.set_env_vars(&vars);

        assert_key_conflict(res, VARS[0].0);
        Ok(())
    }

    #[test]
    fn errors_empty_key() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let err = testenv.set_env_vars(&[("", "value")]).unwrap_err();
        assert!(matches!(err, Error::KeyEmpty));
        Ok(())
    }

    #[test]
    fn from_tuples_slice() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.set_env_vars(VARS.as_slice())?;
        test_vars(&testenv)
    }

    #[test]
    fn from_tuples_ref() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.set_env_vars(&VARS)?;
        test_vars(&testenv)
    }

    #[test]
    fn from_vec_slice() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let vec = VARS.to_vec();
        testenv.set_env_vars(vec.as_slice())?;
        test_vars(&testenv)
    }

    const VARS: [(&str, &str); 2] = [("TEST_KEY", "one_value"), ("TEST_KEY_2", "two_value")];

    fn test_vars(testenv: &TestEnv) -> Result<(), Error> {
        test_env_vars(testenv, &VARS)
    }
}

mod set_work_dir {
    use super::*;

    #[test]
    fn errors_non_existing() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let expected_path = testenv.temp_path().join("subdir");

        let res = testenv.set_work_dir("subdir");

        assert_path_not_found(res, expected_path);
        Ok(())
    }

    #[test]
    fn allow_absolute_path() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let path = testenv.temp_path().join("subdir");
        assert!(path.is_absolute());

        testenv.add_child_dir(&path)?;
        testenv.set_work_dir(&path)?;

        assert_path_exists(&testenv, "subdir");
        Ok(())
    }

    #[test]
    fn relative_path() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;

        testenv.add_child_dir("subdir")?;
        testenv.set_work_dir("subdir")?;

        assert_path_exists(&testenv, "subdir");
        Ok(())
    }

    #[test]
    fn in_testenv() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_child_dir("subdir")?;
        testenv.set_work_dir("subdir")?;
        test_in_env(&testenv, || {
            let current_dir = std::env::current_dir().expect("failed to get current dir");
            assert_eq!(current_dir, testenv.work_dir());
        })
    }

    fn assert_path_not_found(result: Result<&mut TestEnv, Error>, expected_path: impl AsRef<Path>) {
        if let Err(err) = result {
            match err {
                Error::PathNotFound(path) => assert_eq!(expected_path.as_ref(), path),
                _ => panic!("unexpected error: {err}"),
            }
        } else {
            panic!("expected error");
        }
    }
}

mod add_child_dir {
    use super::*;

    #[test]
    fn subdir() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_child_dir("subdir")?;
        assert_path_exists(&testenv, "subdir");
        Ok(())
    }

    #[test]
    fn allow_absolute_path() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        let path = testenv.temp_path().join("subdir");
        assert!(path.is_absolute());

        testenv.add_child_dir(&path)?;

        assert_path_exists(&testenv, "subdir");
        Ok(())
    }

    #[test]
    fn create_parents() -> Result<(), Error> {
        let mut testenv = TestEnv::new()?;
        testenv.add_child_dir("subdir/subsubdir")?;
        assert_path_exists(&testenv, "subdir/subsubdir");
        Ok(())
    }
}

fn assert_key_conflict(result: Result<&mut TestEnv, Error>, key: &str) {
    if let Err(err) = result {
        match err {
            Error::KeyConflict(k) => assert_eq!(key, k),
            _ => panic!("unexpected error: {err}"),
        }
    } else {
        panic!("expected error");
    }
}
