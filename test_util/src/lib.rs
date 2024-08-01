#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::wildcard_imports,
    clippy::module_name_repetitions
)]

//! Test environment setup, assertions and helpers.
//!
//! Setup a [`TestEnv`] and run your tests via [`test_in_env`]. The environment
//! can be tweaked with:
//!
//! - pre-existing environment variables,
//! - different directory layouts,
//! - custom `.env` file contents,
//! - multiple `.env` files,
//! - custom env file name/path.
//!
//! Customize your env files using [`EnvFileContents`].
//!
//! In your tests, call your environment altering functions such as the
//! [`dotenvy`] API, then make use of the `assert_` helpers, such as
//! [`assert_env_var`] and [`assert_env_var_unset`], to check the state of
//! the environment.
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
//! fn dotenv_override_existing_key() -> Result<(), Error> {
//!     // setup testing environment
//!     let mut testenv = TestEnv::new()?;
//!
//!     // with an existing environment variable
//!     testenv.add_env_var(EXISTING_KEY, EXISTING_VAL)?;
//!
//!     // with an env file that overrides it
//!     testenv.add_env_file(
//!         ".env",
//!         format!("{EXISTING_KEY}={OVERRIDING_VAL}"),
//!     )?;
//!
//!     // execute a closure in the testing environment
//!     test_in_env(&testenv, || {
//!         dotenv_override().expect(".env should be loaded");
//!         assert_env_var(EXISTING_KEY, OVERRIDING_VAL);
//!     })
//!     // any changes to environment variables will be reset for other tests
//! }
//! ```
//!
//! [`dotenvy`]: https://docs.rs/dotenvy

mod assertions;
mod env_file;
mod error;
mod testenv;

#[cfg(test)]
mod tests;

pub use assertions::*;
pub use env_file::*;
pub use error::*;
pub use testenv::*;
