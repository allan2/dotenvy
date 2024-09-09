use dotenvy::EnvLoader;
use std::{
    env::{self, VarError},
    error,
};

// `load_and_modify` uses `std::env::set_var` internally, which is not thread-safe.
// As such, loading must be done before the async runtime is spawned.
// This is why we don't use `#[tokio::main]` here.
fn main() -> Result<(), Box<dyn error::Error>> {
    let loader = EnvLoader::with_path("../env-example");
    unsafe { loader.load_and_modify() }?;

    // this is the expansion of `#[tokio::main]`
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            println!("HOST={}", env::var("HOST")?);
            Ok::<_, VarError>(())
        })?;

    Ok(())
}
