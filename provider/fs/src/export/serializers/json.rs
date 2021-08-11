// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::AbstractSerializer;
use super::Error;
use crate::manifest::SyntaxOption;
use std::io;
use std::ops::Deref;

/// A small helper class to convert LF to CRLF on Windows.
/// Workaround for https://github.com/serde-rs/json/issues/535
struct LineEndingFixer<W: io::Write>(pub W);

impl<W: io::Write> io::Write for LineEndingFixer<W> {
    #[cfg(windows)]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.contains(&b'\n') {
            // Note: std::io::Write::write returns the number of bytes of the input buffer that
            // were successfully written.
            let mut bytes_of_buf_written = 0;
            for b in buf.iter() {
                match if *b == b'\n' {
                    self.0.write(b"\r\n")
                } else {
                    self.0.write(&[*b])
                } {
                    Ok(_) => bytes_of_buf_written += 1,
                    Err(e) => {
                        if bytes_of_buf_written == 0 {
                            return Err(e);
                        } else {
                            return Ok(bytes_of_buf_written);
                        }
                    }
                }
            }
            assert_eq!(bytes_of_buf_written, buf.len());
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
    syntax: SyntaxOption,
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

impl Deref for Serializer {
    type Target = SyntaxOption;

    fn deref(&self) -> &Self::Target {
        &self.syntax
    }
}

impl AbstractSerializer for Serializer {
    fn serialize(
        &self,
        obj: &dyn erased_serde::Serialize,
        mut sink: &mut dyn io::Write,
    ) -> Result<(), Error> {
        match self.style {
            StyleOption::Compact => {
                obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
                    &mut serde_json::Serializer::new(&mut sink),
                ))?;
            }
            StyleOption::Pretty => {
                let mut new_sink = LineEndingFixer(sink);
                obj.erased_serialize(&mut <dyn erased_serde::Serializer>::erase(
                    &mut serde_json::Serializer::pretty(&mut new_sink),
                ))?;
                sink = new_sink.0; // return the object to the outer scope
            }
        };
        // Write an empty line at the end of the document
        writeln!(sink)?;
        Ok(())
    }
}

impl Serializer {
    pub fn new(options: Options) -> Self {
        Self {
            syntax: SyntaxOption::Json,
            style: options.style,
        }
    }
}
