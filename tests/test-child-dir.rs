extern crate dotenv;
extern crate tempdir;

mod common;

use std::{env, fs};
use dotenv::*;

use common::*;

#[test]
fn test_child_dir() {
    let dir = make_test_dotenv().unwrap();

    fs::create_dir("child").unwrap();

    env::set_current_dir("child").unwrap();

    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
