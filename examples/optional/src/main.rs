//! This example loads an env file only if the file exists.

use dotenvy::{EnvLoader, EnvSequence};
use std::{error, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = Path::new("../.env");
    let seq = if path.exists() {
        EnvSequence::InputThenEnv
    } else {
        EnvSequence::EnvOnly
    };

    let env_map = EnvLoader::with_path(path).sequence(seq).load()?;

    if let Some(v) = env_map.get("HOST") {
        println!("Host: {v}");
    }
    Ok(())
}
