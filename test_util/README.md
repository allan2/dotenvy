# dotenvy test util

This is an internal package used for testing dotenvy.

## Why

Eases the annoyance of setting up custom `.env` files, managing existing
environment variables, and running multiple tests at once.

## How

By setting up a `TestEnv`, and running a closure via `test_in_env`.

**Before** executing the closure, the `TestEnv` will:

- Create a temporary directory
- Lock the environment from other tests
- Store the existing environment variables
- Add any custom env_vars to the environment
- Create any custom envfiles in the temporary directory

**In the closure** you can use the assertion functions to test the environment.

**After** executing the closure, the `TestEnv` will:

- Remove the temporary directory
- Restore the environment variables to the original state
- Unlock the environment

See the API docs for more details. For now, they must be built locally with
`cargo doc`.

### Commented example

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv_override;

const EXISTING_KEY: &str = "TEST_KEY";
const EXISTING_VAL: &str = "test_val";
const OVERRIDING_VAL: &str = "overriding_val";

#[test]
fn dotenv_override_existing_key() {
    // setup testing environment
    let mut testenv = TestEnv::init();

    // with an existing environment variable
    testenv.add_env_var(EXISTING_KEY, EXISTING_VAL);

    // with an envfile that overrides it
    testenv.add_envfile(
        ".env",
        create_custom_envfile(&[(EXISTING_KEY, OVERRIDING_VAL)]),
    );

    // execute a closure in the testing environment
    test_in_env(&testenv, || {
        dotenv_override().expect(".env should be loaded");
        assert_env_var(EXISTING_KEY, OVERRIDING_VAL);
    });
    // any changes to environment variables will be reset for other tests
}
```

### Default TestEnv

Use the default `TestEnv` for simple tests.

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv;

#[test]
fn dotenv_works() {
    test_in_default_env(|| {
        dotenv().expect(".env should be loaded");        
        assert_env_var(DEFAULT_KEY, DEFAULT_VAL);
    })  
}
```

The default `TestEnv` has 1 existing environment variable:

- `DEFAULT_EXISTING_KEY` set to `DEFAULT_EXISTING_VAL`

It has an envfile `.env` that sets:

- `DEFAULT_TEST_KEY` to `DEFAULT_TEST_VAL`
- `DEFAULT_EXISTING_KEY` to `DEFAULT_OVERRIDING_VAL`

### Customised Envfile

Use the `EnvFileBuilder` to manipulate the content of an envfile. Useful
for byte-order-mark(BOM) testing, and other edge cases.

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv;

#[test]
fn comments_ignored_in_utf8bom_envfile() {
    let mut efb = EnvFileBuilder::new();
    efb.insert_utf8_bom();
    efb.add_strln("# TEST_KEY=TEST_VAL");
    let envfile = efb.into_owned_bytes();

    let testenv = TestEnv::init_with_envfile(envfile);

    test_in_env(&testenv, || {
        dotenv().expect(".env should be loaded");
        assert_env_var_unset("TEST_KEY");
    });
}
```

Or use anything that can be converted to a `Vec<u8>` if your envfile is
simple.

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv;

#[test]
fn comments_ignored() {
    let envfile = "# TEST_KEY=TEST_VAL\n";

    let testenv = TestEnv::init_with_envfile(envfile);

    test_in_env(&testenv, || {
        dotenv().expect(".env should be loaded");
        assert_env_var_unset("TEST_KEY");
    });
}
```

