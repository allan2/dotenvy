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

    /// Loads all variables found in the `reader` into the environment.
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

enum ParseState {
    Complete,
    Escape,
    StrongOpen,
    StrongOpenEscape,
    WeakOpen,
    WeakOpenEscape,
    Comment,
}

fn eval_end_state(prev_state: ParseState, buf: &str) -> ParseState {
    let mut cur_state = prev_state;

    for c in buf.chars() {
        cur_state = match cur_state {
            ParseState::Escape => ParseState::Complete,
            ParseState::Complete => match c {
                '#' => return ParseState::Comment,
                '\\' => ParseState::Escape,
                '"' => ParseState::WeakOpen,
                '\'' => ParseState::StrongOpen,
                _ => ParseState::Complete,
            },
            ParseState::WeakOpen => match c {
                '#' => return ParseState::Comment,
                '\\' => ParseState::WeakOpenEscape,
                '"' => ParseState::Complete,
                _ => ParseState::WeakOpen,
            },
            ParseState::WeakOpenEscape => ParseState::WeakOpen,
            ParseState::StrongOpen => match c {
                '#' => return ParseState::Comment,
                '\\' => ParseState::StrongOpenEscape,
                '\'' => ParseState::Complete,
                _ => ParseState::StrongOpen,
            },
            ParseState::StrongOpenEscape => ParseState::StrongOpen,
            // Comments last the entire line.
            ParseState::Comment => panic!("should have returned early"),
        };
    }
    cur_state
}

impl<B: BufRead> Iterator for QuotedLines<B> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        let mut cur_state = ParseState::Complete;
        let mut buf_pos;
        loop {
            buf_pos = buf.len();
            match self.buf.read_line(&mut buf) {
                Ok(0) => match cur_state {
                    ParseState::Complete => return None,
                    _ => {
                        let len = buf.len();
                        return Some(Err(Error::LineParse(buf, len)));
                    }
                },
                Ok(_n) => {
                    cur_state = eval_end_state(cur_state, &buf[buf_pos..]);

                    match cur_state {
                        ParseState::Complete => {
                            if buf.ends_with('\n') {
                                buf.pop();
                                if buf.ends_with('\r') {
                                    buf.pop();
                                }
                            }
                            return Some(Ok(buf));
                        }
                        ParseState::Escape => {}
                        ParseState::StrongOpen => {}
                        ParseState::StrongOpenEscape => {}
                        ParseState::WeakOpen => {}
                        ParseState::WeakOpenEscape => {}
                        ParseState::Comment => {
                            // Find the start of the comment
                            let idx = buf.find(|c| c == '#').unwrap();
                            // Drop the trailing comment text
                            buf.truncate(idx);
                            return Some(Ok(buf));
                        }
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
