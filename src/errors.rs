use std::io;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error parsing line: '{}'", _0)]
    LineParse(String),
    #[fail(display = "{}", _0)]
    Io(#[cfg_attr(backtrace, cause)] ::std::io::Error),
    #[fail(display = "{}", _0)]
    EnvVar(#[cfg_attr(backtrace, cause)] ::std::env::VarError),
}

impl Error {
    pub fn not_found(&self) -> bool {
        if let Error::Io(ref io_error) = *self {
            return io_error.kind() == io::ErrorKind::NotFound;
        }
        false
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
