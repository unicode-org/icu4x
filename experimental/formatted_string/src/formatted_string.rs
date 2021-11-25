// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedStringBuilderError;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::str;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LocationInPart {
    Begin,
    Extend,
}

pub trait FormattedString<F: Copy, const L: usize>: AsRef<str> {
    fn fields_at(&self, pos: usize) -> [F; L] {
        self.annotation_at(pos).map(|(_, field)| field)
    }

    fn is_field_start(&self, pos: usize, level: usize) -> bool {
        assert!(level < L);
        let (location, _) = self.annotation_at(pos)[level];
        location == LocationInPart::Begin
    }

    #[doc(hidden)]
    // This is used to make the builder more efficient; clients should
    // use fields_at and is_field_start.
    fn annotation_at(&self, pos: usize) -> &[(LocationInPart, F); L];
}

impl<F: Copy> FormattedString<F, 0> for &str {
    fn annotation_at(&self, _pos: usize) -> &[(LocationInPart, F); 0] {
        // Yay we can return dangling references for singleton types!
        &[]
    }
}

/// A string with L levels of formatting annotations. The name refers
/// to the fact that the trait FormattedString is also implemented for
/// &str, but with 0 levels of annotations there isn't much formatting.
#[derive(Debug, PartialEq)]
pub struct ActualFormattedString<F: Copy, const L: usize> {
    string: String,
    // The slice dimension corresponds to the bytes in string,
    // the array dimension are the L levels of annotations
    annotations: Box<[[(LocationInPart, F); L]]>,
}

impl<F: Copy, const L: usize> AsRef<str> for ActualFormattedString<F, L> {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

impl<F: Copy, const L: usize> FormattedString<F, L> for ActualFormattedString<F, L> {
    fn annotation_at(&self, pos: usize) -> &[(LocationInPart, F); L] {
        &self.annotations[pos]
    }
}

/// A builder with L levels of type annotations.
#[derive(Debug)]
pub struct FormattedStringBuilder<F: Copy, const L: usize> {
    // bytes is always valid UTF-8, so from_utf8_unchecked is safe
    bytes: Vec<u8>,
    // The first dimension corresponds to the bytes, the second are the L levels of annotations
    annotations: Vec<[(LocationInPart, F); L]>,
}

impl<F: Copy, const L: usize> FormattedStringBuilder<F, L> {
    pub fn new() -> Self {
        // A builder with 0 annotations doesn't make sense.
        assert!(L > 0);
        Self {
            bytes: Vec::with_capacity(40),
            annotations: Vec::with_capacity(40),
        }
    }

    pub fn append<S, const L1: usize>(&mut self, string: &S, field: F) -> &mut Self
    where
        S: FormattedString<F, L1>,
    {
        assert_eq!(L - 1, L1);
        // len() is always a char boundary
        self.insert_internal(self.bytes.len(), string, field)
    }

    pub fn prepend<S, const L1: usize>(&mut self, string: &S, field: F) -> &mut Self
    where
        S: FormattedString<F, L1>,
    {
        assert_eq!(L - 1, L1);
        // 0 is always a char boundary
        self.insert_internal(0, string, field)
    }

    pub fn insert<S, const L1: usize>(
        &mut self,
        pos: usize,
        string: &S,
        field: F,
    ) -> Result<&mut Self, FormattedStringBuilderError>
    where
        S: FormattedString<F, L1>,
    {
        assert_eq!(L - 1, L1);
        if pos > self.bytes.len() {
            return Err(FormattedStringBuilderError::IndexOutOfBounds(pos));
        }
        // bytes is valid UTF-8 precisely because we do this check before
        // insertion (and string is valid UTF-8)
        let current = unsafe { str::from_utf8_unchecked(&self.bytes) };
        if !current.is_char_boundary(pos) {
            Err(FormattedStringBuilderError::PositionNotCharBoundary(
                pos,
                current.to_owned(),
            ))
        } else {
            Ok(self.insert_internal(pos, string, field))
        }
    }

    // Precondition here is that pos is a char boundary and < buffer_len.
    fn insert_internal<S, const L1: usize>(&mut self, pos: usize, string: &S, field: F) -> &mut Self
    where
        S: FormattedString<F, L1>,
    {
        assert_eq!(L - 1, L1);
        self.bytes.splice(pos..pos, string.as_ref().bytes());
        self.annotations.splice(
            pos..pos,
            (0..string.as_ref().len()).map(|i| {
                let top_level = (
                    if i == 0 {
                        LocationInPart::Begin
                    } else {
                        LocationInPart::Extend
                    },
                    field,
                );
                let mut all_levels = [top_level; L];
                all_levels[1..L].copy_from_slice(string.annotation_at(i));
                all_levels
            }),
        );
        self
    }

    pub fn build(self) -> ActualFormattedString<F, L> {
        ActualFormattedString {
            // bytes is valid UTF-8
            string: unsafe { String::from_utf8_unchecked(self.bytes) },
            annotations: self.annotations.into_boxed_slice(),
        }
    }
}

impl<F: Copy> ActualFormattedString<F, 1> {
    pub fn field_at(&self, pos: usize) -> F {
        self.fields_at(pos)[0]
    }
}

impl<F: Copy, const L: usize> Default for FormattedStringBuilder<F, L> {
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
        let mut builder = FormattedStringBuilder::<Field, 1>::new();
        builder
            .append(&"world", Field::Word)
            .prepend(&" ", Field::Space)
            .prepend(&"hello", Field::Word);
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
        let mut builder = FormattedStringBuilder::<Field, 1>::new();
        builder
            .append(&"world", Field::Word)
            .prepend(&" ", Field::Space)
            .prepend(&"hello", Field::Word);
        let x = builder.build();

        let mut builder = FormattedStringBuilder::<Field, 2>::new();
        builder.append(&x, Field::Greeting);
        let y = builder.build();

        assert_eq!(y.as_ref(), "hello world");
        assert_eq!(y.fields_at(0), [Field::Greeting, Field::Word]);
    }

    #[test]
    fn test_multi_byte() {
        let mut builder = FormattedStringBuilder::<Field, 1>::new();
        builder.append(&"π", Field::Word);
        assert_eq!(
            builder
                .insert(1, &"pi/2", Field::Word)
                .unwrap_err()
                .to_string(),
            "attempted to insert at an index that is not a character boundary: 1 in \"π\""
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
