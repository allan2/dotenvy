use crate::util::*;
use dotenvy::dotenv_iter;
use dotenvy_test_util::*;

#[test]
fn default_env_ok() {
    test_in_default_env(|| {
        dotenv_iter().ok();
        assert_default_existing_var();
        // the envfile shouldn't be loaded into the environment
        assert_env_var_unset(DEFAULT_TEST_KEY);
    });
}

#[test]
fn default_env_unwrap() {
    test_in_default_env(|| {
        dotenv_iter().unwrap();
        assert_default_existing_var();
        assert_env_var_unset(DEFAULT_TEST_KEY);
    });
}

#[test]
fn no_envfile_ok() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        dotenv_iter().ok();
        assert_default_keys_unset();
    });
}

#[test]
fn no_envfile_err() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || match dotenv_iter() {
        Ok(_) => panic!("should have failed"),
        Err(err) => assert_err_not_found(err),
    });
}

#[test]
fn no_vars() {
    let testenv = TestEnv::init_with_envfile("");
    test_in_env(&testenv, || {
        dotenv_iter().unwrap().for_each(|_| {
            panic!("should have no keys");
        });
    });
}

#[test]
fn one_var() {
    let testenv = TestEnv::init_with_envfile("FOOO=bar");
    test_in_env(&testenv, || {
        let (key, value) = dotenv_iter_unwrap_one_item();
        assert_eq!(key, "FOOO");
        assert_eq!(value, "bar");
    });
}

#[test]
fn one_var_only() {
    let testenv = TestEnv::init_with_envfile("FOOO=bar");
    test_in_env(&testenv, || {
        let count = dotenv_iter().expect("valid file").count();
        assert_eq!(1, count);
    });
}

#[test]
fn one_var_empty() {
    let testenv = TestEnv::init_with_envfile("FOOO=");
    test_in_env(&testenv, || {
        let (key, value) = dotenv_iter_unwrap_one_item();
        assert_eq!(key, "FOOO");
        assert_eq!(value, "");
    });
}

#[test]
fn two_vars_into_hash_map() {
    check_iter_default_envfile_into_hash_map(dotenv_iter);
}

#[test]
fn explicit_no_override() {
    let mut testenv = TestEnv::init();
    testenv.add_env_var("FOOO", "bar");
    testenv.add_envfile(".env", "FOOO=notbar");
    test_in_env(&testenv, || {
        dotenv_iter().unwrap();
        assert_env_var("FOOO", "bar");
    })
}

fn dotenv_iter_unwrap_one_item() -> (String, String) {
    dotenv_iter()
        .expect("valid file")
        .next()
        .expect("one item")
        .expect("valid item")
}
