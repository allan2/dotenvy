use super::*;

#[test]
fn create_default() {
    let expected = format!(
        "{}={}\n{}={}",
        DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE, DEFAULT_EXISTING_KEY, DEFAULT_OVERRIDING_VALUE
    );
    let actual = create_default_envfile();
    assert_eq!(
        expected, actual,
        "envfile should be created with default values"
    );
}

#[test]
fn create_invalid() {
    let expected = format!(
        "{}{}\n{}{}",
        DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE, DEFAULT_EXISTING_KEY, DEFAULT_OVERRIDING_VALUE
    );
    let actual = create_invalid_envfile();
    assert_eq!(
        expected, actual,
        "envfile should be created without equals sign"
    );
}

#[test]
fn create_custom() {
    let expected = expected_envfile(CUSTOM_VARS);
    let actual = create_custom_envfile(CUSTOM_VARS);
    assert_eq!(
        expected, actual,
        "envfile should be created with custom values"
    );
}
