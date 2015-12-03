#![feature(rustc_private, plugin_registrar)]

extern crate dotenv_codegen;
extern crate rustc_plugin;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    dotenv_codegen::register(reg);
}
