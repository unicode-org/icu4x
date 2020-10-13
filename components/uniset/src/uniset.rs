// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::{char, ops::RangeBounds, slice::Chunks};

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
    /// use icu_uniset::UnicodeSet;
    /// use icu_uniset::UnicodeSetError;
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
    /// use icu_uniset::UnicodeSet;
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
                    Some(pos - 1)
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
    /// use icu_uniset::UnicodeSet;
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
    /// use icu_uniset::UnicodeSet;
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
    /// use icu_uniset::UnicodeSet;
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

    /// Check if the calling UnicodeSet contains all the characters of the given UnicodeSet
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = vec![65, 70, 85, 91]; // A - E, U - Z
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// let a_to_d = UnicodeSet::from_inversion_list(vec![65, 69]).unwrap();
    /// let f_to_t = UnicodeSet::from_inversion_list(vec![70, 85]).unwrap();
    /// let r_to_x = UnicodeSet::from_inversion_list(vec![82, 88]).unwrap();
    /// assert!(example.contains_set(&a_to_d)); // contains all
    /// assert!(!example.contains_set(&f_to_t)); // contains none
    /// assert!(!example.contains_set(&r_to_x)); // contains some
    /// ```
    pub fn contains_set(&self, set: &UnicodeSet) -> bool {
        if set.size() > self.size() {
            return false;
        }
        let mut set_ranges: Chunks<u32> = set.as_inversion_list().chunks(2);
        let mut check = set_ranges.next();
        for range in self.inv_list.chunks(2) {
            match check {
                Some(r) => {
                    if r[0] >= range[0] && r[1] <= range[1] {
                        check = set_ranges.next();
                    }
                }
                _ => break,
            }
        }
        check.is_none()
    }

    /// Returns the end of the initial substring where the characters are either contained/not contained
    /// in the set.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = vec![65, 68]; // {A, B, C}
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// assert_eq!(example.span("CABXYZ", true), 3);
    /// assert_eq!(example.span("XYZC", false), 3);
    /// assert_eq!(example.span("XYZ", true), 0);
    /// assert_eq!(example.span("ABC", false), 0);
    /// ```
    pub fn span(&self, span_str: &str, contained: bool) -> usize {
        span_str
            .chars()
            .take_while(|&x| self.contains(x) == contained)
            .count()
    }

    /// Returns the start of the trailing substring (starting from end of string) where the characters are
    /// either contained/not contained in the set. Returns the length of the string if no valid return.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSet;
    /// let example_list = vec![65, 68]; // {A, B, C}
    /// let example = UnicodeSet::from_inversion_list(example_list).unwrap();
    /// assert_eq!(example.span_back("XYZCAB", true), 3);
    /// assert_eq!(example.span_back("ABCXYZ", true), 6);
    /// assert_eq!(example.span_back("CABXYZ", false), 3);
    /// ```
    pub fn span_back(&self, span_str: &str, contained: bool) -> usize {
        span_str.len()
            - span_str
                .chars()
                .rev()
                .take_while(|&x| self.contains(x) == contained)
                .count()
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
    fn test_unicodeset_contains_query() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert!(check.contains_query(64).is_none());
        assert_eq!(check.contains_query(65).unwrap(), 0);
        assert_eq!(check.contains_query(68).unwrap(), 0);
        assert!(check.contains_query(70).is_none());
        assert_eq!(check.contains_query(76).unwrap(), 2);
        assert!(check.contains_query(86).is_none());
    }

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
        assert!(check.contains_range(&('C'..'D'))); // 67 - 67
        assert!(check.contains_range(&('L'..'P'))); // 76 - 80
        assert!(!check.contains_range(&('L'..='U'))); // 76 - 85
    }
    #[test]
    fn test_unicodeset_contains_range_false() {
        let ex = vec![65, 70, 75, 85];
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert!(!check.contains_range(&('!'..'A'))); // 33 - 65
        assert!(!check.contains_range(&('F'..'K'))); // 70 - 74
        assert!(!check.contains_range(&('U'..))); // 85 - ..
    }
    #[test]
    fn test_unicodeset_contains_range_invalid() {
        let check = UnicodeSet::all();
        assert!(!check.contains_range(&('A'..'!'))); // 65 - 33
        assert!(!check.contains_range(&('A'..'A'))); // 65 - 65
    }
    #[test]
    fn test_unicodeset_contains_set_u() {
        let ex = vec![10, 20, 40, 50, 70, 80, 100, 110];
        let u = UnicodeSet::from_inversion_list(ex).unwrap();
        let inside = vec![15, 20, 44, 49, 70, 80, 100, 109];
        let s = UnicodeSet::from_inversion_list(inside).unwrap();
        assert!(u.contains_set(&s));
    }
    #[test]
    fn test_unicodeset_contains_set_u_false() {
        let ex = vec![10, 20, 40, 50, 70, 80, 100, 110];
        let u = UnicodeSet::from_inversion_list(ex).unwrap();
        let outside = vec![0, 10, 22, 44, 50, 70, 79, 81, 109, 111];
        let s = UnicodeSet::from_inversion_list(outside).unwrap();
        assert!(!u.contains_set(&s));
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
    #[test]
    fn test_unicodeset_span_contains() {
        let ex = vec![65, 68, 70, 75]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert_eq!(check.span("ABCDE", true), 3);
        assert_eq!(check.span("E", true), 0);
    }
    #[test]
    fn test_unicodeset_span_does_not_contain() {
        let ex = vec![65, 68, 70, 75]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert_eq!(check.span("DEF", false), 2);
        assert_eq!(check.span("KLMA", false), 3);
    }
    #[test]
    fn test_unicodeset_span_back_contains() {
        let ex = vec![65, 68, 70, 75]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert_eq!(check.span_back("XYZABFH", true), 3);
        assert_eq!(check.span_back("ABCXYZ", true), 6);
    }
    #[test]
    fn test_unicodeset_span_back_does_not_contain() {
        let ex = vec![65, 68, 70, 75]; // A - D, F - K
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        assert_eq!(check.span_back("ABCXYZ", false), 3);
        assert_eq!(check.span_back("XYZABC", false), 6);
    }
}
