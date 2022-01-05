// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedWriteableSink;
use alloc::vec::Vec;
use core::fmt;
use core::str;

/// A string with L levels of formatting annotations.
#[derive(Clone, PartialEq)]
pub struct FormattedString {
    // bytes is always valid UTF-8, so from_utf8_unchecked is safe
    bytes: Vec<u8>,
    // The lists of annotations corresponding to each byte
    annotations: Vec<Vec<(LocationInPart, &'static str)>>,
    // The list of annotations for the next byte
    next_annotation: Vec<(LocationInPart, &'static str)>,
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

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.bytes) }
    }

    fn make_next_annotation_extend(&mut self) {
        for entry in self.next_annotation.iter_mut() {
            *entry = (LocationInPart::Extend, entry.1);
        }
    }
}

impl FormattedWriteableSink for FormattedString {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.bytes.extend(s.bytes());
        self.annotations.reserve(s.len());
        self.annotations.push(self.next_annotation.clone());
        self.make_next_annotation_extend();
        for _ in 1..s.len() {
            self.annotations.push(self.next_annotation.clone());
        }
        Ok(())
    }    
    
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> { 
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

    fn write_fmt_str(&mut self, s: &FormattedString) -> Result<(), Self::Error> {
        self.bytes.extend(s.bytes.iter().copied());
        self.annotations.reserve(s.len());
        self.annotations.push(
            s.annotations[0]
                .iter()
                .chain(self.next_annotation.iter())
                .copied()
                .collect(),
        );
        self.make_next_annotation_extend();
        for i in 1..s.len() {
            self.annotations.push(
                s.annotations[i]
                    .iter()
                    .chain(self.next_annotation.iter())
                    .copied()
                    .collect(),
            );
        }
        Ok(())
    }

    fn push_field(&mut self, field: &'static str) -> Result<(), Self::Error> {
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
        // For each level of annotations
        for l in 0..self.annotations.iter().map(Vec::len).max().unwrap_or(0) {
            write!(f, "\n")?;
            let mut boundaries = Vec::new();
            let mut begin = None;
            // Iterating to len()+1 to close the last annotation
            for byte in 0..self.annotations.len() + 1 {
                // The "most significant" annotation is last in the lists, but
                // we want to print if first, so we index from the back.
                match self
                    .annotations
                    .get(byte)
                    .and_then(|a| a.iter().nth_back(l))
                {
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
                write!(f, "\n")?;
                for i in 0..k {
                    // Lines for later annotations
                    write!(f, "{: <1$}", "", str_len_before(i))?;
                    write!(f, "{: <1$}", "┃", str_len_of(i))?;
                }
                write!(f, "{: <1$}", "", str_len_before(k))?;
                write!(
                    f,
                    "┗ {}",
                    self.annotations[boundaries[k].0]
                        [self.annotations[boundaries[k].0].len() - 1 - l]
                        .1
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let mut x = FormattedString::new();
        x.push_field("word").unwrap();
        x.write_str("hello").unwrap();
        x.pop_field().unwrap();
        x.push_field("space").unwrap();
        x.write_str(" ").unwrap();
        x.pop_field().unwrap();
        x.push_field("word").unwrap();
        x.write_str("world").unwrap();
        x.pop_field().unwrap();

        assert_eq!(
            format!("{:?}", x),
            "hello world\n\
             ┏━━━━┏┏━━━━\n\
             ┃    ┃┗ word\n\
             ┃    ┗ space\n\
             ┗ word"
        );
    }

    #[test]
    fn test_multi_level() {
        let mut x = FormattedString::new();
        x.push_field("word").unwrap();
        x.write_str("hello").unwrap();
        x.pop_field().unwrap();
        x.write_str(" ").unwrap();
        x.push_field("word").unwrap();
        x.write_str("world").unwrap();
        x.pop_field().unwrap();

        // hello world
        // ┏━━━━ ┏━━━━
        // ┃     ┗ word
        // ┗ word

        let mut y = FormattedString::new();
        y.push_field("greeting").unwrap();
        y.write_fmt_str(&x).unwrap();
        y.pop_field().unwrap();
        y.push_field("emoji").unwrap();
        y.write_char('😅').unwrap();
        y.pop_field().unwrap();

        // Note that the second level is not complete, the space
        // and emoji aren't annotated.
        assert_eq!(
            format!("{:?}", y),
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
        let mut x = FormattedString::new();
        x.push_field("variable").unwrap();
        x.write_str("π").unwrap();
        x.pop_field().unwrap();
        x.write_str(" ").unwrap();
        x.push_field("operation").unwrap();
        x.write_str("*").unwrap();
        x.pop_field().unwrap();
        x.write_str(" ").unwrap();
        x.push_field("variable").unwrap();
        x.write_str("x").unwrap();

        assert_eq!(
            format!("{:?}", x),
            "π * x\n\
             ┏ ┏ ┏\n\
             ┃ ┃ ┗ variable\n\
             ┃ ┗ operation\n\
             ┗ variable"
        );
    }
}
