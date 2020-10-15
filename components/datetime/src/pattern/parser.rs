use super::error::Error;
use super::{Pattern, PatternItem};
use crate::fields::FieldSymbol;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
enum Segment {
    Symbol { symbol: FieldSymbol, length: u8 },
    Literal { literal: String, quoted: bool },
}

pub struct Parser<'p> {
    source: &'p str,
    state: Segment,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Self {
            source,
            state: Segment::Literal {
                literal: String::new(),
                quoted: false,
            },
        }
    }

    fn handle_quoted_literal(
        &mut self,
        ch: char,
        chars: &mut std::iter::Peekable<std::str::Chars>,
        result: &mut Vec<PatternItem>,
    ) -> Result<bool, Error> {
        if ch == '\'' {
            match (&mut self.state, chars.peek() == Some(&'\'')) {
                (
                    Segment::Literal {
                        ref mut literal, ..
                    },
                    true,
                ) => {
                    literal.push('\'');
                    chars.next();
                }
                (Segment::Literal { ref mut quoted, .. }, false) => {
                    *quoted = !*quoted;
                }
                (Segment::Symbol { symbol, length }, true) => {
                    result.push((*symbol, *length).try_into()?);
                    self.state = Segment::Literal {
                        literal: String::from(ch),
                        quoted: false,
                    };
                    chars.next();
                }
                (Segment::Symbol { symbol, length }, false) => {
                    result.push((*symbol, *length).try_into()?);
                    self.state = Segment::Literal {
                        literal: String::new(),
                        quoted: true,
                    };
                }
            }
            Ok(true)
        } else if let Segment::Literal {
            ref mut literal,
            quoted: true,
        } = self.state
        {
            literal.push(ch);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn collect_segment(state: Segment, result: &mut Vec<PatternItem>) -> Result<(), Error> {
        match state {
            Segment::Symbol { symbol, length } => {
                result.push((symbol, length).try_into()?);
            }
            Segment::Literal { quoted, .. } if quoted => {
                return Err(Error::UnclosedLiteral);
            }
            Segment::Literal { literal, .. } => {
                if !literal.is_empty() {
                    result.push(literal.into());
                }
            }
        }
        Ok(())
    }

    pub fn parse(mut self) -> Result<Vec<PatternItem>, Error> {
        let mut chars = self.source.chars().peekable();
        let mut result = vec![];

        while let Some(ch) = chars.next() {
            if !self.handle_quoted_literal(ch, &mut chars, &mut result)? {
                if let Ok(new_symbol) = FieldSymbol::try_from(ch as u8) {
                    match self.state {
                        Segment::Symbol {
                            ref symbol,
                            ref mut length,
                        } if new_symbol == *symbol => {
                            *length += 1;
                        }
                        segment => {
                            Self::collect_segment(segment, &mut result)?;
                            self.state = Segment::Symbol {
                                symbol: new_symbol,
                                length: 1,
                            };
                        }
                    }
                } else {
                    match self.state {
                        Segment::Symbol { symbol, length } => {
                            result.push((symbol, length).try_into()?);
                            self.state = Segment::Literal {
                                literal: String::from(ch),
                                quoted: false,
                            };
                        }
                        Segment::Literal {
                            ref mut literal, ..
                        } => literal.push(ch),
                    }
                }
            }
        }

        Self::collect_segment(self.state, &mut result)?;

        Ok(result)
    }

    pub fn parse_placeholders(
        mut self,
        mut replacements: Vec<Pattern>,
    ) -> Result<Vec<PatternItem>, Error> {
        let mut chars = self.source.chars().peekable();
        let mut result = vec![];

        while let Some(ch) = chars.next() {
            if !self.handle_quoted_literal(ch, &mut chars, &mut result)? {
                if ch == '{' {
                    Self::collect_segment(self.state, &mut result)?;

                    let ch = chars.next().ok_or(Error::UnclosedPlaceholder)?;
                    let idx: u32 = ch.to_digit(10).ok_or(Error::UnknownSubstitution(ch))?;
                    let replacement = replacements
                        .get_mut(idx as usize)
                        .ok_or(Error::UnknownSubstitution(ch))?;
                    result.append(&mut replacement.0);
                    let ch = chars.next().ok_or(Error::UnclosedPlaceholder)?;
                    if ch != '}' {
                        return Err(Error::UnclosedPlaceholder);
                    }
                    self.state = Segment::Literal {
                        literal: String::new(),
                        quoted: false,
                    };
                } else if let Segment::Literal {
                    ref mut literal, ..
                } = self.state
                {
                    literal.push(ch);
                } else {
                    unreachable!()
                }
            }
        }

        Self::collect_segment(self.state, &mut result)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{self, FieldLength};
    use crate::pattern::Pattern;

    #[test]
    fn pattern_parse_simple() {
        let samples = vec![
            (
                "dd/MM/y",
                vec![
                    (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
                    "/".into(),
                    (fields::Month::Format.into(), FieldLength::TwoDigit).into(),
                    "/".into(),
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                ],
            ),
            (
                "HH:mm:ss",
                vec![
                    (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
                    ":".into(),
                    (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
                    ":".into(),
                    (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
                ],
            ),
            (
                "y年M月d日",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    "年".into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                    "月".into(),
                    (fields::Day::DayOfMonth.into(), FieldLength::One).into(),
                    "日".into(),
                ],
            ),
        ];

        for (string, pattern) in samples {
            assert_eq!(
                Pattern::from_bytes(string).expect("Parsing pattern failed."),
                pattern.into_iter().collect()
            );
        }
    }

    #[test]
    fn pattern_parse_literals() {
        let samples = vec![
            ("", vec![]),
            (" ", vec![" ".into()]),
            ("  ", vec!["  ".into()]),
            (" żółć ", vec![" żółć ".into()]),
            ("''", vec!["'".into()]),
            (" ''", vec![" '".into()]),
            (" '' ", vec![" ' ".into()]),
            ("''''", vec!["''".into()]),
            (" '' '' ", vec![" ' ' ".into()]),
            ("ż'ół'ć", vec!["żółć".into()]),
            ("ż'ó''ł'ć", vec!["żó'łć".into()]),
            (" 'Ymd' ", vec![" Ymd ".into()]),
        ];

        for (string, pattern) in samples {
            assert_eq!(
                Parser::new(string)
                    .parse()
                    .expect("Parsing pattern failed."),
                pattern,
            );

            assert_eq!(
                Parser::new(string)
                    .parse_placeholders(vec![])
                    .expect("Parsing pattern failed."),
                pattern,
            );
        }

        let broken = vec![(" 'foo ", Error::UnclosedLiteral)];

        for (string, error) in broken {
            assert_eq!(Parser::new(string).parse(), Err(error),);
        }
    }

    #[test]
    fn pattern_parse_symbols() {
        let samples = vec![
            (
                "y",
                vec![(fields::Year::Calendar.into(), FieldLength::One).into()],
            ),
            (
                "yy",
                vec![(fields::Year::Calendar.into(), FieldLength::TwoDigit).into()],
            ),
            (
                "yyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Abbreviated).into()],
            ),
            (
                "yyyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Wide).into()],
            ),
            (
                "yyyyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Narrow).into()],
            ),
            (
                "yyyyyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Six).into()],
            ),
            (
                "yM",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (
                "y ",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    " ".into(),
                ],
            ),
            (
                "y M",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    " ".into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''a",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    "'".into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "y'My'M",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    "My".into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (
                "y 'My' M",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    " My ".into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (" 'r'. 'y'. ", vec![" r. y. ".into()]),
            (
                "hh 'o''clock' a",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    " o'clock ".into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''a",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    "'".into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
        ];

        for (string, pattern) in samples {
            assert_eq!(
                Parser::new(string)
                    .parse()
                    .expect("Parsing pattern failed."),
                pattern,
            );
        }

        let broken = vec![(
            "yyyyyyy",
            Error::FieldTooLong(FieldSymbol::Year(fields::Year::Calendar)),
        )];

        for (string, error) in broken {
            assert_eq!(Parser::new(string).parse(), Err(error),);
        }
    }

    #[test]
    fn pattern_parse_placeholders() {
        let samples = vec![
            ("{0}", vec![Pattern(vec!["ONE".into()])], vec!["ONE".into()]),
            (
                "{0}{1}",
                vec![Pattern(vec!["ONE".into()]), Pattern(vec!["TWO".into()])],
                vec!["ONE".into(), "TWO".into()],
            ),
            (
                "{0} 'at' {1}",
                vec![Pattern(vec!["ONE".into()]), Pattern(vec!["TWO".into()])],
                vec!["ONE".into(), " at ".into(), "TWO".into()],
            ),
            (
                "{0}'at'{1}",
                vec![Pattern(vec!["ONE".into()]), Pattern(vec!["TWO".into()])],
                vec!["ONE".into(), "at".into(), "TWO".into()],
            ),
            (
                "'{0}' 'at' '{1}'",
                vec![Pattern(vec!["ONE".into()]), Pattern(vec!["TWO".into()])],
                vec!["{0} at {1}".into()],
            ),
        ];

        for (string, replacements, pattern) in samples {
            assert_eq!(
                Parser::new(string)
                    .parse_placeholders(replacements)
                    .expect("Parsing pattern failed."),
                pattern,
            );
        }

        let broken = vec![
            ("{0}", vec![], Error::UnknownSubstitution('0')),
            ("{a}", vec![], Error::UnknownSubstitution('a')),
            ("{", vec![], Error::UnclosedPlaceholder),
            ("{0", vec![Pattern(vec![])], Error::UnclosedPlaceholder),
            ("{01", vec![Pattern(vec![])], Error::UnclosedPlaceholder),
            ("{00}", vec![Pattern(vec![])], Error::UnclosedPlaceholder),
            ("'{00}", vec![Pattern(vec![])], Error::UnclosedLiteral),
        ];

        for (string, replacements, error) in broken {
            assert_eq!(
                Parser::new(string).parse_placeholders(replacements),
                Err(error),
            );
        }
    }
}
