use dotenvy::{EnvLoader, EnvSequence};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    // The sequence is `EnvThenInput` to load the program environment and file A.
    let map_a = EnvLoader::with_path("../env-example")
        .sequence(EnvSequence::EnvThenInput)
        .load()?;

    // the sequence is `InputOnly` as we aleady loaded the program environment in the previous step. 
    let map_b = EnvLoader::with_path("../env-example-2")
        .sequence(EnvSequence::InputOnly)
        .load()?;

    let mut env_map = map_a.clone();
    env_map.extend(map_b);

    if let Some(v) = env_map.get("HOST") {
        println!("HOST={v}");
    }
    Ok(())
}
