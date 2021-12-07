// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::FormattedStringError;
use alloc::borrow::ToOwned;
use alloc::collections::vec_deque::{self, VecDeque};
use core::cell::RefCell;
use core::iter::Copied;
use core::ops::Range;
use core::str::{self, Bytes};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LocationInPart {
    Begin,
    Extend,
}

/// A string with L levels of annotations of type F. For N = 0, this is
/// implemented for `&str`, for higher N see LayeredFormattedString.
pub trait FormattedStringLike<'a, F: Copy, const L: usize> {
    fn fields_at(&self, pos: usize) -> [F; L] {
        self._annotation_at(pos).map(|(_, field)| field)
    }

    fn is_field_start(&self, pos: usize, level: usize) -> bool {
        debug_assert!(level < L);
        let (location, _) = self._annotation_at(pos)[level];
        location == LocationInPart::Begin
    }

    // These members are not part of the public API.
    #[doc(hidden)]
    fn _annotation_at(&self, pos: usize) -> &[(LocationInPart, F); L];
    #[doc(hidden)]
    type BytesIter: DoubleEndedIterator<Item = u8> + ExactSizeIterator + 'a;
    #[doc(hidden)]
    fn _bytes(&'a self) -> Self::BytesIter;
    #[doc(hidden)]
    fn _len(&self) -> usize;
}

impl<F: Copy, const L: usize> AsRef<str> for LayeredFormattedString<F, L> {
    fn as_ref(&self) -> &str {
        self.bytes.borrow_mut().make_contiguous();
        // Any mutates to bytes other than make_continguous happen in mut
        // contexts. As make_contiguous is idempotent, this is safe.
        unsafe { str::from_utf8_unchecked((*self.bytes.as_ptr()).as_slices().0) }
    }
}

impl<'a, F: Copy> FormattedStringLike<'a, F, 0> for &'a str {
    fn _annotation_at(&self, _pos: usize) -> &[(LocationInPart, F); 0] {
        // Yay we can return dangling references for singleton types!
        &[]
    }

    type BytesIter = Bytes<'a>;
    fn _bytes(&'a self) -> Self::BytesIter {
        self.bytes()
    }

    fn _len(&self) -> usize {
        self.len()
    }
}

/// A string with L levels of formatting annotations.
#[derive(Debug, PartialEq)]
pub struct LayeredFormattedString<F: Copy, const L: usize> {
    // bytes is always valid UTF-8, so from_utf8_unchecked is safe
    bytes: RefCell<VecDeque<u8>>,
    // The vector dimension corresponds to the bytes, the array dimension are the L levels of annotations
    annotations: VecDeque<[(LocationInPart, F); L]>,
}

pub type FormattedString<F> = LayeredFormattedString<F, 1>;

impl<'a, F: Copy, const L: usize> FormattedStringLike<'a, F, L> for LayeredFormattedString<F, L> {
    fn _annotation_at(&self, pos: usize) -> &[(LocationInPart, F); L] {
        &self.annotations[pos]
    }

    type BytesIter = Copied<vec_deque::Iter<'a, u8>>;
    fn _bytes(&'a self) -> Self::BytesIter {
        // We cannot return anything borrowed from bytes. However,
        // as this result is immediately used by the single call
        // site, this deref is safe.
        unsafe { (*self.bytes.as_ptr()).iter().copied() }
    }

    fn _len(&self) -> usize {
        self.annotations.len()
    }
}

impl<F: Copy, const L: usize> LayeredFormattedString<F, L> {
    pub fn new() -> Self {
        debug_assert!(L > 0);
        Self {
            bytes: RefCell::new(VecDeque::new()),
            annotations: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        // A LayeredFormattedString with 0 annotations doesn't make sense.
        debug_assert!(L > 0);
        Self {
            bytes: RefCell::new(VecDeque::with_capacity(capacity)),
            annotations: VecDeque::with_capacity(capacity),
        }
    }

    pub fn capacity(&self) -> usize {
        debug_assert_eq!(self.bytes.borrow().capacity(), self.annotations.capacity());
        self.annotations.capacity()
    }

    pub fn len(&self) -> usize {
        debug_assert_eq!(self.bytes.borrow().len(), self.annotations.len());
        self.annotations.len()
    }

    pub fn append<'a, S, const L1: usize>(&mut self, string: &'a S, field: F) -> &mut Self
    where
        S: FormattedStringLike<'a, F, L1>,
    {
        debug_assert_eq!(L - 1, L1);
        // len() is always a char boundary
        self.insert_internal(self.annotations.len(), string, field)
    }

