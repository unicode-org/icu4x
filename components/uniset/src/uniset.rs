use std::{char, ops::RangeBounds};

use super::UnicodeSetError;
use crate::utils::{deconstruct_range, is_valid};
/// Represents the end code point of the Basic Multilingual Plane range, starting from code point 0, inclusive
const BMP_MAX: u32 = 0xFFFF;

/// UnicodeSet membership wrapper
///
/// Provides exposure to membership functions and constructors from serialized UnicodeSets
/// and predefined ranges.
#[derive(Debug, PartialEq)]
pub struct UnicodeSet {
    // If we wanted to use an array to keep the memory on the stack, there is an unsafe nightly feature
    // https://doc.rust-lang.org/nightly/core/array/trait.FixedSizeArray.html
    // Allows for traits of fixed size arrays

    // Implements an [inversion list.](https://en.wikipedia.org/wiki/Inversion_list)
    inv_list: Vec<u32>,
    size: usize,
}

impl UnicodeSet {
    /// Returns UnicodeSet from an [inversion list.](https://en.wikipedia.org/wiki/Inversion_list)
    /// represented by a `Vec<u32>` of codepoints.
    ///
    /// The inversion list must be of even length, sorted ascending non-overlapping,
    /// and within the bounds of `0x0 -> 0x10FFFF` inclusive, and end points being exclusive.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// use icu_unicodeset::UnicodeSetError;
    /// let empty: Vec<u32> = vec![];
    /// assert_eq!(UnicodeSet::from_inversion_list(empty.clone()), Err(UnicodeSetError::InvalidSet(empty.clone())))
    /// ```
    pub fn from_inversion_list(inv_list: Vec<u32>) -> Result<UnicodeSet, UnicodeSetError> {
        if is_valid(&inv_list) {
            let size: usize = inv_list
                .chunks(2)
                .map(|end_points| end_points[1] - end_points[0])
                .sum::<u32>() as usize;
            Ok(UnicodeSet { inv_list, size })
        } else {
            Err(UnicodeSetError::InvalidSet(inv_list))
        }
    }

    /// Returns UnicodeSet spanning entire Unicode range
    ///
    /// The range spans from `0x0 -> 0x10FFFF` inclusive
    pub fn all() -> UnicodeSet {
        UnicodeSet {
            inv_list: vec![0, (char::MAX as u32) + 1],
            size: (char::MAX as usize) + 1,
        }
    }

    /// Returns UnicodeSet spanning BMP range
    ///
    /// The range spans from `0x0 -> 0xFFFF` inclusive
    pub fn bmp() -> UnicodeSet {
        UnicodeSet {
            inv_list: vec![0, BMP_MAX + 1],
            size: (BMP_MAX as usize) + 1,
        }
    }

    /// Returns the inversion list as a slice
    ///
    /// Public only to the crate, not exposed to public
    pub(crate) fn as_inversion_list(&self) -> &[u32] {
        &self.inv_list
    }

