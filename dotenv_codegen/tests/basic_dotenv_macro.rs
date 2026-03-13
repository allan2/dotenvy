#[test]
fn dotenv_works() {
    // basic operation
    assert_eq!(dotenvy_macro::dotenv!("CODEGEN_TEST_VAR1"), "hello!");
    assert_eq!(
        dotenvy_macro::dotenv!("CODEGEN_TEST_VAR2"),
        "'quotes within quotes'"
    );

    // basic operation specifying var attribute
    assert_eq!(dotenvy_macro::dotenv!(var = "CODEGEN_TEST_VAR1"), "hello!");

    // overriding works
    assert_eq!(
        dotenvy_macro::dotenv!("CODEGEN_TEST_VAR1", override_ = true),
        "hello!"
    );

    // not overriding also works, requires env vars to be set upon running test
    assert_eq!(
        dotenvy_macro::dotenv!(var = "CODEGEN_TEST_VAR1", override_ = false),
        "goodbye!",
        "in order for dotenvy_macro tests to pass, the variable `CODEGEN_TEST_VAR1` must be set to `goodbye!`"
    );

    // custom .env path works
    assert_eq!(
        dotenvy_macro::dotenv!("CODEGEN_TEST_VAR1", path = ".env.alternative"),
        "bye!"
    );

    // custom .env path works if var present but .env missing
    assert_eq!(
        dotenvy_macro::dotenv!("CODEGEN_TEST_VAR1", path = ".env.missing"),
        "goodbye!",
        "in order for dotenvy_macro tests to pass, the variable `CODEGEN_TEST_VAR1` must be set to `goodbye!`"
    );

    // custom .env path works while not overriding
    assert_eq!(
        dotenvy_macro::dotenv!("CODEGEN_TEST_VAR1", path = ".env.alternative", override_ = false),
        "goodbye!",
        "in order for dotenvy_macro tests to pass, the variable `CODEGEN_TEST_VAR1` must be set to `goodbye!`"
    );
}

#[test]
fn dotenv_option_works() {
    // basic operation
    assert_eq!(
        dotenvy_macro::option_dotenv!("CODEGEN_TEST_VAR1"),
        Some("hello!")
    );
    assert_eq!(
        dotenvy_macro::option_dotenv!("CODEGEN_TEST_VAR_UNSET"),
        None
    );

    // overriding works
    assert_eq!(
        dotenvy_macro::option_dotenv!("CODEGEN_TEST_VAR1", override_ = true),
        Some("hello!")
    );

    // not overriding also works, requires env vars to be set upon running test
    assert_eq!(
        dotenvy_macro::option_dotenv!("CODEGEN_TEST_VAR1", override_ = false),
        Some("goodbye!"),
        "in order for dotenvy_macro tests to pass, the variable `CODEGEN_TEST_VAR1` must be set to `goodbye!`"
    );

    // custom .env path works
    assert_eq!(
        dotenvy_macro::option_dotenv!("CODEGEN_TEST_VAR1", path = "./.env.alternative"),
        Some("bye!")
    );

    // custom .env path works if var present but .env missing
    assert_eq!(
        dotenvy_macro::option_dotenv!("CODEGEN_TEST_VAR1", path = "./.env.missing"),
        Some("goodbye!")
    );

    // missing custom .env path returns None
    assert_eq!(
        dotenvy_macro::option_dotenv!("NOT_SET", path = "./.env.missing"),
        None
    );
}
