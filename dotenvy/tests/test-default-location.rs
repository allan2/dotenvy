mod common;

use dotenvy::EnvLoader;

use crate::common::make_test_dotenv;
use std::{env, error};

#[test]
fn test_default_location() -> Result<(), Box<dyn error::Error>> {
    let dir = unsafe { make_test_dotenv() }?;

    unsafe { EnvLoader::from_path("./.env").load_and_modify() }?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
