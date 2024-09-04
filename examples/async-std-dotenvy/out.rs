#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    async fn main() {
        use dotenvy::{EnvLoader, EnvSequence};
        use std::{
            error::Error, io::{self, ErrorKind},
            process,
        };
        let seq = if false { EnvSequence::InputOnly } else { EnvSequence::InputThenEnv };
        let mut loader = EnvLoader::from_path(".env").sequence(seq);
        if let Err(e) = unsafe { loader.load_and_modify() } {
            if let Some(io_err) = e
                .source()
                .and_then(|src| src.downcast_ref::<io::Error>())
            {
                if io_err.kind() == io::ErrorKind::NotFound && !true {}
            }
            {
                ::std::io::_eprint(
                    format_args!(
                        "Failed to load env file from path \'{0}\': {1}\n",
                        ".env",
                        e,
                    ),
                );
            };
            process::exit(1);
        }
        main_inner()
    }
    async fn main_inner() {
        {
            {
                {
                    ::std::io::_print(format_args!("Hello, world!\n"));
                };
            }
        }
    }
    async_std::task::block_on(async { main().await })
}
