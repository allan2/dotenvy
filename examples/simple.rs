extern crate dotenv;

use dotenv::dotenv;
use std::os::env;

fn main() {
  dotenv().ok();

  for (key, value) in env().into_iter() {
    println!("key: {}, value: {}", key, value)
  }
}
