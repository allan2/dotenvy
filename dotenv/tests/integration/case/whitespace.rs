use crate::util::*;
use dotenvy_test_util::*;

#[test]
fn ignore_left() {
    let envfiles = [
        "  FOOO=bar",
        "\nFOOO=bar",
        "\r\n  FOOO=bar",
        "\n\nFOOO=bar",
        "\r\n\r\nFOOO=bar",
        "\tFOOO=bar",
        " \r\n\t FOOO=bar",
    ];
    test_whitespace_envfiles(&envfiles, "FOOO", "bar");
}

#[test]
fn ignore_right() {
    let envfiles = [
        "FOOO=bar  ",
        "FOOO=bar\n",
        "FOOO=bar\n\n",
        "FOOO=bar\r\n",
        "FOOO=bar\r\n\r\n",
        "FOOO=bar \t",
        "FOOO=bar \t \n",
        "FOOO=bar \t \r\n",
        // TODO: This should be allowed.
        // "FOOO=bar\t",
    ];
    test_whitespace_envfiles(&envfiles, "FOOO", "bar");
}

#[test]
fn around_assignment() {
    let testenv = TestEnv::init_with_envfile(format!("{KEY_1} = {VAL_1}"));
    test_key_1_only(&testenv);
}

#[test]
fn escaped_in_value() {
    let testenv = TestEnv::init_with_envfile(r"FOOO=foo\ bar\ baz");
    test_single_key_val(&testenv, "FOOO", "foo bar baz");
}

#[test]
fn double_quoted_value() {
    let testenv = TestEnv::init_with_envfile(r#"FOOO="foo bar baz""#);
    test_single_key_val(&testenv, "FOOO", "foo bar baz");
}

#[test]
fn single_quoted_value() {
    let testenv = TestEnv::init_with_envfile("FOOO='foo bar baz'");
    test_single_key_val(&testenv, "FOOO", "foo bar baz");
}

fn test_whitespace_envfiles(envfiles: &[&str], key: &str, expected: &str) {
    for &envfile in envfiles {
        let testenv = TestEnv::init_with_envfile(envfile);
        test_single_key_val(&testenv, key, expected);
    }
}
