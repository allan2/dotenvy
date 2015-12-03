use dotenv::DotenvError::Parsing;
use dotenv::dotenv;

use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::*;
use syntax::ext::env::expand_env;

pub fn expand_dotenv<'cx>(cx: &'cx mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
                       -> Box<MacResult+'cx> {
    match dotenv() {
        Err(Parsing { line }) => {
            cx.span_err(sp, &format!("Error parsing .env file: {}", line));
            return DummyResult::expr(sp);
        }
        _ => {} // Either everything was fine, or we didn't find a .env file (which we ignore)
    }
    expand_env(cx, sp, tts)
}

