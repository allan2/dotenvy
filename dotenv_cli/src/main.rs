extern crate clap;
extern crate dotenv;
extern crate exec;

use clap::{App, AppSettings, Arg};
use exec::Command;
use std::process;

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

fn main() {
    let matches = App::new("dotenv")
        .about("Run a command using the environment in a .env file")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::TrailingVarArg)
        .setting(AppSettings::UnifiedHelpMessage)
        .arg(Arg::with_name("FILE")
             .short("f")
             .long("file")
             .takes_value(true)
             .help("Use a specific .env file (defaults to .env)"))
        .arg(Arg::with_name("COMMAND")
             .required(true)
             .help("The command to run"))
        .arg(Arg::with_name("ARGS")
             .multiple(true)
             .help("Command arguments (optional)"))
        .get_matches();

    match matches.value_of("FILE") {
        None => dotenv::dotenv(),
        Some(file) => dotenv::from_filename(file),
    }.unwrap_or_else(|e| die!("error: Failed to load environment: {}", e));

    let mut command = Command::new(matches.value_of("COMMAND").unwrap());
    let args = matches.values_of("ARGS")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    for argument in args {
        command.arg(argument);
    }

    let error = command.exec();
    die!("fatal: {}", error);
}
