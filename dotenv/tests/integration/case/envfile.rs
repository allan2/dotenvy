use crate::util::*;
use dotenvy_test_util::*;

const ONE_WORD: &str = "oneword";

#[test]
#[should_panic]
fn none() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, api_fn);
}

#[test]
fn none_err() {
    let testenv = TestEnv::init();
    test_err_not_found(&testenv);
}

#[test]
fn empty() {
    let testenv = create_empty_envfile_testenv();
    test_in_env(&testenv, || {
        api_fn();
        assert_default_existing_var();
    });
}

#[test]
fn empty_path() {
    let testenv = create_empty_envfile_testenv();
    test_default_envfile_path(&testenv);
}

#[test]
#[should_panic]
fn one_word() {
    let testenv = create_one_word_envfile_testenv();
    test_in_env(&testenv, api_fn);
}

#[test]
fn one_word_err() {
    let testenv = create_one_word_envfile_testenv();
    test_err_line_parse(&testenv, ONE_WORD, ONE_WORD.len());
}

#[test]
fn one_line() {
    let testenv = create_one_line_envfile_testenv();
    test_key_1_only(&testenv);
}

#[test]
fn one_line_path() {
    let testenv = create_one_line_envfile_testenv();
    test_default_envfile_path(&testenv);
}

fn create_empty_envfile_testenv() -> TestEnv {
    let mut testenv = create_testenv_with_default_var();
    testenv.add_envfile(".env", "");
    testenv
}

fn create_one_word_envfile_testenv() -> TestEnv {
    let mut testenv = create_testenv_with_default_var();
    testenv.add_envfile(".env", ONE_WORD);
    testenv
}

fn create_one_line_envfile_testenv() -> TestEnv {
    TestEnv::init_with_envfile(KEYVAL_1)
}
