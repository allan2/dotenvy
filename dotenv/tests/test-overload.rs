mod common;

use std::{env, error::Error, result::Result};

use dotenvy::*;

use crate::common::*;

#[test]
fn test_overload() -> Result<(), Box<dyn Error>> {
    let dir = tempdir_with_dotenv(
        "
var=old
var=new
",
    )?;
    overload()?;
    assert_eq!(var("var")?, "new");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
