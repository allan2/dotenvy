use std::fs::File;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use crate::errors::*;
use crate::iter::Iter;

pub struct Finder<'a> {
  filename:  &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new() -> Self {
        Finder {
            filename: Path::new(".env"),
        }
    }

    pub fn filename(mut self, filename: &'a Path) -> Self {
        self.filename = filename;
        self
    }

    pub fn find(self) -> Result<Vec<(PathBuf, Iter<File>)>> {
        let paths = find(&env::current_dir().map_err(Error::Io)?, self.filename)?;

        paths
            .into_iter()
            .map(|path| match File::open(&path) {
                Ok(file) => Ok((path, Iter::new(file))),
                Err(err) => Err(Error::Io(err))
            })
            .collect::<Result<Vec<_>>>()
    }
}

/// Searches for `filename` in `directory` and parent directories until found or root is reached.
pub fn find(directory: &Path, filename: &Path) -> Result<Vec<PathBuf>> {

    let results = directory
        .ancestors()
        .map(|path| path.join(filename))
        .filter_map(|candidate| match fs::metadata(&candidate) {
            Ok(metadata) if metadata.is_file() => {
                Some(Ok(candidate))
            },
            Err(error) if error.kind() != io::ErrorKind::NotFound => {
                Some(Err(Error::Io(error)))
            },
            _ => None,
        })
        .collect::<Result<Vec<_>>>()?;
    
    if results.is_empty() {
        return Err(Error::Io(io::Error::new(io::ErrorKind::NotFound, "path not found")));
    }

    Ok(results)
}
