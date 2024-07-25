use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn alone_single_quote() {
    let testenv =
        TestEnv::init_with_envfile(format!(r#"# {KEYVAL_1} Comment with single ' quote"#));
    test_keys_unset(&testenv)
}

#[test]
fn alone_single_quote_with_space() {
    let testenv =
        TestEnv::init_with_envfile(format!(r#" #  {KEYVAL_1} Comment with single ' quote"#));
    test_keys_unset(&testenv)
}

#[test]
fn alone_double_quote() {
    let testenv =
        TestEnv::init_with_envfile(format!(r#"# {KEYVAL_1} Comment with double " quote"#));
    test_keys_unset(&testenv)
}

#[test]
fn alone_double_quote_with_space() {
    let testenv =
        TestEnv::init_with_envfile(format!(r#" #  {KEYVAL_1} Comment with double " quote"#));
    test_keys_unset(&testenv)
}

#[test]
fn single_quote() {
    let mut efb = EnvFileBuilder::new();
    efb.add_strln(r#"# Comment with single ' quote"#);
    test_open_quote_comment(efb);
}

#[test]
fn single_quote_with_space() {
    let mut efb = EnvFileBuilder::new();
    efb.add_strln(r#" #  Comment with single ' quote"#);
    test_open_quote_comment(efb);
}

#[test]
fn double_quote() {
    let mut efb = EnvFileBuilder::new();
    efb.add_strln(r#"# Comment with double " quote"#);
    test_open_quote_comment(efb);
}

#[test]
fn double_quote_with_space() {
    let mut efb = EnvFileBuilder::new();
    efb.add_strln(r#" #  Comment with double " quote"#);
    test_open_quote_comment(efb);
}

fn test_open_quote_comment(mut efb: EnvFileBuilder) {
    efb.add_strln(KEYVAL_1);
    let testenv = TestEnv::init_with_envfile(efb);
    test_key_1_only(&testenv);
}
