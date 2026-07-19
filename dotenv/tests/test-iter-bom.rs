use dotenvy::{from_path_iter, from_read_iter};
use std::fs;
use std::io::{self, Cursor, Read};

const INPUT_WITH_BOM: &[u8] = b"\xEF\xBB\xBFKEY=value\n";

struct OneByteAtATime<R>(R);

impl<R: Read> Read for OneByteAtATime<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let max = buf.len().min(1);
        self.0.read(&mut buf[..max])
    }
}

#[test]
fn from_read_iter_ignores_bom() {
    let reader = OneByteAtATime(Cursor::new(INPUT_WITH_BOM));
    let mut iter = from_read_iter(reader);

    assert_eq!(
        iter.next().unwrap().unwrap(),
        ("KEY".to_owned(), "value".to_owned())
    );
    assert!(iter.next().is_none());
}

#[test]
fn from_path_iter_ignores_bom() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join(".env");
    fs::write(&path, INPUT_WITH_BOM)?;
    let mut iter = from_path_iter(path)?;

    assert_eq!(
        iter.next().unwrap()?,
        ("KEY".to_owned(), "value".to_owned())
    );
    assert!(iter.next().is_none());
    Ok(())
}
