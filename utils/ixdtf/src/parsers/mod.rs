// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The parser module contains the implementation details for `IXDTFParser` and `ISODurationParser`

use crate::{ParserError, ParserResult};

extern crate alloc;

use alloc::{string::String, vec::Vec};

use self::duration::parse_duration;
use datetime::DateTimeFlags;
use grammar::is_annotation_open;
use records::{DateRecord, DurationParseRecord, IsoParseRecord, TimeZone};

pub mod records;

mod annotations;
pub(crate) mod datetime;
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
pub struct IxdtfParser {
    cursor: Cursor,
}

impl IxdtfParser {
    /// Creates a new `IXDTFParser` from a provided `&str`.
    pub fn new(target: &str) -> Self {
        Self {
            cursor: Cursor::new(target),
        }
    }

    /// Parses the source as a [DateTime string][temporal-dt].
    ///
    /// This is the baseline parser where the TimeRecord, UTCOffset, and all annotations are optional.
    ///
    /// [temporal-dt]: https://tc39.es/proposal-temporal/#prod-TemporalDateTimeString
    pub fn parse_date_time(&mut self) -> ParserResult<IsoParseRecord> {
        datetime::parse_annotated_date_time(DateTimeFlags::empty(), &mut self.cursor)
    }

    /// Parses the source as an [Instant string][temporal-instant]
    ///
    /// An instant string is laid out by the `Temporal` and is a stricter DateTime string with the
    /// TimeRecord and UTCOffset record being required.
    ///
    /// [temporal-instant]: https://tc39.es/proposal-temporal/#prod-TemporalInstantString
    pub fn parse_instant(&mut self) -> ParserResult<IsoParseRecord> {
        datetime::parse_annotated_date_time(
            DateTimeFlags::UTC_REQ | DateTimeFlags::TIME_REQ,
            &mut self.cursor,
        )
    }

    /// Parses the source aa a [ZonedDateTime string][temporal-zdt]
    ///
    /// Parses the string as a datetime string with the time zoned annotation being required.
    ///
    /// [temporal-zdt]: https://tc39.es/proposal-temporal/#prod-AnnotatedDateTime
    pub fn parse_zoned_date_time(&mut self) -> ParserResult<IsoParseRecord> {
        datetime::parse_annotated_date_time(DateTimeFlags::ZONED, &mut self.cursor)
    }

    /// Parses the source as a [YearMonth string][temporal-ym]
    ///
    /// This will parse a valid date time string or year month string.
    ///
    /// [temporal-ym]: https://tc39.es/proposal-temporal/#prod-TemporalYearMonthString
    pub fn parse_year_month(&mut self) -> ParserResult<IsoParseRecord> {
        let ym = datetime::parse_year_month(&mut self.cursor);

        let Ok(year_month) = ym else {
            self.cursor.pos = 0;
            return datetime::parse_annotated_date_time(DateTimeFlags::empty(), &mut self.cursor);
        };

        let calendar = if self.cursor.check_or(false, is_annotation_open) {
            let set = annotations::parse_annotation_set(false, &mut self.cursor)?;
            set.calendar
        } else {
            None
        };

        self.cursor.close()?;

        Ok(IsoParseRecord {
            date: DateRecord {
                year: year_month.0,
                month: year_month.1,
                day: 1,
            },
            time: None,
            tz: None,
            calendar,
        })
    }

    /// Parses the source as a [MonthDay string][temporal-md]
    ///
    /// This will parse a valid date time string or month day string.
    ///
    /// [temporal-md]: https://tc39.es/proposal-temporal/#prod-TemporalMonthDayString
    pub fn parse_month_day(&mut self) -> ParserResult<IsoParseRecord> {
        let md = datetime::parse_month_day(&mut self.cursor);

        let Ok(month_day) = md else {
            self.cursor.pos = 0;
            return datetime::parse_annotated_date_time(DateTimeFlags::empty(), &mut self.cursor);
        };

        let calendar = if self.cursor.check_or(false, is_annotation_open) {
            let set = annotations::parse_annotation_set(false, &mut self.cursor)?;
            set.calendar
        } else {
            None
        };

        self.cursor.close()?;

        Ok(IsoParseRecord {
            date: DateRecord {
                year: 0,
                month: month_day.0,
                day: month_day.1,
            },
            time: None,
            tz: None,
            calendar,
        })
    }

