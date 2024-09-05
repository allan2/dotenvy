use std::{
    env::{self},
    error, fmt, io, result,
};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    LineParse(String, usize),
    /// An IO error may be encountered when reading from a file or reader.
    Io(io::Error),
    EnvVar(env::VarError),
    /// When `load_and_modify` is called with `EnvSequence::EnvOnly`
    ///
    /// There is nothing to modify, so we consider this an invalid operation because of the unnecessary unsafe call.
    InvalidOp,
    /// When a load function is called with no path or reader.
    ///
    /// Only `EnvLoader::default` would have no path or reader.
    NoInput,
}


impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
impl Error {
    #[must_use]
    pub fn not_found(&self) -> bool {
        if let Self::Io(e) = self {
            e.kind() == io::ErrorKind::NotFound
        } else {
            false
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::EnvVar(e) => Some(e),
            Self::InvalidOp | Self::LineParse(_, _) | Self::NoInput => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::EnvVar(e) => e.fmt(f),
            Self::LineParse(line, index) => write!(
                f,
                "Error parsing line: '{line}', error at line index: {index}",
            ),
            Self::InvalidOp => write!(f, "Modify is not permitted with `EnvSequence::EnvOnly`"),
            Self::NoInput => write!(f, "No input provided"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Error;
    use std::{env, error::Error as StdError, io};

    #[test]
    fn test_io_error_source() {
        let err = Error::Io(io::ErrorKind::PermissionDenied.into());
        let io_err = err.source().unwrap().downcast_ref::<io::Error>().unwrap();
        assert_eq!(io::ErrorKind::PermissionDenied, io_err.kind());
    }

    #[test]
    fn test_envvar_error_source() {
        let err = Error::EnvVar(env::VarError::NotPresent);
        let var_err = err
            .source()
            .unwrap()
            .downcast_ref::<env::VarError>()
            .unwrap();
        assert_eq!(&env::VarError::NotPresent, var_err);
    }

    #[test]
    fn test_line_parse_error_source() {
        let e = Error::LineParse("test line".to_string(), 2);
        assert!(e.source().is_none());
    }

    #[test]
    fn test_error_not_found_true() {
        let e = Error::Io(io::ErrorKind::NotFound.into());
        assert!(e.not_found());
    }

    #[test]
    fn test_error_not_found_false() {
        let e = Error::Io(io::ErrorKind::PermissionDenied.into());
        assert!(!e.not_found());
    }

    #[test]
    fn test_io_error_display() {
        let err = Error::Io(io::ErrorKind::PermissionDenied.into());
        let io_err: io::Error = io::ErrorKind::PermissionDenied.into();
        assert_eq!(err.to_string(), io_err.to_string());
    }

    #[test]
    fn test_envvar_error_display() {
        let err = Error::EnvVar(env::VarError::NotPresent);
        let var_err = env::VarError::NotPresent;
        assert_eq!(err.to_string(), var_err.to_string());
    }

    #[test]
    fn test_lineparse_error_display() {
        let err = Error::LineParse("test line".to_owned(), 2);
        assert_eq!(
            "Error parsing line: 'test line', error at line index: 2",
            err.to_string()
        );
    }
}
