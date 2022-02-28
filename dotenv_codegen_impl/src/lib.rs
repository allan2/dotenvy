#![forbid(unsafe_code)]

use std::env::{self, VarError};

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

#[proc_macro_hack]
pub fn dotenv(input: TokenStream) -> TokenStream {
    if let Err(err) = dotenvy::dotenv() {
        let msg = format!("Error loading .env file: {}", err);
        return quote! {
            compile_error!(#msg);
        }
        .into();
    }

    match expand_env(input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_env(input_raw: TokenStream) -> syn::Result<TokenStream> {
    let args = <Punctuated<syn::LitStr, Token![,]>>::parse_terminated
        .parse(input_raw)
        .expect("expected macro to be called with a comma-separated list of string literals");

    let mut iter = args.iter();

    let var_name = iter
        .next()
        .ok_or_else(|| syn::Error::new(args.span(), "dotenv! takes 1 or 2 arguments"))?
        .value();
    let err_msg = iter.next();

    if iter.next().is_some() {
        return Err(syn::Error::new(
            args.span(),
            "dotenv! takes 1 or 2 arguments",
        ));
    }

    match env::var(&var_name) {
        Ok(val) => Ok(quote!(#val).into()),
        Err(e) => Err(syn::Error::new(
            var_name.span(),
            err_msg.map_or_else(
                || match e {
                    VarError::NotPresent => {
                        format!("environment variable `{}` not defined", var_name)
                    }
                    VarError::NotUnicode(s) => format!(
                        "environment variable `{}` was not valid unicode: {:?}",
                        var_name, s
                    ),
                },
                |lit| lit.value(),
            ),
        )),
    }
}
