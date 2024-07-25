use std::path::Path;

use crate::util::*;
use dotenvy::from_filename_iter;
use dotenvy_test_util::*;

#[test]
fn no_file_ok() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        from_filename_iter("nonexistent").ok();
    });
}

#[test]
fn no_file_err() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || match from_filename_iter("nonexistent") {
        Ok(_) => panic!("expected error"),
        Err(err) => assert_err_not_found(err),
    });
}

#[test]
fn empty_default_file() {
    let testenv = TestEnv::init_with_envfile("");
    test_in_env(&testenv, || {
        from_filename_iter(".env").unwrap().for_each(|_| {
            panic!("should have no keys");
        });
    });
}

#[test]
fn empty_custom_file() {
    let mut testenv = TestEnv::init();
    testenv.add_envfile(".custom.env", "");
    test_in_env(&testenv, || {
        from_filename_iter(".custom.env").unwrap().for_each(|_| {
            panic!("should have no keys");
        });
    });
}

#[test]
fn default_file_not_read_on_missing_file() {
    test_in_default_env(|| {
        from_filename_iter("nonexistent.env").ok();
        assert_env_var_unset(DEFAULT_TEST_KEY);
    })
}

#[test]
fn child_dir() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("child/custom", KEYVAL_1);
    test_in_env(&testenv, || {
        assert_single_key_file("child/custom");
    });
}

#[test]
fn parent_dir_relative_path() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("custom.env", KEYVAL_1);
    testenv.set_work_dir("child");
    test_in_env(&testenv, || {
        assert_single_key_file("../custom.env");
    });
}

#[test]
fn parent_dir_absolute_path() {
    let mut testenv = TestEnv::init();
    testenv.add_child_dir("child");
    testenv.add_envfile("custom.env", KEYVAL_1);
    testenv.set_work_dir("child");
    test_in_env(&testenv, || {
        let path = canonicalize_envfile_path(&testenv, "custom.env");
        assert_single_key_file(path);
    });
}

#[test]
fn two_vars_into_hash_map() {
    check_iter_default_envfile_into_hash_map(|| from_filename_iter(".env"));
}

fn assert_single_key_file(path: impl AsRef<Path>) {
    let (key, value) = from_filename_iter_unwrap_one_item(path);
    assert_eq!(key, KEY_1);
    assert_eq!(value, VAL_1);
}

fn from_filename_iter_unwrap_one_item(path: impl AsRef<Path>) -> (String, String) {
    from_filename_iter(path)
        .expect("valid file")
        .next()
        .expect("one item")
        .expect("valid item")
}
