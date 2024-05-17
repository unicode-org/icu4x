// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The parser module contains the implementation details for `IXDTFParser` and `ISODurationParser`

use crate::{ParserError, ParserResult};

#[cfg(feature = "duration")]
use records::DurationParseRecord;
use records::IxdtfParseRecord;

use self::records::Annotation;

pub mod records;

mod annotations;
pub(crate) mod datetime;
#[cfg(feature = "duration")]
pub(crate) mod duration;
mod grammar;
mod time;
pub(crate) mod timezone;

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
/// laid out in [RFC9557][ref-9557] along with other variations laid out in the [`Temporal`][temporal-proposal].
///
/// ```rust
/// use ixdtf::parsers::{IxdtfParser, records::{Sign, TimeZoneRecord, UTCOffsetRecord}};
///
/// let ixdtf_str = "2024-03-02T08:48:00-05:00[America/New_York]";
///
/// let result = IxdtfParser::new(ixdtf_str).parse().unwrap();
///
/// let date = result.date.unwrap();
/// let time = result.time.unwrap();
/// let offset = result.offset.unwrap();
/// let tz_annotation = result.tz.unwrap();
///
/// assert_eq!(date.year, 2024);
/// assert_eq!(date.month, 3);
/// assert_eq!(date.day, 2);
/// assert_eq!(time.hour, 8);
/// assert_eq!(time.minute, 48);
/// assert_eq!(offset.sign, Sign::Negative);
/// assert_eq!(offset.hour, 5);
/// assert_eq!(offset.minute, 0);
/// assert!(!tz_annotation.critical);
/// assert_eq!(tz_annotation.tz, TimeZoneRecord::Name("America/New_York"));
/// ```
///
/// [rfc-9557]: https://datatracker.ietf.org/doc/rfc9557/
/// [temporal-proposal]: https://tc39.es/proposal-temporal/
#[derive(Debug)]
pub struct IxdtfParser<'a> {
    cursor: Cursor<'a>,
}

impl<'a> IxdtfParser<'a> {
    /// Creates a new `IXDTFParser` from a provided `&str`.
    pub fn new(value: &'a str) -> Self {
        Self {
            cursor: Cursor::new(value),
        }
    }

