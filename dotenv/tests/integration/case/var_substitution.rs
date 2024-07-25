use crate::util::*;
use dotenvy_test_util::*;

mod no_quotes {
    use super::*;

    #[test]
    fn from_env() {
        let envfile = format!("{KEY_1}=${KEY_2}");
        test_key_1_set_val_2(&envfile);
    }

    #[test]
    fn plus_extra() {
        let envfile = format!("{KEY_1}=${KEY_2}+extra");
        test_key_1_set_val_2_plus_extra(&envfile);
    }

    #[test]
    fn plus_space() {
        let envfile = format!("{KEY_1}=${KEY_2} + extra");
        let testenv = create_testenv_with_key_2(&envfile);
        let expected = format!("${KEY_2} + extra");
        test_err_line_parse(&testenv, &expected, 8);
    }

    #[test]
    fn braced() {
        let envfile = format!("{KEY_1}=${{{KEY_2}}}");
        test_key_1_set_val_2(&envfile);
    }

    #[test]
    fn braced_plus() {
        let envfile = format!("{KEY_1}=${{{KEY_2}}}1");
        let expected = format!("{VAL_2}1");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn escaped() {
        let envfile = format!("{KEY_1}=\\${KEY_2}");
        let expected = format!("${KEY_2}");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn key_not_set() {
        let envfile = format!("{KEY_1}=${KEY_2}");
        let testenv = TestEnv::init_with_envfile(envfile);
        test_single_key_val(&testenv, KEY_1, "");
    }

    #[test]
    fn prev_entry() {
        let envfile = format!("{KEYVAL_1}\n{KEY_2}=${KEY_1}");
        let testenv = TestEnv::init_with_envfile(envfile);
        test_env_vars(&testenv, &[(KEY_1, VAL_1), (KEY_2, VAL_1)]);
    }

    #[test]
    fn prev_entry_plus_extra() {
        let envfile = format!("{KEYVAL_1}\n{KEY_2}=${KEY_1}+extra");
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected_val_2 = format!("{VAL_1}+extra");
        test_env_vars(&testenv, &[(KEY_1, VAL_1), (KEY_2, &expected_val_2)]);
    }
}

mod double_quote {
    use super::*;

    #[test]
    fn from_env() {
        let envfile = format!(r#"{KEY_1}="${KEY_2}"#);
        let mut testenv = TestEnv::init_with_envfile(envfile.as_str());
        testenv.add_env_var(KEY_2, VAL_2);
        test_err_line_parse(&testenv, &envfile, 11);
    }

    #[test]
    fn plus_extra() {
        let envfile = format!(r#"{KEY_1}="${KEY_2}+extra""#);
        test_key_1_set_val_2_plus_extra(&envfile);
    }

    #[test]
    fn plus_space() {
        let envfile = format!(r#"{KEY_1}="${KEY_2} + extra""#);
        test_key_1_set_val_2_plus_extra_with_space(&envfile);
    }

    #[test]
    fn braced() {
        let envfile = format!("{KEY_1}=\"${{{KEY_2}}}\"");
        test_key_1_set_val_2(&envfile);
    }

    #[test]
    fn braced_plus() {
        let envfile = format!(r#"{KEY_1}="${{{KEY_2}}}1""#);
        let expected = format!("{VAL_2}1");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn escaped() {
        let envfile = format!(r#"{KEY_1}="\\${KEY_2}""#);
        let testenv = create_testenv_with_key_2(&envfile);
        let expected = format!(r#""\\${KEY_2}""#);
        test_err_line_parse(&testenv, &expected, 8);
    }

    #[test]
    fn escaped_plus() {
        let envfile = format!(r#"{KEY_1}="\\${KEY_2}+1""#);
        let expected = format!("\\{VAL_2}+1");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn key_not_set() {
        let envfile = format!(r#"{KEY_1}="${KEY_2}""#);
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected = format!(r#""${KEY_2}""#);
        test_err_line_parse(&testenv, &expected, 6);
    }

    #[test]
    fn key_not_set_plus_extra() {
        let envfile = format!(r#"{KEY_1}="${KEY_2}+extra""#);
        let testenv = TestEnv::init_with_envfile(envfile);
        test_single_key_val(&testenv, KEY_1, "+extra");
    }

    #[test]
    fn prev_entry() {
        let envfile = format!("{KEYVAL_1}\n{KEY_2}=\"${KEY_1}\"");
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected = format!(r#""${KEY_1}""#);
        test_err_line_parse(&testenv, &expected, 6);
    }

    #[test]
    fn prev_entry_plus_extra() {
        let envfile = format!("{KEYVAL_1}\n{KEY_2}=\"${KEY_1}+extra\"");
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected_val_2 = format!("{VAL_1}+extra");
        test_env_vars(&testenv, &[(KEY_1, VAL_1), (KEY_2, &expected_val_2)]);
    }
}

mod single_quote {
    use super::*;

    #[test]
    fn from_env() {
        let envfile = format!("{KEY_1}='${KEY_2}'");
        let expected = format!("${KEY_2}");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn plus_extra() {
        let envfile = format!("{KEY_1}='${KEY_2}+extra'");
        let expected = format!("${KEY_2}+extra");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn plus_space() {
        let envfile = format!("{KEY_1}='${KEY_2} + extra'");
        let expected = format!("${KEY_2} + extra");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn braced() {
        let envfile = format!("{KEY_1}='${{{KEY_2}}}'");
        let expected = format!("${{{KEY_2}}}");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn braced_plus() {
        let envfile = format!("{KEY_1}='${{{KEY_2}}}1'");
        let expected = format!("${{{KEY_2}}}1");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn escaped() {
        let envfile = format!("{KEY_1}='\\${KEY_2}'");
        let expected = format!("\\${KEY_2}");
        test_key_1_set_as_with_key_2(&envfile, &expected);
    }

    #[test]
    fn key_not_set() {
        let envfile = format!("{KEY_1}='${KEY_2}'");
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected = format!("${KEY_2}");
        test_single_key_val(&testenv, KEY_1, &expected);
    }

    #[test]
    fn prev_entry() {
        let envfile = format!("{KEYVAL_1}\n{KEY_2}='${KEY_1}'");
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected = format!("${KEY_1}");
        test_env_vars(&testenv, &[(KEY_1, VAL_1), (KEY_2, &expected)]);
    }

    #[test]
    fn prev_entry_plus_extra() {
        let envfile = format!("{KEYVAL_1}\n{KEY_2}='${KEY_1}+extra'");
        let testenv = TestEnv::init_with_envfile(envfile);
        let expected = format!("${KEY_1}+extra");
        test_env_vars(&testenv, &[(KEY_1, VAL_1), (KEY_2, &expected)]);
    }
}

fn test_key_1_set_as_with_key_2(envfile: &str, expected: &str) {
    let testenv = create_testenv_with_key_2(envfile);
    test_single_key_val(&testenv, KEY_1, expected);
}

fn test_key_1_set_val_2(envfile: &str) {
    let testenv = create_testenv_with_key_2(envfile);
    test_single_key_val(&testenv, KEY_1, VAL_2);
}

fn test_key_1_set_val_2_plus_extra(envfile: &str) {
    let testenv = create_testenv_with_key_2(envfile);
    let exepcted = format!("{VAL_2}+extra");
    test_single_key_val(&testenv, KEY_1, &exepcted);
}

fn test_key_1_set_val_2_plus_extra_with_space(envfile: &str) {
    let testenv = create_testenv_with_key_2(envfile);
    let exepcted = format!("{VAL_2} + extra");
    test_single_key_val(&testenv, KEY_1, &exepcted);
}

fn create_testenv_with_key_2(envfile: &str) -> TestEnv {
    let mut testenv = TestEnv::init_with_envfile(envfile);
    testenv.add_env_var(KEY_2, VAL_2);
    testenv
}
