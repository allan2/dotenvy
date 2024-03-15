mod common;

use crate::common::make_test_dotenv;
use std::{env, error};

#[test]
fn test_var() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    assert_eq!(dotenvy::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
