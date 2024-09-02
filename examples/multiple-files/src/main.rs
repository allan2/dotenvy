use dotenvy::{EnvLoader, EnvSequence};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let map_a = EnvLoader::from_path("../env-example")
        .sequence(EnvSequence::EnvThenInput)
        .load()?;
    let map_b = EnvLoader::from_path("../env-example-2")
        .sequence(EnvSequence::InputOnly) // we already loaded from the environment in map_a
        .load()?;

    let mut env_map = map_a.clone();
    env_map.extend(map_b);

    if let Some(v) = env_map.get("HOST") {
        println!("HOST={v}");
    }
    Ok(())
}
