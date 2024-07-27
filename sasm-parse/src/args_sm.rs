use crate::{error::ParseError, expression::Expression};
use itertools::Itertools;
use std::str::Chars;

pub struct ArgParserStateMachine;

impl ArgParserStateMachine {
    pub fn parse_args(raw: &str) -> Result<Vec<Expression>, ParseError> {
        let mut expressions = Vec::new();
        let mut chars_iter = raw.chars();
        let mut buffer = String::new();

        while let Some(ch) = chars_iter.next() {
            match ch {
                '-' | '0'..='9' | '.' => {
                    buffer.push(ch);

                    collect_rest(chars_iter.by_ref(), char::is_ascii_digit, &mut buffer);
                }
                '"' | '\'' => {
                    buffer = Self::parse_string(chars_iter.by_ref(), ch);

                    buffer.insert(0, ch);
                    buffer.push(ch);
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    buffer.push(ch);

                    collect_rest(
                        chars_iter.by_ref(),
                        |ch| ch.is_ascii_alphabetic() || *ch == '_',
                        &mut buffer,
                    );
                }
                ',' => {
                    expressions.push(Expression::try_from(buffer.as_str())?);
                    buffer.clear();
                }
                other => {
                    return Err(ParseError::UnexpectedToken(other));
                }
            }
        }

        if !buffer.is_empty() {
            let expr = Expression::try_from(buffer.as_str())?;
            expressions.push(expr);
        }

        Ok(expressions)
    }

    fn parse_string<I: Iterator<Item = char>>(chars: I, qt: char) -> String {
        let mut buffer = String::new();
        let mut escape = false;

        for ch in chars {
            if ch == '\\' {
                escape = true;
            }

            if ch == qt && !escape {
                return buffer;
            }

            buffer.push(ch);
        }

        buffer
    }
}

fn collect_rest<F: FnMut(&char) -> bool>(it: &mut Chars<'_>, predicate: F, output: &mut String) {
    it.take_while_ref(predicate).for_each(|ch| output.push(ch));
}

#[cfg(test)]
mod tests {
    use super::ArgParserStateMachine;
    use crate::{expression::Expression, ident::Identifier};

    #[test]
    fn parse_numbers() {
        let parsed = ArgParserStateMachine::parse_args("-4,45,9,0").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::Number(-4),
                Expression::Number(45),
                Expression::Number(9),
                Expression::Number(0),
            ]
        );
    }

    #[test]
    fn parse_number() {
        let parsed = ArgParserStateMachine::parse_args("-1").unwrap();

        assert_eq!(parsed, vec![Expression::Number(-1)]);
    }

    #[test]
    fn parse_strings() {
        let parsed = ArgParserStateMachine::parse_args("'first string',\"second string\"").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::make_string("first string"),
                Expression::make_string("second string"),
            ]
        );
    }

    #[test]
    fn parse_string() {
        let parsed = ArgParserStateMachine::parse_args("'hello there!'").unwrap();

        assert_eq!(parsed, vec![Expression::make_string("hello there!")]);
    }

    #[test]
    fn parse_double_qt_string() {
        let parsed = ArgParserStateMachine::parse_args("\"hello there!\"").unwrap();

        assert_eq!(parsed, vec![Expression::make_string("hello there!")]);
    }

    #[test]
    fn parse_number_string() {
        let parsed = ArgParserStateMachine::parse_args("3,'hello'").unwrap();

        assert_eq!(
            parsed,
            vec![Expression::Number(3), Expression::make_string("hello")]
        );
    }

    #[test]
    fn parse_ident() {
        let parsed = ArgParserStateMachine::parse_args("pi").unwrap();

        assert_eq!(parsed, vec![Expression::Identifier(Identifier::new("pi"))]);
    }

    #[test]
    fn parse_ident_num() {
        let parsed = ArgParserStateMachine::parse_args("pi,-3").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::Identifier(Identifier::new("pi")),
                Expression::Number(-3)
            ]
        );
    }

    #[test]
    fn parse_floats() {
        let parsed = ArgParserStateMachine::parse_args("1.43,3.10,-9.23").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::Float(1.43),
                Expression::Float(3.10),
                Expression::Float(-9.23)
            ]
        );
    }

    #[test]
    fn parse_float() {
        let parsed = ArgParserStateMachine::parse_args("7.5284").unwrap();

        assert_eq!(parsed, vec![Expression::Float(7.5284)]);
    }

    #[test]
    fn parse_scientific_floats() {
        let parsed = ArgParserStateMachine::parse_args("10.13e8,-2.0e9").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::Float(1013000000.0),
                Expression::Float(-2_000_000_000.0)
            ],
        );
    }

    #[test]
    fn parse_ident_num_str() {
        let parsed = ArgParserStateMachine::parse_args("pi,-3,'hello!'").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::Identifier(Identifier::new("pi")),
                Expression::Number(-3),
                Expression::make_string("hello!")
            ]
        );
    }
}
