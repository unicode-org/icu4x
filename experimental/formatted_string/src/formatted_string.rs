// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{Field, FormattedWriteableSink};
use alloc::vec::Vec;
use core::fmt;
use core::str;

/// A string with formatting annotations.
#[derive(Clone, PartialEq)]
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

    pub fn all_fields(&self) -> Vec<(usize, usize, Field)> {
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
}

impl Default for FormattedString {
    fn default() -> Self {
        Self::new()
    }
}

impl FormattedWriteableSink for FormattedString {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
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

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
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

    fn write_fmt_str(&mut self, s: &FormattedString) -> Result<(), Self::Error> {
        if !s.is_empty() {
            self.bytes.extend(s.bytes.iter().copied());
            self.annotations.reserve(s.len());
            self.annotations.push(
                self.next_annotation
                    .iter()
                    .chain(s.annotations[0].iter())
                    .copied()
                    .collect(),
            );
            self.make_next_annotation_extend();
            for i in 1..s.len() {
                self.annotations.push(
                    self.next_annotation
                        .iter()
                        .chain(s.annotations[i].iter())
                        .copied()
                        .collect(),
                );
            }
        }
        Ok(())
    }

    fn push_field(&mut self, field: Field) -> Result<(), Self::Error> {
        self.next_annotation.push((LocationInPart::Begin, field));
        Ok(())
    }

    fn pop_field(&mut self) -> Result<(), Self::Error> {
        self.next_annotation.pop();
        Ok(())
    }
}

impl fmt::Debug for FormattedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())?;
        for (begin, end, field) in self.all_fields() {
            write!(
                f,
                "\n{: <3$}{: <4$}) {}",
                "",
                "[",
                field,
                self.as_str()[..begin].chars().count(),
                self.as_str()[begin..end].chars().count()
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut x = FormattedString::new();
        x.push_field(Field("word")).unwrap();
        x.write_str("hello").unwrap();
        x.pop_field().unwrap();
        x.write_str(" ").unwrap();
        x.push_field(Field("word")).unwrap();
        x.write_str("world").unwrap();
        x.pop_field().unwrap();

        let mut y = FormattedString::new();
        y.push_field(Field("greeting")).unwrap();
        y.write_fmt_str(&x).unwrap();
        y.pop_field().unwrap();
        y.push_field(Field("emoji")).unwrap();
        y.write_char('😅').unwrap();
        y.pop_field().unwrap();

        assert_eq!(y.as_str(), "hello world😅");

        assert_eq!(
            y.all_fields(),
            [
                (0, 11, Field("greeting")),
                (11, 15, Field("emoji")),
                (0, 5, Field("word")),
                (6, 11, Field("word"))
            ]
        );
    }
}
