use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn ignore_export() {
    let testenv = TestEnv::init_with_envfile(format!("export {KEYVAL_1}"));
    test_key_1_only(&testenv);
}

#[test]
fn ignore_export_whitespace() {
    let testenv = TestEnv::init_with_envfile(format!(" export {KEYVAL_1}"));
    test_key_1_only(&testenv);
}

#[test]
fn ignore_export_and_comment() {
    let testenv = TestEnv::init_with_envfile(format!("export {KEYVAL_1} # {KEYVAL_2}"));
    test_key_1_only(&testenv);
}
