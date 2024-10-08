//! This example modifies the existing environment.
//!
//! This makes environment varaibles from available to subprocesses, e.g., a Python script.
//!
//! The *modify-macro* example contains a more ergonomic way to do this.

use dotenvy::{EnvLoader, EnvSequence};
use std::{env, error, fs, io, process::Command};

fn main() -> Result<(), Box<dyn error::Error>> {
    // to override, set sequence to `EnvThenInput` or `InputOnly`
    let loader = EnvLoader::with_path("../env-example").sequence(EnvSequence::InputThenEnv);
    unsafe { loader.load_and_modify() }?;

    println!("HOST={}", env::var("HOST").unwrap());
    print_host_py()?;
    Ok(())
}

/// Prints the host using Python's `os.environ.get`.
fn print_host_py() -> io::Result<()> {
    let script = fs::read_to_string("../modify/print_host.py")?;
    let output = Command::new("python3").arg("-c").arg(script).output()?;
    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
