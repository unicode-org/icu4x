// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedStringBuilderError;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::{vec, vec::Vec};
use core::str::from_utf8_unchecked;

/// A string with L levels of type annotations
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct LayeredFormattedString<F: Copy, const L: usize> {
    string: String,
    // The first dimension corresponds to the bytes, the second are the L levels of annotations
    annotations: Box<[Annotation<F, L>]>,
}

pub type FormattedString<F> = LayeredFormattedString<F, 1>;

/// A string builder with L levels of type annotations.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct LayeredFormattedStringBuilder<F: Copy, const L: usize> {
    // bytes is always valid UTF-8, so from_utf8_unchecked is safe
    bytes: Vec<u8>,
    // The first dimension corresponds to the bytes, the second are the L levels of annotations
    annotations: Vec<Annotation<F, L>>,
}

pub type FormattedStringBuilder<F> = LayeredFormattedStringBuilder<F, 1>;

#[derive(Copy, Clone, PartialEq, Debug)]
enum LocationInPart {
    Begin,
    Extend,
}

// An L-level deep annotation for a single character, using F as field types
type Annotation<F, const L: usize> = [(LocationInPart, F); L];

// Transforms a list of Annotation<F, L-1>s into the same length list of Annotation<F, L>, adding the given field as the 0th level
fn raise_annotation<F: Copy, const L: usize, const L1: usize>(
    top_level: F,
    lower_levels: Vec<Annotation<F, L1>>,
) -> Vec<Annotation<F, L>> {
    assert_eq!(L - 1, L1);

    // Transforms an Annotation<F, L-1> into Annotation<F, L> by prepending the given level
    fn add_level<F: Copy, const L: usize, const L1: usize>(
        lower_levels: &Annotation<F, L1>,
        top_level: (LocationInPart, F),
    ) -> Annotation<F, L> {
        assert_eq!(L - 1, L1);
        let mut all_levels = [top_level; L];
        all_levels[1..L].clone_from_slice(&lower_levels[..L1]);
        all_levels
    }

    match lower_levels.len() {
        0 => vec![],
        n => {
            let mut vec: Vec<Annotation<F, L>> = Vec::with_capacity(n);
            vec.push(add_level(
                &lower_levels[0],
                (LocationInPart::Begin, top_level),
            ));
            for item in &lower_levels[1..n] {
                vec.push(add_level(item, (LocationInPart::Extend, top_level)));
            }
            vec
        }
    }
}

impl<F: Copy, const L: usize> LayeredFormattedStringBuilder<F, L> {
    pub fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(40),
            annotations: Vec::with_capacity(40),
        }
    }

    pub fn append_layered<const L1: usize>(
        &mut self,
        string: &LayeredFormattedString<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        // len() is always a char boundary
        self.insert_layered_internal(self.bytes.len(), string, field)
    }

    pub fn prepend_layered<const L1: usize>(
        &mut self,
        string: &LayeredFormattedString<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        // 0 is always a char boundary
        self.insert_layered_internal(0, string, field)
    }

    pub fn insert_layered<const L1: usize>(
        &mut self,
        pos: usize,
        string: &LayeredFormattedString<F, L1>,
        field: F,
    ) -> Result<&mut Self, FormattedStringBuilderError> {
        assert_eq!(L - 1, L1);
        if pos > self.bytes.len() {
            Err(FormattedStringBuilderError::IndexOutOfBounds(pos))
        } else if !unsafe { from_utf8_unchecked(&self.bytes) }.is_char_boundary(pos) {
            Err(FormattedStringBuilderError::PositionNotCharBoundary(
                pos,
                unsafe { String::from_utf8_unchecked(self.bytes.clone()) },
            ))
        } else {
            Ok(self.insert_layered_internal(pos, string, field))
        }
    }

    // Precondition here is that pos is a char boundary and < buffer_len.
    fn insert_layered_internal<const L1: usize>(
        &mut self,
        pos: usize,
        string: &LayeredFormattedString<F, L1>,
        field: F,
    ) -> &mut Self {
        assert_eq!(L - 1, L1);
        self.bytes.splice(pos..pos, string.as_ref().bytes());
        self.annotations.splice(
            pos..pos,
            raise_annotation(field, Vec::from(string.annotations.as_ref())),
        );
        self
    }

    pub fn build(self) -> LayeredFormattedString<F, L> {
        LayeredFormattedString {
            string: unsafe { String::from_utf8_unchecked(self.bytes) },
            annotations: self.annotations.into_boxed_slice(),
        }
    }
}

