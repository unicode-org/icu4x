// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::error::PatternError,
    super::{GenericPatternItem, PatternItem},
    GenericPattern, Pattern,
};
use crate::fields::{self, Field, FieldLength, FieldSymbol};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::{
    convert::{TryFrom, TryInto},
    mem,
};

#[derive(Debug, PartialEq)]
struct SegmentSymbol {
    symbol: FieldSymbol,
    length: u8,
}

impl SegmentSymbol {
    fn finish(self, result: &mut Vec<PatternItem>) -> Result<(), PatternError> {
        (self.symbol, self.length)
            .try_into()
            .map(|item| result.push(item))
    }
}

#[derive(Debug, PartialEq)]
struct SegmentSecondSymbol {
    integer_digits: u8,
    seen_decimal_separator: bool,
    fraction_digits: u8,
}

impl SegmentSecondSymbol {
    fn finish(self, result: &mut Vec<PatternItem>) -> Result<(), PatternError> {
        let second_symbol = FieldSymbol::Second(fields::Second::Second);
        let symbol = if self.fraction_digits == 0 {
            second_symbol
        } else {
            let decimal_second = fields::DecimalSecond::from_idx(self.fraction_digits)
                .map_err(|_| PatternError::FieldLengthInvalid(second_symbol))?;
            FieldSymbol::DecimalSecond(decimal_second)
        };
        let length = FieldLength::from_idx(self.integer_digits)
            .map_err(|_| PatternError::FieldLengthInvalid(symbol))?;
        result.push(PatternItem::Field(Field { symbol, length }));
        if self.seen_decimal_separator && self.fraction_digits == 0 {
            result.push(PatternItem::Literal('.'));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct SegmentLiteral {
    literal: String,
    quoted: bool,
}

impl SegmentLiteral {
    fn finish(self, result: &mut Vec<PatternItem>) -> Result<(), PatternError> {
        if !self.literal.is_empty() {
            result.extend(self.literal.chars().map(PatternItem::from));
        }
        Ok(())
    }

    fn finish_generic(self, result: &mut Vec<GenericPatternItem>) -> Result<(), PatternError> {
        if !self.literal.is_empty() {
            result.extend(self.literal.chars().map(GenericPatternItem::from));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Segment {
    Symbol(SegmentSymbol),
    SecondSymbol(SegmentSecondSymbol),
    Literal(SegmentLiteral),
}

impl Segment {
    fn finish(self, result: &mut Vec<PatternItem>) -> Result<(), PatternError> {
        match self {
            Self::Symbol(v) => v.finish(result),
            Self::SecondSymbol(v) => v.finish(result),
            Self::Literal(v) => v.finish(result),
        }
    }

    fn finish_generic(self, result: &mut Vec<GenericPatternItem>) -> Result<(), PatternError> {
        match self {
            Self::Symbol(_) => unreachable!("no symbols in generic pattern"),
            Self::SecondSymbol(_) => unreachable!("no symbols in generic pattern"),
            Self::Literal(v) => v.finish_generic(result),
        }
    }
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
            state: Segment::Literal(SegmentLiteral {
                literal: String::new(),
                quoted: false,
            }),
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
                (Segment::Literal(ref mut literal), true) => {
                    literal.literal.push('\'');
                    chars.next();
                }
                (Segment::Literal(ref mut literal), false) => {
                    literal.quoted = !literal.quoted;
                }
                (state, true) => {
                    mem::replace(
                        state,
                        Segment::Literal(SegmentLiteral {
                            literal: String::from(ch),
                            quoted: false,
                        }),
                    )
                    .finish(result)?;
                    chars.next();
                }
                (state, false) => {
                    mem::replace(
                        state,
                        Segment::Literal(SegmentLiteral {
                            literal: String::new(),
                            quoted: true,
                        }),
                    )
                    .finish(result)?;
                }
            }
            Ok(true)
        } else if let Segment::Literal(SegmentLiteral {
            ref mut literal,
            quoted: true,
        }) = self.state
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
                (Segment::Literal(literal), true) => {
                    literal.literal.push('\'');
                    chars.next();
                }
                (Segment::Literal(literal), false) => {
                    literal.quoted = !literal.quoted;
                }
                _ => unreachable!("Generic pattern has no symbols."),
            }
            Ok(true)
        } else if let Segment::Literal(SegmentLiteral {
            ref mut literal,
            quoted: true,
        }) = self.state
        {
            literal.push(ch);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn parse(mut self) -> Result<Vec<PatternItem>, PatternError> {
        let mut chars = self.source.chars().peekable();
        let mut result = vec![];

        while let Some(ch) = chars.next() {
            if !self.handle_quoted_literal(ch, &mut chars, &mut result)? {
                if let Ok(new_symbol) = FieldSymbol::try_from(ch) {
                    match &mut self.state {
                        Segment::Symbol(SegmentSymbol {
                            ref symbol,
                            ref mut length,
                        }) if new_symbol == *symbol => {
                            *length += 1;
                        }
                        Segment::SecondSymbol(SegmentSecondSymbol {
                            ref mut integer_digits,
                            seen_decimal_separator: false,
                            ..
                        }) if matches!(new_symbol, FieldSymbol::Second(fields::Second::Second)) => {
                            *integer_digits += 1;
                        }
                        state => {
                            mem::replace(
                                state,
                                if matches!(new_symbol, FieldSymbol::Second(fields::Second::Second))
                                {
                                    Segment::SecondSymbol(SegmentSecondSymbol {
                                        integer_digits: 1,
                                        seen_decimal_separator: false,
                                        fraction_digits: 0,
                                    })
                                } else {
                                    Segment::Symbol(SegmentSymbol {
                                        symbol: new_symbol,
                                        length: 1,
                                    })
                                },
                            )
                            .finish(&mut result)?;
                        }
                    }
                } else {
                    match &mut self.state {
                        Segment::SecondSymbol(
                            second_symbol @ SegmentSecondSymbol {
                                seen_decimal_separator: false,
                                ..
                            },
                        ) if ch == '.' => second_symbol.seen_decimal_separator = true,
                        Segment::SecondSymbol(second_symbol) if ch == 'S' => {
                            // Note: this accepts both "ssSSS" and "ss.SSS"
                            // We say we've seen the separator to switch to fraction mode
                            second_symbol.seen_decimal_separator = true;
                            second_symbol.fraction_digits += 1;
                        }
                        Segment::Literal(literal) => literal.literal.push(ch),
                        state => {
                            mem::replace(
                                state,
                                Segment::Literal(SegmentLiteral {
                                    literal: String::from(ch),
                                    quoted: false,
                                }),
                            )
                            .finish(&mut result)?;
                        }
                    }
                }
            }
        }

        if matches!(
            self.state,
            Segment::Literal(SegmentLiteral { quoted: true, .. })
        ) {
            return Err(PatternError::UnclosedLiteral);
        }

        self.state.finish(&mut result)?;

        Ok(result)
    }

    pub fn parse_generic(mut self) -> Result<Vec<GenericPatternItem>, PatternError> {
        let mut chars = self.source.chars().peekable();
        let mut result = vec![];

        while let Some(ch) = chars.next() {
            if !self.handle_generic_quoted_literal(ch, &mut chars)? {
                if ch == '{' {
                    mem::replace(
                        &mut self.state,
                        Segment::Literal(SegmentLiteral {
                            literal: String::new(),
                            quoted: false,
                        }),
                    )
                    .finish_generic(&mut result)?;

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
                } else if let Segment::Literal(SegmentLiteral {
                    ref mut literal, ..
                }) = self.state
                {
                    literal.push(ch);
                } else {
                    unreachable!()
                }
            }
        }

        if matches!(
            self.state,
            Segment::Literal(SegmentLiteral { quoted: true, .. })
        ) {
            return Err(PatternError::UnclosedLiteral);
        }

        self.state.finish_generic(&mut result)?;

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
        let samples = [
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
                    (
                        fields::DecimalSecond::SecondF2.into(),
                        FieldLength::TwoDigit,
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
        let samples = [
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

        let broken = [(" 'foo ", PatternError::UnclosedLiteral)];

        for (string, error) in broken {
            assert_eq!(Parser::new(string).parse(), Err(error),);
        }
    }

    #[test]
    fn pattern_parse_symbols() {
        let samples = [
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
                "s.SS",
                vec![(fields::DecimalSecond::SecondF2.into(), FieldLength::One).into()],
            ),
            (
                "sSS",
                vec![(fields::DecimalSecond::SecondF2.into(), FieldLength::One).into()],
            ),
            (
                "s.. z",
                vec![
                    (fields::Second::Second.into(), FieldLength::One).into(),
                    '.'.into(),
                    '.'.into(),
                    ' '.into(),
                    (fields::TimeZone::LowerZ.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.SSz",
                vec![
                    (fields::DecimalSecond::SecondF2.into(), FieldLength::One).into(),
                    (fields::TimeZone::LowerZ.into(), FieldLength::One).into(),
                ],
            ),
            (
                "sSSz",
                vec![
                    (fields::DecimalSecond::SecondF2.into(), FieldLength::One).into(),
                    (fields::TimeZone::LowerZ.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.SSss",
                vec![
                    (fields::DecimalSecond::SecondF2.into(), FieldLength::One).into(),
                    (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
                ],
            ),
            (
                "sSSss",
                vec![
                    (fields::DecimalSecond::SecondF2.into(), FieldLength::One).into(),
                    (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
                ],
            ),
            (
                "s.z",
                vec![
                    (fields::Second::Second.into(), FieldLength::One).into(),
                    '.'.into(),
                    (fields::TimeZone::LowerZ.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.ss",
                vec![
                    (fields::Second::Second.into(), FieldLength::One).into(),
                    '.'.into(),
                    (fields::Second::Second.into(), FieldLength::TwoDigit).into(),
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

        let broken = [
            (
                "yyyyyyy",
                PatternError::FieldLengthInvalid(FieldSymbol::Year(fields::Year::Calendar)),
            ),
            (
                "hh:mm:ss.SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS",
                PatternError::FieldLengthInvalid(FieldSymbol::Second(fields::Second::Second)),
            ),
        ];

        for (string, error) in broken {
            assert_eq!(Parser::new(string).parse(), Err(error),);
        }
    }

    #[test]
    fn pattern_parse_placeholders() {
        let samples = [
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

        let broken = [
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
