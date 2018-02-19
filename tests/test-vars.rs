extern crate dotenv;
extern crate tempdir;

mod common;

use std::collections::HashMap;
use dotenv::*;

use common::*;

#[test]
fn test_vars() {
    let dir = make_test_dotenv().unwrap();

    let vars: HashMap<String, String> = vars().collect();
      
    assert_eq!(vars["TESTKEY"], "test_val");

    dir.close().unwrap();
}
