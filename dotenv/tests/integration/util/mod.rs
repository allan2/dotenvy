mod assertions;
mod envfile;
mod testenv;
mod tests;
mod wrapper;

pub use assertions::*;
pub use envfile::*;
pub use testenv::*;
pub use wrapper::*;

/// Default key used in envfile
pub const DEFAULT_TEST_KEY: &str = "DEFAULT_TEST_KEY";
/// Default value used in envfile
pub const DEFAULT_TEST_VALUE: &str = "default_test_val";

/// Default existing key set before test is run
pub const DEFAULT_EXISTING_KEY: &str = "DEFAULT_EXISTING_KEY";
/// Default existing value set before test is run
pub const DEFAULT_EXISTING_VALUE: &str = "loaded_from_env";
/// Default overriding value in envfile
pub const TEST_OVERRIDING_VALUE: &str = "loaded_from_file";
