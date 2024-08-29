mod common;

use crate::common::tempdir_with_dotenv;
use std::{env, error};

#[test]
fn test_ignore_bom() -> Result<(), Box<dyn error::Error>> {
    let txt = format!("\u{feff}TESTKEY=test_val");
    let dir = unsafe { tempdir_with_dotenv(&txt) }?;

    let mut path = env::current_dir()?;
    path.push(".env");

    unsafe { dotenvy::from_path(&path) }?;

    assert_eq!(env::var("TESTKEY")?, "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
