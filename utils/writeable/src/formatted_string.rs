// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Field, FormattedWriteableSink, Writeable};
use alloc::vec::Vec;
use core::fmt::{self, Write};
use core::str;

/// A string with formatting annotations.
#[derive(Clone, PartialEq)]
pub struct FormattedString {
    // bytes is always valid UTF-8, so from_utf8_unchecked is safe
    bytes: Vec<u8>,
    // The lists of annotations corresponding to each byte
    annotations: Vec<Vec<(LocationInPart, Field)>>,
    // The list of annotations for the next byte
    next_annotation: Vec<(LocationInPart, Field)>,
}

#[derive(Copy, Clone, PartialEq)]
enum LocationInPart {
    Begin,
    Extend,
}

impl FormattedString {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            annotations: Vec::new(),
            next_annotation: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
            annotations: Vec::with_capacity(capacity),
            next_annotation: Vec::new(),
        }
    }

    pub fn from_writeable<W: Writeable + ?Sized>(w: &W) -> Result<Self, fmt::Error> {
        let mut sink = Self::with_capacity(w.write_len().capacity());
        w.write_to_fmt(&mut sink)?;
        Ok(sink)
    }

    pub fn capacity(&self) -> usize {
        debug_assert_eq!(self.bytes.capacity(), self.annotations.capacity());
        self.bytes.capacity()
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.bytes.len(), self.annotations.len());
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(self.bytes.is_empty(), self.annotations.is_empty());
        self.bytes.is_empty()
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.bytes) }
    }

    fn make_next_annotation_extend(&mut self) {
        for entry in self.next_annotation.iter_mut() {
            *entry = (LocationInPart::Extend, entry.1);
        }
    }
}

impl Default for FormattedString {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for FormattedString {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.bytes.extend(s.bytes());
        self.annotations.reserve(s.len());
        self.annotations.push(self.next_annotation.clone());
        self.make_next_annotation_extend();
        for _ in 1..s.len() {
            self.annotations.push(self.next_annotation.clone());
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let len = c.len_utf8();
        self.bytes.resize(self.bytes.len() + len, 0);
        c.encode_utf8(&mut self.bytes[self.annotations.len()..]);
        self.annotations.push(self.next_annotation.clone());
        self.make_next_annotation_extend();
        for _ in 1..len {
            self.annotations.push(self.next_annotation.clone());
        }
        Ok(())
    }
}

impl FormattedWriteableSink for FormattedString {
    fn push_field(&mut self, field: Field) -> fmt::Result {
        self.next_annotation.push((LocationInPart::Begin, field));
        Ok(())
    }

