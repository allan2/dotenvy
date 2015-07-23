extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
  dotenv().ok();

  for (key, value) in env::vars().into_iter() {
    println!("key: {}, value: {}", key, value)
  }
}
