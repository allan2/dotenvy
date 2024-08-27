use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn double_value() {
    let testenv = TestEnv::init_with_envfile(format!(r#"{KEY_1}="{VAL_1}""#));
    test_key_1_only(&testenv);
}

#[test]
fn single_value() {
    let testenv = TestEnv::init_with_envfile(format!(r"{KEY_1}='{VAL_1}'"));
    test_key_1_only(&testenv);
}

#[test]
fn double_and_single_value() {
    let testenv = TestEnv::init_with_envfile(r#"fooo="'bar'""#);
    test_single_key_val(&testenv, "fooo", "'bar'");
}

#[test]
fn single_and_double_value() {
    let testenv = TestEnv::init_with_envfile(r#"fooo='"bar"'"#);
    test_single_key_val(&testenv, "fooo", "\"bar\"");
}

#[test]
fn double_key() {
    let line = r#""FOOO"=bar"#;
    let testenv = TestEnv::init_with_envfile(line);
    test_err_line_parse(&testenv, line, 0);
}

#[test]
fn single_key() {
    let line = "'FOOO'=bar";
    let testenv = TestEnv::init_with_envfile(line);
    test_err_line_parse(&testenv, line, 0);
}

#[test]
fn double_in_double_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO="outer "inner"""#);
    test_single_key_val(&testenv, "FOOO", "outer inner");
}

#[test]
fn double_in_single_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO='outer "inner"'"#);
    test_single_key_val(&testenv, "FOOO", "outer \"inner\"");
}

#[test]
fn single_in_double_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO="outer 'inner'""#);
    test_single_key_val(&testenv, "FOOO", "outer 'inner'");
}

#[test]
fn single_in_single_value() {
    let testenv = TestEnv::init_with_envfile("FOOO='outer 'inner''");
    test_single_key_val(&testenv, "FOOO", "outer inner");
}

#[test]
fn escaped_double_in_double_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO="outer \"inner\"""#);
    test_single_key_val(&testenv, "FOOO", "outer \"inner\"");
}

#[test]
fn escaped_double_in_single_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO='outer \"inner\"'"#);
    test_single_key_val(&testenv, "FOOO", r#"outer \"inner\""#);
}

#[test]
fn escaped_single_in_double_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO="outer \'inner\'""#);
    test_single_key_val(&testenv, "FOOO", "outer 'inner'");
}

#[test]
fn escaped_single_in_single_value() {
    let line = br"FOOO='outer \'inner\''";
    let testenv = TestEnv::init_with_envfile(*line);
    test_err_line_parse(&testenv, "'outer \\'inner\\''", 16);
}
