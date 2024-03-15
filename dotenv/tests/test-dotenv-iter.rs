mod common;

use crate::common::make_test_dotenv;
use std::{env, error};

#[test]
fn test_dotenv_iter() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    let iter = dotenvy::dotenv_iter()?;
    assert!(env::var("TESTKEY").is_err());

    iter.load()?;
    assert_eq!(env::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
