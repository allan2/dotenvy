mod common;

use crate::common::make_test_dotenv;
use dotenvy::from_path_iter;
use std::{env, error};

#[test]
fn test_from_path_iter() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    let iter = from_path_iter(&path)?;

    assert!(env::var("TESTKEY").is_err());

    iter.load()?;

    assert_eq!(env::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
