use super::*;

pub fn test_default_envfile_path(testenv: &TestEnv) {
    test_in_env(testenv, || {
        let path = api_fn_path();
        assert_default_envfile_path(testenv, &path);
    })
}

pub fn test_err_line_parse(testenv: &TestEnv, line: &str, index: usize) {
    test_in_env(testenv, || {
        let err = api_fn_err();
        assert_err_line_parse(line, index, err);
    })
}

pub fn test_err_not_found(testenv: &TestEnv) {
    test_in_env(testenv, || {
        let err = api_fn_err();
        assert_err_not_found(err);
    })
}

pub fn test_invalid_utf8(testenv: &TestEnv) {
    test_in_env(testenv, || {
        let err = api_fn_err();
        assert_err_invalid_utf8(err);
    })
}

pub fn test_key_1_only(testenv: &TestEnv) {
    test_in_env(testenv, || {
        api_fn();
        assert_env_var(KEY_1, VAL_1);
        assert_env_var_unset(KEY_2);
    });
}

pub fn test_key_2_only(testenv: &TestEnv) {
    test_in_env(testenv, || {
        api_fn();
        assert_env_var_unset(KEY_1);
        assert_env_var(KEY_2, VAL_2);
    });
}

pub fn test_keys_unset(testenv: &TestEnv) {
    test_in_env(testenv, || {
        api_fn();
        assert_env_var_unset(KEY_1);
        assert_env_var_unset(KEY_2);
    });
}

pub fn test_key_1_with_hash_val(testenv: &TestEnv) {
    test_in_env(testenv, || {
        api_fn();
        assert_env_var(KEY_1, &format!("{VAL_1}#{KEYVAL_2}"));
        assert_env_var_unset(KEY_2);
    });
}

pub fn test_single_key_val(testenv: &TestEnv, key: &str, expected: &str) {
    test_in_env(testenv, || {
        api_fn();
        assert_env_var(key, expected);
    });
}

pub fn test_env_vars(testenv: &TestEnv, vars: &[(&str, &str)]) {
    test_in_env(testenv, || {
        api_fn();
        assert_env_vars(vars)
    });
}
