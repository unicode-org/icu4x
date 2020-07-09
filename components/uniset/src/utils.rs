use std::{
    char::MAX,
    ops::{Bound::*, RangeBounds},
};

/// Returns whether the vector is sorted ascending non inclusive
pub fn is_sorted(v: &[u32]) -> bool {
    v.chunks(2).all(|chunk| chunk[0] < chunk[1])
}

/// Returns start (inclusive) and end (exclusive) bounds of RangeBounds
pub fn deconstruct_range(range: impl RangeBounds<char>) -> (u32, u32) {
    let from = match range.start_bound() {
        Included(b) => (*b as u32),
        Excluded(b) => (*b as u32),
        Unbounded => 0,
    };
    let till = match range.end_bound() {
        Included(b) => (*b as u32) + 1,
        Excluded(b) => (*b as u32),
        Unbounded => MAX as u32,
    };
    (from, till)
}

#[cfg(test)]
mod tests {
    use super::{deconstruct_range, is_sorted};
    use std::char::MAX;

    #[test]
    fn test_is_sorted() {
        let check = vec![2, 3, 4, 5];
        assert!(is_sorted(&check));
    }
    #[test]
    fn test_is_sorted_out_of_order() {
        let check = vec![5, 4, 5, 6, 7];
        assert!(!is_sorted(&check));
    }
    #[test]
    fn test_is_sorted_duplicate() {
        let check = vec![1, 2, 3, 3, 5];
        assert!(!is_sorted(&check));
    }

    // deconstruct_range
    #[test]
    fn test_deconstruct_range() {
        let expected = (65, 69);
        let check = deconstruct_range('A'..'E'); // Range
        assert_eq!(check, expected);
        let check = deconstruct_range('A'..='D'); // Range Inclusive
        assert_eq!(check, expected);
        let check = deconstruct_range('A'..); // Range From
        assert_eq!(check, (65, MAX as u32));
        let check = deconstruct_range(..'A'); // Range To
        assert_eq!(check, (0, 65));
        let check = deconstruct_range(..='A'); // Range To Inclusive
        assert_eq!(check, (0, 66));
        let check = deconstruct_range(..); // Range Full
        assert_eq!(check, (0, MAX as u32));
    }
}
