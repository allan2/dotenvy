#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate quote;

extern crate dotenv;

extern crate proc_macro2;

#[macro_use]
extern crate syn;

use std::env;

use dotenv::dotenv;

use syn::punctuated::Punctuated;
use syn::buffer::TokenBuffer;

proc_macro_expr_impl! {
    pub fn expand_dotenv(input: &str) -> String {
        if let Err(err) = dotenv() {
            if let &dotenv::ErrorKind::LineParse(ref line) = err.kind() {
                panic!("Error parsing .env file: {}", line);
            } else {
                panic!("Error loading .env file: {}", err);
            }
        }

        // Either everything was fine, or we didn't find an .env file (which we ignore)
        expand_env(input)
    }
}

fn expand_env(input_raw: &str) -> String {
    // we include () so that we can parse it as a tuple. `syn` 0.12
    let stream = input_raw
        .parse()
        .expect("expected macro usage to be valid rust code, but it was not");

    let buf = TokenBuffer::new2(stream);

    let args: Punctuated<syn::LitStr, Token![,]> = Punctuated::parse_terminated(buf.begin())
        .expect("expected macro to be called with a comma-separated list of string literals")
        .0;

    let mut iter = args.iter();

    let var_name = match iter.next() {
        Some(s) => s.value(),
        None => panic!("expected 1 or 2 arguments, found none"),
    };

    let err_msg = match iter.next() {
        Some(lit) => lit.value(),
        None => format!("environment variable `{}` not defined", var_name).into(),
    };

    if iter.next().is_some() {
        panic!("expected 1 or 2 arguments, found 3 or more");
    }

    match env::var(var_name) {
        Ok(val) => quote!(#val).to_string(),
        Err(_) => panic!("{}", err_msg),
    }
}
