use std::error::Error as StdError;
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    CanonicalizingPath(PathBuf, io::Error),
    CreatingChildDir(PathBuf, io::Error),
    CreatingEnvFile(PathBuf, io::Error),
    CreatingTempDir(io::Error),
    EnvFileConflict(PathBuf),
    EnvFilePathSameAsTempDir,
    InvalidUtf8(FromUtf8Error),
    KeyConflict(String),
    KeyEmpty,
    PathNotFound(PathBuf),
    SettingCurrentDir(PathBuf, io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanonicalizingPath(path, err) => {
                write!(f, "canonicalizing path `{}`: {err}", path.display())
            }
            Self::CreatingChildDir(path, err) => {
                write!(f, "creating child directory `{}`: {err}", path.display())
            }
            Self::CreatingEnvFile(path, err) => {
                write!(f, "creating env file `{}`: {err}", path.display())
            }
            Self::CreatingTempDir(err) => {
                write!(f, "creating temporary directory: {err}")
            }
            Self::EnvFileConflict(path) => {
                write!(
                    f,
                    "env file path `{}` already in test environment",
                    path.display()
                )
            }
            Self::EnvFilePathSameAsTempDir => {
                write!(
                    f,
                    "env file path cannot be the same as the temporary directory"
                )
            }
            Self::InvalidUtf8(err) => write!(f, "invalid utf8: {err}"),
            Self::KeyConflict(key) => {
                write!(f, "key `{key}` already in test environment")
            }
            Self::KeyEmpty => write!(f, "key cannot be empty"),
            Self::PathNotFound(path) => write!(f, "path not found: {}", path.display()),
            Self::SettingCurrentDir(path, err) => {
                write!(
                    f,
                    "setting current directory to `{}`: {err}",
                    path.display()
                )
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidUtf8(err) => Some(err),
            Self::CanonicalizingPath(_, err)
            | Self::CreatingChildDir(_, err)
            | Self::CreatingEnvFile(_, err)
            | Self::CreatingTempDir(err)
            | Self::SettingCurrentDir(_, err) => Some(err),
            _ => None,
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidUtf8(err)
    }
}
