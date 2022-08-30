mod common;
use std::env;

use common::tempdir_with_dotenv;
use dotenvy::dotenv;

#[test]
fn test_issue_12() {
    let _f = tempdir_with_dotenv(
        r#"
# Start of .env file
# Comment line with single ' quote
# Comment line with double " quote
 # Comment line with double " quote and starts with a space
TESTKEY=test_val # A '" comment
# End of .env file
"#,
    )
    .expect("should write test env");

    dotenv().expect("should succeed");
    assert_eq!(
        env::var("TESTKEY").expect("test env key not set"),
        "test_val"
    );
}
