mod common;

use std::{env, error};

use dotenvy::from_path_override;

use crate::common::make_test_dotenv;

#[test]
fn test_from_path_override() -> Result<(), Box<dyn error::Error>> {
    let dir = unsafe { make_test_dotenv() }?;

    let mut path = env::current_dir()?;
    path.push(".env");

    unsafe { from_path_override(&path) }?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
