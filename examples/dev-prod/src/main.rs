use std::{
    fmt::{self},
    str::FromStr,
};

#[derive(PartialEq)]
enum AppEnv {
    Dev,
    Prod,
}

/// A common setup that:
///  - loads from a .env file in dev mode
///  - loads from the envrironment in prod mode
///
/// A few commands to try:
/// 1) `APP_ENV=prod HOST=prod.com cargo run`
/// 2) `APP_ENV=dev HOST=prod.com cargo run`
/// 3) `APP_ENV=prod cargo run`
///
/// To have the .env file take priority, use `dotenv_orderride()`.
/// Try replacing `dotenv()` with `dotenv_override()` on line 2 and re-running command 2.
fn main() {
    let app_env = std::env::var("APP_ENV")
        .unwrap_or("dev".to_owned())
        .parse::<AppEnv>()
        .unwrap();

    println!("Running in {app_env} mode");

    if app_env == AppEnv::Dev {
        match dotenvy::dotenv() {
            Ok(path) => println!(".env read successfully from {}", path.display()),
            Err(e) => println!("Could not load .env file: {e}"),
        };
    }

    let host = dotenvy::var("HOST").expect("HOST not set");
    println!("Host: {host}");
}

impl FromStr for AppEnv {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(AppEnv::Dev),
            "prod" => Ok(AppEnv::Prod),
            _ => Err(format!("Unknown app env: {s}")),
        }
    }
}

impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}
