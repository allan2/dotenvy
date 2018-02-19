extern crate dotenv;
extern crate tempdir;

mod common;

use dotenv::*;
use std::env;

use common::tempdir_with_dotenv;

#[test]
fn default_location() {
    let dir = tempdir_with_dotenv("TESTKEY=test_val").unwrap();

    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
