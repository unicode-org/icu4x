// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use eyre::Error;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
    path::Path,
    rc::Rc,
};

use crate::core::{ParseError, ParseInput, ParseResult, Parsed};

/// A struct to read bytes from files that is capable of implementing [`ParseInput`].
///
/// The [`FileByteReader`] itself is easily clonable. Each clone shares the same internal [`BufReader`]
/// and only seek to new positions within the buffer itself.
///
/// This is no intended for use in multi-threaded contexts, but is very convenient for
/// single-threaded backtracking in a parser that may explore fallible branches.
#[derive(Debug)]
pub struct FileByteReader<R: Read + Seek> {
    /// The [`BufReader`].
    pub reader: Rc<RefCell<BufReader<R>>>,

    /// The position of the next byte to be read.
    pub position: u64,
}

impl<R: Read + Seek> Clone for FileByteReader<R> {
    fn clone(&self) -> Self {
        Self {
            reader: self.reader.clone(),
            position: self.position,
        }
    }
}

impl<R: Read + Seek> FileByteReader<R> {
    /// Reads `n` bytes and returns them in a vector.
    ///
    /// Returns an [`Err`] if the read failed, or if there is no more input.
    fn read(&self, n: usize) -> eyre::Result<Vec<u8>> {
        let mut buffer = vec![0; n];
        let mut reader = self.reader.borrow_mut();
        reader.seek(SeekFrom::Start(self.position))?;
        reader.read_exact(&mut buffer).map_err(|err| {
            match reader.fill_buf().map(|buf| buf.is_empty()) {
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
    fn advanced_by(&self, n: u64) -> Self {
        let mut this = self.clone();
        this.position += n;
        this
    }
}

impl FileByteReader<File> {
    /// Create a [`FileByteReader`] instance from a file path.
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> eyre::Result<Self> {
        Ok(FileByteReader {
            reader: Rc::new(RefCell::new(BufReader::new(File::open(path)?))),
            position: 0,
        })
    }
}

impl<R: Read + Seek> ParseInput<u8, Vec<u8>> for FileByteReader<R> {
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
        Ok(Parsed::new(buffer, self.advanced_by(n as u64)))
    }
}

#[cfg(test)]
mod test {
    use crate::{core::ParseInput, reader::file::FileByteReader};
    use std::io::prelude::*;

    #[test]
    fn next() {
        let mut source = FileByteReader::try_from_path("tests/alphabet.txt").unwrap();
        let mut parsed = source.next().unwrap();
        assert_eq!(*parsed.value(), b'a');
        let mut rest = Vec::new();
        parsed
            .source()
            .reader
            .borrow_mut()
            .read_to_end(&mut rest)
            .unwrap();
        assert_eq!(rest, b"bcdefg");
    }

    #[test]
    fn chain_next() {
        let mut source = FileByteReader::try_from_path("tests/alphabet.txt").unwrap();
        let mut parsed = source.next().next().unwrap();
        assert_eq!(*parsed.value(), b'b');
        let mut rest = Vec::new();
        parsed
            .source()
            .reader
            .borrow_mut()
            .read_to_end(&mut rest)
            .unwrap();
        assert_eq!(rest, b"cdefg");
    }

    #[test]
    fn take() {
        let mut source = FileByteReader::try_from_path("tests/alphabet.txt").unwrap();
        let mut parsed = source.take(3).unwrap();
        assert_eq!(*parsed.value(), b"abc");
        let mut rest = Vec::new();
        parsed
            .source()
            .reader
            .borrow_mut()
            .read_to_end(&mut rest)
            .unwrap();
        assert_eq!(rest, b"defg");
    }

    #[test]
    fn end_of_input() {
        let mut source = FileByteReader::try_from_path("tests/alphabet.txt").unwrap();
        let mut parsed1 = source.next().unwrap();
        let mut parsed2 = parsed1.next().unwrap();
        let mut parsed3 = parsed2.next().unwrap();
        let mut parsed4 = parsed3.next().unwrap();
        let mut parsed5 = parsed4.next().unwrap();
        let mut parsed6 = parsed5.next().unwrap();
        let mut parsed7 = parsed6.next().unwrap();

        assert!(!source.end_of_input());
        assert!(!parsed1.end_of_input());
        assert!(!parsed2.end_of_input());
        assert!(!parsed3.end_of_input());
        assert!(!parsed4.end_of_input());
        assert!(!parsed5.end_of_input());
        assert!(!parsed6.end_of_input());
        assert!(parsed7.end_of_input());
        assert!(parsed7.next().is_err());

        let mut chained = source.next().next().next().next().next().next().next();
        assert!(chained.is_ok());
        assert!(chained.end_of_input());
        assert!(chained.next().is_err());
    }
}
