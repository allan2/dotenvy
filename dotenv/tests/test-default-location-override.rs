mod common;

use crate::common::make_test_dotenv;
use std::{env, error};

#[test]
fn test_default_location_override() -> Result<(), Box<dyn error::Error>> {
    let dir = unsafe { make_test_dotenv() }?;

    unsafe { dotenvy::dotenv_override() }?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
