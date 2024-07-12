/// Create envfile contents for testing
///
/// Represented as bytes to allow for advanced manipulation and BOM testing.
#[derive(Debug, Default)]
pub struct EnvFileBuilder {
    contents: Vec<u8>,
}

impl EnvFileBuilder {
    pub fn new() -> Self {
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
    /// ## Panics
    ///
    /// If the contents of the builder are not valid UTF-8.
    pub fn build_string(&self) -> String {
        String::from_utf8(self.contents.clone()).expect("valid UTF-8")
    }

    /// Transform the builder into a byte vector.
    pub fn into_owned_bytes(self) -> Vec<u8> {
        self.contents
    }

    /// Transform the builder into a string.
    ///
    /// ## Panics
    ///
    /// If the contents of the builder are not valid UTF-8.
    pub fn into_owned_string(self) -> String {
        String::from_utf8(self.contents).expect("valid UTF-8")
    }

    /// Get a reference to the contents of the builder.
    pub fn as_bytes(&self) -> &[u8] {
        &self.contents
    }

    /// Add
    pub fn add_vars(&mut self, env_vars: &[(&str, &str)]) -> &mut Self {
        let mut many = String::new();
        for (key, value) in env_vars {
            many.push_str(key);
            many.push('=');
            many.push_str(value);
            many.push('\n');
        }
        self.add_str(&many);
        self
    }

    /// Add a key-value pair and newline
    pub fn add_key_value(&mut self, key: &str, value: &str) -> &mut Self {
        self.add_strln(&format!("{}={}", key, value))
    }

    /// Add a string without a newline
    pub fn add_str(&mut self, s: &str) -> &mut Self {
        self.add_bytes(s.as_bytes())
    }

    /// Add a string with a newline
    pub fn add_strln(&mut self, line: &str) -> &mut Self {
        self.add_str(line).add_byte(b'\n')
    }

    /// Add a byte slice
    pub fn add_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.contents.extend_from_slice(bytes);
        self
    }

    /// Add a single byte
    pub fn add_byte(&mut self, byte: u8) -> &mut Self {
        self.contents.push(byte);
        self
    }

    /// Insert the UTF-8 Byte Order Mark at the beginning of the file
    pub fn insert_utf8_bom(&mut self) -> &mut Self {
        // https://www.compart.com/en/unicode/U+FEFF
        let bom = b"\xEF\xBB\xBF";
        self.contents.splice(0..0, bom.iter().cloned());
        self
    }
}
