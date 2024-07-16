mod common;

use crate::common::make_test_dotenv;
use std::{env, error, fs::File};

#[test]
fn test_from_read_override() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    dotenvy::from_read_override(File::open(".env")?)?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
