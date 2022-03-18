// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::Error;
use std::io::{BufRead, BufReader, Read};

use crate::core::{ParseError, ParseInput, ParseResult, Parsed};

/// A struct to read bytes from slices that is capable of implementing [`ParseInput`].
///
/// The [`SliceByteReader`] itself is easily clonable. Each clone shares the same internal
/// reference to the bytes and may differ in position.
///
/// This is not intended for use in multi-threaded contexts, but is very convenient for
/// single-threaded backtracking in a parser that may explore fallible branches.
pub struct SliceByteReader<'a> {
    /// The slice.
    pub bytes: &'a [u8],

    /// The position of the next byte to be read.
    pub position: usize,
}

impl<'a> Clone for SliceByteReader<'a> {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes,
            position: self.position,
        }
    }
}

impl<'a> SliceByteReader<'a> {
    /// Create a [`SliceByteReader`] instance from a slice of bytes.
    pub fn with_slice_source(bytes: &'a [u8]) -> Self {
        SliceByteReader { bytes, position: 0 }
    }

    /// Create a [`SliceByteReader`] instance from a string slice.
    pub fn with_str_source(s: &'a str) -> Self {
        SliceByteReader {
            bytes: s.as_bytes(),
            position: 0,
        }
    }

    /// Reads `n` bytes and returns them in a vector.
    ///
    /// Returns an [`Err`] if the read failed, or if there is no more input.
    fn read(&self, n: usize) -> eyre::Result<Vec<u8>> {
        let mut buffer = vec![0; n];
        let mut sub_slice = BufReader::new(&self.bytes[self.position..]);
        sub_slice.read_exact(&mut buffer).map_err(|err| {
            match sub_slice.fill_buf().map(|buf| buf.is_empty()) {
                Ok(bool) => {
                    if bool {
                        Error::from(ParseError::EndOfInput)
                    } else {
                        Error::from(err)
                    }
                }
                Err(err) => Error::from(err),
            }
        })?;
        Ok(buffer)
    }

    /// Advances the position by `n` bytes.
    fn advanced_by(&self, n: usize) -> Self {
        let mut this = self.clone();
        this.position += n;
        this
    }
}

impl<'a> ParseInput<u8, Vec<u8>> for SliceByteReader<'a> {
    /// Returns the source.
    fn source(&mut self) -> eyre::Result<Self> {
        Ok(self.clone())
    }

    /// Reads and returns the next byte, returning both the byte and
    /// the source that is ready to read subsequent bytes from the input.
    fn next(&mut self) -> ParseResult<u8, Self> {
        let buffer = self.read(1)?;
        Ok(Parsed::new(buffer[0], self.advanced_by(1)))
    }

    /// Reads `n` bytes and returns them in a vector, returning both the bytes
    /// and the source that is ready to read subsequent bytes from the input.
    fn take(&mut self, n: usize) -> ParseResult<Vec<u8>, Self> {
        let buffer = self.read(n)?;
        Ok(Parsed::new(buffer, self.advanced_by(n)))
    }
}

#[cfg(test)]
mod test {
    use crate::{core::ParseInput, reader::slice::SliceByteReader};

    #[test]
    fn next() {
        let mut source = SliceByteReader::with_str_source("abcdefg");
        let mut parsed = source.next().unwrap();
        assert_eq!(*parsed.value(), b'a');
        let position = parsed.source().position;
        let rest = &parsed.source().bytes[position..];
        assert_eq!(rest, b"bcdefg");
    }

    #[test]
    fn chain_next() {
        let mut source = SliceByteReader::with_str_source("abcdefg");
        let mut parsed = source.next().next().unwrap();
        assert_eq!(*parsed.value(), b'b');
        let position = parsed.source().position;
        let rest = &parsed.source().bytes[position..];
        assert_eq!(rest, b"cdefg");
    }

    #[test]
    fn take() {
        let mut source = SliceByteReader::with_str_source("abcdefg");
        let mut parsed = source.take(2).unwrap();
        assert_eq!(parsed.value(), b"ab");
        let position = parsed.source().position;
        let rest = &parsed.source().bytes[position..];
        assert_eq!(rest, b"cdefg");
    }

    #[test]
    fn end_of_input() {
        let mut source = SliceByteReader::with_str_source("123");
        let mut parsed1 = source.next().unwrap();
        let mut parsed2 = parsed1.next().unwrap();
        let mut parsed3 = parsed2.next().unwrap();

        assert!(!source.end_of_input());
        assert!(!parsed1.end_of_input());
        assert!(!parsed2.end_of_input());
        assert!(parsed3.end_of_input());
        assert!(parsed3.next().is_err());

        let mut chained = source.next().next().next();
        assert!(chained.is_ok());
        assert!(chained.end_of_input());
        assert!(chained.next().is_err());
    }
}
