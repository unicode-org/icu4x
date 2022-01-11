// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use super::Error;
use icu_provider::buf::BufferFormat;
use std::io::{self, Write};

/// A small helper class to convert LF to CRLF on Windows.
/// Workaround for https://github.com/serde-rs/json/issues/535
struct BufWriterWithLineEndingFix<W: io::Write>(io::BufWriter<W>);

impl<W: io::Write> BufWriterWithLineEndingFix<W> {
    pub fn new(inner: W) -> Self {
        Self(io::BufWriter::with_capacity(4096, inner))
    }
}

impl<W: io::Write> io::Write for BufWriterWithLineEndingFix<W> {
    #[cfg(windows)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.contains(&b'\n') {
            for b in buf.iter() {
                if *b == b'\n' {
                    // Note: Since we need to emit the \r, we are adding extra bytes than were in
                    // the input buffer. BufWriter helps because short writes (less than 4096 B)
                    // will always write or fail in their entirety.
                    self.0.write(b"\r\n")
                } else {
                    self.0.write(&[*b])
                }?;
            }
            // The return value is the number of *input* bytes that were written.
            Ok(buf.len())
        } else {
            self.0.write(buf)
        }
    }

    #[cfg(not(windows))]
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StyleOption {
    /// Print the smallest possible JSON, to reduce file size.
    Compact,
    /// Pretty-print JSON, to make it more readable.
    Pretty,
}

/// A serializer for JavaScript Object Notation (JSON).
pub struct Serializer {
    style: StyleOption,
}

/// Options bag for initializing a [`serde_json::Serializer`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    /// Format style to use when dumping output.
    pub style: StyleOption,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            style: StyleOption::Compact,
        }
    }
}

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        let mut sink = BufWriterWithLineEndingFix::new(sink);
        match self.style {
            StyleOption::Compact => {
                obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
                    &mut serde_json::Serializer::new(&mut sink),
                ))?;
            }
            StyleOption::Pretty => {
                obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
                    &mut serde_json::Serializer::pretty(&mut sink),
                ))?;
            }
        };
        // Write an empty line at the end of the document
        writeln!(sink)?;
        Ok(())
    }

    fn get_buffer_format(&self) -> BufferFormat {
        BufferFormat::Json
    }
}

impl Serializer {
    pub fn new(options: Options) -> Self {
        Self {
            style: options.style,
        }
    }
}