impl<F: Copy, const L: usize> LayeredFormattedString<F, L> {
    pub fn fields_at(&self, pos: usize) -> [F; L] {
        let mut res = [self.annotations[pos][0].1; L];
        for (i, (_bies, field)) in self.annotations[pos][1..L].iter().enumerate() {
            res[i + 1] = *field;
        }
        res
    }

    pub fn is_field_start(&self, pos: usize, level: usize) -> bool {
        assert!(level < L);
        let (location, _) = self.annotations[pos][level];
        location == LocationInPart::Begin
    }
}

impl<F: Copy, const L: usize> AsRef<str> for LayeredFormattedString<F, L> {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

impl<F: Copy> FormattedStringBuilder<F> {
    pub fn append(&mut self, string: &str, field: F) -> &mut FormattedStringBuilder<F> {
        // len() is always a char boundary
        self.insert_internal(self.bytes.len(), string, field)
    }

    pub fn prepend(&mut self, string: &str, field: F) -> &mut FormattedStringBuilder<F> {
        // 0 is always a char boundary
        self.insert_internal(0, string, field)
    }

    pub fn insert(
        &mut self,
        pos: usize,
        string: &str,
        field: F,
    ) -> Result<&mut FormattedStringBuilder<F>, FormattedStringBuilderError> {
        if pos > self.bytes.len() {
            Err(FormattedStringBuilderError::IndexOutOfBounds(pos))
        } else if !unsafe { from_utf8_unchecked(&self.bytes) }.is_char_boundary(pos) {
            Err(FormattedStringBuilderError::PositionNotCharBoundary(
                pos,
                unsafe { String::from_utf8_unchecked(self.bytes.clone()) },
            ))
        } else {
            Ok(self.insert_internal(pos, string, field))
        }
    }

    // Precondition here is that pos is a char boundary and < buffer_len.
    fn insert_internal(
        &mut self,
        pos: usize,
        string: &str,
        field: F,
    ) -> &mut FormattedStringBuilder<F> {
        self.bytes.splice(pos..pos, string.bytes());
        self.annotations
            .splice(pos..pos, raise_annotation(field, vec![[]; string.len()]));
        self
    }
}

impl<F: Copy> FormattedString<F> {
    pub fn field_at(&self, pos: usize) -> F {
        self.annotations[pos][0].1
    }
}

impl<F: Copy, const L: usize> Default for LayeredFormattedStringBuilder<F, L> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fmt::Debug, panic};

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Field {
        Word,
        Space,
        Greeting,
    }

    #[test]
    fn test_basic() {
        let mut builder = FormattedStringBuilder::<Field>::new();
        builder
            .append("world", Field::Word)
            .prepend(" ", Field::Space)
            .prepend("hello", Field::Word);
        let x = builder.build();

        assert_eq!(x.as_ref(), "hello world");

        for i in 0.."hello".len() {
            assert_eq!(x.field_at(i), Field::Word);
        }
        assert_eq!(x.field_at(5), Field::Space);
        for i in 0.."world".len() {
            assert_eq!(x.field_at(i), Field::Word);
        }
        assert_panics(|| x.field_at(11));
    }

    #[test]
    fn test_multi_level() {
        let mut builder = FormattedStringBuilder::<Field>::new();
        builder
            .append("world", Field::Word)
            .prepend(" ", Field::Space)
            .prepend("hello", Field::Word);
        let x = builder.build();

        let mut builder = LayeredFormattedStringBuilder::<Field, 2>::new();
        builder.append_layered(&x, Field::Greeting);
        let y = builder.build();

        assert_eq!(y.as_ref(), "hello world");
        assert_eq!(y.fields_at(0), [Field::Greeting, Field::Word]);
    }

    #[test]
    fn test_multi_byte() {
        let mut builder = FormattedStringBuilder::<Field>::new();
        builder.append("π", Field::Word);
        assert_eq!(
            builder.insert(1, "pi/2", Field::Word),
            Err(FormattedStringBuilderError::PositionNotCharBoundary(
                1,
                "π".to_owned()
            ))
        );
        let x = builder.build();

        assert_eq!(x.as_ref(), "π");
        assert_eq!(x.field_at(0), Field::Word);
        assert_eq!(x.field_at(1), Field::Word);
        assert_panics(|| x.field_at(2));
    }

    fn assert_panics<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        assert!(result.is_err());
    }
}
