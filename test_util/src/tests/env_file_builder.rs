use super::*;

#[test]
fn new_builds_empty() {
    let efb = EnvFileBuilder::new();
    assert!(efb.is_empty());
}

#[test]
fn default_builds_empty() {
    let efb = EnvFileBuilder::default();
    assert!(efb.is_empty());
}

#[test]
fn add_key_empty_value() {
    let mut efb = EnvFileBuilder::new();
    efb.add_key_value(DEFAULT_TEST_KEY, "");
    let expected = format!("{DEFAULT_TEST_KEY}=\n");
    assert_contents_str(efb, &expected);
}

#[test]
fn add_key_value() {
    let mut efb = EnvFileBuilder::new();
    efb.add_key_value(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
    let expected = format!("{DEFAULT_TEST_KEY}={DEFAULT_TEST_VALUE}\n");
    assert_contents_str(efb, &expected);
}

#[test]
fn add_multiple_key_values() {
    let mut efb = EnvFileBuilder::new();
    efb.add_key_value(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
    efb.add_key_value(DEFAULT_EXISTING_KEY, DEFAULT_OVERRIDING_VALUE);
    let expected = expected_env_file(&[
        (DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE),
        (DEFAULT_EXISTING_KEY, DEFAULT_OVERRIDING_VALUE),
    ]);
    assert_contents_str(efb, &expected);
}

#[test]
fn add_vars() {
    let mut efb = EnvFileBuilder::new();
    efb.add_vars(CUSTOM_VARS);
    let expected = expected_env_file(CUSTOM_VARS);
    assert_contents_str(efb, &expected);
}

#[test]
fn add_str() {
    let mut efb = EnvFileBuilder::new();
    efb.add_str("test");
    assert_contents_str(efb, "test");
}

#[test]
fn add_bytes() {
    let mut efb = EnvFileBuilder::new();
    efb.add_bytes(b"test");
    assert_contents_str(efb, "test");
}

#[test]
fn add_byte() {
    let mut efb = EnvFileBuilder::new();
    efb.add_byte(b't');
    assert_contents_str(efb, "t");
}

#[test]
fn insert_utf8_bom() {
    let mut efb = EnvFileBuilder::new();
    efb.add_str("test");
    efb.insert_utf8_bom();
    assert_contents_str(efb, "\u{FEFF}test");
}

#[test]
fn add_strln() {
    let mut efb = EnvFileBuilder::new();
    efb.add_strln("test");
    assert_contents_str(efb, "test\n");
}

#[test]
fn from_vec_u8() {
    let vec: Vec<u8> = Vec::from(create_default_env_file());
    let efb = EnvFileBuilder::from(vec);
    assert_contents_str(efb, &create_default_env_file());
}

#[test]
fn to_vec_u8() {
    let mut efb = EnvFileBuilder::new();
    efb.add_str(create_default_env_file().as_str());
    let vec = Vec::from(efb);
    let expected = create_default_env_file().into_bytes();
    assert_eq!(expected, vec);
}

#[test]
fn from_string() {
    let efb = EnvFileBuilder::from(create_default_env_file());
    assert_contents_str(efb, &create_default_env_file());
}

fn assert_contents_str(efb: EnvFileBuilder, expected: &str) {
    let contents = efb.into_owned_string();
    assert_eq!(expected, contents,);
}
