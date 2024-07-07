#[test]
fn dotenv_works() {
    dotenvy_macro::dotenv!();
    assert_eq!(env!("CODEGEN_TEST_VAR1"), "hello!");
}
