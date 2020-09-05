use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::io::BufReader;

use crate::errors::*;
use crate::parse;

pub struct Iter<R> {
    lines: QuotedLines<BufReader<R>>,
    substitution_data: HashMap<String, Option<String>>,
}

impl<R: Read> Iter<R> {
    pub fn new(reader: R) -> Iter<R> {
        Iter {
            lines: QuotedLines {
                buf: BufReader::new(reader),
            },
            substitution_data: HashMap::new(),
        }
    }

    pub fn load(self) -> Result<()> {
        for item in self {
            let (key, value) = item?;
            if env::var(&key).is_err() {
                env::set_var(&key, value);
            }
        }

        Ok(())
    }
}

struct QuotedLines<B> {
    buf: B,
}

fn is_complete(buf: &String) -> bool {
    let mut escape = false;
    let mut strong_quote = false;
    let mut weak_quote = false;
    let mut count = 0_u32;

    for c in buf.chars() {
        if escape {
            escape = false
        } else {
            match c {
                '\\' => escape = true,
                '"' if !strong_quote => {
                    count += 1;
                    weak_quote = true
                }
                '\'' if !weak_quote => {
                    count += 1;
                    strong_quote = true
                }
                _ => (),
            }
        }
    }
    count % 2 == 0
}

impl<B: BufRead> Iterator for QuotedLines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        loop {
            match self.buf.read_line(&mut buf) {
                Ok(0) => return None,
                Ok(_n) => {
                    if is_complete(&buf) {
                        if buf.ends_with('\n') {
                            buf.pop();
                            if buf.ends_with('\r') {
                                buf.pop();
                            }
                        }
                        return Some(Ok(buf));
                    }
                }
                Err(e) => return Some(Err(Error::Io(e))),
            }
        }
    }
}

impl<R: Read> Iterator for Iter<R> {
    type Item = Result<(String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Some(Err(err)),
                None => return None,
            };

            match parse::parse_line(&line, &mut self.substitution_data) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {}
                Err(err) => return Some(Err(err)),
            }
        }
    }
}
