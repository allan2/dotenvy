use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn one() {
    let testenv = TestEnv::init_with_envfile(format!("# {KEYVAL_1}"));
    test_keys_unset(&testenv);
}

#[test]
fn one_whitespace() {
    let testenv = TestEnv::init_with_envfile(format!(" # {KEYVAL_1}"));
    test_keys_unset(&testenv);
}

#[test]
fn one_and_one_assign() {
    let testenv = TestEnv::init_with_envfile(format!("# {KEYVAL_1}\n{KEYVAL_2}"));
    test_key_2_only(&testenv);
}

#[test]
fn one_and_one_assign_whitespace() {
    let testenv = TestEnv::init_with_envfile(format!(" # {KEYVAL_1}\n{KEYVAL_2}"));
    test_key_2_only(&testenv);
}

#[test]
fn assign_same_line() {
    let testenv = TestEnv::init_with_envfile(format!("{KEYVAL_1} # {KEYVAL_2}"));
    test_key_1_only(&testenv);
}

#[test]
fn hash_in_value() {
    let testenv = TestEnv::init_with_envfile(format!("{KEYVAL_1}#{KEYVAL_2}"));
    test_key_1_with_hash_val(&testenv);
}

#[test]
fn hash_in_value_single_quoted() {
    let testenv = TestEnv::init_with_envfile(format!("{KEY_1}='{VAL_1}'#{KEYVAL_2}"));
    test_key_1_with_hash_val(&testenv);
}

#[test]
fn hash_in_value_double_quoted() {
    let testenv = TestEnv::init_with_envfile(format!(r##"{KEY_1}="{VAL_1}"#{KEYVAL_2}"##));
    test_key_1_with_hash_val(&testenv);
}

#[test]
fn hash_in_key() {
    let testenv = TestEnv::init_with_envfile("FOO#1=bar");
    test_err_line_parse(&testenv, "FOO#1=bar", 3);
}
