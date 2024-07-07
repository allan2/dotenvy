#[test]
fn dotenv_works() {
    dotenvy_macro::dotenv!();
    assert_eq!(env!("CODEGEN_TEST_VAR1"), "hello!");
}

#[test]
fn dotenv_override_works() {
    dotenvy_macro::dotenv_override!();
    assert_eq!(env!("USER"), "dotenv!");
}
