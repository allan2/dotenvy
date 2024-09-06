//! This example loads an env file only if the file exists.
//!
//! `HOST=abc cargo run`
use dotenvy::{EnvLoader, EnvSequence};
use std::{error, fs::File, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = Path::new("non-existent-env");

    // rather than checking with `Path::exists` and then opening the file handle, we call `File::open` directly to avoid a race condition where the file is inaccessible between the exist check and open
    let loader = match File::open(path) {
        Ok(file) => EnvLoader::with_reader(file).sequence(EnvSequence::InputThenEnv),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                EnvLoader::default().sequence(EnvSequence::EnvOnly)
            } else {
                return Err(e.into());
            }
        }
    };

    let env_map = loader.load()?;

    if let Some(v) = env_map.get("HOST") {
        println!("Host: {v}");
    }
    Ok(())
}
