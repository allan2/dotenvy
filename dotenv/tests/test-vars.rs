mod common;

use crate::common::make_test_dotenv;
use std::{collections::HashMap, env, error};

#[test]
fn test_vars() -> Result<(), Box<dyn error::Error>> {
    let dir = make_test_dotenv()?;

    let vars: HashMap<String, String> = dotenvy::vars().collect();

    assert_eq!(vars["TESTKEY"], "test_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
