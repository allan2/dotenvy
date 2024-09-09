#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::uninlined_format_args, clippy::wildcard_imports)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn, LitBool, LitStr,
};

/// Loads environment variables from a file and modifies the environment.
///
/// Three optional arguments are supported: `path`, `required`, and `override`.
/// Usage is like `#[dotenvy::load(path = ".env", required = true, override = true)]`.
///
/// The default path is ".env". The default sequence is `EnvSequence::InputThenEnv`.
#[proc_macro_attribute]
pub fn load(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as LoadInput);
    let item = parse_macro_input!(item as ItemFn);

    let path = attrs.path;
    let required = attrs.required;
    let override_ = attrs.override_;

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

    let attrs = &item.attrs;
    let block = &item.block;
    let sig = &item.sig;
    let vis = &item.vis;
    let fn_name = &item.sig.ident;
    let output = &item.sig.output;
    let new_fn_name = format_ident!("{fn_name}_inner");

    let expanded = if sig.asyncness.is_some() {
        // this works with `tokio::main`` but not `async_std::main``
        quote! {
            // non-async wrapper function
            #vis fn #fn_name() #output {
                #load_env
                #new_fn_name()
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

struct LoadInput {
    path: String,
    required: bool,
    override_: bool,
}

impl Parse for LoadInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut path = "./.env".to_owned();
        let mut required = true;
        let mut override_ = false;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            match ident.to_string().as_str() {
                "path" => {
                    path = input.parse::<LitStr>()?.value();
                }
                "required" => {
                    required = input.parse::<LitBool>()?.value();
                }
                "override_" => {
                    override_ = input.parse::<LitBool>()?.value();
                }
                _ => return Err(syn::Error::new(ident.span(), "unknown attribute")),
            }
            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(Self {
            path,
            required,
            override_,
        })
    }
}
