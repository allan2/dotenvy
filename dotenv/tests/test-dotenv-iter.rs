mod common;

use dotenvy::*;
use std::env;

use crate::common::*;

#[test]
fn test_dotenv_iter() {
    let dir = make_test_dotenv().unwrap();

    let iter = dotenv_iter().unwrap();

    assert!(env::var("TESTKEY").is_err());

    iter.load().ok();

    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
