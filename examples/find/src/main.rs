//! This example shows finding an env file by filename.
use dotenvy::EnvLoader;
use std::{
    env, error, fs, io,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let filename = "env-example";

    println!("Looking for env file with filename: `{filename}`");
    let path = find(env::current_dir()?.as_path(), filename)?;
    println!("Env file found at `{}`", path.display());

    let env_map = EnvLoader::with_path(path).load()?;
    if let Some(v) = env_map.get("HOST") {
        println!("HOST={v}");
    }
    Ok(())
}

/// Searches for the filename in the directory and parent directories until the file is found or the filesystem root is reached.
pub fn find(mut dir: &Path, filename: &str) -> io::Result<PathBuf> {
    loop {
        let candidate = dir.join(filename);

        match fs::metadata(&candidate) {
            Ok(metadata) => {
                if metadata.is_file() {
                    return Ok(candidate);
                }
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::NotFound {
                    return Err(e);
                }
            }
        }

        if let Some(parent) = dir.parent() {
            dir = parent;
        } else {
            return Err(io::ErrorKind::NotFound.into());
        }
    }
}
