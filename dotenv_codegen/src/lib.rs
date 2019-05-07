#[macro_use]
extern crate proc_macro_hack;

extern crate dotenv_codegen_impl;

#[proc_macro_hack]
pub use dotenv_codegen_impl::dotenv;