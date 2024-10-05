//! The `load` attribute loads and modifies the environment.
//!
//! This is more ergonomic than the *modify* example.

use std::env;

#[dotenvy::load(path = "../env-example", required = true, override_ = true)]
fn main() {
    println!("HOST={}", env::var("HOST").unwrap());
}
