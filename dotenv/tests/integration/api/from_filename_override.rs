use crate::util::*;
use dotenvy::from_filename_override;
use dotenvy_test_util::*;

#[test]
fn no_file_ok() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        from_filename_override("nonexistent").ok();
    });
}

#[test]
fn no_file_err() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        let err = from_filename_override("nonexistent.env").unwrap_err();
        assert_err_not_found(err);
    });
}

#[test]
fn empty_default_file() {
    let testenv = TestEnv::init_with_envfile("");
    test_in_env(&testenv, || {
        assert!(from_filename_override(".env").is_ok());
    });
}

#[test]
fn empty_custom_file() {
    let mut testenv = TestEnv::init();
    testenv.add_envfile(".custom.env", "");
    test_in_env(&testenv, || {
        assert!(from_filename_override(".custom.env").is_ok());
    });
}

#[test]
fn return_path_valid() {
    let testenv = TestEnv::default();
    test_in_env(&testenv, || {
        let actual = from_filename_override(".env").unwrap();
        assert_default_envfile_path(&testenv, &actual);
    });
}

#[test]
fn one_var_custom_file() {
    let mut testenv = TestEnv::init();
    testenv.add_envfile(".custom.env", KEYVAL_1);
    test_in_env(&testenv, || {
        assert!(from_filename_override(".custom.env").is_ok());
        assert_env_var(KEY_1, VAL_1);
    });
}

#[test]
fn override_existing_var_custom_file() {
    let mut testenv = TestEnv::init();
    testenv.add_env_var("FOOO", "from_env");
    testenv.add_envfile(".custom.env", "FOOO=from_file");
    test_in_env(&testenv, || {
        assert_env_var("FOOO", "from_env");
        from_filename_override(".custom.env").unwrap();
        assert_env_var("FOOO", "from_file");
    });
}

#[test]
fn default_override() {
    test_in_default_env(|| {
        from_filename_override(".env").unwrap();
        assert_env_var(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
        assert_env_var(DEFAULT_EXISTING_KEY, DEFAULT_OVERRIDING_VALUE);
    })
}

#[test]
fn substitute_self() {
    let mut testenv = TestEnv::init();
    testenv.add_env_var("FOOO", "test");
    testenv.add_envfile(".custom.env", "FOOO=$FOOO+1");
    test_in_env(&testenv, || {
        assert_env_var("FOOO", "test");
        from_filename_override(".custom.env").unwrap();
        assert_env_var("FOOO", "test+1");
    });
}

#[test]
fn substitute_self_two_files() {
    let mut testenv = TestEnv::init();
    testenv.add_env_var("FOOO", "test");
    testenv.add_envfile(".custom1.env", "FOOO=$FOOO+1");
    testenv.add_envfile(".custom2.env", "FOOO=$FOOO+1");
    test_in_env(&testenv, || {
        assert_env_var("FOOO", "test");
        from_filename_override(".custom1.env").unwrap();
        from_filename_override(".custom2.env").unwrap();
        assert_env_var("FOOO", "test+1+1");
    });
}
