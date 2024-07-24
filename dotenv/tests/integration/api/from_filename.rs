use crate::util::*;
use dotenvy::from_filename;
use dotenvy_test_util::*;

#[test]
fn no_file() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        let err = from_filename("nonexistent.env").unwrap_err();
        assert_err_not_found(err);
    });
}

#[test]
fn empty_default_file() {
    let testenv = TestEnv::init_with_envfile("");
    test_in_env(&testenv, || {
        assert!(from_filename(".env").is_ok());
    });
}

#[test]
fn empty_custom_file() {
    let mut testenv = TestEnv::init();
    testenv.add_envfile(".custom.env", "");
    test_in_env(&testenv, || {
        assert!(from_filename(".custom.env").is_ok());
    });
}

#[test]
fn default_file_not_read_on_missing_file() {
    test_in_default_env(|| {
        let err = from_filename("nonexistent.env").unwrap_err();
        assert_err_not_found(err);
        assert_env_var_unset(DEFAULT_TEST_KEY);
    })
}

#[test]
fn dotenv_then_custom() {
    let mut testenv = TestEnv::default();
    testenv.add_envfile("custom", KEYVAL_1);
    test_in_env(&testenv, || {
        dotenvy::dotenv().unwrap();
        from_filename("custom").unwrap();
        assert_env_var(KEY_1, VAL_1);
        assert_default_keys();
    });
}

#[test]
fn dotenv_then_custom_no_override() {
    let mut testenv = TestEnv::default();
    testenv.add_envfile("custom", format!("{DEFAULT_TEST_KEY}=from_custom"));
    test_in_env(&testenv, || {
        dotenvy::dotenv().unwrap();
        from_filename("custom").unwrap();
        assert_default_keys();
    });
}

#[test]
fn explicit_no_override() {
    let mut testenv = TestEnv::init();
    testenv.add_env_var(KEY_1, VAL_1);
    testenv.add_envfile("custom", format!("{KEY_1}=from_custom"));
    test_in_env(&testenv, || {
        from_filename("custom").unwrap();
        assert_env_var(KEY_1, VAL_1);
    });
}

#[test]
fn child_dir() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("child/custom", KEYVAL_1);
    test_in_env(&testenv, || {
        from_filename("child/custom").unwrap();
        assert_env_var(KEY_1, VAL_1);
    });
}

#[test]
fn parent_dir_relative_path() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("custom.env", KEYVAL_1);
    testenv.set_work_dir("child");
    test_in_env(&testenv, || {
        from_filename("../custom.env").unwrap();
        assert_env_var(KEY_1, VAL_1);
    });
}

#[test]
fn parent_dir_absolute_path() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("custom.env", KEYVAL_1);
    testenv.set_work_dir("child");
    test_in_env(&testenv, || {
        let path = testenv
            .temp_path()
            .join("custom.env")
            .canonicalize()
            .expect("canonicalize envfile");
        from_filename(path).unwrap();
        assert_env_var(KEY_1, VAL_1);
    });
}
