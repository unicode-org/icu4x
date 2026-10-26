// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{
    super::error::PatternError,
    super::{GenericPatternItem, PatternItem},
    tokenizer::{Token, Uts35DateTimePatternTokenizer},
};
#[cfg(test)]
use super::{GenericPattern, Pattern};
use crate::provider::fields::{self, Field, FieldLength, FieldSymbol, TimeZone};
use alloc::vec;
use alloc::vec::Vec;

/// A parser for [UTS #35 Date Format Patterns] and generic combination ("glue") patterns.
///
/// [UTS #35 Date Format Patterns]: https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns
#[derive(Debug)]
pub struct Parser<'p> {
    source: &'p str,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        Self { source }
    }

    /// Parses a [UTS #35 Date Format Pattern] (such as `"yyyy-MM-dd HH:mm:ss.SSS"`) into
    /// a sequence of [`PatternItem`]s (date/time fields and literal characters).
    ///
    /// # Syntax Summary
    /// - **Field symbols**: Runs of identical ASCII letters (`a`–`z`, `A`–`Z`) are parsed into
    ///   a [`Field`] whose [`FieldSymbol`] is determined by the character (e.g. `'y'` for year,
    ///   `'M'` for month, `'d'` for day) and whose [`FieldLength`] is determined by the number
    ///   of repetitions (e.g. `"yyyy"` has length 4).
    /// - **Fractional seconds**: An `'s'` field immediately followed by `'S'` (or `".S"`) is
    ///   combined into a [`FieldSymbol::DecimalSecond`].
    /// - **Quoted literals**: Text enclosed in single quotes (`'...'`) is emitted as literal
    ///   characters (`PatternItem::Literal`). Two consecutive single quotes (`''`), either inside
    ///   or outside a quoted literal, produce a single `'`.
    /// - **Unquoted literals**: Non-alphabetic characters outside quotes (such as `:`, `/`, `-`,
    ///   spaces, and `{` / `}`) are emitted directly as `PatternItem::Literal`.
    ///
    /// Unlike [`Self::parse_generic`], this method interprets ASCII letters as date/time fields
    /// and does not parse `{0}`-style placeholders.
    ///
    /// [UTS #35 Date Format Pattern]: https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns
    pub fn parse(self) -> Result<Vec<PatternItem>, PatternError> {
        let mut tokenizer = Uts35DateTimePatternTokenizer(self.source);
        let mut result = vec![];

        while let Some(token) = tokenizer.step() {
            match token {
                Token::Symbol('s', integer_digits) => {
                    // Note: this accepts both "ssSSS" and "ss.SSS"
                    let mut lookahead = tokenizer.clone(); // trivial clone
                    let fraction_digits = match lookahead.step() {
                        Some(Token::Symbol('S', fraction_digits)) => {
                            tokenizer = lookahead;
                            fraction_digits
                        }
                        Some(Token::Literal(".")) => match lookahead.step() {
                            Some(Token::Symbol('S', fraction_digits)) => {
                                tokenizer = lookahead;
                                fraction_digits
                            }
                            _ => 0,
                        },
                        _ => 0,
                    };
                    let second_symbol = FieldSymbol::Second(fields::Second::Second);
                    let symbol = if fraction_digits == 0 {
                        second_symbol
                    } else {
                        let decimal_second = u8::try_from(fraction_digits)
                            .ok()
                            .and_then(|d| fields::DecimalSecond::from_idx(d).ok())
                            .ok_or(PatternError::FieldLengthInvalid(second_symbol))?;
                        FieldSymbol::DecimalSecond(decimal_second)
                    };
                    let length = u8::try_from(integer_digits)
                        .ok()
                        .and_then(|d| FieldLength::from_idx(d).ok())
                        .ok_or(PatternError::FieldLengthInvalid(symbol))?;
                    result.push(PatternItem::Field(Field { symbol, length }));
                }
                Token::Symbol('Z', length) => {
                    let (symbol, length) = match length {
                        // Z..ZZZ => xxxx
                        1..=3 => (FieldSymbol::TimeZone(TimeZone::Iso), FieldLength::Four),
                        // ZZZZ => OOOO
                        4 => (
                            FieldSymbol::TimeZone(TimeZone::LocalizedOffset),
                            FieldLength::Four,
                        ),
                        // ZZZZZ => XXXXX
                        5 => (FieldSymbol::TimeZone(TimeZone::IsoWithZ), FieldLength::Five),
                        _ => return Err(PatternError::UnknownSubstitution('Z')),
                    };
                    result.push(PatternItem::Field(Field { symbol, length }));
                }
                Token::Symbol(ch, length) => {
                    if let Ok(symbol) = FieldSymbol::try_from(ch) {
                        let length = u8::try_from(length)
                            .ok()
                            .and_then(|d| FieldLength::from_idx(d).ok())
                            .ok_or(PatternError::FieldLengthInvalid(symbol))?;
                        result.push(PatternItem::Field(Field { symbol, length }));
                    } else {
                        result.extend(core::iter::repeat_n(PatternItem::Literal(ch), length));
                    }
                }
                Token::Literal(s) => {
                    result.extend(s.chars().map(PatternItem::Literal));
                }
                Token::UnclosedLiteral(_) => {
                    return Err(PatternError::UnclosedLiteral);
                }
            }
        }

        Ok(result)
    }

    /// Parses a [UTS #35 date-time combination ("glue") pattern] (such as `"{1} 'at' {0}"`) into
    /// a sequence of [`GenericPatternItem`]s (numbered placeholders and literal characters).
    ///
    /// Unlike [`Self::parse`], which interprets unquoted ASCII letters as date/time field symbols
    /// (`y`, `M`, `d`, etc.) and treats `{` / `}` as literals, `parse_generic` parses `{0}`..`{9}`
    /// substitutions into [`GenericPatternItem::Placeholder`] and treats all other unquoted or
    /// single-quoted characters (including ASCII letters) as [`GenericPatternItem::Literal`].
    ///
    /// [UTS #35 date-time combination ("glue") pattern]: https://unicode.org/reports/tr35/tr35-dates.html#dateTimeFormat
    pub fn parse_generic(self) -> Result<Vec<GenericPatternItem>, PatternError> {
        #[derive(Debug)]
        struct DigitPlaceholder(u8);

        impl core::str::FromStr for DigitPlaceholder {
            type Err = PatternError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut it = s.chars();
                let ch = it.next().ok_or(PatternError::UnclosedPlaceholder)?;
                let idx = ch
                    .to_digit(10)
                    .ok_or(PatternError::UnknownSubstitution(ch))? as u8;
                if it.next().is_some() {
                    return Err(PatternError::UnclosedPlaceholder);
                }
                Ok(Self(idx))
            }
        }

        let mut parser = icu_pattern::Parser::<DigitPlaceholder>::new(
            self.source,
            icu_pattern::QuoteMode::QuotingSupported.into(),
        );
        let mut result = vec![];

        while let Some(item) = parser.try_next().map_err(|e| match e {
            icu_pattern::ParserError::InvalidPlaceholder(err) => err,
            icu_pattern::ParserError::UnclosedPlaceholder => PatternError::UnclosedPlaceholder,
            icu_pattern::ParserError::UnclosedQuotedLiteral => PatternError::UnclosedLiteral,
            icu_pattern::ParserError::IllegalCharacter(ch) => PatternError::InvalidSymbol(ch),
            _ => PatternError::UnclosedPlaceholder,
        })? {
            match item {
                icu_pattern::ParsedPatternItem::Placeholder(DigitPlaceholder(idx)) => {
                    result.push(GenericPatternItem::Placeholder(idx));
                }
                icu_pattern::ParsedPatternItem::Literal { content, .. } => {
                    result.extend(content.chars().map(GenericPatternItem::Literal));
                }
                _ => {}
            }
        }

        Ok(result)
    }

    #[cfg(test)]
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
    use super::super::super::reference::Pattern;
    use super::*;
    use crate::provider::fields::{self, FieldLength};

    #[test]
    fn pattern_parse_simple() {
        let samples = [
            (
                "dd/MM/y",
                vec![
                    (fields::Day::DayOfMonth.into(), FieldLength::Two).into(),
                    '/'.into(),
                    (fields::Month::Format.into(), FieldLength::Two).into(),
                    '/'.into(),
                    (fields::Year::Calendar.into(), FieldLength::One).into(),
                ],
            ),
            (
                "HH:mm:ss",
                vec![
                    (fields::Hour::H23.into(), FieldLength::Two).into(),
                    ':'.into(),
                    (FieldSymbol::Minute, FieldLength::Two).into(),
                    ':'.into(),
                    (fields::Second::Second.into(), FieldLength::Two).into(),
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
                    (fields::Hour::H23.into(), FieldLength::Two).into(),
                    ':'.into(),
                    (FieldSymbol::Minute, FieldLength::Two).into(),
                    ':'.into(),
                    (fields::DecimalSecond::Subsecond2.into(), FieldLength::Two).into(),
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
                vec![(fields::Year::Calendar.into(), FieldLength::Two).into()],
            ),
            (
                "yyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Three).into()],
            ),
            (
                "yyyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Four).into()],
            ),
            (
                "yyyyy",
                vec![(fields::Year::Calendar.into(), FieldLength::Five).into()],
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
                    (fields::Hour::H12.into(), FieldLength::Two).into(),
                    '\''.into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''b",
                vec![
                    (fields::Hour::H12.into(), FieldLength::Two).into(),
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
                    (fields::Hour::H12.into(), FieldLength::Two).into(),
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
                    (fields::Hour::H12.into(), FieldLength::Two).into(),
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
                    (fields::Hour::H12.into(), FieldLength::Two).into(),
                    '\''.into(),
                    (fields::DayPeriod::AmPm.into(), FieldLength::One).into(),
                ],
            ),
            (
                "hh''b",
                vec![
                    (fields::Hour::H12.into(), FieldLength::Two).into(),
                    '\''.into(),
                    (fields::DayPeriod::NoonMidnight.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.SS",
                vec![(fields::DecimalSecond::Subsecond2.into(), FieldLength::One).into()],
            ),
            (
                "sSS",
                vec![(fields::DecimalSecond::Subsecond2.into(), FieldLength::One).into()],
            ),
            (
                "s.. z",
                vec![
                    (fields::Second::Second.into(), FieldLength::One).into(),
                    '.'.into(),
                    '.'.into(),
                    ' '.into(),
                    (TimeZone::SpecificNonLocation.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.SSz",
                vec![
                    (fields::DecimalSecond::Subsecond2.into(), FieldLength::One).into(),
                    (TimeZone::SpecificNonLocation.into(), FieldLength::One).into(),
                ],
            ),
            (
                "sSSz",
                vec![
                    (fields::DecimalSecond::Subsecond2.into(), FieldLength::One).into(),
                    (TimeZone::SpecificNonLocation.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.SSss",
                vec![
                    (fields::DecimalSecond::Subsecond2.into(), FieldLength::One).into(),
                    (fields::Second::Second.into(), FieldLength::Two).into(),
                ],
            ),
            (
                "sSSss",
                vec![
                    (fields::DecimalSecond::Subsecond2.into(), FieldLength::One).into(),
                    (fields::Second::Second.into(), FieldLength::Two).into(),
                ],
            ),
            (
                "s.z",
                vec![
                    (fields::Second::Second.into(), FieldLength::One).into(),
                    '.'.into(),
                    (TimeZone::SpecificNonLocation.into(), FieldLength::One).into(),
                ],
            ),
            (
                "s.ss",
                vec![
                    (fields::Second::Second.into(), FieldLength::One).into(),
                    '.'.into(),
                    (fields::Second::Second.into(), FieldLength::Two).into(),
                ],
            ),
            (
                "z",
                vec![(TimeZone::SpecificNonLocation.into(), FieldLength::One).into()],
            ),
            ("Z", vec![(TimeZone::Iso.into(), FieldLength::Four).into()]),
            ("ZZ", vec![(TimeZone::Iso.into(), FieldLength::Four).into()]),
            (
                "ZZZ",
                vec![(TimeZone::Iso.into(), FieldLength::Four).into()],
            ),
            (
                "ZZZZ",
                vec![(TimeZone::LocalizedOffset.into(), FieldLength::Four).into()],
            ),
            (
                "ZZZZZ",
                vec![(TimeZone::IsoWithZ.into(), FieldLength::Five).into()],
            ),
            (
                "O",
                vec![(TimeZone::LocalizedOffset.into(), FieldLength::One).into()],
            ),
            (
                "v",
                vec![(TimeZone::GenericNonLocation.into(), FieldLength::One).into()],
            ),
            (
                "V",
                vec![(TimeZone::Location.into(), FieldLength::One).into()],
            ),
            ("x", vec![(TimeZone::Iso.into(), FieldLength::One).into()]),
            (
                "X",
                vec![(TimeZone::IsoWithZ.into(), FieldLength::One).into()],
            ),
        ];

        for (string, pattern) in samples {
            assert_eq!(
                Parser::new(string)
                    .parse()
                    .expect("Parsing pattern failed."),
                pattern,
                "{string}",
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
