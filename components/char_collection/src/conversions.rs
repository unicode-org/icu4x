// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//! Conversion (`From`) implementations for [CharCollection], via [MultiCharRange].
use std::boxed::Box;
use std::convert::TryFrom;
use std::iter;
use std::ops::RangeInclusive;
// use unic_char_range::CharRange;
// use unic_ucd_block::Block;
// use unicode_blocks::UnicodeBlockId;
use crate::{CharRange, CharCollection, MultiCharRange};
macro_rules! impl_for_range_inclusive_int_type {
    ($($t:ty),*) => {$(
        impl MultiCharRange for RangeInclusive<$t> {
            fn iter_ranges(&self) -> Box<dyn Iterator<Item=CharRange>> {
                Box::new(iter::once(to_char_range!(self)))
            }
            fn range_count(&self) -> usize {
                1
            }
    })*}
}
// This macro is needed because there is no way to express "can be cast as u32" using traits.
macro_rules! to_char_range {
    ($range:expr) => {
        CharRange::closed(
            char::try_from(*$range.start() as u32).unwrap(),
            char::try_from(*$range.end() as u32).unwrap(),
        )
    };
}
impl MultiCharRange for char {
    fn iter_ranges(&self) -> Box<dyn Iterator<Item = CharRange>> {
        Box::new(std::iter::once(CharRange::closed(*self, *self)))
    }
    fn range_count(&self) -> usize {
        1
    }
}
impl MultiCharRange for CharRange {
    fn iter_ranges(&self) -> Box<dyn Iterator<Item = CharRange>> {
        Box::new(iter::once(self.clone()))
    }
    fn range_count(&self) -> usize {
        1
    }
}
impl MultiCharRange for RangeInclusive<char> {
    fn iter_ranges(&self) -> Box<dyn Iterator<Item = CharRange>> {
        Box::new(iter::once(CharRange::closed(*self.start(), *self.end())))
    }
    fn range_count(&self) -> usize {
        1
    }
}
impl_for_range_inclusive_int_type!(u8, i8, u32, i32);
// impl MultiCharRange for UnicodeBlockId {
//     fn iter_ranges(&self) -> Box<dyn Iterator<Item = CharRange>> {
//         self.block().iter_ranges()
//     }
//     fn range_count(&self) -> usize {
//         1
//     }
// }
// impl MultiCharRange for Block {
//     fn iter_ranges<'a>(&'a self) -> Box<dyn Iterator<Item = CharRange> + 'a> {
//         Box::new(self.range.iter_ranges())
//     }
//     fn range_count(&self) -> usize {
//         1
//     }
// }
impl<T: MultiCharRange> From<&T> for CharCollection {
    fn from(source: &T) -> Self {
        let mut collection = CharCollection::new();
        collection.insert(source);
        collection
    }
}
#[cfg(test)]
mod multi_char_range_tests {
    use crate::{MultiCharRange, CharRange};
    use paste;
    // use unic_char_range::{chars, CharRange};
    #[test]
    fn test_char() {
        let source = 'a';
        assert_eq!(source.iter_ranges().collect::<Vec<CharRange>>(), vec![chars!('a'..='a')]);
        assert_eq!(source.range_count(), 1);
    }
    #[test]
    fn test_char_range() {
        let source = chars!('d'..='g');
        assert_eq!(source.iter_ranges().collect::<Vec<CharRange>>(), vec![chars!('d'..='g')]);
        assert_eq!(source.range_count(), 1);
    }
    #[test]
    fn test_range_inclusive_char() {
        let source = 'd'..='g';
        assert_eq!(source.iter_ranges().collect::<Vec<CharRange>>(), vec![chars!('d'..='g')]);
        assert_eq!(source.range_count(), 1);
    }
    macro_rules! test_range_inclusive_int {
        ($t:ty) => {
            paste::item! {
                #[test]
                fn [<test_char_range_inclusive_ $t>]() {
                    let source: std::ops::RangeInclusive<$t> = 0x0..=0x9;
                        assert_eq!(
                            source.iter_ranges().collect::<Vec<CharRange>>(),
                            vec![chars!('\u{0}'..='\u{9}')]
                    );
                    assert_eq!(source.range_count(), 1);
                }
            }
        };
    }
    test_range_inclusive_int!(u8);
    test_range_inclusive_int!(i8);
    test_range_inclusive_int!(u32);
    test_range_inclusive_int!(i32);
    // #[test]
    // fn test_unicode_block_id() {
    //     let source = unicode_blocks::UnicodeBlockId::BasicLatin;
    //     assert_eq!(
    //         source.iter_ranges().collect::<Vec<CharRange>>(),
    //         vec![chars!('\u{0000}'..='\u{007f}')]
    //     );
    //     assert_eq!(source.range_count(), 1);
    // }
    // #[test]
    // fn test_unicode_block() {
    //     let source = unicode_blocks::UnicodeBlockId::BasicLatin.block();
    //     assert_eq!(
    //         source.iter_ranges().collect::<Vec<CharRange>>(),
    //         vec![chars!('\u{0000}'..='\u{007f}')]
    //     );
    //     assert_eq!(source.range_count(), 1);
    // }
}
#[cfg(test)]
mod from_tests {
    use crate::CharCollection;
    // use unicode_blocks::UnicodeBlockId;
    #[test]
    fn test_char() {
        let actual: CharCollection = (&'a').into();
        assert_eq!(actual, char_collect!('a'..='a'));
    }
    // #[test]
    // fn test_unicode_block_id() {
    //     let actual: CharCollection = (&UnicodeBlockId::BasicLatin).into();
    //     assert_eq!(actual, char_collect!('\u{0000}'..='\u{007f}'));
    // }
}