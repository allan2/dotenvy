//! `load_and_modify` uses `std::env::set_var` internally, which is not thread-safe.
//! 
//! When modifying the environment, loading must be executed before the async runtime is spawned.
//! 
//! The *modify-tokio-macro* example contains a more ergonomic way to do this.

use dotenvy::EnvLoader;
use std::{
    env::{self, VarError},
    error,
};


fn main() -> Result<(), Box<dyn error::Error>> {
    let loader = EnvLoader::with_path("../env-example");
    unsafe { loader.load_and_modify() }?;

    // this is the expansion of `#[tokio::main]`
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            println!("HOST={}", env::var("HOST").unwrap());
            Ok::<_, VarError>(())
        })?;

    Ok(())
}
