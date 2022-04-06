// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use alloc::vec::Vec;

/// Internal: A vector that supports no-allocation, constant values if length 0 or 1.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum ShortVec<T> {
    Empty,
    Single(T),
    Multi(Vec<T>),
}

impl<T> ShortVec<T> {
    #[inline]
    pub const fn new() -> Self {
        Self::Empty
    }

    #[inline]
    pub const fn new_single(item: T) -> Self {
        Self::Single(item)
    }

    pub fn push(&mut self, item: T) {
        *self = match core::mem::replace(self, Self::Empty) {
            ShortVec::Empty => ShortVec::Single(item),
            ShortVec::Single(prev_item) => ShortVec::Multi(vec![prev_item, item]),
            ShortVec::Multi(mut items) => {
                items.push(item);
                ShortVec::Multi(items)
            }
        };
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        match self {
            ShortVec::Empty => &[],
            ShortVec::Single(v) => core::slice::from_ref(v),
            ShortVec::Multi(v) => v.as_slice(),
        }
    }
}

impl<T> From<Vec<T>> for ShortVec<T> {
    fn from(v: Vec<T>) -> Self {
        match v.len() {
            0 => ShortVec::Empty,
            #[allow(clippy::unwrap_used)] // we know that the vec is not empty
            1 => ShortVec::Single(v.into_iter().next().unwrap()),
            _ => ShortVec::Multi(v),
        }
    }
}

macro_rules! impl_writeable_for_single_subtag {
    ($type:tt, $sample:literal) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl writeable::Writeable for $type {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                sink.write_str(self.as_str())
            }
            #[inline]
            fn write_len(&self) -> writeable::LengthHint {
                writeable::LengthHint::exact(self.0.len())
            }
        }

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(&$type::from_str($sample).unwrap(), $sample);
        }
    };
}

macro_rules! impl_writeable_for_each_subtag_str_no_test {
    ($type:tt) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                writeable::Writeable::write_to(self, f)
            }
        }

        impl writeable::Writeable for $type {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                let mut initial = true;
                self.for_each_subtag_str(&mut |subtag| {
                    if initial {
                        initial = false;
                    } else {
                        sink.write_char('-')?;
                    }
                    sink.write_str(subtag)
                })
            }

            #[inline]
            fn write_len(&self) -> writeable::LengthHint {
                let mut result = writeable::LengthHint::exact(0);
                let mut initial = true;
                self.for_each_subtag_str::<core::convert::Infallible, _>(&mut |subtag| {
                    if initial {
                        initial = false;
                    } else {
                        result += 1;
                    }
                    result += subtag.len();
                    Ok(())
                })
                .expect("infallible");
                result
            }
        }
    };
}

macro_rules! impl_writeable_for_subtag_list {
    ($type:tt, $sample1:literal, $sample2:literal) => {
        impl_writeable_for_each_subtag_str_no_test!($type);

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(&$type::default(), "");
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(alloc::vec![$sample1.parse().unwrap()]),
                $sample1,
            );
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(alloc::vec![
                    $sample1.parse().unwrap(),
                    $sample2.parse().unwrap()
                ]),
                core::concat!($sample1, "-", $sample2),
            );
        }
    };
}

macro_rules! impl_writeable_for_tinystr_list {
    ($type:tt, $if_empty:literal, $sample1:literal, $sample2:literal) => {
        impl_writeable_for_each_subtag_str_no_test!($type);

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(vec![$sample1.parse().unwrap()]),
                $sample1,
            );
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(vec![
                    $sample1.parse().unwrap(),
                    $sample2.parse().unwrap()
                ]),
                core::concat!($sample1, "-", $sample2),
            );
        }
    };
}

macro_rules! impl_writeable_for_key_value {
    ($type:tt, $key1:literal, $value1:literal, $key2:literal, $expected2:literal) => {
        impl_writeable_for_each_subtag_str_no_test!($type);

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(&$type::default(), "");
            writeable::assert_writeable_eq!(
                &$type::from_tuple_vec(vec![($key1.parse().unwrap(), $value1.parse().unwrap())]),
                core::concat!($key1, "-", $value1),
            );
            writeable::assert_writeable_eq!(
                &$type::from_tuple_vec(vec![
                    ($key1.parse().unwrap(), $value1.parse().unwrap()),
                    ($key2.parse().unwrap(), "true".parse().unwrap())
                ]),
                core::concat!($key1, "-", $value1, "-", $expected2),
            );
        }
    };
}
