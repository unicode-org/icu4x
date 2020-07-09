use crate::utils::deconstruct_range;
use crate::UnicodeSet;
use std::{
    convert::TryFrom,
    ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

fn try_from_range_impl(range: impl RangeBounds<char>) -> Result<UnicodeSet, (u32, u32)> {
    let (from, till) = deconstruct_range(range);
    if from < till {
        let set = vec![from, till];
        Ok(UnicodeSet::try_from(set).unwrap())
    } else {
        Err((from, till))
    }
}

impl TryFrom<Range<char>> for UnicodeSet {
    type Error = String;

    fn try_from(range: Range<char>) -> Result<Self, Self::Error> {
        match try_from_range_impl(range) {
            Ok(u) => Ok(u),
            Err((from, till)) => Err(format!("Range must be ascending: {} - {}", from, till)),
        }
    }
}

impl TryFrom<RangeFrom<char>> for UnicodeSet {
    type Error = String;

    fn try_from(range: RangeFrom<char>) -> Result<Self, Self::Error> {
        match try_from_range_impl(range) {
            Ok(u) => Ok(u),
            Err((from, till)) => Err(format!("Range must be ascending: {} - {}", from, till)),
        }
    }
}

impl TryFrom<RangeFull> for UnicodeSet {
    type Error = String;

    fn try_from(_: RangeFull) -> Result<Self, Self::Error> {
        Ok(UnicodeSet::all())
    }
}

impl TryFrom<RangeInclusive<char>> for UnicodeSet {
    type Error = String;

    fn try_from(range: RangeInclusive<char>) -> Result<Self, Self::Error> {
        match try_from_range_impl(range) {
            Ok(u) => Ok(u),
            Err((from, till)) => Err(format!("Range must be ascending: {} - {}", from, till)),
        }
    }
}

impl TryFrom<RangeTo<char>> for UnicodeSet {
    type Error = String;

    fn try_from(range: RangeTo<char>) -> Result<Self, Self::Error> {
        match try_from_range_impl(range) {
            Ok(u) => Ok(u),
            Err((from, till)) => Err(format!("Range must be ascending: {} - {}", from, till)),
        }
    }
}

impl TryFrom<RangeToInclusive<char>> for UnicodeSet {
    type Error = String;

    fn try_from(range: RangeToInclusive<char>) -> Result<Self, Self::Error> {
        Ok(try_from_range_impl(range).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::UnicodeSet;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_range() {
        assert!(UnicodeSet::try_from('A'..'B').is_ok());
    }
    #[test]
    fn test_try_from_range_error() {
        assert!(UnicodeSet::try_from('A'..'A').is_err());
    }
    #[test]
    fn test_try_from_range_inclusive() {
        assert!(UnicodeSet::try_from('A'..='A').is_ok());
    }
    #[test]
    fn test_try_from_range_inclusive_err() {
        assert!(UnicodeSet::try_from('B'..='A').is_err());
    }
    #[test]
    fn test_try_from_range_from() {
        assert!(UnicodeSet::try_from('A'..).is_ok());
    }
    #[test]
    fn test_try_from_range_from_err() {
        assert!(UnicodeSet::try_from((std::char::MAX)..).is_err());
    }
    #[test]
    fn test_try_from_range_to() {
        assert!(UnicodeSet::try_from(..'A').is_ok());
    }
    #[test]
    fn test_try_from_range_to_err() {
        assert!(UnicodeSet::try_from(..(0 as char)).is_err());
    }
    #[test]
    fn test_try_from_range_to_inclusive() {
        assert!(UnicodeSet::try_from(..='A').is_ok());
    }
    #[test]
    fn test_try_from_range_full() {
        assert!(UnicodeSet::try_from(..).is_ok());
    }
}
