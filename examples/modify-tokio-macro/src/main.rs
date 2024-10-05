//! This is more ergonomic than the *modify-tokio* example.
//!
//! The attribute macro executes `load_and_modify` before the tokio runtime is spawned.
//! When using this method, `#[dotenvy::load]` be put above `#[tokio::main]`.

use std::env;

#[dotenvy::load(path = "../env-example")]
#[tokio::main]
async fn main() {
    println!("HOST={}", env::var("HOST").unwrap());
}
