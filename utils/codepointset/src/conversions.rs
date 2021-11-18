// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use core::{
    convert::TryFrom,
    ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use super::CodePointSetError;
use crate::utils::deconstruct_range;
use crate::CodePointSet;
use zerovec::ZeroVec;

fn try_from_range<'data, 'r>(
    range: &'r impl RangeBounds<char>,
) -> Result<CodePointSet<'data>, CodePointSetError> {
    let (from, till) = deconstruct_range(range);
    if from < till {
        let set = vec![from, till];
        let inv_list: ZeroVec<u32> = ZeroVec::clone_from_slice(&set);
        Ok(CodePointSet::from_inversion_list(inv_list).unwrap())
    } else {
        Err(CodePointSetError::InvalidRange(from, till))
    }
}

impl<'data> TryFrom<&Range<char>> for CodePointSet<'data> {
    type Error = CodePointSetError;

    fn try_from(range: &Range<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeFrom<char>> for CodePointSet<'data> {
    type Error = CodePointSetError;

    fn try_from(range: &RangeFrom<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeFull> for CodePointSet<'data> {
    type Error = CodePointSetError;

    fn try_from(_: &RangeFull) -> Result<Self, Self::Error> {
        Ok(Self::all())
    }
}

impl<'data> TryFrom<&RangeInclusive<char>> for CodePointSet<'data> {
    type Error = CodePointSetError;

    fn try_from(range: &RangeInclusive<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeTo<char>> for CodePointSet<'data> {
    type Error = CodePointSetError;

    fn try_from(range: &RangeTo<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

impl<'data> TryFrom<&RangeToInclusive<char>> for CodePointSet<'data> {
    type Error = CodePointSetError;

    fn try_from(range: &RangeToInclusive<char>) -> Result<Self, Self::Error> {
        try_from_range(range)
    }
}

#[cfg(test)]
mod tests {
    use super::CodePointSetError;
    use crate::CodePointSet;
    use core::{char, convert::TryFrom};

    #[test]
    fn test_try_from_range() {
        let check: Vec<char> = CodePointSet::try_from(&('A'..'B'))
            .unwrap()
            .iter_chars()
            .collect();
        assert_eq!(vec!['A'], check);
    }
    #[test]
    fn test_try_from_range_error() {
        let check = CodePointSet::try_from(&('A'..'A'));
        assert!(matches!(
            check,
            Err(CodePointSetError::InvalidRange(65, 65))
        ));
    }
    #[test]
    fn test_try_from_range_inclusive() {
        let check: Vec<char> = CodePointSet::try_from(&('A'..='A'))
            .unwrap()
            .iter_chars()
            .collect();
        assert_eq!(vec!['A'], check);
    }
    #[test]
    fn test_try_from_range_inclusive_err() {
        let check = CodePointSet::try_from(&('B'..'A'));
        assert!(matches!(
            check,
            Err(CodePointSetError::InvalidRange(66, 65))
        ));
    }
    #[test]
    fn test_try_from_range_from() {
        let uset = CodePointSet::try_from(&('A'..)).unwrap();
        let check: usize = uset.size();
        let expected: usize = (char::MAX as usize) + 1 - 65;
        assert_eq!(expected, check);
    }
    #[test]
    fn test_try_from_range_to() {
        let uset = CodePointSet::try_from(&(..'A')).unwrap();
        let check: usize = uset.size();
        let expected: usize = 65;
        assert_eq!(expected, check);
    }
    #[test]
    fn test_try_from_range_to_err() {
        let check = CodePointSet::try_from(&(..(0x0 as char)));
        assert!(matches!(check, Err(CodePointSetError::InvalidRange(0, 0))));
    }
    #[test]
    fn test_try_from_range_to_inclusive() {
        let uset = CodePointSet::try_from(&(..='A')).unwrap();
        let check: usize = uset.size();
        let expected: usize = 66;
        assert_eq!(expected, check);
    }
    #[test]
    fn test_try_from_range_full() {
        let uset = CodePointSet::try_from(&(..)).unwrap();
        let check: usize = uset.size();
        let expected: usize = (char::MAX as usize) + 1;
        assert_eq!(expected, check);
    }
}
