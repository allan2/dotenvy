#![feature(phase)]

extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use std::io::{File, IoResult, IoError, OtherIoError};
use std::collections::HashMap;
use std::default::Default;
use std::os;

pub struct Dotenv {
	env: HashMap<Vec<u8>, Vec<u8>>
}

fn merge_env_hashes(
	a: HashMap<Vec<u8>, Vec<u8>>,
	b: HashMap<Vec<u8>, Vec<u8>>,
) -> HashMap<Vec<u8>, Vec<u8>> {
	let mut result = a.clone();
	result.extend(b.into_iter());
	result
}

fn parse_env(data: Vec<u8>) -> HashMap<Vec<u8>, Vec<u8>> {
	let line_splitter = regex!(r"[\r\n]+");
	let env_format = regex!(
		r"^(?P<key>[A-Za-z_][A-Za-z0-9_]*)\s*=\s*(?P<value>.+)$"
	);

	let data_string = String::from_utf8_lossy(data.as_slice());
	let data_slice = data_string.as_slice();

	line_splitter.split(data_slice).filter_map(|line| {
		env_format.captures(line).and_then(|captures| {
			let key = captures.name("key");
			let value = captures.name("value");

			// regex captures return empty string on failure
			if key == "" || value == "" {
				None
			} else {
				Some((
					String::from_str(key).into_bytes(),
					String::from_str(value).into_bytes()
				))
			}
		})
	}).collect()
}

impl Dotenv {
	pub fn dotenv() -> Dotenv {
		Dotenv::from_filename(".env").ok().unwrap_or_default()
	}

	pub fn from_str(s: &str) -> Dotenv {
		Dotenv::from_bytes(String::from_str(s).into_bytes())
	}

	pub fn from_bytes(bytes: Vec<u8>) -> Dotenv {
		let dotenv_hash = parse_env(bytes);
		let env_hash = os::env_as_bytes().into_iter().collect();
		Dotenv{env: merge_env_hashes(env_hash, dotenv_hash)}
	}

	pub fn from_file(mut file: File) -> IoResult<Dotenv> {
		file.read_to_end().map(|file_contents|
			Dotenv::from_bytes(file_contents)
		)
	}

	pub fn from_filename(n: &str) -> IoResult<Dotenv> {
		os::self_exe_path().as_mut().map(|path| {
			path.push(n);
			Dotenv::from_path(path)
		}).unwrap_or(Err(IoError{
			kind: OtherIoError,
			desc: "Could not fetch the path of this executable",
			detail: None
		}))
	}

	pub fn from_path(path: &Path) -> IoResult<Dotenv> {
		File::open(path).and_then(|file| {
			Dotenv::from_file(file)
		})
	}

	pub fn getenv(&self, n: &str) -> Option<String> {
		let n_bytes = &String::from_str(n).into_bytes();
		self.env.get(n_bytes).and_then(|bytes| {
			String::from_utf8(bytes.clone()).ok()
		})
	}

	pub fn getenv_as_bytes(&self, n: &str) -> Option<Vec<u8>> {
		let n_bytes = &String::from_str(n).into_bytes();
		self.env.get(n_bytes).map(|bytes| {
			bytes.clone()})
		}

	pub fn env(&self) -> Vec<(String, String)> {
		self.env.iter().filter_map(|(key, value)| {
			String::from_utf8(key.clone()).ok().and_then(|key_string| {
				String::from_utf8(value.clone()).ok().map(|value_string| {
					(key_string.clone(), value_string)
				})
			})
		}).collect()
	}

	pub fn env_as_bytes(&self) -> Vec<(Vec<u8>, Vec<u8>)> {
		self.env.iter().map(|(key, value)| {
			(key.clone(), value.clone())
		}).collect()
	}
}

impl Default for Dotenv {
	fn default() -> Dotenv {
		Dotenv::from_bytes(Default::default())
	}
}

#[test]
fn merge_env_hashes_test() {
	let hash_a: HashMap<Vec<u8>, Vec<u8>> = vec![
		(
			String::from_str("bacon").into_bytes(),
			String::from_str("bad").into_bytes()
		), (
			String::from_str("cabbage").into_bytes(),
			String::from_str("neutral").into_bytes()
		)
	].into_iter().collect();

	let hash_b: HashMap<Vec<u8>, Vec<u8>> = vec![
		(
			String::from_str("bacon").into_bytes(),
			String::from_str("good").into_bytes()
		)
	].into_iter().collect();

	let expected_hash: HashMap<Vec<u8>, Vec<u8>> = vec![
		(
			String::from_str("bacon").into_bytes(),
			String::from_str("good").into_bytes()
		), (
			String::from_str("cabbage").into_bytes(),
			String::from_str("neutral").into_bytes()
		)
	].into_iter().collect();

	let actual_hash = merge_env_hashes(hash_a, hash_b);

	assert_eq!(expected_hash, actual_hash)
}

#[test]
fn parse_env_test_basic() {
	let input = String::from_str(concat!(
		"bacon=good", "\n",
		"cabbage=neutral", "\n"
	)).into_bytes();

	let expected_hash: HashMap<Vec<u8>, Vec<u8>> = vec![
		(
			String::from_str("bacon").into_bytes(),
			String::from_str("good").into_bytes()
		), (
			String::from_str("cabbage").into_bytes(),
			String::from_str("neutral").into_bytes()
		)
	].into_iter().collect();

	let actual_hash = parse_env(input);

	assert_eq!(expected_hash, actual_hash);
}

#[test]
fn parse_env_test_edges() {
	let input = String::from_str(concat!(
		"chili=spicy=EVIL", "\n",       // equals on value, uppercase on value
		"to_MAH_to=okay", "\n",         // uppercase and underscores on key
		"bacon   =  good", "\n\r\n\r",  // spaces around equal, newlines
		"cabbage=not evil"              // space on value, no final newline
	)).into_bytes();

	let expected_hash: HashMap<Vec<u8>, Vec<u8>> = vec![
		(
			String::from_str("chili").into_bytes(),
			String::from_str("spicy=EVIL").into_bytes()
		), (
			String::from_str("to_MAH_to").into_bytes(),
			String::from_str("okay").into_bytes()
		), (
			String::from_str("bacon").into_bytes(),
			String::from_str("good").into_bytes()
		), (
			String::from_str("cabbage").into_bytes(),
			String::from_str("not evil").into_bytes()
		)
	].into_iter().collect();

	let actual_hash = parse_env(input);

	assert_eq!(expected_hash, actual_hash);
}
