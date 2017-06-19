//! This crate provides a configuration loader in the style of the [ruby dotenv
//! gem](https://github.com/bkeepers/dotenv). This library is meant to be used
//! on development or testing environments in which setting environment
//! variables is not practical. It loads environment variables from a .env
//! file, if available, and mashes those with the actual environment variables
//! provided by the operating system.

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate derive_error_chain;
extern crate regex;

use std::env::{self, Vars};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::sync::{Once, ONCE_INIT};
use regex::{Captures, Regex};

#[derive(Debug, error_chain)]
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

static START: Once = ONCE_INIT;

/// After loading the dotenv file, fetches the environment variable key from the current process.
///
/// The returned result is Ok(s) if the environment variable is present and is valid unicode. If the
/// environment variable is not present, or it is not valid unicode, then Err will be returned.
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    START.call_once(|| { dotenv().ok(); });
    env::var(key).map_err(Error::from)
}

/// After loading the dotenv file, returns an iterator of (variable, value) pairs of strings,
/// for all the environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment variables at the
/// time of this invocation, modifications to environment variables afterwards will not be
/// reflected in the returned iterator.
pub fn vars() -> Vars {
    START.call_once(|| { dotenv().ok(); });
    env::vars()
}

// for readability's sake
type ParsedLine = Result<Option<(String, String)>>;

fn named_string(captures: &Captures, name: &str) -> Option<String> {
    captures.name(name).and_then(|v| Some(v.as_str().to_owned()))
}

fn parse_value(input: &str) -> Result<String> {
    let mut strong_quote = false; // '
    let mut weak_quote = false; // "
    let mut escaped = false;
    let mut expecting_end = false;

    //FIXME can this be done without yet another allocation per line?
    let mut output = String::new();

    for c in input.chars() {
        //the regex _should_ already trim whitespace off the end
        //expecting_end is meant to permit: k=v #comment
        //without affecting: k=v#comment
        //and throwing on: k=v w
        if expecting_end {
            if c == ' ' || c == '\t' {
                continue;
            } else if c == '#' {
                break;
            } else {
                bail!(ErrorKind::LineParse(input.to_owned()));
            }
        } else if strong_quote {
            if c == '\'' {
                strong_quote = false;
            } else {
                output.push(c);
            }
        } else if weak_quote {
            if escaped {
                //TODO variable expansion perhaps
                //not in this update but in the future
                //$ requires escape anyway for conformance
                //and so as not to make that future change breaking
                //TODO I tried handling literal \n \r but various issues
                //imo not worth worrying about until there's a use case
                //(actually handling backslash 0x10 would be a whole other matter)
                //then there's \v \f bell hex... etc
                match c {
                    '\\' | '"' | '$' => output.push(c),
                    _ => bail!(ErrorKind::LineParse(input.to_owned())),
                }

                escaped = false;
            } else if c == '"' {
                weak_quote = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                output.push(c);
            }
        } else {
            if escaped {
                match c {
                    '\\' | '\'' | '"' | '$' | ' ' => output.push(c),
                    _ => bail!(ErrorKind::LineParse(input.to_owned())),
                }

                escaped = false;
            } else if c == '\'' {
                strong_quote = true;
            } else if c == '"' {
                weak_quote = true;
            } else if c == '\\' {
                escaped = true;
            } else if c == '$' {
                //variable interpolation goes here later
                bail!(ErrorKind::LineParse(input.to_owned()));
            } else if c == ' ' || c == '\t' {
                expecting_end = true;
            } else {
                output.push(c);
            }
        }
    }

    //XXX also fail if escaped? or...
    if strong_quote || weak_quote {
        Err(ErrorKind::LineParse(input.to_owned()).into())
    } else {
        Ok(output)
    }
}

fn parse_line(line: String) -> ParsedLine {
    let line_regex = try!(Regex::new(concat!(r"^(\s*(",
                                             r"#.*|", // A comment, or...
                                             r"\s*|", // ...an empty string, or...
                                             r"(export\s+)?", // ...(optionally preceded by "export")...
                                             r"(?P<key>[A-Za-z_][A-Za-z0-9_]*)", // ...a key,...
                                             r"=", // ...then an equal sign,...
                                             r"(?P<value>.+?)?", // ...and then its corresponding value.
                                             r")\s*)[\r\n]*$")));

    line_regex.captures(&line)
        .map_or(Err(ErrorKind::LineParse(line.clone()).into()), |captures| {
            let key = named_string(&captures, "key");
            let value = named_string(&captures, "value");

            match (key, value) {
                (Some(k), Some(v)) => {
                    let parsed_value = try!(parse_value(&v));

                    Ok(Some((k, parsed_value)))
                }
                (Some(k), None) => {
                    // Empty string for value.
                    Ok(Some((k, String::from(""))))
                }
                _ => {
                    // If there's no key, but capturing did not
                    // fail, we're dealing with a comment
                    Ok(None)
                }
            }
        })
}

/// Loads the specified file.
fn from_file(file: File) -> Result<()> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = try!(line);
        let parsed = try!(parse_line(line));
        if let Some((key, value)) = parsed {
            if env::var(&key).is_err() {
                env::set_var(&key, value);
            }
        }
    }
    Ok(())
}

