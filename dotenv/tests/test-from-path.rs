mod common;

use crate::common::make_test_dotenv;
use std::{env, error};

#[test]
fn test_from_path() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    dotenvy::from_path(&path)?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