    fn pop_field(&mut self) -> fmt::Result {
        self.next_annotation.pop();
        Ok(())
    }
}

impl fmt::Debug for FormattedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())?;
        // For each level of annotations
        for l in 0..self.annotations.iter().map(Vec::len).max().unwrap_or(0) {
            writeln!(f)?;
            let mut boundaries = Vec::new();
            let mut begin = None;
            // Iterating to len()+1 to close the last annotation
            for byte in 0..self.annotations.len() + 1 {
                // The "most significant" annotation is last in the lists, but
                // we want to print if first, so we index from the back.
                match self.annotations.get(byte).and_then(|a| a.get(l)) {
                    None => {
                        // No annotation at this level/byte
                        if let Some(b) = begin {
                            boundaries.push((b, byte));
                        }
                        begin = None;
                    }
                    Some((lip, _)) if lip == &LocationInPart::Begin => {
                        // New annotation start
                        if let Some(b) = begin {
                            boundaries.push((b, byte));
                        }
                        begin = Some(byte);
                    }
                    _ => {}
                }
            }

            let str_len_before = |i: usize| {
                self.as_str()[if i == 0 { 0 } else { boundaries[i - 1].1 }..boundaries[i].0]
                    .chars()
                    .count()
            };
            let str_len_of = |i: usize| {
                self.as_str()[boundaries[i].0..boundaries[i].1]
                    .chars()
                    .count()
            };

            // First row, underlines all annotated chars
            for i in 0..boundaries.len() {
                write!(f, "{: <1$}", "", str_len_before(i))?;
                write!(f, "{:━<1$}", "┏", str_len_of(i))?;
            }
            // Prints one annotation per row
            for k in (0..boundaries.len()).rev() {
                writeln!(f)?;
                for i in 0..k {
                    // Lines for later annotations
                    write!(f, "{: <1$}", "", str_len_before(i))?;
                    write!(f, "{: <1$}", "┃", str_len_of(i))?;
                }
                write!(f, "{: <1$}", "", str_len_before(k))?;
                write!(f, "┗ {}", self.annotations[boundaries[k].0][l].1)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_annotations() {
        assert_eq!(
            format!("{:?}", FormattedString::from_writeable(&17).unwrap()),
            "17"
        );
    }

    #[test]
    fn test_basic() {
        struct TestWriteable;
        impl Writeable for TestWriteable {
            fn write_to_fmt<S: FormattedWriteableSink + ?Sized>(
                &self,
                sink: &mut S,
            ) -> fmt::Result {
                sink.push_field(Field("word"))?;
                sink.write_str("hello")?;
                sink.pop_field()?;
                sink.push_field(Field("space"))?;
                sink.write_str(" ")?;
                sink.pop_field()?;
                sink.push_field(Field("word"))?;
                sink.write_str("world")?;
                sink.pop_field()
            }
        }

        assert_eq!(
            format!(
                "{:?}",
                FormattedString::from_writeable(&TestWriteable).unwrap()
            ),
            "hello world\n\
             ┏━━━━┏┏━━━━\n\
             ┃    ┃┗ word\n\
             ┃    ┗ space\n\
             ┗ word"
        );
    }

    #[test]
    fn test_multi_level() {
        struct TestWriteable;
        impl Writeable for TestWriteable {
            fn write_to_fmt<S: FormattedWriteableSink + ?Sized>(
                &self,
                sink: &mut S,
            ) -> fmt::Result {
                sink.push_field(Field("greeting"))?;
                sink.push_field(Field("word"))?;
                sink.write_str("hello")?;
                sink.pop_field()?;
                sink.write_str(" ")?;
                sink.push_field(Field("word"))?;
                sink.write_str("world")?;
                sink.pop_field()?;
                sink.pop_field()?;
                sink.push_field(Field("emoji"))?;
                sink.write_char('😅')?;
                sink.pop_field()
            }
        }

        // Note that the second level is not complete, the space
        // and emoji aren't annotated.
        assert_eq!(
            format!(
                "{:?}",
                FormattedString::from_writeable(&TestWriteable).unwrap()
            ),
            "hello world😅\n\
             ┏━━━━━━━━━━┏\n\
             ┃          ┗ emoji\n\
             ┗ greeting\n\
             ┏━━━━ ┏━━━━\n\
             ┃     ┗ word\n\
             ┗ word"
        );
    }

    #[test]
    fn test_multi_byte() {
        struct TestWriteable;
        impl Writeable for TestWriteable {
            fn write_to_fmt<S: FormattedWriteableSink + ?Sized>(
                &self,
                sink: &mut S,
            ) -> fmt::Result {
                sink.push_field(Field("variable"))?;
                sink.write_str("π")?;
                sink.pop_field()?;
                sink.write_str(" ")?;
                sink.push_field(Field("operation"))?;
                sink.write_str("*")?;
                sink.pop_field()?;
                sink.write_str(" ")?;
                sink.push_field(Field("variable"))?;
                sink.write_str("x")
            }
        }

        assert_eq!(
            format!(
                "{:?}",
                FormattedString::from_writeable(&TestWriteable).unwrap()
            ),
            "π * x\n\
             ┏ ┏ ┏\n\
             ┃ ┃ ┗ variable\n\
             ┃ ┗ operation\n\
             ┗ variable"
        );
    }
}
