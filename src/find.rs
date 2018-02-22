use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use errors::*;
use iter::Iter;

pub struct Finder {
  directory: Option<PathBuf>,
  filename:  PathBuf,
}

impl Finder {
    pub fn new() -> Self {
        Finder {
            directory: None,
            filename: ".env".into(),
        }
    }

    pub fn filename<P: AsRef<Path>>(mut self, filename: P) -> Self {
        self.filename = filename.as_ref().into();
        self
    }

    pub fn find(self) -> Result<(PathBuf, Iter<File>)> {
        let directory = if let Some(directory) = self.directory {
            directory
        } else {
            env::current_dir()?
        };

        let path = find(directory, self.filename)?;
        let file = File::open(&path)?;
        let iter = Iter::new(file);
        Ok((path, iter))
    }
}

/// Searches for `filename` in `directory` and parent directories until found or root is reached.
pub fn find<P: AsRef<Path>>(directory: P, filename: P) -> Result<PathBuf> {
    let candidate = directory.as_ref().join(filename.as_ref());

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

    if let Some(parent) = directory.as_ref().parent() {
        find(parent, filename.as_ref())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "path not found").into())
    }
}
