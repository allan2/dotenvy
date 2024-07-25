//! Test environment setup, assertions and helpers.
//!
//! Setup a [`TestEnv`] and run your tests via [`test_in_env`]. The environment
//! can be tweaked with:
//!
//! - pre-existing environment variables,
//! - different directory layouts,
//! - custom `.env` file contents,
//! - multiple `.env` files,
//! - custom envfile name/path.
//!
//! Use the `create_` helper functions, such as [`create_custom_envfile`], to
//! generate the `.env` file contents. If you need more control of the
//! envfile's bytes, use the [`EnvFileBuilder`].
//!
//! In your tests, call the [`dotenvy`] API, then make use of the `assert_`
//! helpers, such as [`assert_env_var`] and [`assert_env_var_unset`], to check
//! the state of the environment.
//!
//! ## Example
//!
//! ```no_run
//! use dotenvy_test_util::*;
//! use dotenvy::dotenv_override;
//!
//! const EXISTING_KEY: &str = "TEST_KEY";
//! const EXISTING_VAL: &str = "test_val";
//! const OVERRIDING_VAL: &str = "overriding_val";
//!
//! #[test]
//! fn dotenv_override_existing_key() {
//!     // setup testing environment
//!     let mut testenv = TestEnv::init();
//!
//!     // with an existing environment variable
//!     testenv.add_env_var(EXISTING_KEY, EXISTING_VAL);
//!
//!     // with an envfile that overrides it
//!     testenv.add_envfile(
//!         ".env",
//!         create_custom_envfile(&[(EXISTING_KEY, OVERRIDING_VAL)]),
//!     );
//!
//!     // execute a closure in the testing environment
//!     test_in_env(&testenv, || {
//!         dotenv_override().expect(".env should be loaded");
//!         assert_env_var(EXISTING_KEY, OVERRIDING_VAL);
//!     });
//!     // any changes to environment variables will be reset for other tests
//! }
//! ```

mod assertions;
mod envfile;
mod testenv;

#[cfg(test)]
mod tests;

pub use assertions::*;
pub use envfile::*;
pub use testenv::*;

/// Default key used in envfile
pub const DEFAULT_TEST_KEY: &str = "DEFAULT_TEST_KEY";
/// Default value used in envfile
pub const DEFAULT_TEST_VALUE: &str = "default_test_val";

/// Default existing key set before test is run
pub const DEFAULT_EXISTING_KEY: &str = "DEFAULT_EXISTING_KEY";
/// Default existing value set before test is run
pub const DEFAULT_EXISTING_VALUE: &str = "loaded_from_env";
/// Default overriding value in envfile
pub const DEFAULT_OVERRIDING_VALUE: &str = "loaded_from_file";
