use std::path::PathBuf;

use crate::Error;

#[derive(Debug, Clone)]
/// Simple path and byte contents representing a `.env` file
pub struct EnvFile {
    pub path: PathBuf,
    pub contents: Vec<u8>,
}

/// `.env` file builder.
///
/// Represented as bytes to allow for advanced manipulation and BOM testing.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EnvFileContents {
    contents: Vec<u8>,
}

impl EnvFileContents {
    pub const fn new() -> Self {
        Self {
            contents: Vec::new(),
        }
    }

    /// Build a byte vector from the contents of the builder.
    pub fn build(&self) -> Vec<u8> {
        self.contents.clone()
    }

    /// Build a string from the contents of the builder.
    ///
    /// ## Errors
    ///
    /// If the contents of the builder is not valid UTF-8.
    pub fn build_string(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.contents.clone())?)
    }

    /// Transform the builder into a byte vector.
    pub fn into_owned_bytes(self) -> Vec<u8> {
        self.contents
    }

    /// Transform the builder into a string.
    ///
    /// ## Errors
    ///
    /// If the contents of the builder is not valid UTF-8.
    pub fn into_owned_string(self) -> Result<String, Error> {
        Ok(String::from_utf8(self.contents)?)
    }

    /// Get a reference to the contents of the builder.
    pub fn as_bytes(&self) -> &[u8] {
        &self.contents
    }

    /// Returns true when the contents of the builder is empty.
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Append a key-value pair and newline
    pub fn add_var(&mut self, key: &str, value: &str) -> &mut Self {
        self.push_str(&format!("{key}={value}\n"))
    }

    /// Apeend a string
    pub fn push_str(&mut self, s: &str) -> &mut Self {
        self.push_bytes(s.as_bytes())
    }

    /// Append a byte slice
    pub fn push_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.contents.extend_from_slice(bytes);
        self
    }

    /// Append a single byte
    pub fn push(&mut self, byte: u8) -> &mut Self {
        self.contents.push(byte);
        self
    }
}

impl From<EnvFileContents> for Vec<u8> {
    fn from(builder: EnvFileContents) -> Self {
        builder.into_owned_bytes()
    }
}

impl From<Vec<u8>> for EnvFileContents {
    fn from(contents: Vec<u8>) -> Self {
        Self { contents }
    }
}

impl From<String> for EnvFileContents {
    fn from(contents: String) -> Self {
        Self {
            contents: contents.into_bytes(),
        }
    }
}

impl AsRef<[u8]> for EnvFileContents {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl PartialEq<String> for EnvFileContents {
    fn eq(&self, other: &String) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<str> for EnvFileContents {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<Vec<u8>> for EnvFileContents {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.as_bytes() == other
    }
}

impl PartialEq<[u8]> for EnvFileContents {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_bytes() == other
    }
}

impl PartialEq<EnvFileContents> for String {
    fn eq(&self, other: &EnvFileContents) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<EnvFileContents> for &str {
    fn eq(&self, other: &EnvFileContents) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<EnvFileContents> for Vec<u8> {
    fn eq(&self, other: &EnvFileContents) -> bool {
        self == other.as_bytes()
    }
}

impl PartialEq<EnvFileContents> for &[u8] {
    fn eq(&self, other: &EnvFileContents) -> bool {
        *self == other.as_bytes()
    }
}
