mod common;

use std::env;
use dotenv::*;

use crate::common::*;

#[test]
#[allow(deprecated)]
fn test_dotenv_iter() {
    let dir = make_test_dotenv().unwrap();

    let iter = dotenv_iter().unwrap();

    iter.filter_map(Result::ok).any(|(key, value)| key == "TESTKEY" && value == "test_val");

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}

#[test]
#[allow(deprecated)]
fn test_dotenv_subdir_iter() {
    let dir = make_layered_test_dotenv().unwrap();

    let iter = dotenv_iter().unwrap();

    let pairs = iter.filter_map(Result::ok).collect::<Vec<_>>();
    
    assert!(pairs.contains(&("TESTKEY".into(), "test_val".into())));
    assert!(pairs.contains(&("TESTKEY2".into(), "test_val_inner".into())));

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
