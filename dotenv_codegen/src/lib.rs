#![forbid(unsafe_code)]

use quote::quote;

#[proc_macro]
pub fn dotenv(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Err(err) = dotenvy::dotenv() {
        let msg = format!("Error loading .env file: {}", err);
        return quote! {
            compile_error!(#msg);
        }
        .into();
    }

    quote! {}.into()
}

#[proc_macro]
pub fn dotenv_override(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Err(err) = dotenvy::dotenv_override() {
        let msg = format!("Error loading .env file: {}", err);
        return quote! {
            compile_error!(#msg);
        }
        .into();
    }

    quote! {}.into()
}
