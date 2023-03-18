///! Use dotenvy with custom .env file path
///!
///! Run:
///! ```sh
///! cd examples/from-path
///! cargo run
///! ```   
use dotenvy::{from_path, Result};
use std::{env::current_dir, ffi::OsStr};

/// Get dotenv file path from CWD
fn dotenv_file_path() -> String {
    let path = current_dir().unwrap();
    let dir = path.parent().unwrap();
    format!("{:?}/.env", dir).replace("\"", "")
}

fn dotenv_init() -> Result<()> {
    from_path(OsStr::new(&dotenv_file_path()))
}

fn main() {
    dotenv_init().expect(".env file is missing");
    println!(
        "host: {}",
        std::env::var("HOST").expect("HOST must be set.")
    );
}
