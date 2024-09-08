//! A CLI tool that loads an env file before running a command.
//!
//! # Example
//!
//! Given a file *env.txt* with body `FOO=bar`, running
//!
//! ```sh
//! dotenvy -f env.txt printenv FOO
//! ```
//!
//! will output `bar`.
use clap::{Parser, Subcommand};
use dotenvy::{EnvLoader, EnvSequence};
use std::{error, fs::File, io::ErrorKind, os::unix::process::CommandExt, path::PathBuf, process};

macro_rules! die {
    ($fmt:expr) => ({
        eprintln!($fmt);
        process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprintln!($fmt, $($arg)*);
        process::exit(1);
    });
}

fn mk_cmd(program: &str, args: &[String]) -> process::Command {
    let mut cmd = process::Command::new(program);
    for arg in args {
        cmd.arg(arg);
    }
    cmd
}

#[derive(Parser)]
#[command(
    name = "dotenvy",
    version,
    about = "Run a command using an environment loaded from a .env file",
    arg_required_else_help = true,
    allow_external_subcommands = true
)]
struct Cli {
    #[arg(short, long, default_value = ".env")]
    file: PathBuf,
    #[clap(subcommand)]
    subcmd: Subcmd,
    #[arg(long, default_value_t = true)]
    required: bool,
    #[arg(long, default_value_t = false)]
    r#override: bool,
}

#[derive(Subcommand)]
enum Subcmd {
    #[clap(external_subcommand)]
    External(Vec<String>),
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    match File::open(&cli.file) {
        Ok(file) => {
            let seq = if cli.r#override {
                EnvSequence::EnvThenInput
            } else {
                EnvSequence::InputThenEnv
            };

            // load the file
            let loader = EnvLoader::with_reader(file).path(&cli.file).sequence(seq);
            if let Err(e) = unsafe { loader.load_and_modify() } {
                die!("Failed to load {path}: {e}", path = cli.file.display());
            }
        }
        Err(e) => {
            if cli.required && e.kind() == ErrorKind::NotFound {
                die!("Failed to load {path}: {e}", path = cli.file.display());
            }
        }
    };

    // prepare the command
    let Subcmd::External(args) = cli.subcmd;
    let (program, args) = args.split_first().unwrap();
    let mut cmd = mk_cmd(program, args);

    // run the command
    if cfg!(target_os = "windows") {
        match cmd.spawn().and_then(|mut child| child.wait()) {
            Ok(status) => process::exit(status.code().unwrap_or(1)),
            Err(e) => die!("fatal: {e}"),
        };
    } else {
        die!("fatal: {}", cmd.exec());
    };
}
