//! `#[dotenvy::load]` must go before `#[tokio::main]`.

use std::{env, error};

#[dotenvy::load(path = "../env-example")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("HOST={}", env::var("HOST")?);
    Ok(())
}
