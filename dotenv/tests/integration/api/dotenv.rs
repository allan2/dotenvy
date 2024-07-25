use crate::util::*;
use dotenvy::dotenv;
use dotenvy_test_util::*;

#[test]
fn default_env_ok() {
    test_in_default_env(|| {
        dotenv().ok();
        assert_default_keys();
    });
}

#[test]
fn default_env_unwrap() {
    test_in_default_env(|| {
        dotenv().unwrap();
        assert_default_keys();
    });
}

#[test]
fn default_env_unwrap_path() {
    let testenv = TestEnv::default();
    test_default_envfile_path(&testenv);
}

#[test]
fn explicit_no_override() {
    let mut testenv = TestEnv::init();
    testenv.add_env_var("FOOO", "bar");
    testenv.add_envfile(".env", "FOOO=notbar");
    test_in_env(&testenv, || {
        dotenv().unwrap();
        assert_env_var("FOOO", "bar");
    })
}
