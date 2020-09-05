mod common;

use dotenv::*;
use std::env;

use crate::common::*;

#[test]
fn test_multiline() {
    let value = "-----BEGIN PRIVATE KEY-----\n-----END PRIVATE KEY-----\\n\\\"QUOTED\\\"";
    let weak = "-----BEGIN PRIVATE KEY-----\n-----END PRIVATE KEY-----\n\"QUOTED\"";
    let dir = tempdir_with_dotenv(&format!("WEAK=\"{}\"\nSTRONG='{}'", value, value)).unwrap();

    dotenv().ok();
    assert_eq!(var("WEAK").unwrap(), weak);
    assert_eq!(var("STRONG").unwrap(), value);

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
