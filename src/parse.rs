use regex::{Captures, Regex};
use errors::*;

// for readability's sake
pub type ParsedLine = Result<Option<(String, String)>>;

pub fn parse_line(line: String) -> ParsedLine {
    let line_regex = try!(Regex::new(concat!(
        r"^(\s*(",
        r"#.*|",                            // A comment, or...
        r"\s*|",                            // ...an empty string, or...
        r"(export\s+)?",                    // ...(optionally preceded by "export")...
        r"(?P<key>[A-Za-z_][A-Za-z0-9_]*)", // ...a key,...
        r"=",                               // ...then an equal sign,...
        r"(?P<value>.+?)?",                 // ...and then its corresponding value.
        r")\s*)[\r\n]*$"
    )));

    line_regex
        .captures(&line)
        .map_or(Err(ErrorKind::LineParse(line.clone()).into()), |captures| {
            let key = named_string(&captures, "key");
            let value = named_string(&captures, "value");

            match (key, value) {
                (Some(k), Some(v)) => {
                    let parsed_value = try!(parse_value(&v));

                    Ok(Some((k, parsed_value)))
                }
                (Some(k), None) => {
                    // Empty string for value.
                    Ok(Some((k, String::from(""))))
                }
                _ => {
                    // If there's no key, but capturing did not
                    // fail, we're dealing with a comment
                    Ok(None)
                }
            }
        })
}

fn named_string(captures: &Captures, name: &str) -> Option<String> {
    captures
        .name(name)
        .and_then(|v| Some(v.as_str().to_owned()))
}

fn parse_value(input: &str) -> Result<String> {
    let mut strong_quote = false; // '
    let mut weak_quote = false; // "
    let mut escaped = false;
    let mut expecting_end = false;

    //FIXME can this be done without yet another allocation per line?
    let mut output = String::new();

    for c in input.chars() {
        //the regex _should_ already trim whitespace off the end
        //expecting_end is meant to permit: k=v #comment
        //without affecting: k=v#comment
        //and throwing on: k=v w
        if expecting_end {
            if c == ' ' || c == '\t' {
                continue;
            } else if c == '#' {
                break;
            } else {
                bail!(ErrorKind::LineParse(input.to_owned()));
            }
        } else if strong_quote {
            if c == '\'' {
                strong_quote = false;
            } else {
                output.push(c);
            }
        } else if weak_quote {
            if escaped {
                //TODO variable expansion perhaps
                //not in this update but in the future
                //$ requires escape anyway for conformance
                //and so as not to make that future change breaking
                //TODO I tried handling literal \n \r but various issues
                //imo not worth worrying about until there's a use case
                //(actually handling backslash 0x10 would be a whole other matter)
                //then there's \v \f bell hex... etc
                match c {
                    '\\' | '"' | '$' => output.push(c),
                    _ => bail!(ErrorKind::LineParse(input.to_owned())),
                }

                escaped = false;
            } else if c == '"' {
                weak_quote = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                output.push(c);
            }
        } else {
            if escaped {
                match c {
                    '\\' | '\'' | '"' | '$' | ' ' => output.push(c),
                    _ => bail!(ErrorKind::LineParse(input.to_owned())),
                }

                escaped = false;
            } else if c == '\'' {
                strong_quote = true;
            } else if c == '"' {
                weak_quote = true;
            } else if c == '\\' {
                escaped = true;
            } else if c == '$' {
                //variable interpolation goes here later
                bail!(ErrorKind::LineParse(input.to_owned()));
            } else if c == ' ' || c == '\t' {
                expecting_end = true;
            } else {
                output.push(c);
            }
        }
    }

    //XXX also fail if escaped? or...
    if strong_quote || weak_quote {
        Err(ErrorKind::LineParse(input.to_owned()).into())
    } else {
        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line_env() {
        let input_iter = vec![
            "KEY=1",
            r#"KEY2="2""#,
            "KEY3='3'",
            "KEY4='fo ur'",
            r#"KEY5="fi ve""#,
            r"KEY6=s\ ix",
            "KEY7=",
            "KEY8=     ",
            "KEY9=   # foo",
            "export   SHELL_LOVER=1",
        ].into_iter()
            .map(|input| input.to_string());
        let actual_iter = input_iter.map(|input| parse_line(input));

        let expected_iter = vec![
            ("KEY", "1"),
            ("KEY2", "2"),
            ("KEY3", "3"),
            ("KEY4", "fo ur"),
            ("KEY5", "fi ve"),
            ("KEY6", "s ix"),
            ("KEY7", ""),
            ("KEY8", ""),
            ("KEY9", ""),
            ("SHELL_LOVER", "1"),
        ].into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));

        for (expected, actual) in expected_iter.zip(actual_iter) {
            assert!(actual.is_ok());
            assert!(actual.as_ref().unwrap().is_some());
            assert_eq!(expected, actual.ok().unwrap().unwrap());
        }
    }

    #[test]
    fn test_parse_line_comment() {
        let input_iter = vec!["# foo=bar", "    #    "]
            .into_iter()
            .map(|input| input.to_string());
        let actual_iter = input_iter.map(|input| parse_line(input));

        for actual in actual_iter {
            assert!(actual.is_ok());
            assert!(actual.ok().unwrap().is_none());
        }
    }

    #[test]
    fn test_parse_line_invalid() {
        let input_iter = vec![
            "  invalid    ",
            "KEY =val",
            "KEY2= val",
            "very bacon = yes indeed",
            "=value",
        ].into_iter()
            .map(|input| input.to_string());
        let actual_iter = input_iter.map(|input| parse_line(input));

        for actual in actual_iter {
            assert!(actual.is_err());
        }
    }

    #[test]
    fn test_parse_value_escapes() {
        let input_iter = vec![
            r#"KEY=my\ cool\ value"#,
            r#"KEY2=\$sweet"#,
            r#"KEY3="awesome stuff \"mang\"""#,
            r#"KEY4='sweet $\fgs'\''fds'"#,
            r#"KEY5="'\"yay\\"\ "stuff""#,
            r##"KEY6="lol" #well you see when I say lol wh"##,
        ].into_iter()
            .map(|input| input.to_string());
        let actual_iter = input_iter.map(|input| parse_line(input));

        let expected_iter = vec![
            ("KEY", r#"my cool value"#),
            ("KEY2", r#"$sweet"#),
            ("KEY3", r#"awesome stuff "mang""#),
            ("KEY4", r#"sweet $\fgs'fds"#),
            ("KEY5", r#"'"yay\ stuff"#),
            ("KEY6", "lol"),
        ].into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()));

        for (expected, actual) in expected_iter.zip(actual_iter) {
            assert!(actual.is_ok());
            assert!(actual.as_ref().unwrap().is_some());
            assert_eq!(expected, actual.unwrap().unwrap());
        }
    }

    #[test]
    fn test_parse_value_escapes_invalid() {
        let input_iter = vec![
            r#"KEY=my uncool value"#,
            r#"KEY2=$notcool"#,
            r#"KEY3="why"#,
            r#"KEY4='please stop''"#,
            r#"KEY5=h\8u"#,
        ].into_iter()
            .map(|input| input.to_string());
        let actual_iter = input_iter.map(|input| parse_line(input));

        for actual in actual_iter {
            assert!(actual.is_err());
        }
    }
}
