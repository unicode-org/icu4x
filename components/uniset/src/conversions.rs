use super::USetError;
use crate::utils::deconstruct_range;
use crate::UnicodeSet;
use std::{
    convert::TryFrom,
    ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

fn try_from_range_impl(range: &impl RangeBounds<char>) -> Result<UnicodeSet, USetError> {
    let (from, till) = deconstruct_range(range);
    if from < till {
        let set = vec![from, till];
        Ok(UnicodeSet::try_from(set).unwrap())
    } else {
        Err(USetError::InvalidRange(from, till))
    }
}

impl TryFrom<&Range<char>> for UnicodeSet {
    type Error = USetError;

    fn try_from(range: &Range<char>) -> Result<Self, Self::Error> {
        try_from_range_impl(range)
    }
}

impl TryFrom<&RangeFrom<char>> for UnicodeSet {
    type Error = USetError;

    fn try_from(range: &RangeFrom<char>) -> Result<Self, Self::Error> {
        try_from_range_impl(range)
    }
}

impl TryFrom<&RangeFull> for UnicodeSet {
    type Error = USetError;

    fn try_from(_: &RangeFull) -> Result<Self, Self::Error> {
        Ok(UnicodeSet::all())
    }
}

impl TryFrom<&RangeInclusive<char>> for UnicodeSet {
    type Error = USetError;

    fn try_from(range: &RangeInclusive<char>) -> Result<Self, Self::Error> {
        try_from_range_impl(range)
    }
}

impl TryFrom<&RangeTo<char>> for UnicodeSet {
    type Error = USetError;

    fn try_from(range: &RangeTo<char>) -> Result<Self, Self::Error> {
        try_from_range_impl(range)
    }
}

impl TryFrom<&RangeToInclusive<char>> for UnicodeSet {
    type Error = USetError;

    fn try_from(range: &RangeToInclusive<char>) -> Result<Self, Self::Error> {
        try_from_range_impl(range)
    }
}

#[cfg(test)]
mod tests {
    use super::USetError;
    use crate::UnicodeSet;
    use std::convert::TryFrom;
    #[test]
    fn test_try_from_range() {
        let check: Vec<char> = UnicodeSet::try_from(&('A'..'B')).unwrap().iter().collect();
        assert_eq!(vec!['A'], check);
    }
    #[test]
    fn test_try_from_range_error() {
        let check = UnicodeSet::try_from(&('A'..'A'));
        assert_eq!(Err(USetError::InvalidRange(65, 65)), check);
    }
    #[test]
    fn test_try_from_range_inclusive() {
        let check: Vec<char> = UnicodeSet::try_from(&('A'..='A')).unwrap().iter().collect();
        assert_eq!(vec!['A'], check);
    }
    #[test]
    fn test_try_from_range_inclusive_err() {
        let check = UnicodeSet::try_from(&('B'..'A'));
        assert_eq!(Err(USetError::InvalidRange(66, 65)), check);
    }
    #[test]
    fn test_try_from_range_from() {
        let uset = UnicodeSet::try_from(&('A'..)).unwrap();
        let check: Vec<&u32> = uset.ranges().collect();
        assert_eq!(vec![&65, &((std::char::MAX as u32) + 1)], check);
    }
    #[test]
    fn test_try_from_range_to() {
        let uset = UnicodeSet::try_from(&(..'A')).unwrap();
        let check: Vec<&u32> = uset.ranges().collect();
        assert_eq!(vec![&0, &65], check);
    }
    #[test]
    fn test_try_from_range_to_err() {
        let check = UnicodeSet::try_from(&(..(0 as char)));
        assert_eq!(Err(USetError::InvalidRange(0, 0)), check);
    }
    #[test]
    fn test_try_from_range_to_inclusive() {
        let uset = UnicodeSet::try_from(&(..='A')).unwrap();
        let check: Vec<&u32> = uset.ranges().collect();
        assert_eq!(vec![&0, &66], check);
    }
    #[test]
    fn test_try_from_range_full() {
        let uset = UnicodeSet::try_from(&(..)).unwrap();
        let check: Vec<&u32> = uset.ranges().collect();
        assert_eq!(vec![&0, &((std::char::MAX as u32) + 1)], check);
    }
}
