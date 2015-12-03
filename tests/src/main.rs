extern crate dotenv;

use dotenv::*;
use std::env;
use std::path::Path;
use std::fs;

#[test]
fn test_dotenv_default_location() {
    dotenv().ok();

    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");
}

#[test]
fn test_dotenv_child_dir() {
    let new_path = Path::new("./src");
    assert!(env::set_current_dir(&new_path).is_ok());
    let mut paths = fs::read_dir("./").unwrap();
    let path_string = paths.nth(0).unwrap().unwrap().path().display().to_string();
    assert_eq!("./main.rs".to_string(), path_string);
    dotenv().ok();
    assert_eq!(env::var("TESTKEY").unwrap(), "test_val");
}
