mod common;

use dotenvy::EnvLoader;

use crate::common::make_test_dotenv;
use std::{env, error, fs};

#[test]
fn test_child_dir() -> Result<(), Box<dyn error::Error>> {
    let dir = unsafe { make_test_dotenv() }?;
    fs::create_dir("child")?;
    env::set_current_dir("child")?;

    unsafe { EnvLoader::from_path("../.env").load_and_modify() }?;
    assert_eq!(env::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
