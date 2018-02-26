#[macro_use]
extern crate dotenv_codegen;

#[test]
fn dotenv_works() {
    assert_eq!(dotenv!("CODEGEN_TEST_VAR1"), "hello!");
}

#[test]
fn two_argument_form_works() {
    assert_eq!(
        dotenv!(
            "CODEGEN_TEST_VAR2",
            "err, you should be running this in the 'dotenv_codegen' \
             directory to pick up the right .env file."
        ),
        "'quotes within quotes'"
    );
}
