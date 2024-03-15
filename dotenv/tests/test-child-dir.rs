mod common;

use crate::common::make_test_dotenv;
use std::{env, error, fs};

#[test]
fn test_child_dir() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    fs::create_dir("child")?;

    env::set_current_dir("child")?;

    dotenvy::dotenv()?;
    assert_eq!(env::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
