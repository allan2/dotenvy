extern crate dotenv;
extern crate tempdir;

mod common;

use std::env;
use dotenv::*;

use common::*;

#[test]
fn test_from_filename_iter() {
    let dir = make_test_dotenv().unwrap();

    let iter = from_filename_iter(".env").unwrap();

    assert!(env::var("TESTKEY").is_err());

    iter.load().ok();

    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
