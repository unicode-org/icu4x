// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::iter::FromIterator;

use alloc::vec;
use alloc::vec::Vec;
use litemap::store::*;

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

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        match self {
            ShortVec::Empty => &mut [],
            ShortVec::Single(v) => core::slice::from_mut(v),
            ShortVec::Multi(v) => v.as_mut_slice(),
        }
    }

    #[inline]
    pub const fn single(&self) -> Option<&T> {
        match self {
            ShortVec::Single(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        match self {
            ShortVec::Empty => 0,
            ShortVec::Single(_) => 1,
            ShortVec::Multi(ref v) => v.len(),
        }
    }

    pub fn insert(&mut self, index: usize, elt: T) {
        assert!(
            index <= self.len(),
            "insertion index (is {}) should be <= len (is {})",
            index,
            self.len()
        );

        *self = match core::mem::replace(self, ShortVec::Empty) {
            ShortVec::Empty => ShortVec::Single(elt),
            ShortVec::Single(item) => {
                let items = if index == 0 {
                    vec![elt, item]
                } else {
                    vec![item, elt]
                };
                ShortVec::Multi(items)
            }
            ShortVec::Multi(mut items) => {
                items.insert(index, elt);
                ShortVec::Multi(items)
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(
            index < self.len(),
            "removal index (is {}) should be < len (is {})",
            index,
            self.len()
        );

        let (replaced, removed_item) = match core::mem::replace(self, ShortVec::Empty) {
            ShortVec::Empty => unreachable!(),
            ShortVec::Single(v) => (ShortVec::Empty, v),
            ShortVec::Multi(mut v) => {
                let removed_item = v.remove(index);
                match v.len() {
                    #[allow(clippy::unwrap_used)]
                    // we know that the vec has exactly one element left
                    1 => (ShortVec::Single(v.pop().unwrap()), removed_item),
                    // v has atleast 2 elements, create a Multi variant
                    _ => (ShortVec::Multi(v), removed_item),
                }
            }
        };
        *self = replaced;
        removed_item
    }

    #[inline]
    pub fn clear(&mut self) {
        let _ = core::mem::replace(self, ShortVec::Empty);
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

impl<T> Default for ShortVec<T> {
    fn default() -> Self {
        ShortVec::Empty
    }
}

impl<T> FromIterator<T> for ShortVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl<K, V> StoreConstEmpty<K, V> for ShortVec<(K, V)> {
    const EMPTY: ShortVec<(K, V)> = ShortVec::Empty;
}

impl<K, V> Store<K, V> for ShortVec<(K, V)> {
    #[inline]
    fn lm_len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn lm_is_empty(&self) -> bool {
        matches!(self, ShortVec::Empty)
    }

    #[inline]
    fn lm_get(&self, index: usize) -> Option<(&K, &V)> {
        self.as_slice().get(index).map(|elt| (&elt.0, &elt.1))
    }

    #[inline]
    fn lm_last(&self) -> Option<(&K, &V)> {
        match self {
            ShortVec::Empty => None,
            ShortVec::Single(v) => Some(v),
            ShortVec::Multi(v) => v.as_slice().last(),
        }
        .map(|elt| (&elt.0, &elt.1))
    }

    #[inline]
    fn lm_binary_search_by<F>(&self, mut cmp: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> core::cmp::Ordering,
    {
        self.as_slice().binary_search_by(|(k, _)| cmp(k))
    }
}

impl<K, V> StoreMut<K, V> for ShortVec<(K, V)> {
    fn lm_with_capacity(_capacity: usize) -> Self {
        ShortVec::Empty
    }

    // ShortVec supports reserving capacity for additional elements only if we have already allocated a vector
    fn lm_reserve(&mut self, additional: usize) {
        if let ShortVec::Multi(ref mut v) = self {
            v.reserve(additional)
        }
    }

    fn lm_get_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.as_mut_slice()
            .get_mut(index)
            .map(|elt| (&elt.0, &mut elt.1))
    }

    fn lm_push(&mut self, key: K, value: V) {
        self.push((key, value))
    }

    fn lm_insert(&mut self, index: usize, key: K, value: V) {
        self.insert(index, (key, value))
    }

    fn lm_remove(&mut self, index: usize) -> (K, V) {
        self.remove(index)
    }

    fn lm_clear(&mut self) {
        self.clear();
    }
}

impl<'a, K: 'a, V: 'a> StoreIterable<'a, K, V> for ShortVec<(K, V)> {
    type KeyValueIter =
        core::iter::Map<core::slice::Iter<'a, (K, V)>, for<'r> fn(&'r (K, V)) -> (&'r K, &'r V)>;

    fn lm_iter(&'a self) -> Self::KeyValueIter {
        self.as_slice().iter().map(|elt| (&elt.0, &elt.1))
    }
}

impl<K, V> StoreFromIterator<K, V> for ShortVec<(K, V)> {}

#[test]
fn test_shortvec_impl() {
    litemap::testing::check_litemap::<ShortVec<(u32, u64)>>();
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
