// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The parser module contains the implementation details for `IXDTFParser` and `ISODurationParser`

use crate::{ParserError, ParserResult};

extern crate alloc;

use self::{
    duration::parse_duration,
    grammar::is_annotation_open,
    records::{DateRecord, TimeZone},
};
use alloc::vec::Vec;
use records::{DurationParseRecord, IsoParseRecord};

pub mod records;

mod annotations;
pub(crate) mod datetime;
#[cfg(feature = "duration")]
pub(crate) mod duration;
mod grammar;
mod time;
pub(crate) mod time_zone;

#[cfg(test)]
mod tests;

/// `assert_syntax!` is a parser specific utility macro for asserting a syntax test, and returning the
/// the provided provided error if the assertion fails.
#[macro_export]
macro_rules! assert_syntax {
    ($cond:expr, $err:ident $(,)?) => {
        if !$cond {
            return Err(ParserError::$err);
        }
    };
}

/// `IxdtfParser` is the primary parser implementation of `ixdtf`.
///
/// This parser provides various options for parsing date/time strings with the extended notation
/// laid out in [sedate's IXDTF][ixdtf-draft] along with other variations laid out in the [`Temporal`][temporal-proposal].
///
/// [ixdtf-draft]: https://datatracker.ietf.org/doc/draft-ietf-sedate-datetime-extended/
/// [temporal-proposal]: https://tc39.es/proposal-temporal/
#[derive(Debug)]
pub struct IxdtfParser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> IxdtfParser<'a> {
    /// Creates a new `IXDTFParser` from a provided `&str`.
    pub fn new(target: &'a str) -> Self {
        Self {
            cursor: Cursor::new(target),
        }
    }

    /// Parses the source as a [DateTime string][temporal-dt].
    ///
    /// This is the baseline parser where the TimeRecord, UTCOffset, and all annotations are optional.
    ///
    /// [temporal-dt]: https://tc39.es/proposal-temporal/#prod-TemporalDateTimeString
    pub fn parse(&mut self) -> ParserResult<IsoParseRecord> {
        datetime::parse_annotated_date_time(&mut self.cursor)
    }
}

/// An extended parser for parsing additional extended datetime formats like
/// `AnnotatedYearMonth`s, `AnnotatedMonthDay`s and `AnnotatedTime`s
#[cfg(feature = "temporal")]
#[derive(Debug)]
pub struct TemporalParser<'a> {
    cursor: Cursor<'a>,
}

#[cfg(feature = "temporal")]
impl<'a> TemporalParser<'a> {
    /// Creates a new `TemporalParser` from a provided `&str`.
    pub fn new(target: &'a str) -> Self {
        Self {
            cursor: Cursor::new(target),
        }
    }

    /// Parses the target value as an extended month-day string.
    pub fn parse_month_day(&mut self) -> ParserResult<IsoParseRecord> {
        let md = datetime::parse_month_day(&mut self.cursor);

        let Ok(month_day) = md else {
            self.cursor.pos = 0;
            return datetime::parse_annotated_date_time(&mut self.cursor);
        };

        let (tz, calendar, annotations) = if self.cursor.check_or(false, is_annotation_open) {
            let set = annotations::parse_annotation_set(&mut self.cursor)?;

            let tz = if let Some(tz_anno) = set.tz {
                Some(tz_anno.tz)
            } else {
                None
            };

            (tz, set.calendar, set.annotations)
        } else {
            (None, None, Vec::default())
        };

        self.cursor.close()?;

        Ok(IsoParseRecord {
            date: Some(DateRecord {
                year: 0,
                month: month_day.0,
                day: month_day.1,
            }),
            time: None,
            offset: None,
            tz,
            calendar,
            annotations,
        })
    }

    /// Parses the target value as an extended year-month string.
    pub fn parse_year_month(&mut self) -> ParserResult<IsoParseRecord> {
        let ym = datetime::parse_year_month(&mut self.cursor);

        let Ok(year_month) = ym else {
            self.cursor.pos = 0;
            return datetime::parse_annotated_date_time(&mut self.cursor);
        };

        let (tz, calendar, annotations) = if self.cursor.check_or(false, is_annotation_open) {
            let set = annotations::parse_annotation_set(&mut self.cursor)?;

            let tz = if let Some(tz_anno) = set.tz {
                Some(tz_anno.tz)
            } else {
                None
            };

            (tz, set.calendar, set.annotations)
        } else {
            (None, None, Vec::default())
        };

        self.cursor.close()?;

        Ok(IsoParseRecord {
            date: Some(DateRecord {
                year: year_month.0,
                month: year_month.1,
                day: 1,
            }),
            time: None,
            offset: None,
            tz,
            calendar,
            annotations,
        })
    }

