#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate quote;

extern crate dotenv;
extern crate syn;

use std::env;

use dotenv::dotenv;
use syn::{TokenTree, Token, Lit};

proc_macro_expr_impl! {
    pub fn expand_dotenv(input: &str) -> String {
        if let Err(err) = dotenv() {
            if let &dotenv::ErrorKind::LineParse(ref line) = err.kind() {
                panic!("Error parsing .env file: {}", line);
            }
        }

        // Either everything was fine, or we didn't find an .env file (which we ignore)
        expand_env(input)
    }
}

fn expand_env(input: &str) -> String {
    let tts = syn::parse_token_trees(input).unwrap();

    if tts.is_empty() {
        panic!("dotenv! takes 1 or 2 arguments");
    }

    let mut tts = tts.into_iter();

    let var = match tts.next().unwrap() {
        TokenTree::Token(Token::Literal(Lit::Str(s, _))) => s,
        _ => panic!("expected a string literal as the first argument"),
    };

    match tts.next() {
        Some(TokenTree::Token(Token::Comma)) | None => (),
        _ => panic!("expected a comma-separated list of expressions"),
    }

    let err_msg = match tts.next() {
        Some(TokenTree::Token(Token::Literal(Lit::Str(s, _)))) => s,
        None => format!("environment variable `{}` not defined", var),
        _ => panic!("expected a string literal as the second argument"),
    };

    if tts.next().is_some() {
        panic!("dotenv! takes 1 or 2 arguments");
    }

    match env::var(&var) {
        Ok(val) => quote!(#val).to_string(),
        Err(_) => panic!("{}", err_msg),
    }
}
