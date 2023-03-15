#[test]
fn dotenv_works() {
    assert_eq!(dotenvy_macro::dotenv!("CODEGEN_TEST_VAR1"), "hello!");
}

#[test]
fn two_argument_form_works() {
    assert_eq!(
        dotenvy_macro::dotenv!(
            "CODEGEN_TEST_VAR2",
            "err, you should be running this in the 'dotenv_codegen' \
             directory to pick up the right .env file."
        ),
        "'quotes within quotes'"
    );
}

#[test]
fn optional_dotenv() {
    assert_eq!(
        dotenvy_macro::try_dotenv!("non existing env", "unexpected sentence"),
        "unexpected sentence"
    );
}
