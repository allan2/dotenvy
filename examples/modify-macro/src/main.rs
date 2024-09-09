use std::{env, error};

#[dotenvy::load(path = "../env-example", required = true, override_ = true)]
fn main() -> Result<(), Box<dyn error::Error>> {
    println!("HOST={}", env::var("HOST")?);
    Ok(())
}
