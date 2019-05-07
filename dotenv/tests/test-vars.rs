extern crate dotenv;
extern crate tempfile;

mod common;

use std::collections::HashMap;
use std::env;

use dotenv::*;

use common::*;

#[test]
fn test_vars() {
    let dir = make_test_dotenv().unwrap();

    let vars: HashMap<String, String> = vars().collect();
      
    assert_eq!(vars["TESTKEY"], "test_val");

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
