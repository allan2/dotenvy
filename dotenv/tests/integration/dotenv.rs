use crate::util::*;

#[test]
fn dotenv_ok_default_env() {
    test_in_default_env(|| {
        dotenvy::dotenv().ok();
        assert_env_var(TEST_EXISTING_KEY, TEST_EXISTING_VALUE);
        assert_env_var(TEST_KEY, TEST_VALUE);
    });
}

