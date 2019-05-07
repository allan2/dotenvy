use regex::{Captures, Regex};
use errors::*;

// for readability's sake
pub type ParsedLine = Result<Option<(String, String)>>;

pub fn parse_line(line: &str) -> ParsedLine {
    lazy_static! {
      static ref LINE_REGEX: Regex = Regex::new(r#"(?x)
        ^(
          \s*
          (
            \#.*|                           # A comment, or...
            \s*|                            # ...an empty string, or...
            (export\s+)?                    # ...(optionally preceded by "export")...
            (?P<key>[A-Za-z_][A-Za-z0-9_]*) # ...a key,...
            =                               # ...then an equal sign,...
            (?P<value>.+?)?                 # ...and then its corresponding value.
          )\s*
        )
        [\r\n]*
        $
      "#).unwrap();
    }

    LINE_REGEX
        .captures(line)
        .map_or(Err(Error::LineParse(line.into()).into()), |captures| {
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
                return Err(Error::LineParse(input.to_owned()));
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
                    _ => return Err(Error::LineParse(input.to_owned())),
                }

                escaped = false;
            } else if c == '"' {
                weak_quote = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                output.push(c);
            }
        } else if escaped {
            match c {
                '\\' | '\'' | '"' | '$' | ' ' => output.push(c),
                _ => return Err(Error::LineParse(input.to_owned())),
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
            return Err(Error::LineParse(input.to_owned()));
        } else if c == ' ' || c == '\t' {
            expecting_end = true;
        } else {
            output.push(c);
        }
    }

    //XXX also fail if escaped? or...
    if strong_quote || weak_quote {
        Err(Error::LineParse(input.to_owned()).into())
    } else {
        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use iter::Iter;

    #[test]
    fn test_parse_line_env() {
        let actual_iter = Iter::new(r#"
KEY=1
KEY2="2"
KEY3='3'
KEY4='fo ur'
KEY5="fi ve"
KEY6=s\ ix
KEY7=
KEY8=     
KEY9=   # foo
export   SHELL_LOVER=1
"#.as_bytes());

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

        let mut count = 0;
        for (expected, actual) in expected_iter.zip(actual_iter) {
            assert!(actual.is_ok());
            assert_eq!(expected, actual.ok().unwrap());
            count += 1;
        }

        assert_eq!(count, 10);
    }

    #[test]
    fn test_parse_line_comment() {
        let result: Result<Vec<(String, String)>> = Iter::new(r#"
# foo=bar
#    "#.as_bytes()).collect();
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_parse_line_invalid() {
        let actual_iter = Iter::new(r#"
  invalid    
KEY =val
KEY2= val
very bacon = yes indeed
=value"#.as_bytes());

        let mut count = 0;
        for actual in actual_iter {
            assert!(actual.is_err());
            count += 1;
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_parse_value_escapes() {
        let actual_iter = Iter::new(r#"
KEY=my\ cool\ value
KEY2=\$sweet
KEY3="awesome stuff \"mang\""
KEY4='sweet $\fgs'\''fds'
KEY5="'\"yay\\"\ "stuff"
KEY6="lol" #well you see when I say lol wh
"#.as_bytes());

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
            assert_eq!(expected, actual.unwrap());
        }
    }

    #[test]
    fn test_parse_value_escapes_invalid() {
        let actual_iter = Iter::new(r#"
KEY=my uncool value
KEY2=$notcool
KEY3="why
KEY4='please stop''
KEY5=h\8u
"#.as_bytes());

        for actual in actual_iter {
            assert!(actual.is_err());
        }
    }
}
