use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use tempdir::TempDir;

pub fn tempdir_with_dotenv(dotenv_text: &str) -> io::Result<TempDir> {
    let dir = TempDir::new("rust-dotenv-test")?;
    env::set_current_dir(dir.path())?;
    let dotenv_path = dir.path().join(".env");
    let mut dotenv_file = File::create(dotenv_path)?;
    dotenv_file.write_all(dotenv_text.as_bytes())?;
    dotenv_file.sync_all()?;
    Ok(dir)
}
