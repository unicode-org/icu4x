// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Core functionality for `ixdtf`'s parsers

use private::Sealed;

use crate::{ParseError, ParserResult};

mod private {
    pub trait Sealed {}
}


/// A trait for defining various supported encodings
/// and implementing functionality that is encoding
/// sensitive / specific.
pub trait UtfEncodingType : private::Sealed {
    type Encoding: PartialEq + core::fmt::Debug + Clone;

    /// Get a slice from the underlying source using for start..end
    fn slice(source: &[Self::Encoding], start: usize, end: usize) -> Option<&[Self::Encoding]>;

    /// Retrieve the provided index and return the value constrained to a
    /// representable ASCII range if required.
    fn get_ascii(source: &[Self::Encoding], index: usize) -> ParserResult<Option<u8>>;

    /// Checks for the known calendar annotation key `u-ca`.
    fn check_calendar_key(key: &[Self::Encoding]) -> bool;
}

/// A marker trait for providing a UTF-16 encoding to `ixdtf`'s parsers.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::exhaustive_structs)] // ZST Marker trait, no fields should be added
pub struct Utf16;

impl Sealed for Utf16 {}

impl UtfEncodingType for Utf16 {
    type Encoding = u16;
    fn slice(source: &[Self::Encoding], start: usize, end: usize) -> Option<&[Self::Encoding]> {
        source.get(start..end)
    }

    fn get_ascii(source: &[Self::Encoding], index: usize) -> ParserResult<Option<u8>> {
        source
            .get(index)
            .copied()
            .map(to_ascii_byte)
            .transpose()
            .map_err(|_| ParseError::Utf16NonAsciiChar)
    }

    fn check_calendar_key(key: &[Self::Encoding]) -> bool {
        key == [0x75, 0x2d, 0x63, 0x61]
    }
}

/// A marker trait for providing a UTF-8 encoding to `ixdtf`'s parsers.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::exhaustive_structs)] // ZST Marker trait, no fields should be added.
pub struct Utf8;

impl Sealed for Utf8 {}

impl UtfEncodingType for Utf8 {
    type Encoding = u8;

    fn slice<'a>(source: &[Self::Encoding], start: usize, end: usize) -> Option<&[Self::Encoding]> {
        source.get(start..end)
    }

    fn get_ascii(source: &[Self::Encoding], index: usize) -> ParserResult<Option<u8>> {
        Ok(source.get(index).copied())
    }

    fn check_calendar_key(key: &[Self::Encoding]) -> bool {
        key == "u-ca".as_bytes()
    }
}

#[inline]
fn to_ascii_byte(b: u16) -> ParserResult<u8> {
    if !(0x01..0x7F).contains(&b) {
        return Err(ParseError::Utf16NonAsciiChar);
    }
    Ok(b as u8)
}

// ==== Mini cursor implementation for Iso8601 targets ====

/// `Cursor` is a small cursor implementation for parsing Iso8601 grammar.
#[derive(Debug)]
pub(crate) struct Cursor<'a, T: UtfEncodingType> {
    pos: usize,
    source: &'a [T::Encoding],
}

impl<'a, T: UtfEncodingType> Cursor<'a, T> {
    /// Create a new cursor from a source UTF8 string.
    #[must_use]
    pub fn new(source: &'a [T::Encoding]) -> Self {
        Self { pos: 0, source }
    }

    /// Returns a string value from a slice of the cursor.
    pub(crate) fn slice(&self, start: usize, end: usize) -> Option<&'a [T::Encoding]> {
        T::slice(self.source, start, end)
    }

    /// Get current position
    pub(crate) const fn pos(&self) -> usize {
        self.pos
    }

    /// Peek the value at next position (current + 1).
    pub(crate) fn peek(&self) -> ParserResult<Option<u8>> {
        self.peek_n(1)
    }

    /// Returns current position in source as `char`.
    pub(crate) fn current(&self) -> ParserResult<Option<u8>> {
        self.peek_n(0)
    }

    /// Peeks the value at `n` as a `char`.
    pub(crate) fn peek_n(&self, n: usize) -> ParserResult<Option<u8>> {
        T::get_ascii(self.source, self.pos + n)
    }

    /// Runs the provided check on the current position.
    pub(crate) fn check<F>(&self, f: F) -> ParserResult<Option<bool>>
    where
        F: FnOnce(u8) -> bool,
    {
        Ok(self.current()?.map(f))
    }

    /// Runs the provided check on current position returns the default value if None.
    pub(crate) fn check_or<F>(&self, default: bool, f: F) -> ParserResult<bool>
    where
        F: FnOnce(u8) -> bool,
    {
        Ok(self.current()?.map_or(default, f))
    }

    /// Returns `Cursor`'s current char and advances to the next position.
    pub(crate) fn next(&mut self) -> ParserResult<Option<u8>> {
        let result = self.current();
        self.advance_n(1);
        result
    }

    /// Returns the next value as a digit
    ///
    /// # Errors
    ///   - Returns an AbruptEnd error if cursor ends.
    pub(crate) fn next_digit(&mut self) -> ParserResult<Option<u8>> {
        let ascii_char = self.next_or(ParseError::AbruptEnd { location: "digit" })?;
        if ascii_char.is_ascii_digit() {
            Ok(Some(ascii_char - 48))
        } else {
            Ok(None)
        }
    }

    /// A utility next method that returns an `AbruptEnd` error if invalid.
    pub(crate) fn next_or(&mut self, err: ParseError) -> ParserResult<u8> {
        self.next()?.ok_or(err)
    }

    /// Advances the cursor's position by n code points.
    pub(crate) fn advance_n(&mut self, n: usize) {
        self.pos += n;
    }

    // Advances the cursor by 1 code point.
    pub(crate) fn advance(&mut self) {
        self.advance_n(1)
    }

    /// Utility function to advance when a condition is true
    pub(crate) fn advance_if(&mut self, condition: bool) {
        if condition {
            self.advance();
        }
    }

    /// Closes the current cursor by checking if all contents have been consumed. If not, returns an error for invalid syntax.
    pub(crate) fn close(&mut self) -> ParserResult<()> {
        if self.pos < self.source.len() {
            return Err(ParseError::InvalidEnd);
        }
        Ok(())
    }
}
