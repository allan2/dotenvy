#[macro_use]
extern crate proc_macro_hack;

extern crate dotenv_codegen_implementation;

#[proc_macro_hack]
pub use dotenv_codegen_implementation::dotenv;