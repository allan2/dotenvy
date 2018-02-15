#[derive(Debug, ErrorChain)]
#[cfg_attr(not(feature = "backtrace"), error_chain(backtrace = "false"))]
pub enum ErrorKind {
    // generic error string, required by derive_error_chain
    Msg(String),
    #[error_chain(custom)]
    #[error_chain(description = r#"|_| "Parsing Error""#)]
    #[error_chain(display = r#"|l| write!(f, "Error parsing line: '{}'", l)"#)]
    LineParse(String),
    #[error_chain(foreign)]
    ParseFormatter(::regex::Error),
    #[error_chain(foreign)]
    Io(::std::io::Error),
    #[error_chain(foreign)]
    EnvVar(::std::env::VarError),
}

