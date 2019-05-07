extern crate dotenv;
extern crate tempfile;

mod common;

use std::env;
use dotenv::*;

use common::*;

#[test]
fn test_from_filename() {
    let dir = make_test_dotenv().unwrap();

    from_filename(".env").ok();

    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