    /// Parses the source as an [extended Date/Time string][rfc-9557].
    ///
    /// This is the baseline parser where the TimeRecord, UTCOffset, and all annotations are optional.
    ///
    /// [rfc-9557]: https://datatracker.ietf.org/doc/rfc9557/
    pub fn parse(&mut self) -> ParserResult<IxdtfParseRecord<'a>> {
        self.parse_with_annotation_handler(Some)
    }

    /// Parses the source as an extended Date/Time string with an Annotation handler.
    ///
    /// For more, see [Implementing Annotation Handlers](crate#implementing-annotation-handlers)
    pub fn parse_with_annotation_handler(
        &mut self,
        handler: impl FnMut(Annotation<'a>) -> Option<Annotation<'a>>,
    ) -> ParserResult<IxdtfParseRecord<'a>> {
        datetime::parse_annotated_date_time(&mut self.cursor, handler)
    }

    /// Parses the source as an extended [YearMonth string][temporal-ym].
    ///
    /// ```rust
    /// # use ixdtf::parsers::IxdtfParser;
    ///
    /// let extended_year_month = "2020-11[u-ca=iso8601]";
    ///
    /// let result = IxdtfParser::new(extended_year_month).parse_year_month().unwrap();
    ///
    /// let date = result.date.unwrap();
    ///
    /// assert_eq!(date.year, 2020);
    /// assert_eq!(date.month, 11);
    ///
    /// ```
    ///
    /// [temporal-ym]: https://tc39.es/proposal-temporal/#prod-TemporalYearMonthString
    pub fn parse_year_month(&mut self) -> ParserResult<IxdtfParseRecord<'a>> {
        self.parse_year_month_with_annotation_handler(Some)
    }

    /// Parses the source as an extended YearMonth string with an Annotation handler.
    ///
    /// For more, see [Implementing Annotation Handlers](crate#implementing-annotation-handlers)
    pub fn parse_year_month_with_annotation_handler(
        &mut self,
        handler: impl FnMut(Annotation<'a>) -> Option<Annotation<'a>>,
    ) -> ParserResult<IxdtfParseRecord<'a>> {
        datetime::parse_annotated_year_month(&mut self.cursor, handler)
    }

    /// Parses the source as an extended [MonthDay string][temporal-md].
    ///
    /// ```rust
    /// # use ixdtf::parsers::IxdtfParser;
    /// let extended_month_day = "1107[+04:00]";
    ///
    /// let result = IxdtfParser::new(extended_month_day).parse_month_day().unwrap();
    ///
    /// let date = result.date.unwrap();
    ///
    /// assert_eq!(date.month, 11);
    /// assert_eq!(date.day, 7);
    ///
    /// ```
    ///
    /// [temporal-md]: https://tc39.es/proposal-temporal/#prod-TemporalMonthDayString
    pub fn parse_month_day(&mut self) -> ParserResult<IxdtfParseRecord<'a>> {
        self.parse_month_day_with_annotation_handler(Some)
    }

    /// Parses the source as an extended MonthDay string with an Annotation handler.
    ///
    /// For more, see [Implementing Annotation Handlers](crate#implementing-annotation-handlers)
    pub fn parse_month_day_with_annotation_handler(
        &mut self,
        handler: impl FnMut(Annotation<'a>) -> Option<Annotation<'a>>,
    ) -> ParserResult<IxdtfParseRecord<'a>> {
        datetime::parse_annotated_month_day(&mut self.cursor, handler)
    }

    /// Parses the source as an extended [Time string][temporal-time].
    ///
    /// ```rust
    /// # use ixdtf::parsers::{IxdtfParser, records::{Sign, TimeZoneRecord}};
    /// let extended_time = "12:01:04-05:00[America/New_York][u-ca=iso8601]";
    ///
    /// let result = IxdtfParser::new(extended_time).parse_time().unwrap();
    ///
    /// let time = result.time.unwrap();
    /// let offset = result.offset.unwrap();
    /// let tz_annotation = result.tz.unwrap();
    ///
    /// assert_eq!(time.hour, 12);
    /// assert_eq!(time.minute, 1);
    /// assert_eq!(time.second, 4);
    /// assert_eq!(offset.sign, Sign::Negative);
    /// assert_eq!(offset.hour, 5);
    /// assert_eq!(offset.minute, 0);
    /// assert!(!tz_annotation.critical);
    /// assert_eq!(tz_annotation.tz, TimeZoneRecord::Name("America/New_York"));
    /// ```
    ///
    /// [temporal-time]: https://tc39.es/proposal-temporal/#prod-TemporalTimeString
    pub fn parse_time(&mut self) -> ParserResult<IxdtfParseRecord<'a>> {
        self.parse_time_with_annotation_handler(Some)
    }

    /// Parses the source as an extended Time string with an Annotation handler.
    ///
    /// For more, see [Implementing Annotation Handlers](crate#implementing-annotation-handlers)
    pub fn parse_time_with_annotation_handler(
        &mut self,
        handler: impl FnMut(Annotation<'a>) -> Option<Annotation<'a>>,
    ) -> ParserResult<IxdtfParseRecord<'a>> {
        time::parse_annotated_time_record(&mut self.cursor, handler)
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
/// let date_duration = result.date.unwrap();
///
/// assert_eq!(date_duration.years, 1);
/// assert_eq!(date_duration.months, 2);
/// assert_eq!(date_duration.weeks, 3);
/// assert_eq!(date_duration.days, 1);
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
    pub fn new(value: &'a str) -> Self {
        Self {
            cursor: Cursor::new(value),
        }
    }

    /// Parse the contents of this `IsoDurationParser` into a `DurationParseRecord`.
    pub fn parse(&mut self) -> ParserResult<DurationParseRecord> {
        duration::parse_duration(&mut self.cursor)
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
        let index = self.pos + n;
        self.source.get(index..index + 1)
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
