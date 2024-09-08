use std::{error, ffi::OsString, fmt, io, path::PathBuf};

use crate::iter::ParseBufError;

#[derive(Debug)]
pub enum Error {
    LineParse(String, usize),
    /// An IO error may be encountered when reading from a file or reader.
    Io(io::Error, Option<PathBuf>),
    /// The variable was not found in the environment. The `String` is the name of the variable.
    NotPresent(String),
    /// The variable was not valid unicode. The `String` is the name of the variable.
    NotUnicode(OsString, String),
    /// When `load_and_modify` is called with `EnvSequence::EnvOnly`
    ///
    /// There is nothing to modify, so we consider this an invalid operation because of the unnecessary unsafe call.
    InvalidOp,
    /// When a load function is called with no path or reader.
    ///
    /// Only `EnvLoader::default` would have no path or reader.
    NoInput,
}

impl Error {
    #[must_use]
    pub fn not_found(&self) -> bool {
        if let Self::Io(e, _) = self {
            e.kind() == io::ErrorKind::NotFound
        } else {
            false
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e, _) => Some(e),
            Self::LineParse(_, _)
            | Self::NotPresent(_)
            | Self::NotUnicode(_, _)
            | Self::InvalidOp
            | Self::NoInput => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e, path) => {
                if let Some(path) = path {
                    write!(f, "error reading '{}':, {e}", path.to_string_lossy())
                } else {
                    e.fmt(f)
                }
            }
            Self::LineParse(line, index) => write!(
                f,
                "error parsing line: '{line}', error at line index: {index}",
            ),
            Self::NotPresent(s) => write!(f, "{s} is not set"),
            Self::NotUnicode(os_str, s) => {
                write!(f, "{s} is not valid Unicode: {os_str:?}",)
            }
            Self::InvalidOp => write!(f, "modify is not permitted with `EnvSequence::EnvOnly`"),
            Self::NoInput => write!(f, "no input provided"),
        }
    }
}

impl From<(io::Error, PathBuf)> for Error {
    fn from((e, path): (io::Error, PathBuf)) -> Self {
        Self::Io(e, Some(path))
    }
}
impl From<(ParseBufError, Option<PathBuf>)> for Error {
    fn from((e, path): (ParseBufError, Option<PathBuf>)) -> Self {
        match e {
            ParseBufError::LineParse(line, index) => Self::LineParse(line, index),
            ParseBufError::Io(e) => Self::Io(e, path),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Error;
    use std::{error::Error as StdError, io};

    #[test]
    fn test_io_error_source() {
        let err = Error::Io(io::ErrorKind::PermissionDenied.into(), None);
        let io_err = err.source().unwrap().downcast_ref::<io::Error>().unwrap();
        assert_eq!(io::ErrorKind::PermissionDenied, io_err.kind());
    }

    #[test]
    fn test_line_parse_error_source() {
        let e = Error::LineParse("test line".to_string(), 2);
        assert!(e.source().is_none());
    }

    #[test]
    fn test_error_not_found_true() {
        let e = Error::Io(io::ErrorKind::NotFound.into(), None);
        assert!(e.not_found());
    }

    #[test]
    fn test_error_not_found_false() {
        let e = Error::Io(io::ErrorKind::PermissionDenied.into(), None);
        assert!(!e.not_found());
    }

    #[test]
    fn test_io_error_display() {
        let err = Error::Io(io::ErrorKind::PermissionDenied.into(), None);
        let io_err: io::Error = io::ErrorKind::PermissionDenied.into();
        assert_eq!(err.to_string(), io_err.to_string());
    }

    #[test]
    fn test_lineparse_error_display() {
        let err = Error::LineParse("test line".to_owned(), 2);
        assert_eq!(
            "error parsing line: 'test line', error at line index: 2",
            err.to_string()
        );
    }
}
