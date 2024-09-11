use dotenvy::EnvLoader;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::env::{self, VarError};
use syn::{parse::Parser, punctuated::Punctuated, spanned::Spanned, Token};

#[proc_macro]
/// TODO: add safety warning
pub fn dotenv(input: TokenStream) -> TokenStream {
    let input = input.into();
    unsafe { dotenv_inner(input) }.into()
}

unsafe fn dotenv_inner(input: TokenStream2) -> TokenStream2 {
    let loader = EnvLoader::new();
    if let Err(e) = unsafe { loader.load_and_modify() } {
        let msg = e.to_string();
        return quote! {
            compile_error!(#msg);
        };
    }

    match expand_env(input) {
        Ok(stream) => stream,
        Err(e) => e.to_compile_error(),
    }
}

#[proc_macro]
pub fn option_dotenv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    option_dotenv_inner(input.into()).into()
}

fn option_dotenv_inner(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    if dotenvy::dotenv().is_err() {
        return quote! {::core::option::Option::None};
    }

    match expand_env(input) {
        Ok(stream) => quote! {::core::option::Option::Some(#stream)},
        Err(_) => {
            quote! {::core::option::Option::None}
        }
    }
}

fn expand_env(input_raw: TokenStream2) -> syn::Result<TokenStream2> {
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
                        format!("environment variable `{var_name}` not defined")
                    }

                    VarError::NotUnicode(s) => {
                        format!("environment variable `{var_name}` was not valid Unicode: {s:?}",)
                    }
                },
                |lit| lit.value(),
            ),
        )),
    }
}