    /// Yields an iterator going through the character set in the UnicodeSet
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// let example_list = vec![65, 68, 69, 70];
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// let mut example_iter = example.iter();
    /// assert_eq!(Some('A'), example_iter.next());
    /// assert_eq!(Some('B'), example_iter.next());
    /// assert_eq!(Some('C'), example_iter.next());
    /// assert_eq!(Some('E'), example_iter.next());
    /// assert_eq!(None, example_iter.next());
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.inv_list
            .chunks(2)
            .flat_map(|pair| (pair[0]..pair[1]))
            .filter_map(char::from_u32)
    }

    /// Returns the number of elements of the UnicodeSet
    pub fn size(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        self.size
    }

    /// Returns whether or not the UnicodeSet is empty
    pub fn is_empty(&self) -> bool {
        self.inv_list.is_empty()
    }

    /// Wrapper for contains
    ///
    /// Returns an `Option` as to whether or not it is possible for the query to be contained.
    /// The value in the `Option` is the start index of the range that contains the query.
    fn contains_query(&self, query: u32) -> Option<usize> {
        match self.inv_list.binary_search(&query) {
            Ok(pos) => {
                if pos % 2 == 0 {
                    Some(pos)
                } else {
                    None
                }
            }
            Err(pos) => {
                if pos % 2 != 0 && pos < self.inv_list.len() {
                    Some(pos)
                } else {
                    None
                }
            }
        }
    }

    /// Checks to see the query is in the UnicodeSet
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// in the set using `std` implementation
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// let example_list = vec![65, 67, 68, 69];
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// assert!(example.contains('A'));
    /// assert!(!example.contains('C'));
    /// ```
    pub fn contains(&self, query: char) -> bool {
        self.contains_query(query as u32).is_some()
    }

    /// Checks to see if the range is in the UnicodeSet, returns a Result
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// in the set using `std::vec::Vec` implementation Only runs the search once on the `start`
    /// parameter, while the `end` parameter is checked in a single `O(1)` step
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// let example_list = vec![65, 67, 68, 69];
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// assert!(example.contains_range(&('A'..'C')));
    /// assert!(example.contains_range(&('A'..='B')));
    /// assert!(!example.contains_range(&('A'..='C')));
    /// ```
    ///
    /// Surrogate points (`0xD800 -> 0xDFFF`) will return false if the Range contains them but the
    /// UnicodeSet does not.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// use std::char;
    /// let check = char::from_u32(0xD7FE).unwrap() .. char::from_u32(0xE001).unwrap();
    /// let example_list = vec![0xD7FE, 0xD7FF, 0xE000, 0xE001];
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// assert!(!example.contains_range(&(check)));
    /// ```
    pub fn contains_range(&self, range: &impl RangeBounds<char>) -> bool {
        let (from, till) = deconstruct_range(range);
        if from >= till {
            return false;
        }
        match self.contains_query(from) {
            Some(pos) => (till) <= self.inv_list[pos + 1],
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{UnicodeSet, UnicodeSetError, BMP_MAX};
    use std::{char, vec::Vec};

    #[test]
    fn test_unicodeset_try_from_vec() {
        let ex = vec![2, 3, 4, 5];
        let check = UnicodeSet::from_inversion_list(ex.clone()).unwrap();
        assert_eq!(ex, check.inv_list);
        assert_eq!(2, check.size());
    }
    #[test]
    fn test_unicodeset_try_from_vec_error() {
        let check = vec![1, 1, 2, 3, 4];
        let set = UnicodeSet::from_inversion_list(check.clone());
        assert_eq!(Err(UnicodeSetError::InvalidSet(check)), set);
    }
    #[test]
    fn test_unicodeset_all() {
        let expected = vec![0, (char::MAX as u32) + 1];
        assert_eq!(UnicodeSet::all().inv_list, expected);
        assert_eq!(
            UnicodeSet::all().size(),
            (expected[1] - expected[0]) as usize
        )
    }
    #[test]
    fn test_unicodeset_bmp() {
        let expected = vec![0, BMP_MAX + 1];
        assert_eq!(UnicodeSet::bmp().inv_list, expected);
        assert_eq!(
            UnicodeSet::bmp().size(),
            (expected[1] - expected[0]) as usize
        );
    }

    // UnicodeSet membership functions
    #[test]
    fn test_unicodeset_contains() {
        let ex = vec![2, 5, 10, 15];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert!(check.contains(2 as char));
        assert!(check.contains(4 as char));
        assert!(check.contains(10 as char));
        assert!(check.contains(14 as char));
    }
    #[test]
    fn test_unicodeset_contains_false() {
        let ex = vec![2, 5, 10, 15];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert!(!check.contains(1 as char));
        assert!(!check.contains(5 as char));
        assert!(!check.contains(9 as char));
        assert!(!check.contains(15 as char));
        assert!(!check.contains(16 as char));
    }
    #[test]
    fn test_unicodeset_contains_range() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert!(check.contains_range(&('A'..='E'))); // 65 - 69
        assert!(check.contains_range(&('K'..'U'))); // 75 - 84
    }
    #[test]
    fn test_unicodeset_contains_range_false() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert!(!check.contains_range(&('!'..'A'))); // 33 - 65
        assert!(!check.contains_range(&('F'..'K'))); // 70 - 74
        assert!(!check.contains_range(&('U'..)));
    }
    #[test]
    fn test_unicodeset_contains_range_invalid() {
        let check = UnicodeSet::all();
        assert!(!check.contains_range(&('A'..'!'))); // 65 - 33
        assert!(!check.contains_range(&('A'..'A')));
    }
    #[test]
    fn test_unicodeset_size() {
        let ex = vec![2, 5, 10, 15];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert_eq!(8, check.size());
        let check = UnicodeSet::all();
        let expected = (char::MAX as u32) + 1;
        assert_eq!(expected as usize, check.size());
        let check = UnicodeSet {
            inv_list: Vec::new(),
            size: 0,
        };
        assert_eq!(check.size(), 0);
    }
    #[test]
    fn test_unicodeset_is_empty() {
        let check = UnicodeSet {
            inv_list: vec![],
            size: 0,
        };
        assert!(check.is_empty());
    }
    #[test]
    fn test_unicodeset_is_not_empty() {
        let check = UnicodeSet::all();
        assert!(!check.is_empty());
    }
    #[test]
    fn test_unicodeset_iter() {
        let ex = vec![65, 68, 69, 70, 0xD800, 0xD801];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        let mut iter = check.iter();
        assert_eq!(Some('A'), iter.next());
        assert_eq!(Some('B'), iter.next());
        assert_eq!(Some('C'), iter.next());
        assert_eq!(Some('E'), iter.next());
        assert_eq!(None, iter.next());
    }
}
