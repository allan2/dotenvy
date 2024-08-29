mod common;

use crate::common::make_test_dotenv;
use std::{env, error, fs::File};

#[test]
fn test_from_read() -> Result<(), Box<dyn error::Error>> {
    let dir = unsafe { make_test_dotenv() }?;
    let rdr = File::open(".env")?;
    unsafe { dotenvy::from_read(rdr) }?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
