//! This example loads an env file only if the file exists.
//!
//! `HOST=abc cargo run`
use dotenvy::{EnvLoader, EnvSequence};
use std::{error, fs::File, io, path::Path};

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = Path::new("non-existent-env");

    // Rather than checking with `Path::exists` and then opening the file handle, we call `File::open` directly to avoid a race condition where the file is inaccessible between the exist check and the open call.

    // The loader is unaware of the file path because we construct it using a reader. We can still inform the reader of the file path using the `path` setter, which allows us to have a more informative error message.
    let loader = match File::open(path) {
        Ok(file) => EnvLoader::with_reader(file)
            .path(path)
            .sequence(EnvSequence::InputThenEnv),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
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
