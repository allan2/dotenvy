use dotenvy_test_util::*;

const TEST_KEY: &str = "TEST_KEY";
const TEST_VALUE: &str = "test_val";
const EXISTING_KEY: &str = "EXISTING_KEY";
const EXISTING_VALUE: &str = "loaded_from_env";

const TEST_ENV_FILE: &str = r"
TEST_KEY=test_val
EXISTING_KEY=loaded_from_file
";

#[test]
fn dotenv_ok() {
    let mut testenv = TestEnv::new();
    testenv.add_env_file(".env", TEST_ENV_FILE);
    testenv.add_env_var(EXISTING_KEY, EXISTING_VALUE);
    test_in_env(&testenv, || {
        dotenvy::dotenv().ok();
        assert_env_var(TEST_KEY, TEST_VALUE);
        assert_env_var(EXISTING_KEY, EXISTING_VALUE);
    });
}
