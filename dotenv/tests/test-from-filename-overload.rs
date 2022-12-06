mod common;

use dotenvy::*;
use std::{env, error::Error, result::Result};

use crate::common::*;

#[test]
fn test_from_filename_overload() -> Result<(), Box<dyn Error>> {
    let dir = make_test_dotenv()?;

    from_filename_overload(".env")?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
