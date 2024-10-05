//! dotenvy only handles UTF-8.
//!
//! If a source file is non-UTF-8, it can still be loaded with help from the `encoding_rs_io` crate.

use dotenvy::EnvLoader;
use encoding_rs_io::DecodeReaderBytes;
use std::{
    error::{self, Error},
    fs,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = "env-example-utf16";

    // this will fail
    let e = EnvLoader::with_path(path).load().unwrap_err();
    let io_err = e.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert_eq!(io_err.kind(), io::ErrorKind::InvalidData);

    // with `encoding_rs_io`, this will work
    let bytes = fs::read(path)?;
    let mut decoder = DecodeReaderBytes::new(&bytes[..]);
    let mut dest = Vec::new();

    // read to end to ensure the stream is fully decoded
    decoder.read_to_end(&mut dest)?;

    // `path` setter provides the path to error messages
    let env_map = EnvLoader::with_reader(&dest[..]).path(path).load()?;

    println!("HOST={}", env_map.var("HOST")?);

    Ok(())
}
