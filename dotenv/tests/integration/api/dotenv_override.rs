use crate::util::*;
use dotenvy::dotenv_override;
use dotenvy_test_util::*;

#[test]
fn no_file_ok() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        dotenv_override().ok();
    });
}

#[test]
fn no_file_err() {
    let testenv = TestEnv::init();
    test_in_env(&testenv, || {
        let err = dotenv_override().unwrap_err();
        assert_err_not_found(err);
    });
}

#[test]
fn empty_file_is_ok() {
    let testenv = TestEnv::init_with_envfile("");
    test_in_env(&testenv, || {
        assert!(dotenv_override().is_ok());
    });
}

#[test]
fn one_new_var() {
    let testenv = TestEnv::init_with_envfile("FOOO=bar");
    test_in_env(&testenv, || {
        dotenv_override().unwrap();
        assert_env_var("FOOO", "bar");
    });
}

#[test]
fn one_old_var() {
    let mut testenv = TestEnv::init_with_envfile("FOOO=from_file");
    testenv.add_env_var("FOOO", "from_env");
    test_in_env(&testenv, || {
        assert_env_var("FOOO", "from_env");
        dotenv_override().unwrap();
        assert_env_var("FOOO", "from_file");
    });
}

#[test]
fn one_old_var_one_new_var() {
    let vars = [("FOOO", "from_file"), ("BARR", "new")];
    let envfile = create_custom_envfile(&vars);
    let mut testenv = TestEnv::init_with_envfile(envfile);
    testenv.add_env_var("FOOO", "from_env");
    test_in_env(&testenv, || {
        assert_env_var_unset("BARR");
        dotenv_override().unwrap();
        assert_env_vars(&vars);
    });
}

#[test]
fn substitute_self() {
    let mut testenv = TestEnv::init_with_envfile("FOOO=$FOOO+1");
    testenv.add_env_var("FOOO", "test");
    test_in_env(&testenv, || {
        assert_env_var("FOOO", "test");
        dotenv_override().unwrap();
        assert_env_var("FOOO", "test+1");
    });
}

#[test]
fn substitute_self_twice() {
    let mut testenv = TestEnv::init_with_envfile("FOOO=$FOOO+1\nFOOO=$FOOO+1");
    testenv.add_env_var("FOOO", "test");
    test_in_env(&testenv, || {
        assert_env_var("FOOO", "test");
        dotenv_override().unwrap();
        assert_env_var("FOOO", "test+1+1");
    });
}
