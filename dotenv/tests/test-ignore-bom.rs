mod common;

use crate::common::tempdir_with_dotenv;
use std::{env, error};

#[test]
fn test_ignore_bom() -> Result<(), Box<dyn error::Error>> {
    let bom = "\u{feff}";
    let dir = tempdir_with_dotenv(&format!("{}TESTKEY=test_val", bom))?;

    let mut path = env::current_dir()?;
    path.push(".env");

    dotenvy::from_path(&path)?;

    assert_eq!(env::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