    /// Parses the target value as an extended `Time` string
    pub fn parse_time(&mut self) -> ParserResult<IsoParseRecord> {
        match time::parse_annotated_time_record(&mut self.cursor) {
            Ok(None) => {
                self.cursor.pos = 0;
                datetime::parse_annotated_date_time(&mut self.cursor)
            }
            Ok(Some(result)) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Parses the target value as a `TimeZone`.
    pub fn parse_time_zone(&mut self) -> ParserResult<TimeZone<'a>> {
        time_zone::parse_time_zone(&mut self.cursor)
    }
}

/// A parser for Duration strings.
///
/// # Exmaple
///
/// ```
/// use ixdtf::parsers::{IsoDurationParser, records::DurationParseRecord };
///
/// let duration_str = "P1Y2M3W1D";
///
/// let result = IsoDurationParser::new(duration_str).parse().unwrap();
///
/// assert_eq!(result.years, 1);
/// assert_eq!(result.months, 2);
/// assert_eq!(result.weeks, 3);
/// assert_eq!(result.days, 1);
///
/// ```
#[cfg(feature = "duration")]
#[derive(Debug)]
pub struct IsoDurationParser<'a> {
    cursor: Cursor<'a>,
}

#[cfg(feature = "duration")]
impl<'a> IsoDurationParser<'a> {
    /// Creates a new `IsoDurationParser` from a target `&str`.
    pub fn new(target: &'a str) -> Self {
        Self {
            cursor: Cursor::new(target),
        }
    }

    /// Parse the contents of this `IsoDurationParser` into a `DurationParseRecord`.
    pub fn parse(&mut self) -> ParserResult<DurationParseRecord> {
        parse_duration(&mut self.cursor)
    }
}

// ==== Mini cursor implementation for Iso8601 targets ====

/// `Cursor` is a small cursor implementation for parsing Iso8601 grammar.
#[derive(Debug)]
pub(crate) struct Cursor<'a> {
    pos: usize,
    source: &'a str,
}

impl<'a> Cursor<'a> {
    /// Create a new cursor from a source `String` value.
    #[must_use]
    pub(crate) fn new(source: &'a str) -> Self {
        Self { pos: 0, source }
    }

    /// Returns a string value from a slice of the cursor.
    fn slice(&self, start: usize, end: usize) -> Option<&'a str> {
        self.source.get(start..end)
    }

    /// Get current position
    const fn pos(&self) -> usize {
        self.pos
    }

    /// Peek the value at next position (current + 1).
    fn peek(&self) -> Option<char> {
        self.peek_n_char(1)
    }

    /// Returns current position in source as `char`.
    fn current(&self) -> Option<char> {
        self.peek_n_char(0)
    }

    /// Peek the value at n len from current.
    fn peek_n(&self, n: usize) -> Option<&'a str> {
        let target = self.pos + n;
        self.source.get(target..target + 1)
    }

    /// Peeks the value at `n` as a `char`.
    fn peek_n_char(&self, n: usize) -> Option<char> {
        self.peek_n(n).map(str::chars).and_then(|mut c| c.next())
    }

    /// Runs the provided check on the current position.
    fn check<F>(&self, f: F) -> Option<bool>
    where
        F: FnOnce(char) -> bool,
    {
        self.current().map(f)
    }

    /// Runs the provided check on current position returns the default value if None.
    fn check_or<F>(&self, default: bool, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        self.current().map_or(default, f)
    }

    /// Returns `Cursor`'s current char and advances to the next position.
    fn next(&mut self) -> Option<char> {
        let result = self.current();
        self.advance();
        result
    }

    /// Returns the next value as a digit.
    ///
    /// # Errors
    ///   - Returns a SyntaxError if value is not an ascii digit
    ///   - Returns an AbruptEnd error if cursor ends.
    fn next_digit(&mut self) -> ParserResult<Option<u8>> {
        let p_digit = self.next_or(ParserError::InvalidEnd)?.to_digit(10);
        let Some(digit) = p_digit else {
            return Ok(None);
        };
        Ok(Some(digit as u8))
    }

    /// A utility next method that returns an `AbruptEnd` error if invalid.
    fn next_or(&mut self, err: ParserError) -> ParserResult<char> {
        self.next().ok_or(err)
    }

    /// Advances the cursor's position by 1.
    fn advance(&mut self) {
        self.pos += 1;
    }

    /// Utility function to advance when a condition is true
    fn advance_if(&mut self, condition: bool) {
        if condition {
            self.advance();
        }
    }

    /// Closes the current cursor by checking if all contents have been consumed. If not, returns an error for invalid syntax.
    fn close(&mut self) -> ParserResult<()> {
        if self.pos < self.source.len() {
            return Err(ParserError::InvalidEnd);
        }
        Ok(())
    }
}
