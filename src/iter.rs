use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::env;

use errors::*;
use parse;

pub struct Iter<R> {
    lines: Lines<BufReader<R>>,
}

impl<R: Read> Iter<R> {
    pub fn new(reader: R) -> Iter<R> {
        Iter {
            lines: BufReader::new(reader).lines()
        }
    }

    pub fn load(self) -> Result<()> {
        for parsed_line in self {
          if let Some((key, value)) = parsed_line? {
              if env::var(&key).is_err() {
                  env::set_var(&key, value);
              }
          }
        }

        Ok(())
    }
}

impl<R: Read> Iterator for Iter<R> {
  type Item = parse::ParsedLine;

  fn next(&mut self) -> Option<Self::Item> {
    self.lines.next().map(|line| {
      parse::parse_line(line?)
    })
  }
}
