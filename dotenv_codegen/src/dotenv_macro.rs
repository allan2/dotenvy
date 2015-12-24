use dotenv::DotenvError::Parsing;
use dotenv::dotenv;

use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::*;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;
use syntax::parse::token;

use std::env;

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

fn expand_env(cx: &mut ExtCtxt, sp: Span, tts: &[ast::TokenTree])
    -> Box<base::MacResult>
{
    let mut exprs = match get_exprs_from_tts(cx, sp, tts) {
        Some(ref exprs) if exprs.is_empty() => {
            cx.span_err(sp, "env! takes 1 or 2 arguments");
            return DummyResult::expr(sp);
        }
        None => return DummyResult::expr(sp),
        Some(exprs) => exprs.into_iter()
    };

    let var = match expr_to_string(cx,
                                   exprs.next().unwrap(),
                                   "expected string literal") {
        None => return DummyResult::expr(sp),
        Some((v, _style)) => v
    };
    let msg = match exprs.next() {
        None => {
            token::intern_and_get_ident(&format!("environment variable `{}` \
                                                 not defined",
                                                 var))
        }
        Some(second) => {
            match expr_to_string(cx, second, "expected string literal") {
                None => return DummyResult::expr(sp),
                Some((s, _style)) => s
            }
        }
    };

    match exprs.next() {
        None => {}
        Some(_) => {
            cx.span_err(sp, "env! takes 1 or 2 arguments");
            return DummyResult::expr(sp);
        }
    }

    let e = match env::var(&var[..]) {
        Err(_) => {
            cx.span_err(sp, &msg);
            cx.expr_usize(sp, 0)
        }
        Ok(s) => cx.expr_str(sp, token::intern_and_get_ident(&s))
    };
    MacEager::expr(e)
}
