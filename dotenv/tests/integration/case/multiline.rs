use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn no_quote_two_lines() {
    let testenv = TestEnv::init_with_envfile("FOOBAR=foo\nbar");
    test_err_line_parse(&testenv, "bar", 3);
}

#[test]
fn double_quote_two_lines_no_close() {
    let line = "FOOBAR=\"foo\nbar";
    let testenv = TestEnv::init_with_envfile(line);
    test_err_line_parse(&testenv, line, 15);
}

#[test]
fn single_quote_two_lines_no_close() {
    let line = "FOOBAR='foo\nbar";
    let testenv = TestEnv::init_with_envfile(line);
    test_err_line_parse(&testenv, line, 15);
}

#[test]
fn double_quote_two_lines() {
    let envfile = r#"FOOBAR="foo
bar""#;
    let testenv = TestEnv::init_with_envfile(envfile);
    test_single_key_val(&testenv, "FOOBAR", "foo\nbar");
}

#[test]
fn single_quote_two_lines() {
    let testenv = TestEnv::init_with_envfile("FOOBAR='foo\nbar'");
    test_single_key_val(&testenv, "FOOBAR", "foo\nbar");
}

#[test]
fn double_quote_three_lines() {
    let envfile = r#"FOOBAR="foo
bar
baz""#;
    let testenv = TestEnv::init_with_envfile(envfile);
    test_single_key_val(&testenv, "FOOBAR", "foo\nbar\nbaz");
}

const COMPLEX_VALUE: &str = "-BEGIN PRIVATE KEY-\n-END PRIVATE KEY-\n\"QUOTED\"";
const COMPLEX_VALUE_ESCAPED: &str = "-BEGIN PRIVATE KEY-\n-END PRIVATE KEY-\\n\\\"QUOTED\\\"";

#[test]
fn complex_escaped_in_double_quotes() {
    let envfile = format!("WEAK=\"{COMPLEX_VALUE_ESCAPED}\"");
    let testenv = TestEnv::init_with_envfile(envfile);
    test_single_key_val(&testenv, "WEAK", COMPLEX_VALUE);
}

#[test]
fn complex_escaped_in_single_quotes() {
    let envfile = format!("STRONG='{COMPLEX_VALUE_ESCAPED}'");
    let testenv = TestEnv::init_with_envfile(envfile);
    test_single_key_val(&testenv, "STRONG", COMPLEX_VALUE_ESCAPED);
}
