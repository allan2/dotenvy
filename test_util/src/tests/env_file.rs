use super::*;

#[test]
fn new_builds_empty() {
    let efc = EnvFileContents::new();
    assert!(efc.is_empty());
}

#[test]
fn default_builds_empty() {
    let efc = EnvFileContents::default();
    assert!(efc.is_empty());
}

#[test]
fn add_key_empty_value() {
    let mut efc = EnvFileContents::new();
    efc.add_var(TEST_KEY, "");
    let expected = format!("{TEST_KEY}=\n");
    assert_eq!(expected, efc);
}

#[test]
fn add_key_value() {
    let mut efc = EnvFileContents::new();
    efc.add_var(TEST_KEY, TEST_VALUE);
    let expected = format!("{TEST_KEY}={TEST_VALUE}\n");
    assert_eq!(expected, efc);
}

#[test]
fn add_multiple_key_values() {
    let mut efc = EnvFileContents::new();
    efc.add_var(TEST_KEY, TEST_VALUE);
    efc.add_var(EXISTING_KEY, OVERRIDING_VALUE);
    let expected = expected_env_file(&[(TEST_KEY, TEST_VALUE), (EXISTING_KEY, OVERRIDING_VALUE)]);
    assert_eq!(expected, efc);
}

#[test]
fn add_str() {
    let mut efc = EnvFileContents::new();
    efc.push_str("test");
    assert_eq!("test", efc);
}

#[test]
fn add_bytes() {
    let mut efc = EnvFileContents::new();
    efc.push_bytes(b"test");
    assert_eq!("test", efc);
}

#[test]
fn add_byte() {
    let mut efc = EnvFileContents::new();
    efc.push(b't');
    assert_eq!("t", efc);
}

#[test]
fn insert_utf8_bom() {
    let mut efc = EnvFileContents::new();
    efc.push_bytes(&[0xEF, 0xBB, 0xBF]);
    efc.push_str("test");
    assert_eq!("\u{FEFF}test", efc);
}

#[test]
fn from_vec_u8() {
    let vec: Vec<u8> = Vec::from(create_test_env_file());
    let efc = EnvFileContents::from(vec);
    assert_eq!(create_test_env_file(), efc);
}

#[test]
fn to_vec_u8() {
    let mut efc = EnvFileContents::new();
    efc.push_str(create_test_env_file().as_str());
    let vec = Vec::from(efc);
    let expected = create_test_env_file().into_bytes();
    assert_eq!(expected, vec);
}

#[test]
fn from_string() {
    let efc = EnvFileContents::from(create_test_env_file());
    assert_eq!(create_test_env_file(), efc);
}
