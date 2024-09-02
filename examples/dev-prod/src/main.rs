//! This example loads from an env file in development but from the environment only in production.
//!
/// Commands to try:
/// 1) `cargo run`
/// 2) `APP_ENV=prod cargo run`
/// 3) `APP_ENV=prod HOST=prod.com cargo run`
use dotenvy::{EnvLoader, EnvSequence};
use std::{env, error, str::FromStr};

fn main() -> Result<(), Box<dyn error::Error>> {
    let app_env = env::var("APP_ENV")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(AppEnv::Dev);

    let env_map = EnvLoader::from_path("../env-example")
        .sequence(app_env.into())
        .load()?;

    if let Some(v) = env_map.get("HOST") {
        println!("Host: {v}");
    } else {
        println!("HOST not set");
    }
    Ok(())
}

enum AppEnv {
    Dev,
    Prod,
}

impl FromStr for AppEnv {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            s => Err(format!("Invalid AppEnv: {s}")),
        }
    }
}

impl From<AppEnv> for EnvSequence {
    fn from(v: AppEnv) -> Self {
        match v {
            AppEnv::Dev => Self::InputThenEnv,
            AppEnv::Prod => Self::EnvOnly,
        }
    }
}