/// Attempts to load from given directory and parent directories until file is found or root is reached.
fn try_dir_with_parents(mut dir: PathBuf, filename: &str) -> Result<PathBuf> {
    let env_path = dir.join(filename);

    match from_path(&env_path) {
        Ok(()) => Ok(env_path),
        Err(Error(ErrorKind::Io(io_error), err_data)) => {
            match io_error.kind() {
                std::io::ErrorKind::NotFound => {
                    // Reuse allocation of parent path directory.
                    if dir.pop() {
                        try_dir_with_parents(dir, filename)
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "path not found").into())
                    }
                },
                _ => Err(Error(ErrorKind::Io(io_error), err_data)),
            }
        },
        Err(other) => Err(other),
    }
}

/// Loads the file at the specified absolute path.
///
/// Examples
///
/// ```
/// use dotenv;
/// use std::env;
/// use std::path::{Path};
///
/// let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
/// dotenv::from_path(my_path.as_path());
/// ```
pub fn from_path(path: &Path) -> Result<()> {
    File::open(path).map(from_file)?
}

/// Loads the specified file from the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv;
/// dotenv::from_filename("custom.env").ok();
/// ```
///
/// It is also possible to do the following, but it is equivalent to using dotenv::dotenv(), which
/// is preferred.
///
/// ```
/// use dotenv;
/// dotenv::from_filename(".env").ok();
/// ```
pub fn from_filename(filename: &str) -> Result<PathBuf> {
    let path = env::current_dir()?;

    try_dir_with_parents(path, filename)
}

/// This is usually what you want.
/// It loads the .env file located in the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv;
/// dotenv::dotenv().ok();
/// ```
pub fn dotenv() -> Result<PathBuf> {
    from_filename(&".env")
}

#[test]
fn test_parse_line_env() {
    let input_iter = vec!["KEY=1",
                          r#"KEY2="2""#,
                          "KEY3='3'",
                          "KEY4='fo ur'",
                          r#"KEY5="fi ve""#,
                          r"KEY6=s\ ix",
                          "KEY7=",
                          "KEY8=     ",
                          "KEY9=   # foo",
                          "export   SHELL_LOVER=1"]
        .into_iter()
        .map(|input| input.to_string());
    let actual_iter = input_iter.map(|input| parse_line(input));

    let expected_iter = vec![("KEY", "1"),
                             ("KEY2", "2"),
                             ("KEY3", "3"),
                             ("KEY4", "fo ur"),
                             ("KEY5", "fi ve"),
                             ("KEY6", "s ix"),
                             ("KEY7", ""),
                             ("KEY8", ""),
                             ("KEY9", ""),
                             ("SHELL_LOVER", "1")]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()));

    for (expected, actual) in expected_iter.zip(actual_iter) {
        assert!(actual.is_ok());
        assert!(actual.as_ref().unwrap().is_some());
        assert_eq!(expected, actual.ok().unwrap().unwrap());
    }
}

#[test]
fn test_parse_line_comment() {
    let input_iter = vec!["# foo=bar", "    #    "]
        .into_iter()
        .map(|input| input.to_string());
    let actual_iter = input_iter.map(|input| parse_line(input));

    for actual in actual_iter {
        assert!(actual.is_ok());
        assert!(actual.ok().unwrap().is_none());
    }
}

#[test]
fn test_parse_line_invalid() {
    let input_iter =
        vec!["  invalid    ", "KEY =val", "KEY2= val", "very bacon = yes indeed", "=value"]
            .into_iter()
            .map(|input| input.to_string());
    let actual_iter = input_iter.map(|input| parse_line(input));

    for actual in actual_iter {
        assert!(actual.is_err());
    }
}

#[test]
fn test_parse_value_escapes() {
    let input_iter = vec![r#"KEY=my\ cool\ value"#,
                          r#"KEY2=\$sweet"#,
                          r#"KEY3="awesome stuff \"mang\"""#,
                          r#"KEY4='sweet $\fgs'\''fds'"#,
                          r#"KEY5="'\"yay\\"\ "stuff""#,
                          r##"KEY6="lol" #well you see when I say lol wh"##]
        .into_iter()
        .map(|input| input.to_string());
    let actual_iter = input_iter.map(|input| parse_line(input));

    let expected_iter = vec![("KEY", r#"my cool value"#),
                             ("KEY2", r#"$sweet"#),
                             ("KEY3", r#"awesome stuff "mang""#),
                             ("KEY4", r#"sweet $\fgs'fds"#),
                             ("KEY5", r#"'"yay\ stuff"#),
                             ("KEY6", "lol")]
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.to_string()));

    for (expected, actual) in expected_iter.zip(actual_iter) {
        assert!(actual.is_ok());
        assert!(actual.as_ref().unwrap().is_some());
        assert_eq!(expected, actual.unwrap().unwrap());
    }
}

#[test]
fn test_parse_value_escapes_invalid() {
    let input_iter = vec![r#"KEY=my uncool value"#,
                          r#"KEY2=$notcool"#,
                          r#"KEY3="why"#,
                          r#"KEY4='please stop''"#,
                          r#"KEY5=h\8u"#]
        .into_iter()
        .map(|input| input.to_string());
    let actual_iter = input_iter.map(|input| parse_line(input));

    for actual in actual_iter {
        assert!(actual.is_err());
    }
}
