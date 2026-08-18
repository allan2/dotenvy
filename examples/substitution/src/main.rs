//! Demonstrates variable substitution in `.env` files.
//!
//! dotenvy supports referencing previously defined variables using `$VAR` or
//! `${VAR}` syntax. Use `${VAR}` when the variable name contains underscores.
//! Single-quoted values and escaped dollar signs (`\$`) are treated literally.
//!
//! Run with: `cargo run`

use dotenvy::{EnvLoader, EnvSequence};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let env_map = EnvLoader::with_path("../env-substitution")
        .sequence(EnvSequence::InputOnly)
        .load()?;

    println!("--- Variable Substitution Demo ---\n");

    println!("Base values:");
    println!("  BASE_URL = {}", env_map.var("BASE_URL")?);
    println!("  PORT     = {}", env_map.var("PORT")?);

    println!("\nSubstituted with ${{VAR}} syntax:");
    println!("  API_URL    = {}", env_map.var("API_URL")?);
    println!("  HEALTH_URL = {}", env_map.var("HEALTH_URL")?);

    println!("\nSubstituted with $VAR syntax:");
    println!("  MESSAGE = {}", env_map.var("MESSAGE")?);

    println!("\nComposed DATABASE_URL:");
    println!("  DATABASE_URL = {}", env_map.var("DATABASE_URL")?);

    println!("\nSingle quotes prevent substitution:");
    println!("  LITERAL = {}", env_map.var("LITERAL")?);

    println!("\nEscaped dollar sign:");
    println!("  ESCAPED = {}", env_map.var("ESCAPED")?);

    Ok(())
}