    pub fn prepend<'a, S, const L1: usize>(&mut self, string: &'a S, field: F) -> &mut Self
    where
        S: FormattedStringLike<'a, F, L1>,
    {
        debug_assert_eq!(L - 1, L1);
        // 0 is always a char boundary
        self.insert_internal(0, string, field)
    }

    pub fn insert<'a, S, const L1: usize>(
        &mut self,
        pos: usize,
        string: &'a S,
        field: F,
    ) -> Result<&mut Self, FormattedStringError>
    where
        S: FormattedStringLike<'a, F, L1>,
    {
        debug_assert_eq!(L - 1, L1);
        if pos > self.annotations.len() {
            return Err(FormattedStringError::IndexOutOfBounds(pos));
        }
        // This is bit magic equivalent to: b >= 128 && b < 192, i.e. b is
        // not a UTF-8 character boundary. Lifted from str::is_char_boundary
        if pos < self.annotations.len() && (self.bytes.borrow()[pos] as i8) < -0x40 {
            Err(FormattedStringError::PositionNotCharBoundary(
                pos,
                self.as_ref().to_owned(),
            ))
        } else {
            Ok(self.insert_internal(pos, string, field))
        }
    }

    // Precondition here is that pos is a char boundary and < bytes.len().
    fn insert_internal<'a, S, const L1: usize>(
        &mut self,
        pos: usize,
        string: &'a S,
        field: F,
    ) -> &mut Self
    where
        S: FormattedStringLike<'a, F, L1>,
    {
        debug_assert_eq!(L - 1, L1);
        self.bytes.borrow_mut().splice(pos..pos, string._bytes());
        self.annotations.splice(
            pos..pos,
            (0..string._len()).map(|i| {
                let top_level = (
                    if i == 0 {
                        LocationInPart::Begin
                    } else {
                        LocationInPart::Extend
                    },
                    field,
                );
                let mut all_levels = [top_level; L];
                all_levels[1..L].copy_from_slice(string._annotation_at(i));
                all_levels
            }),
        );
        self
    }

    pub fn field_at(&self, pos: usize) -> F {
        self.fields_at(pos)[0]
    }
}

impl<F: Copy, const L: usize> Default for LayeredFormattedString<F, L> {
    fn default() -> Self {
        Self::new()
    }
}

// Very basic implementation of VecDeque::splice.
// https://github.com/rust-lang/rust/issues/69939
trait Splice<T> {
    fn splice<I>(&mut self, range: Range<usize>, values: I)
    where
        I: DoubleEndedIterator<Item = T> + ExactSizeIterator;
}

impl<T> Splice<T> for VecDeque<T> {
    fn splice<I>(&mut self, range: Range<usize>, values: I)
    where
        I: DoubleEndedIterator<Item = T> + ExactSizeIterator,
    {
        debug_assert_eq!(range.start, range.end);

        self.reserve(values.len());

        if range.start == self.len() {
            self.extend(values)
        } else if range.start == 0 {
            for value in values.rev() {
                self.push_front(value)
            }
        } else {
            for (i, v) in values.enumerate() {
                self.insert(range.start + i, v)
            }
        }
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
        let mut x = FormattedString::<Field>::new();
        x.append(&"world", Field::Word)
            .prepend(&" ", Field::Space)
            .prepend(&"hello", Field::Word);

        assert_eq!(x.as_ref(), "hello world");

        for i in 0.."hello".len() {
            assert_eq!(x.field_at(i), Field::Word);
        }
        assert_eq!(x.field_at(5), Field::Space);
        for i in 0.."world".len() {
            assert_eq!(x.field_at(6 + i), Field::Word);
        }
        assert_panics(|| {
            let mut x = FormattedString::<Field>::new();
            x.append(&"world", Field::Word)
                .prepend(&" ", Field::Space)
                .prepend(&"hello", Field::Word);
            x.field_at(11);
        });
    }

    #[test]
    fn test_multi_level() {
        let mut x = FormattedString::<Field>::new();
        x.append(&"world", Field::Word)
            .prepend(&" ", Field::Space)
            .prepend(&"hello", Field::Word);

        let mut y = LayeredFormattedString::<Field, 2>::new();
        y.append(&x, Field::Greeting);

        assert_eq!(y.as_ref(), "hello world");
        assert_eq!(y.fields_at(0), [Field::Greeting, Field::Word]);
    }

    #[test]
    fn test_multi_byte() {
        let mut x = FormattedString::<Field>::new();
        x.append(&"π", Field::Word);
        assert_eq!(
            x.insert(1, &"pi/2", Field::Word).unwrap_err().to_string(),
            "index 1 is not a character boundary in \"π\"",
        );

        assert_eq!(x.as_ref(), "π");
        assert_eq!(x.field_at(0), Field::Word);
        assert_eq!(x.field_at(1), Field::Word);
        assert_panics(|| {
            let mut x = FormattedString::<Field>::new();
            x.append(&"π", Field::Word);
            x.field_at(2);
        });
    }

    #[test]
    fn test_level_assert() {
        // The correct-depth asserts are (debug) runtime errors as long
        // as const generics aren't const-evaluatable:
        // https://github.com/rust-lang/rust/issues/76560
        assert_panics(|| {
            let mut x = LayeredFormattedString::<Field, 2>::new();
            x.append(&"foo", Field::Word);
        });
    }

    fn assert_panics<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = panic::catch_unwind(f);
        panic::set_hook(prev_hook);
        assert!(result.is_err());
    }
}
