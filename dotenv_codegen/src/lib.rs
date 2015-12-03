#![cfg_attr(not(feature = "with-syntex"), feature(rustc_private))]
#![deny(warnings)]

extern crate dotenv;

#[cfg(feature = "with-syntex")]
extern crate syntex;

#[cfg(feature = "with-syntex")]
extern crate syntex_syntax as syntax;

#[cfg(not(feature = "with-syntex"))]
extern crate syntax;

#[cfg(not(feature = "with-syntex"))]
extern crate rustc_plugin;

mod dotenv_macro;

#[cfg(feature = "with-syntex")]
pub fn register(reg: &mut syntex::Registry) {
    reg.add_macro("dotenv", dotenv_macro::expand_dotenv);
}

#[cfg(not(feature = "with-syntex"))]
pub fn register(reg: &mut rustc_plugin::Registry) {
    reg.register_macro("dotenv", dotenv_macro::expand_dotenv);
}
