use std::{env, io};
use std::fs::File;
use std::io::prelude::*;
use tempfile::{tempdir, TempDir};

pub fn tempdir_with_dotenv(dotenv_text: &str) -> io::Result<TempDir> {
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;
    let dotenv_path = dir.path().join(".env");
    let mut dotenv_file = File::create(dotenv_path)?;
    dotenv_file.write_all(dotenv_text.as_bytes())?;
    dotenv_file.sync_all()?;
    Ok(dir)
}

pub fn make_test_dotenv() -> io::Result<TempDir> {
    tempdir_with_dotenv("TESTKEY=test_val")
}

pub fn make_layered_test_dotenv() -> io::Result<TempDir> {
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;
    
    let dotenv_path = dir.path().join(".env");
    let mut dotenv_file = File::create(dotenv_path)?;
    dotenv_file.write_all("TESTKEY=test_val\nTESTKEY2=test_val_outer".as_bytes())?;
    dotenv_file.sync_all()?;

    let inner_dir = dir.path().join("inner");
    std::fs::create_dir(&inner_dir)?;
    env::set_current_dir(&inner_dir)?;

    let inner_dotenv_path = inner_dir.join(".env");
    let mut inner_dotenv_file = File::create(inner_dotenv_path)?;
    inner_dotenv_file.write_all("TESTKEY2=test_val_inner".as_bytes())?;
    inner_dotenv_file.sync_all()?;

    Ok(dir)
}

