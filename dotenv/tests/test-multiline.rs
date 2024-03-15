mod common;

use crate::common::tempdir_with_dotenv;
use std::{env, error};

#[test]
fn test_multiline() -> Result<(), Box<dyn error::Error>> {
    let value = "-----BEGIN PRIVATE KEY-----\n-----END PRIVATE KEY-----\\n\\\"QUOTED\\\"";
    let weak = "-----BEGIN PRIVATE KEY-----\n-----END PRIVATE KEY-----\n\"QUOTED\"";
    let dir = tempdir_with_dotenv(&format!(
        r#"
KEY=my\ cool\ value
KEY3="awesome \"stuff\"
more
on other
lines"
KEY4='hello '\''world'"
good ' \'morning"
WEAK="{}"
STRONG='{}'
"#,
        value, value
    ))?;

    dotenvy::dotenv()?;
    assert_eq!(env::var("KEY")?, r#"my cool value"#);
    assert_eq!(
        env::var("KEY3")?,
        r#"awesome "stuff"
more
on other
lines"#
    );
    assert_eq!(
        env::var("KEY4")?,
        r#"hello 'world
good ' 'morning"#
    );
    assert_eq!(env::var("WEAK")?, weak);
    assert_eq!(env::var("STRONG")?, value);

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
