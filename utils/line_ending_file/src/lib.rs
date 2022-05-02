// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`BufWriterWithLineEndingFix`].

use std::io::{BufWriter, Result, Write};

/// A small helper class to convert LF to CRLF on Windows.
/// Workaround for <https://github.com/serde-rs/json/issues/535>
pub struct BufWriterWithLineEndingFix<W: Write>(BufWriter<W>);

impl<W: Write> BufWriterWithLineEndingFix<W> {
    pub fn new(inner: W) -> Self {
        Self(BufWriter::with_capacity(4096, inner))
    }
}

impl<W: Write> Write for BufWriterWithLineEndingFix<W> {
    #[cfg(windows)]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
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
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}
