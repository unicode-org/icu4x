use std::{
    char::{from_u32, MAX},
    convert::TryFrom,
    ops::RangeBounds,
    slice::Iter,
};

use super::USetError;
use crate::utils::{deconstruct_range, is_valid};
/// Represents the end code point of the Basic Multilingual Plane range, starting from code point 0 , inclusive
const BMP_MAX: u32 = 0xFFFF;

/// UnicodeSet membership wrapper
///
/// Provides exposure to membership functions and constructors from serialized UnicodeSets
/// and predefined ranges.
/// Implements an [inversion list.](https://en.wikipedia.org/wiki/Inversion_list)
#[derive(Debug, PartialEq)]
pub struct UnicodeSet {
    // If we wanted to use an array to keep the memory on the stack, there is an unsafe nightly feature
    // https://doc.rust-lang.org/nightly/core/array/trait.FixedSizeArray.html
    // Allows for traits of fixed size arrays
    inv_list: Vec<u32>,
}

impl TryFrom<Vec<u32>> for UnicodeSet {
    type Error = USetError;

    fn try_from(set: Vec<u32>) -> Result<Self, Self::Error> {
        if is_valid(&set) {
            Ok(UnicodeSet { inv_list: set })
        } else {
            Err(USetError::InvalidSet(set))
        }
    }
}

impl UnicodeSet {
    /// Returns UnicodeSet spanning entire Unicode range
    ///
    /// The range spans from `0x0 -> 0x10FFFF` inclusive
    pub fn all() -> UnicodeSet {
        UnicodeSet {
            inv_list: vec![0, (MAX as u32) + 1],
        }
    }

    /// Returns UnicodeSet spanning BMP range
    ///
    /// The range spans from `0x0 -> 0xFFFF` inclusive
    pub fn bmp() -> UnicodeSet {
        UnicodeSet {
            inv_list: vec![0, BMP_MAX + 1],
        }
    }

    /// Yields an iterator of start and stop points of ranges in the UnicodeSet
    ///
    /// Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// use std::convert::TryFrom;
    /// let example_list = vec![0, 10, 15, 20];
    /// let example = UnicodeSet::try_from(example_list).unwrap();
    /// let mut example_ranges = example.ranges();
    /// assert_eq!(Some(&0), example_ranges.next());
    /// assert_eq!(Some(&10), example_ranges.next());
    /// assert_eq!(Some(&15), example_ranges.next());
    /// assert_eq!(Some(&20), example_ranges.next());     
    /// assert_eq!(None, example_ranges.next());
    /// ```
    pub fn ranges(&self) -> Iter<u32> {
        self.inv_list.iter()
    }

