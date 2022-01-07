// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Field, WriteFormatted, Writeable};
use alloc::vec::Vec;
use core::fmt::{self, Write};
use core::str;

/// A test utility that collects a Writeable  to a string and field annotations.
pub struct FormattedString {
    // bytes is always valid UTF-8, so from_utf8_unchecked is safe
    bytes: Vec<u8>,
    // The lists of annotations corresponding to each byte.
    annotations: Vec<Vec<(LocationInPart, Field)>>,
    // The list of annotations for the next byte.
    // The first entry is the top level.
    next_annotation: Vec<(LocationInPart, Field)>,
}

#[derive(Copy, Clone, PartialEq)]
enum LocationInPart {
    Begin,
    Extend,
}

impl FormattedString {
    pub fn from_writeable<W: Writeable + ?Sized>(w: &W) -> Result<Self, fmt::Error> {
        let len = w.write_len().capacity();
        let mut sink = Self {
            bytes: Vec::with_capacity(len),
            annotations: Vec::with_capacity(len),
            next_annotation: Vec::new(),
        };
        w.write_to_fmt(&mut sink)?;
        Ok(sink)
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.bytes) }
    }

    pub fn fields(&self) -> Vec<(usize, usize, Field)> {
        let mut output = Vec::<(usize, usize, Field)>::new();
        for l in 0..self.annotations.iter().map(Vec::len).max().unwrap_or(0) {
            let mut begin = None;
            // Iterating to len()+1 to close the last annotation
            for byte in 0..self.annotations.len() + 1 {
                match self.annotations.get(byte).and_then(|a| a.get(l)) {
                    None => {
                        // No annotation at this level/byte
                        if let Some(b) = begin {
                            output.push((b, byte, self.annotations[b][l].1));
                        }
                        begin = None;
                    }
                    Some((LocationInPart::Begin, _)) => {
                        // New field
                        if let Some(b) = begin {
                            output.push((b, byte, self.annotations[b][l].1));
                        }
                        begin = Some(byte);
                    }
                    _ => {}
                }
            }
        }
        output
    }

    fn make_next_annotation_extend(&mut self) {
        for entry in self.next_annotation.iter_mut() {
            *entry = (LocationInPart::Extend, entry.1);
        }
    }
}

impl Write for FormattedString {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if !s.is_empty() {
            self.bytes.extend(s.bytes());
            self.annotations.reserve(s.len());
            self.annotations.push(self.next_annotation.clone());
            self.make_next_annotation_extend();
            for _ in 1..s.len() {
                self.annotations.push(self.next_annotation.clone());
            }
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let len = c.len_utf8();
        self.bytes.resize(self.bytes.len() + len, 0);
        c.encode_utf8(&mut self.bytes[self.annotations.len()..]);
        self.annotations.reserve(len);
        self.annotations.push(self.next_annotation.clone());
        self.make_next_annotation_extend();
        for _ in 1..len {
            self.annotations.push(self.next_annotation.clone());
        }
        Ok(())
    }
}

impl WriteFormatted for FormattedString {
    fn push_field(&mut self, field: Field) -> fmt::Result {
        self.next_annotation.push((LocationInPart::Begin, field));
        Ok(())
    }

    fn pop_field(&mut self) -> fmt::Result {
        self.next_annotation.pop();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        struct TestWriteable;
        impl Writeable for TestWriteable {
            fn write_to_fmt<W: WriteFormatted + ?Sized>(&self, sink: &mut W) -> fmt::Result {
                sink.push_field(Field("greeting"))?;
                sink.push_field(Field("word"))?;
                sink.write_str("hello")?;
                sink.pop_field()?;
                sink.write_str(" ")?;
                sink.push_field(Field("number"))?;
                360.write_to(sink)?;
                sink.pop_field()?;
                sink.pop_field()?;
                sink.write_char(' ')?;
                sink.push_field(Field("emoji"))?;
                sink.write_char('😅')?;
                sink.pop_field()
            }
        }

        let materialized = FormattedString::from_writeable(&TestWriteable).unwrap();

        assert_eq!(materialized.as_str(), "hello 360 😅");
        assert_eq!(
            materialized.fields(),
            [
                (0, 9, Field("greeting")),
                (10, 14, Field("emoji")),
                (0, 5, Field("word")),
                (6, 9, Field("number"))
            ]
        );
    }
}
