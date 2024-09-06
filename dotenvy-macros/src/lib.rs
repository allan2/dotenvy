#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::uninlined_format_args, clippy::wildcard_imports)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

/// Loads environment variables from a file and modifies the environment.
///
/// Three optional arguments are supported: `path`, `required`, and `override`.
/// Usage is like `#[dotenvy::load(path = ".env", required = true, override = true)]`.
///
/// The default path is ".env". The default sequence is `EnvSequence::InputThenEnv`.
#[proc_macro_attribute]
pub fn load(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let mut path = ".env".to_owned();
    let mut required = true;
    let mut override_ = false;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(v)) = arg {
            if v.path.is_ident("path") {
                if let Lit::Str(lit_str) = v.lit {
                    path = lit_str.value();
                }
            } else if v.path.is_ident("required") {
                if let Lit::Bool(lit_bool) = v.lit {
                    required = lit_bool.value();
                }
            } else if v.path.is_ident("override") {
                if let Lit::Bool(lit_bool) = v.lit {
                    override_ = lit_bool.value();
                }
            }
        }
    }

    let load_env = quote! {
        use dotenvy::{EnvLoader, EnvSequence};
        use std::{error::Error, io::{self, ErrorKind}, process};

        let seq = if #override_ {
            EnvSequence::InputOnly
        } else {
            EnvSequence::InputThenEnv
        };
        let mut loader = EnvLoader::with_path(#path).sequence(seq);
        if let Err(e) = unsafe { loader.load_and_modify() } {
            if let Some(io_err) = e.source().and_then(|src| src.downcast_ref::<io::Error>()) {
                if io_err.kind() == io::ErrorKind::NotFound && !#required {
                    // `required` is false and file not found, so continue
                }
            }
            eprintln!("Failed to load env file from path '{}': {e}", #path);
            process::exit(1);
        }
    };

    let attrs = &input.attrs;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;
    let fn_name = &input.sig.ident;
    let output = &input.sig.output;
    let new_fn_name = format_ident!("{fn_name}_inner");

    let expanded = if sig.asyncness.is_some() {
        // this works with `tokio::main`` but not `async_std::main``
        quote! {
            // non-async wrapper function
            #vis async fn #fn_name() #output {
                #load_env
                #new_fn_name().await
            }

            // orig async function, but renamed
            #(#attrs)*
            #vis async fn #new_fn_name() #output {
                #block
            }
        }
    } else {
        // not using async, just inject `load_env` at the start
        quote! {
            #(#attrs)*
            #vis #sig {
                #load_env
                #block
            }
        }
    };

    TokenStream::from(expanded)
}
