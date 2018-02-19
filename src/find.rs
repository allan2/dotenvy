use std::fs::File;
use std::path::PathBuf;
use std::{env, fs, io};

use errors::*;
use iter::Iter;

pub struct Finder<'a> {
  directory: Option<PathBuf>,
  filename:  Option<&'a str>,
}

impl<'a> Finder<'a> {
  pub fn new() -> Finder<'a> {
    Finder {
      directory: None,
      filename:  None,
    }
  }

  pub fn filename(mut self, filename: &'a str) -> Finder<'a> {
    self.filename = Some(filename);
    self
  }

  pub fn find(self) -> Result<(PathBuf, Iter<File>)> {
    let directory = if let Some(directory) = self.directory {
      directory
    } else {
      env::current_dir()?
    };

    let filename = if let Some(filename) = self.filename {
      filename
    } else {
      ".env"
    };

    let path = find(directory, PathBuf::from(filename))?;
    let file = File::open(&path)?;
    let iter = Iter::new(file);
    Ok((path, iter))
  }
}

/// Searches for `filename` in `directory` and parent directories until found or root is reached.
pub fn find(mut directory: PathBuf, filename: PathBuf) -> Result<PathBuf> {
    let candidate = directory.join(&filename);

    match fs::metadata(&candidate) {
        Ok(metadata) => if metadata.is_file() {
            return Ok(candidate);
        },
        Err(error) => {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
    }

    if directory.pop() {
        find(directory, filename)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "path not found").into())
    }
}
