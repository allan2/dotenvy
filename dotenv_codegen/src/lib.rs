#![forbid(unsafe_code)]

use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use dotenvy_codegen_impl::dotenv;
