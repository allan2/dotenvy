extern crate dotenv;
extern crate tempdir;

mod common;

use dotenv::*;

use common::*;

#[test]
fn test_var() {
    let dir = make_test_dotenv().unwrap();

    assert_eq!(var("TESTKEY").unwrap(), "test_val");

    dir.close().unwrap();
}
