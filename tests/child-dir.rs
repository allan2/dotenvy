extern crate dotenv;
extern crate tempdir;

mod common;

use dotenv::*;
use std::{env, fs};

use common::tempdir_with_dotenv;

#[test]
fn child_dir() {
    let dir = tempdir_with_dotenv("TESTKEY=test_val").unwrap();

    fs::create_dir("child").unwrap();

    env::set_current_dir("child").unwrap();

    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
