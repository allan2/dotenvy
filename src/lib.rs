/*!
This crate provides a configuration loader in the style of the [ruby dotenv gem](https://github.com/bkeepers/dotenv).
This library is meant to be used on development or testing environments in which setting environment
variables is not practical. It loads environment variables from a .env file, if available, and
mashes those with the actual environment variables provided by the operative system.
*/

extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use std::result::Result;
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ParseError {
    line: String
}

// for readability's sake
type ParsedLine = Result<Option<(String, String)>, ParseError>;
type ParsedLines = Result<Vec<(String, String)>, ParseError>;

fn parse_line(line: String) -> ParsedLine {
    let line_regex = Regex::new(concat!(r"^(\s*(",
                                        r"#.*|", // A comment, or...
                                        r"\s*|", // ...an empty string, or...
                                        r"(export\s+)?", // ...(optionally preceded by "export")...
                                        r"(?P<key>[A-Za-z_][A-Za-z0-9_]*)", // ...a key,...
                                        r"\s*=\s*", // ...then an equal sign,...
                                        r"(?P<value>.+?)", // ...and then its corresponding value.
                                        r")\s*)[\r\n]*$")).unwrap();

    line_regex.captures(&line).map_or(
        Err(ParseError{line: line.clone()}),
        |captures| {
            let key = captures.name("key");
            let value = captures.name("value");

            if key.is_some() && value.is_some() {
                Ok(Some((key.unwrap().to_string(), value.unwrap().to_string())))
            } else {
                // If there's no key and value, but capturing did not fail,
                // then this means we're dealing with a comment or an empty
                // string.
                Ok(None)
            }
        }
        )
}

fn from_file(file: File) -> Result<(), ParseError> {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parsed = parse_line(line).unwrap();
        match parsed {
            Some((key, value)) => {
                if env::var(&key).is_err() {
                    env::set_var(&key, value);
                }
                ()
            },
            None => ()
        }
    }
    Ok(())
}

/// Loads the file at the specified path.
pub fn from_path(path: &Path) -> Result<(), ParseError> {
    match File::open(path) {
        Ok(file) => from_file(file),
        Err(_) => Err(ParseError {line: "IO error".to_string()})
    }
}

/// Loads the specified file from the same directory as the current executable.
pub fn from_filename(filename: &str) -> Result<(), ParseError> {
    match env::current_exe() {
        Ok(path) => from_path(path.with_file_name(filename).as_path()),
        Err(_) => Err(ParseError {line: "Could not fetch the path of this executable".to_string()})
    }
}

/// This is usually what you want.
/// It loads the .env file located in the same directory as the current executable.
pub fn dotenv() -> Result<(), ParseError> {
    from_filename(".env")
}

#[test]
fn test_parse_line_env() {
    let input_iter = vec![
        "THIS_IS_KEY=hi this is value",
        "   many_spaces  =   wow a  maze   ",
        "export   SHELL_LOVER=1"
    ].into_iter().map(|input| input.to_string());
    let actual_iter = input_iter.map(|input| parse_line(input));

    let expected_iter = vec![
        ("THIS_IS_KEY", "hi this is value"),
        ("many_spaces", "wow a  maze"),
        ("SHELL_LOVER", "1")
    ].into_iter().map(|(key, value)| (key.to_string(), value.to_string()));

    for (expected, actual) in expected_iter.zip(actual_iter) {
        assert!(actual.is_ok());
        assert!(actual.clone().ok().unwrap().is_some());
        assert_eq!(expected, actual.ok().unwrap().unwrap());
    }
}

#[test]
fn test_parse_line_comment() {
    let input_iter = vec![
        "# foo=bar",
        "    #    "
    ].into_iter().map(|input| input.to_string());
    let mut actual_iter = input_iter.map(|input| parse_line(input));

    for actual in actual_iter {
        assert!(actual.is_ok());
        assert!(actual.ok().unwrap().is_none());
    }
}

#[test]
fn test_parse_line_invalid() {
    let input_iter = vec![
        "  invalid    ",
        "very bacon = yes indeed",
        "key=",
        "=value"
    ].into_iter().map(|input| input.to_string());
    let mut actual_iter = input_iter.map(|input| parse_line(input));

    for actual in actual_iter {
        assert!(actual.is_err());
    }
}
