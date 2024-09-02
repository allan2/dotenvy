use std::{
    env,
    fs::File,
    io::{self, Write},
};
use tempfile::{tempdir, TempDir};

pub unsafe fn tempdir_with_dotenv(text: &str) -> io::Result<TempDir> {
    unsafe { env::set_var("EXISTING", "from_env") };
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;
    let path = dir.path().join(".env");
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    file.sync_all()?;
    Ok(dir)
}

#[allow(dead_code)]
pub unsafe fn make_test_dotenv() -> io::Result<TempDir> {
    unsafe {
        tempdir_with_dotenv("TESTKEY=test_val\nTESTKEY=test_val_overridden\nEXISTING=from_file")
    }
}
