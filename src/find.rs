use std::{fs, io};
use std::path::PathBuf;

use errors::*;

/// Searches for `filename` in `directory` and parent directories until found or root is reached.
pub fn find(mut directory: PathBuf, filename: &str) -> Result<PathBuf> {
    let candidate = directory.join(filename);

    match fs::metadata(&candidate) {
      Ok(metadata) => if metadata.is_file() {
          return Ok(candidate);
      }
      Err(error) => {
        if error.kind() != io::ErrorKind::NotFound {
          return Err(error.into())
        }
      }
    }

    if directory.pop() {
      find(directory, filename)
    } else {
      Err(io::Error::new(io::ErrorKind::NotFound, "path not found").into())
    }
}
