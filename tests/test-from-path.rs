extern crate dotenv;
extern crate tempdir;

mod common;

use std::env;
use dotenv::*;

use common::*;

#[test]
fn test_from_path() {
    let dir = make_test_dotenv().unwrap();

    let mut path = env::current_dir().unwrap();
    path.push(".env");

    from_path(&path).ok();

    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
