use std::{io, path::Path};

use dotenvy::Error;
use dotenvy_test_util::*;

pub fn assert_err_line_parse(line: &str, index: usize, actual: Error) {
    match actual {
        Error::LineParse(s, i) => {
            assert_eq!(line, s, "expected line parse error for line `{line}`");
            assert_eq!(index, i, "expected line parse error at index {index}");
        }
        _ => panic!("expected line parse error"),
    }
}

pub fn assert_err_not_found(actual: Error) {
    match actual {
        Error::Io(err) => assert_eq!(err.kind(), io::ErrorKind::NotFound),
        _ => panic!("expected `NotFound` error"),
    }
}

pub fn assert_err_invalid_utf8(actual: Error) {
    match actual {
        Error::Io(err) => assert_eq!(err.kind(), io::ErrorKind::InvalidData),
        _ => panic!("expected `InvalidData` error"),
    }
}

pub fn assert_default_envfile_path(testenv: &TestEnv, path: &Path) {
    let expected = testenv
        .temp_path()
        .join(".env")
        .canonicalize()
        .expect("failed to canonicalize");
    assert_eq!(expected, path);
}
