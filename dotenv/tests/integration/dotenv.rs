use crate::util::*;

#[test]
fn dotenv_ok_default_env() {
    test_in_default_env(|| {
        dotenvy::dotenv().ok();
        assert_env_var(DEFAULT_EXISTING_KEY, DEFAULT_EXISTING_VALUE);
        assert_env_var(DEFAULT_TEST_KEY, DEFAULT_TEST_VALUE);
    });
}
