#![forbid(unsafe_code)]

use quote::quote;
use std::env::{self, VarError};
use syn::{parse::Parser, punctuated::Punctuated, spanned::Spanned, Token};

#[proc_macro]
pub fn dotenv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dotenv_inner(input.into()).into()
}

fn dotenv_inner(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if let Err(err) = dotenvy::dotenv() {
        let msg = format!("Error loading .env file: {}", err);
        return quote! {
            compile_error!(#msg);
        };
    }

    match expand_env(input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error(),
    }
}

fn expand_env(input_raw: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let args = <Punctuated<syn::LitStr, Token![,]>>::parse_terminated
        .parse(input_raw.into())
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
        Ok(val) => Ok(quote!(#val)),
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