    /// Parses the source as a [Time string][temporal-time].
    ///
    /// This method will parse as an annotated time string or annotated date time string.
    ///
    /// [temporal-time]: https://tc39.es/proposal-temporal/#prod-TemporalTimeString
    pub fn parse_time(&mut self) -> ParserResult<IsoParseRecord> {
        match time::parse_annotated_time_record(&mut self.cursor) {
            Ok(None) => {
                self.cursor.pos = 0;
                datetime::parse_annotated_date_time(DateTimeFlags::TIME_REQ, &mut self.cursor)
            }
            Ok(Some(result)) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// Parses the source as a Time Zone string.
    pub fn parse_time_zone(&mut self) -> ParserResult<TimeZone> {
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
#[derive(Debug)]
pub struct IsoDurationParser {
    cursor: Cursor,
}

impl IsoDurationParser {
    /// Creates a new `IsoDurationParser` from a target `&str`.
    pub fn new(target: &str) -> Self {
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
pub(crate) struct Cursor {
    pos: u32,
    source: Vec<char>,
}

impl Cursor {
    /// Create a new cursor from a source `String` value.
    #[must_use]
    pub(crate) fn new(source: &str) -> Self {
        Self {
            pos: 0,
            source: source.chars().collect(),
        }
    }

    /// Returns a string value from a slice of the cursor.
    fn slice(&self, start: u32, end: u32) -> String {
        let slice = self.source.get(start as usize..end as usize);
        let Some(slice) = slice else {
            return String::default();
        };
        slice.iter().collect()
    }

    /// Get current position
    const fn pos(&self) -> u32 {
        self.pos
    }

    /// Peek the value at next position (current + 1).
    fn peek(&self) -> Option<char> {
        self.peek_n(1)
    }

    /// Peek the value at n len from current.
    fn peek_n(&self, n: u32) -> Option<char> {
        let target = (self.pos + n) as usize;
        self.source.get(target).copied()
    }

    /// Runs the provided check on the current position.
    fn check<F>(&self, f: F) -> Option<bool>
    where
        F: FnOnce(char) -> bool,
    {
        self.peek_n(0).map(f)
    }

    /// Runs the provided check on current position returns the default value if None.
    fn check_or<F>(&self, default: bool, f: F) -> bool
    where
        F: FnOnce(char) -> bool,
    {
        self.peek_n(0).map_or(default, f)
    }

    /// Returns `Cursor`'s current char and advances to the next position.
    fn next(&mut self) -> Option<char> {
        let result = self.peek_n(0);
        self.advance();
        result
    }

    /// Returns the next value as a digit.
    ///
    /// # Errors
    ///   - Returns a SyntaxError if value is not an ascii digit
    ///   - Returns an AbruptEnd error if cursor ends.
    fn next_digit(&mut self) -> ParserResult<Option<u8>> {
        let p_digit = self.abrupt_next()?.to_digit(10);
        let Some(digit) = p_digit else {
            return Ok(None);
        };
        Ok(Some(digit as u8))
    }

    /// A utility next method that returns an `AbruptEnd` error if invalid.
    fn abrupt_next(&mut self) -> ParserResult<char> {
        self.next().ok_or_else(ParserError::abrupt_end)
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

    /// Advances the cursor's position by `n`.
    fn advance_n(&mut self, n: u32) {
        self.pos += n;
    }

    /// Closes the current cursor by checking if all contents have been consumed. If not, returns an error for invalid syntax.
    fn close(&mut self) -> ParserResult<()> {
        if (self.pos as usize) < self.source.len() {
            return Err(ParserError::InvalidEnd);
        }
        Ok(())
    }
}
