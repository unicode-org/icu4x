// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::error::PatternError,
    super::{GenericPatternItem, PatternItem},
    GenericPattern, Pattern,
};
use crate::fields::FieldSymbol;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
enum Segment {
    Symbol { symbol: FieldSymbol, length: u8 },
    Literal { literal: String, quoted: bool },
}

#[derive(Debug)]
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
        chars: &mut core::iter::Peekable<core::str::Chars>,
        result: &mut Vec<PatternItem>,
    ) -> Result<bool, PatternError> {
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

    fn handle_generic_quoted_literal(
        &mut self,
        ch: char,
        chars: &mut core::iter::Peekable<core::str::Chars>,
    ) -> Result<bool, PatternError> {
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
                _ => unreachable!("Generic pattern has no symbols."),
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

    fn collect_segment(state: Segment, result: &mut Vec<PatternItem>) -> Result<(), PatternError> {
        match state {
            Segment::Symbol { symbol, length } => {
                result.push((symbol, length).try_into()?);
            }
            Segment::Literal { quoted, .. } if quoted => {
                return Err(PatternError::UnclosedLiteral);
            }
            Segment::Literal { literal, .. } => {
                result.extend(literal.chars().map(PatternItem::from));
            }
        }
        Ok(())
    }

    fn collect_generic_segment(
        state: Segment,
        result: &mut Vec<GenericPatternItem>,
    ) -> Result<(), PatternError> {
        match state {
            Segment::Literal { quoted, .. } if quoted => {
                return Err(PatternError::UnclosedLiteral);
            }
            Segment::Literal { literal, .. } => {
                if !literal.is_empty() {
                    result.extend(literal.chars().map(GenericPatternItem::from))
                }
            }
            _ => unreachable!("Generic pattern has no symbols."),
        }
        Ok(())
    }

    pub fn parse(mut self) -> Result<Vec<PatternItem>, PatternError> {
        let mut chars = self.source.chars().peekable();
        let mut result = vec![];

        while let Some(ch) = chars.next() {
            if !self.handle_quoted_literal(ch, &mut chars, &mut result)? {
                if let Ok(new_symbol) = FieldSymbol::try_from(ch) {
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

    pub fn parse_generic(mut self) -> Result<Vec<GenericPatternItem>, PatternError> {
        let mut chars = self.source.chars().peekable();
        let mut result = vec![];

        while let Some(ch) = chars.next() {
            if !self.handle_generic_quoted_literal(ch, &mut chars)? {
                if ch == '{' {
                    Self::collect_generic_segment(self.state, &mut result)?;

                    let ch = chars.next().ok_or(PatternError::UnclosedPlaceholder)?;
                    let idx = ch
                        .to_digit(10)
                        .ok_or(PatternError::UnknownSubstitution(ch))?
                        as u8;
                    result.push(GenericPatternItem::Placeholder(idx));
                    let ch = chars.next().ok_or(PatternError::UnclosedPlaceholder)?;
                    if ch != '}' {
                        return Err(PatternError::UnclosedPlaceholder);
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

        Self::collect_generic_segment(self.state, &mut result)?;

        Ok(result)
    }

    pub fn parse_placeholders(
        self,
        replacements: Vec<Pattern>,
    ) -> Result<Vec<PatternItem>, PatternError> {
        let generic_items = self.parse_generic()?;

        let gp = GenericPattern::from(generic_items);
        Ok(gp.combined(replacements)?.items.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{self, FieldLength};
    use crate::pattern::reference::Pattern;

    #[test]
    fn pattern_parse_simple() {
        let samples = vec![
            (
                "dd/MM/y",
                vec![
                    (fields::Day::DayOfMonth.into(), FieldLength::TwoDigit).into(),
                    '/'.into(),
                    (fields::Month::Format.into(), FieldLength::TwoDigit).into(),
                    '/'.into(),
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                ],
            ),
            (
                "HH:mm:ss",
                vec![
                    (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
                    ':'.into(),
                    (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
                    ':'.into(),
                    (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
                ],
            ),
            (
                "y年M月d日",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    '年'.into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                    '月'.into(),
                    (fields::Day::DayOfMonth.into(), FieldLength::One).into(),
                    '日'.into(),
                ],
            ),
            (
                "HH:mm:ss.SS",
                vec![
                    (fields::Hour::H23.into(), FieldLength::TwoDigit).into(),
                    ':'.into(),
                    (FieldSymbol::Minute, FieldLength::TwoDigit).into(),
                    ':'.into(),
                    (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
                    '.'.into(),
                    (
                        fields::Second::FractionalSecond.into(),
                        FieldLength::Fixed(2),
                    )
                        .into(),
                ],
            ),
        ];

        for (string, items) in samples {
            assert_eq!(
                string.parse::<Pattern>().expect("Parsing pattern failed."),
                Pattern::from(items)
            );
        }
    }

    fn str2pis(input: &str) -> Vec<PatternItem> {
        input.chars().map(Into::into).collect()
    }

    #[test]
    fn pattern_parse_literals() {
        let samples = vec![
            ("", ""),
            (" ", " "),
            ("  ", "  "),
            (" żółć ", " żółć "),
            ("''", "'"),
            (" ''", " '"),
            (" '' ", " ' "),
            ("''''", "''"),
            (" '' '' ", " ' ' "),
            ("ż'ół'ć", "żółć"),
            ("ż'ó''ł'ć", "żó'łć"),
            (" 'Ymd' ", " Ymd "),
            ("الأسبوع", "الأسبوع"),
        ];

        for (string, pattern) in samples {
            assert_eq!(
                Parser::new(string)
                    .parse()
                    .expect("Parsing pattern failed."),
                str2pis(pattern),
            );

            assert_eq!(
                Parser::new(string)
                    .parse_placeholders(vec![])
                    .expect("Parsing pattern failed."),
                str2pis(pattern),
            );
        }

        let broken = vec![(" 'foo ", PatternError::UnclosedLiteral)];

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
                    ' '.into(),
                ],
            ),
            (
                "y M",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    ' '.into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''a",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    '\''.into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''b",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    '\''.into(),
                    (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
                ],
            ),
            (
                "y'My'M",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    'M'.into(),
                    'y'.into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (
                "y 'My' M",
                vec![
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                    ' '.into(),
                    'M'.into(),
                    'y'.into(),
                    ' '.into(),
                    (fields::Month::Format.into(), FieldLength::One).into(),
                ],
            ),
            (
                " 'r'. 'y'. ",
                vec![
                    ' '.into(),
                    'r'.into(),
                    '.'.into(),
                    ' '.into(),
                    'y'.into(),
                    '.'.into(),
                    ' '.into(),
                ],
            ),
            (
                "hh 'o''clock' a",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    ' '.into(),
                    'o'.into(),
                    '\''.into(),
                    'c'.into(),
                    'l'.into(),
                    'o'.into(),
                    'c'.into(),
                    'k'.into(),
                    ' '.into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh 'o''clock' b",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    ' '.into(),
                    'o'.into(),
                    '\''.into(),
                    'c'.into(),
                    'l'.into(),
                    'o'.into(),
                    'c'.into(),
                    'k'.into(),
                    ' '.into(),
                    (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''a",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    '\''.into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''b",
                vec![
                    (fields::Hour::H12.into(), FieldLength::TwoDigit).into(),
                    '\''.into(),
                    (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
                ],
            ),
            (
                "z",
                vec![(fields::TimeZone::LowerZ.into(), FieldLength::One).into()],
            ),
            (
                "Z",
                vec![(fields::TimeZone::UpperZ.into(), FieldLength::One).into()],
            ),
            (
                "O",
                vec![(fields::TimeZone::UpperO.into(), FieldLength::One).into()],
            ),
            (
                "v",
                vec![(fields::TimeZone::LowerV.into(), FieldLength::One).into()],
            ),
            (
                "V",
                vec![(fields::TimeZone::UpperV.into(), FieldLength::One).into()],
            ),
            (
                "x",
                vec![(fields::TimeZone::LowerX.into(), FieldLength::One).into()],
            ),
            (
                "X",
                vec![(fields::TimeZone::UpperX.into(), FieldLength::One).into()],
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

        let broken = vec![
            (
                "yyyyyyy",
                PatternError::FieldLengthInvalid(FieldSymbol::Year(fields::Year::Calendar)),
            ),
            (
                "hh:mm:ss.SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS",
                PatternError::FieldLengthInvalid(FieldSymbol::Second(fields::Second::FractionalSecond)),
            ),
        ];

        for (string, error) in broken {
            assert_eq!(Parser::new(string).parse(), Err(error),);
        }
    }

    #[test]
    fn pattern_parse_placeholders() {
        let samples = vec![
            ("{0}", vec![Pattern::from("ONE")], str2pis("ONE")),
            (
                "{0}{1}",
                vec![Pattern::from("ONE"), Pattern::from("TWO")],
                str2pis("ONETWO"),
            ),
            (
                "{0} 'at' {1}",
                vec![Pattern::from("ONE"), Pattern::from("TWO")],
                str2pis("ONE at TWO"),
            ),
            (
                "{0}'at'{1}",
                vec![Pattern::from("ONE"), Pattern::from("TWO")],
                str2pis("ONEatTWO"),
            ),
            (
                "'{0}' 'at' '{1}'",
                vec![Pattern::from("ONE"), Pattern::from("TWO")],
                str2pis("{0} at {1}"),
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
            ("{0}", vec![], PatternError::UnknownSubstitution('0')),
            ("{a}", vec![], PatternError::UnknownSubstitution('a')),
            ("{", vec![], PatternError::UnclosedPlaceholder),
            (
                "{0",
                vec![Pattern::from(vec![])],
                PatternError::UnclosedPlaceholder,
            ),
            (
                "{01",
                vec![Pattern::from(vec![])],
                PatternError::UnclosedPlaceholder,
            ),
            (
                "{00}",
                vec![Pattern::from(vec![])],
                PatternError::UnclosedPlaceholder,
            ),
            (
                "'{00}",
                vec![Pattern::from(vec![])],
                PatternError::UnclosedLiteral,
            ),
        ];

        for (string, replacements, error) in broken {
            assert_eq!(
                Parser::new(string).parse_placeholders(replacements),
                Err(error),
            );
        }
    }
}