    /// Yields an iterator going through the character set in the UnicodeSet
    ///
    /// Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// use std::convert::TryFrom;
    /// let example_list = vec![65, 68, 69, 70];
    /// let example = UnicodeSet::try_from(example_list).unwrap();
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
            .map(|val| from_u32(val).unwrap())
    }

    /// Returns the number of elements of the UnicodeSet
    pub fn size(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let s: u32 = self
            .inv_list
            .chunks(2)
            .map(|end_points| end_points[1] - end_points[0])
            .sum();
        s as usize
    }

    /// Returns whether or not the UnicodeSet is empty
    pub fn is_empty(&self) -> bool {
        self.inv_list.is_empty()
    }

    /// Wrapper for contains
    ///
    /// Returns an Option as to whether or not it is possible for the query to be contained
    fn contains_impl(&self, query: u32) -> Option<usize> {
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
    /// in the set using `std::vec::Vec` implementation
    ///
    /// Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// use std::convert::TryFrom;
    /// let example_list = vec![65, 67, 68, 69];
    /// let example = UnicodeSet::try_from(example_list).unwrap();
    /// assert!(example.contains('A'));
    /// assert!(!example.contains('C'));
    /// ```
    pub fn contains(&self, query: char) -> bool {
        self.contains_impl(query as u32).is_some()
    }

    /// Checks to see if the range is in the UnicodeSet, returns a Result
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// in the set using `std::vec::Vec` implementation
    ///
    /// Only runs the search once on the `start` parameter, while the `end` parameter is checked
    /// in a single `O(1)` step
    ///
    /// Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSet;
    /// use std::convert::TryFrom;
    /// let example_list = vec![65, 67, 68, 69];
    /// let example = UnicodeSet::try_from(example_list).unwrap();
    /// assert!(example.contains_range(&('A'..'C')));
    /// assert!(example.contains_range(&('A'..='B')));
    /// assert!(!example.contains_range(&('A'..='C')));
    /// ```
    pub fn contains_range(&self, range: &impl RangeBounds<char>) -> bool {
        let (from, till) = deconstruct_range(range);
        if from >= till {
            return false;
        }
        match self.contains_impl(from) {
            Some(pos) => (till) <= self.inv_list[pos + 1],
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{USetError, UnicodeSet, BMP_MAX};
    use std::{char::MAX, convert::TryFrom, vec::Vec};

    #[test]
    fn test_unicodeset_try_from_vec() {
        let check = UnicodeSet::try_from(vec![2, 3, 4, 5]).unwrap().inv_list;
        assert_eq!(vec![2, 3, 4, 5], check);
    }
    #[test]
    fn test_unicodeset_try_from_vec_error() {
        let check = vec![1, 1, 2, 3, 4];
        let set = UnicodeSet::try_from(vec![1, 1, 2, 3, 4]);
        assert_eq!(Err(USetError::InvalidSet(check)), set);
    }
    #[test]
    fn test_unicodeset_all() {
        let expected = vec![0, (MAX as u32) + 1];
        assert_eq!(UnicodeSet::all().inv_list, expected);
    }
    #[test]
    fn test_unicodeset_bmp() {
        let expected = vec![0, BMP_MAX + 1];
        assert_eq!(UnicodeSet::bmp().inv_list, expected);
    }

    // UnicodeSet membership functions
    #[test]
    fn test_unicodeset_contains() {
        let ex = vec![2, 5, 10, 15];
        let check = UnicodeSet::try_from(ex).unwrap();
        assert!(check.contains(2 as char));
        assert!(check.contains(4 as char));
        assert!(check.contains(10 as char));
        assert!(check.contains(14 as char));
    }
    #[test]
    fn test_unicodeset_contains_false() {
        let ex = vec![2, 5, 10, 15];
        let check = UnicodeSet::try_from(ex).unwrap();
        assert!(!check.contains(1 as char));
        assert!(!check.contains(5 as char));
        assert!(!check.contains(9 as char));
        assert!(!check.contains(15 as char));
        assert!(!check.contains(16 as char));
    }
    #[test]
    fn test_unicodeset_contains_range() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::try_from(ex).unwrap();
        assert!(check.contains_range(&('A'..='E'))); // 65 - 69
        assert!(check.contains_range(&('K'..'U'))); // 75 - 84
    }
    #[test]
    fn test_unicodeset_contains_range_false() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::try_from(ex).unwrap();
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
        let check = UnicodeSet::try_from(ex).unwrap();
        assert_eq!(8, check.size());
        let check = UnicodeSet::all();
        let expected = (MAX as u32) + 1;
        assert_eq!(expected as usize, check.size());
        let check = UnicodeSet {
            inv_list: Vec::new(),
        };
        assert_eq!(check.size(), 0);
    }
    #[test]
    fn test_unicodeset_is_empty() {
        let check = UnicodeSet { inv_list: vec![] };
        assert!(check.is_empty());
    }
    #[test]
    fn test_unicodeset_is_not_empty() {
        let check = UnicodeSet::all();
        assert!(!check.is_empty());
    }
    #[test]
    fn test_unicodeset_ranges() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::try_from(ex).unwrap();
        let mut iter = check.ranges();
        assert_eq!(iter.next(), Some(&65));
        assert_eq!(iter.next(), Some(&70));
        assert_eq!(iter.next(), Some(&75));
        assert_eq!(iter.next(), Some(&85));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_unicodeset_iter() {
        let ex = vec![65, 68, 69, 70];
        let check = UnicodeSet::try_from(ex).unwrap();
        let mut iter = check.iter();
        assert_eq!(Some('A'), iter.next());
        assert_eq!(Some('B'), iter.next());
        assert_eq!(Some('C'), iter.next());
        assert_eq!(Some('E'), iter.next());
        assert_eq!(None, iter.next());
    }
}
