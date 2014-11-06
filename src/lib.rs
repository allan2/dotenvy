#![feature(phase)]

extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use std::io::{BufferedReader, File, IoError, OtherIoError};
use std::os::{getenv, setenv, self_exe_path};

#[deriving(Show, Clone)]
pub struct ParseError {
	line: String
}

#[deriving(Show, Clone)]
pub enum DotenvError {
	Parse(ParseError),
	Io(IoError)
}

fn parse_line(line: String) -> Result<Option<(String, String)>, ParseError> {
	let line_regex = regex!(concat!(r"^(\s*(",
		r"#.*|", // A comment, or...
		r"(export\s+)?", // ...(optionally preceded by "export")...
		r"(?P<key>[A-Za-z_][A-Za-z0-9_]*)", // ...a key,...
		r"\s*=\s*", // ...then an equal sign,...
		r"(?P<value>.+?)", // ...and then its corresponding value.
	r")\s*)$"));

	line_regex.captures(line.as_slice()).map_or(
		Err(ParseError{line: line.clone()}),
		|captures| {
			let key = captures.name("key");
			let value = captures.name("value");

			if key == "" || value == "" {
				// If there's no key and value, but capturing did not fail,
				// then this means we're dealing with a comment.
				Ok(None)
			} else {
				Ok(Some((key.to_string(), value.to_string())))
			}
		}
	)
}

fn parse_line_iter<T: Iterator<String>>(lines: T) -> Result<Vec<(String, String)>, ParseError> {
	let mut parsed_lines = lines.map(parse_line);
	let failure = parsed_lines.find(|line| line.is_err());

	if failure.is_some() {
		return Err(failure.unwrap().err().unwrap());
	}

	Ok(parsed_lines.filter_map(|line| {
		line.clone().unwrap()
	}).collect())
}

fn lines_to_env(lines: Vec<(String, String)>) {
	for (key, value) in lines.into_iter() {
		if getenv(key.as_slice()).is_none() {
			setenv(key.as_slice(), value);
		}
	}
}

fn from_file(file: File) -> Result<(), DotenvError> {
	let mut reader = BufferedReader::new(file);
	let lines = reader.lines();

	parse_line_iter(lines.filter_map(|result| {
		result.ok()
	})).map(lines_to_env).map_err(|err| {
		Parse(err)
	})
}

pub fn from_path(path: &Path) -> Result<(), DotenvError> {
	match File::open(path) {
		Ok(file) => from_file(file),
		Err(err) => Err(Io(err))
	}
}

pub fn from_filename(filename: &str) -> Result<(), DotenvError> {
	self_exe_path().as_mut().map(|path| {
		path.push(filename);
		from_path(path)
	}).unwrap_or(Err(Io(IoError{
		kind: OtherIoError,
		desc: "Could not fetch the path of this executable",
		detail: None
	})))
}

pub fn dotenv() -> Result<(), DotenvError> {
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
		assert!(actual.clone().ok().unwrap().is_some())
		assert_eq!(expected, actual.ok().unwrap().unwrap())
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

#[test]
fn test_from_line_iter_valid() {
	let input = vec![
		"test_env_one=1",
		"# a comment",
		"test_env_two=2"
	].into_iter().map(|line| line.to_string());
	let actual = parse_line_iter(input);

	assert!(actual.is_ok())
}

#[test]
fn test_from_line_iter_invalid() {
	let input = vec![
		"test_env_one=1",
		"# a comment",
		"not valid"
	].into_iter().map(|line| line.to_string());
	let actual = parse_line_iter(input);

	assert!(actual.is_err())
}
