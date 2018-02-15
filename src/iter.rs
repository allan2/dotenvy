use std::io::prelude::*;
use std::io::{BufReader, Lines};

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
}

impl<R: Read> Iterator for Iter<R> {
  type Item = parse::ParsedLine;

  fn next(&mut self) -> Option<Self::Item> {
    self.lines.next().map(|line| {
      parse::parse_line(line?)
    })
  }
}
