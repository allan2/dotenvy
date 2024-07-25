use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn utf8_no_vars() {
    let mut efb = EnvFileBuilder::new();
    efb.insert_utf8_bom();
    let testenv = TestEnv::init_with_envfile(efb);
    test_keys_unset(&testenv);
}

#[test]
fn utf8_one_var() {
    let mut efb = EnvFileBuilder::new();
    efb.insert_utf8_bom();
    efb.add_key_value(KEY_1, VAL_1);
    let testenv = TestEnv::init_with_envfile(efb);
    test_key_1_only(&testenv);
}

#[test]
fn utf8_two_vars() {
    let mut efb = EnvFileBuilder::new();
    efb.insert_utf8_bom();
    let vars = [(KEY_1, VAL_1), (KEY_2, VAL_2)];
    efb.add_vars(&vars);
    let testenv = TestEnv::init_with_envfile(efb);
    test_env_vars(&testenv, &vars);
}

#[test]
fn invalid_no_vars() {
    let mut efb = EnvFileBuilder::new();
    efb.add_bytes(b"\xFA\xFA");
    let testenv = TestEnv::init_with_envfile(efb);
    test_invalid_utf8(&testenv);
}

#[test]
fn invalid_one_var() {
    let mut efb = EnvFileBuilder::new();
    efb.add_bytes(b"\xFE\xFF");
    efb.add_key_value(KEY_1, VAL_1);
    let testenv = TestEnv::init_with_envfile(efb);
    test_invalid_utf8(&testenv);
}

#[test]
fn utf16_no_vars() {
    let mut efb = EnvFileBuilder::new();
    efb.add_bytes(b"\xFE\xFF");
    let testenv = TestEnv::init_with_envfile(efb);
    test_invalid_utf8(&testenv);
}
