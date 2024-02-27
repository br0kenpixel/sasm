use std::str::Chars;

use crate::{error::ParseError, expression::Expression};
use itertools::Itertools;

pub struct ArgParserStateMachine;

impl ArgParserStateMachine {
    pub fn parse_args(raw: &str) -> Result<Vec<Expression>, ParseError> {
        let mut expressions = Vec::new();
        let mut chars_iter = raw.chars();
        let mut buffer = String::new();

        while let Some(ch) = chars_iter.next() {
            match ch {
                '-' | '0'..='9' => {
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
                _ => unreachable!(),
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
                Expression::String("first string".into()),
                Expression::String("second string".into()),
            ]
        );
    }

    #[test]
    fn parse_string() {
        let parsed = ArgParserStateMachine::parse_args("'hello there!'").unwrap();

        assert_eq!(parsed, vec![Expression::String("hello there!".into())]);
    }

    #[test]
    fn parse_double_qt_string() {
        let parsed = ArgParserStateMachine::parse_args("\"hello there!\"").unwrap();

        assert_eq!(parsed, vec![Expression::String("hello there!".into())]);
    }

    #[test]
    fn parse_number_string() {
        let parsed = ArgParserStateMachine::parse_args("3,'hello'").unwrap();

        assert_eq!(
            parsed,
            vec![Expression::Number(3), Expression::String("hello".into())]
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
    fn parse_ident_num_str() {
        let parsed = ArgParserStateMachine::parse_args("pi,-3,'hello!'").unwrap();

        assert_eq!(
            parsed,
            vec![
                Expression::Identifier(Identifier::new("pi")),
                Expression::Number(-3),
                Expression::String("hello!".into())
            ]
        );
    }
}
