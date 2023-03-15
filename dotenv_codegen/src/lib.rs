#![forbid(unsafe_code)]

use std::env::{self, VarError};

use quote::{quote, ToTokens};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Token;

#[proc_macro]
pub fn dotenv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Err(error) = dotenvy::dotenv() {
        let msg = format!("Error loading .env file: {}", error);
        return quote! {
            compile_error!(#msg);
        }
        .into();
    };
    match expand_env(input.into()) {
        Ok(result) => result,
        Err(error) => error.to_compile_error(),
    }
    .into()
}

#[proc_macro]
pub fn try_dotenv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    drop(dotenvy::dotenv());
    let tokens: proc_macro2::TokenStream = input.into();
    let mut iter = tokens.into_iter();
    let env_key = iter.next().unwrap();
    // comma
    iter.next().unwrap();
    expand_env(env_key.to_token_stream())
        .unwrap_or_else(|_| {
            let env_default = iter.next().unwrap();
            env_default.to_token_stream()
        })
        .into()
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
