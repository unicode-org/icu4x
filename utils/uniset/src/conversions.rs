// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use core::{
    convert::TryFrom,
    iter::FromIterator,
    ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use super::UnicodeSetError;
use crate::utils::deconstruct_range;
use crate::{UnicodeSet, UnicodeSetBuilder};
use zerovec::ZeroVec;

fn try_from_range<'data, 'r>(
    range: &'r impl RangeBounds<char>,
) -> Result<UnicodeSet<'data>, UnicodeSetError> {
    let (from, till) = deconstruct_range(range);
    if from < till {
        let set = vec![from, till];
        let inv_list: ZeroVec<u32> = ZeroVec::alloc_from_slice(&set);
        #[allow(clippy::unwrap_used)] // TODO(#1688) Clippy exceptions need docs or fixing.
        Ok(UnicodeSet::from_inversion_list(inv_list).unwrap())
    } else {
        Err(UnicodeSetError::InvalidRange(from, till))
    }
}

impl<'data> TryFrom<&Range<char>> for UnicodeSet<'data> {
    type Error = UnicodeSetError;

    fn try_from(range: &Range<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeFrom<char>> for UnicodeSet<'data> {
    type Error = UnicodeSetError;

    fn try_from(range: &RangeFrom<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeFull> for UnicodeSet<'data> {
    type Error = UnicodeSetError;

    fn try_from(_: &RangeFull) -> Result<Self, Self::Error> {
        Ok(Self::all())
    }
}

impl<'data> TryFrom<&RangeInclusive<char>> for UnicodeSet<'data> {
    type Error = UnicodeSetError;

    fn try_from(range: &RangeInclusive<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeTo<char>> for UnicodeSet<'data> {
    type Error = UnicodeSetError;

    fn try_from(range: &RangeTo<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeToInclusive<char>> for UnicodeSet<'data> {
    type Error = UnicodeSetError;

    fn try_from(range: &RangeToInclusive<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl FromIterator<RangeInclusive<u32>> for UnicodeSet<'_> {
    fn from_iter<I: IntoIterator<Item = RangeInclusive<u32>>>(iter: I) -> Self {
        let mut builder = UnicodeSetBuilder::new();
        for range in iter {
            builder.add_range_u32(&range);
        }
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnicodeSet;
    use core::{char, convert::TryFrom};

    #[test]
    fn test_try_from_range() {
        let check: Vec<char> = UnicodeSet::try_from(&('A'..'B'))
            .unwrap()
            .iter_chars()
            .collect();
        assert_eq!(vec!['A'], check);
    }

    #[test]
    fn test_try_from_range_error() {
        let check = UnicodeSet::try_from(&('A'..'A'));
        assert!(matches!(check, Err(UnicodeSetError::InvalidRange(65, 65))));
    }

    #[test]
    fn test_try_from_range_inclusive() {
        let check: Vec<char> = UnicodeSet::try_from(&('A'..='A'))
            .unwrap()
            .iter_chars()
            .collect();
        assert_eq!(vec!['A'], check);
    }

    #[test]
    fn test_try_from_range_inclusive_err() {
        let check = UnicodeSet::try_from(&('B'..'A'));
        assert!(matches!(check, Err(UnicodeSetError::InvalidRange(66, 65))));
    }

    #[test]
    fn test_try_from_range_from() {
        let uset = UnicodeSet::try_from(&('A'..)).unwrap();
        let check: usize = uset.size();
        let expected: usize = (char::MAX as usize) + 1 - 65;
        assert_eq!(expected, check);
    }

    #[test]
    fn test_try_from_range_to() {
        let uset = UnicodeSet::try_from(&(..'A')).unwrap();
        let check: usize = uset.size();
        let expected: usize = 65;
        assert_eq!(expected, check);
    }

    #[test]
    fn test_try_from_range_to_err() {
        let check = UnicodeSet::try_from(&(..(0x0 as char)));
        assert!(matches!(check, Err(UnicodeSetError::InvalidRange(0, 0))));
    }

    #[test]
    fn test_try_from_range_to_inclusive() {
        let uset = UnicodeSet::try_from(&(..='A')).unwrap();
        let check: usize = uset.size();
        let expected: usize = 66;
        assert_eq!(expected, check);
    }

    #[test]
    fn test_try_from_range_full() {
        let uset = UnicodeSet::try_from(&(..)).unwrap();
        let check: usize = uset.size();
        let expected: usize = (char::MAX as usize) + 1;
        assert_eq!(expected, check);
    }

    #[test]
    fn test_from_range_iterator() {
        let ranges: Vec<RangeInclusive<u32>> = vec![
            RangeInclusive::new(0, 0x3FFF),
            RangeInclusive::new(0x4000, 0x7FFF),
            RangeInclusive::new(0x8000, 0xBFFF),
            RangeInclusive::new(0xC000, 0xFFFF),
        ];
        let ranges_iter = ranges.into_iter();
        let expected = UnicodeSet::from_inversion_list_slice(&[0x0, 0x1_0000]).unwrap();
        let actual = UnicodeSet::from_iter(ranges_iter);
        assert_eq!(expected, actual);
    }
}
