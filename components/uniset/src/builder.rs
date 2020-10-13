// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::{char, cmp::Ordering, ops::RangeBounds};

use crate::{uniset::UnicodeSet, utils::deconstruct_range};

/// UnicodeSet builder wrapper
///
/// Provides exposure to builder functions and conversion to UnicodeSet
#[derive(Default)]
pub struct UnicodeSetBuilder {
    intervals: Vec<u32>,
}

impl UnicodeSetBuilder {
    /// Returns empty UnicodeSetBuilder
    pub fn new() -> UnicodeSetBuilder {
        UnicodeSetBuilder { intervals: vec![] }
    }

    /// Returns a UnicodeSet and consumes the UnicodeSetBuilder
    pub fn build(self) -> UnicodeSet {
        UnicodeSet::from_inversion_list(self.intervals).unwrap()
    }

    /// Abstraction for adding/removing a range from start..end
    ///
    /// If add is true add, else remove
    fn add_remove_middle(&mut self, start: u32, end: u32, add: bool) {
        let start_res = self.intervals.binary_search(&start);
        let end_res = self.intervals.binary_search(&end);
        let mut start_ind = start_res.unwrap_or_else(|x| x);
        let mut end_ind = end_res.unwrap_or_else(|x| x);
        let start_pos_check = (start_ind % 2 == 0) == add;
        let end_pos_check = (end_ind % 2 == 0) == add;
        let start_eq_end = start_ind == end_ind;

        if start_eq_end && start_pos_check && end_res.is_err() {
            let ins = &[start, end];
            self.intervals
                .splice(start_ind..end_ind, ins.iter().copied());
        } else {
            if start_pos_check {
                self.intervals[start_ind] = start;
                start_ind += 1;
            }
            if end_pos_check {
                if end_res.is_ok() {
                    end_ind += 1;
                } else {
                    end_ind -= 1;
                    self.intervals[end_ind] = end;
                }
            }
            if start_ind < end_ind {
                self.intervals.drain(start_ind..end_ind);
            }
        }
    }

    /// Add the range to the UnicodeSetBuilder
    ///
    /// Accomplishes this through binary search for the start and end indices and merges intervals
    /// in between with inplace memory. Performs `O(1)` operation if adding to end of list, and `O(N)` otherwise,
    /// where `N` is the number of endpoints.
    fn add(&mut self, start: u32, end: u32) {
        if start >= end {
            return;
        }
        if self.intervals.is_empty() {
            self.intervals.extend_from_slice(&[start, end]);
            return;
        }
        self.add_remove_middle(start, end, true);
    }

    /// Add the character to the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_char('a');
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('a'));
    /// ```
    pub fn add_char(&mut self, c: char) {
        let to_add = c as u32;
        self.add(to_add, to_add + 1);
    }

    /// Add the range of characters to the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='Z'));
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('A'));
    /// ```  
    pub fn add_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        self.add(start, end);
    }

    /// Add the UnicodeSet reference to the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::{UnicodeSet, UnicodeSetBuilder};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 76]).unwrap();
    /// builder.add_set(&set);
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('A'));
    /// ```
    pub fn add_set(&mut self, set: &UnicodeSet) {
        for range in set.as_inversion_list().chunks(2) {
            self.add(range[0], range[1]);
        }
    }

    /// Removes the range from the UnicodeSetBuilder
    ///
    /// Performs binary search to find start and end affected intervals, then removes in an `O(N)` fashion
    /// where `N` is the number of endpoints, with in-place memory.
    fn remove(&mut self, start: u32, end: u32) {
        if start >= end || self.intervals.is_empty() {
            return;
        }
        if let Some(last) = self.intervals.last() {
            if start <= self.intervals[0] && end >= *last {
                self.intervals.clear();
            } else {
                self.add_remove_middle(start, end, false);
            }
        }
    }

    /// Remove the character from the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='Z'));
    /// builder.remove_char('A');
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('B'));
    pub fn remove_char(&mut self, c: char) {
        let to_remove = c as u32;
        self.remove(to_remove, to_remove + 1);
    }

    /// Remove the range of characters from the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='Z'));
    /// builder.remove_range(&('A'..='C'));
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('D'));
    pub fn remove_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        self.remove(start, end);
    }

    /// Remove the UnicodeSet from the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::{UnicodeSet, UnicodeSetBuilder};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 70]).unwrap();
    /// builder.add_range(&('A'..='Z'));
    /// builder.remove_set(&set); // removes 'A'..='E'
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('F'));
    pub fn remove_set(&mut self, set: &UnicodeSet) {
        for range in set.as_inversion_list().chunks(2) {
            self.remove(range[0], range[1]);
        }
    }

    /// Retain the specified character in the UnicodeSetBuilder if it exists
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='Z'));
    /// builder.retain_char('A');
    /// let set = builder.build();
    /// let mut check = set.iter();
    /// assert_eq!(check.next(), Some('A'));
    /// assert_eq!(check.next(), None);
    /// ```
    pub fn retain_char(&mut self, c: char) {
        let code_point = c as u32;
        self.remove(0, code_point);
        self.remove(code_point + 1, (char::MAX as u32) + 1);
    }

    /// Retain the range of characters located within the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='Z'));
    /// builder.retain_range(&('A'..='B'));
    /// let set = builder.build();
    /// let mut check = set.iter();
    /// assert_eq!(check.next(), Some('A'));
    /// assert_eq!(check.next(), Some('B'));
    /// assert_eq!(check.next(), None);
    /// ```
    pub fn retain_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        self.remove(0, start);
        self.remove(end, (char::MAX as u32) + 1);
    }

    /// Retain the elements in the specified set within the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::{UnicodeSetBuilder, UnicodeSet};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 70]).unwrap();
    /// builder.add_range(&('A'..='Z'));
    /// builder.retain_set(&set); // retains 'A'..='E'
    /// let check = builder.build();
    /// assert!(check.contains('A'));
    /// assert!(!check.contains('G'));
    /// ```
    pub fn retain_set(&mut self, set: &UnicodeSet) {
        let mut prev = 0;
        for range in set.as_inversion_list().chunks(2) {
            self.remove(prev, range[0]);
            prev = range[1];
        }
        self.remove(prev, (char::MAX as u32) + 1);
    }

    /// Computes the complement of the argument, adding any elements that do not yet exist in the builder,
    /// and removing any elements that already exist in the builder. See public functions for examples.
    ///
    /// Performs in `O(B + S)`, where `B` is the number of endpoints in the Builder, and `S` is the number
    /// of endpoints in the argument.
    fn complement_list(&mut self, set: &[u32]) {
        let mut res: Vec<u32> = vec![]; // not the biggest fan of having to allocate new memory
        let mut ai = self.intervals.iter();
        let mut bi = set.iter();
        let mut a = ai.next();
        let mut b = bi.next();
        while let (Some(c), Some(d)) = (a, b) {
            match c.cmp(&d) {
                Ordering::Less => {
                    res.push(*c);
                    a = ai.next();
                }
                Ordering::Greater => {
                    res.push(*d);
                    b = bi.next();
                }
                Ordering::Equal => {
                    a = ai.next();
                    b = bi.next();
                }
            }
        }
        if let Some(c) = a {
            res.push(*c)
        }
        if let Some(d) = b {
            res.push(*d)
        }
        res.extend(ai);
        res.extend(bi);
        self.intervals = res;
    }

    /// Computes the complement of the builder, inverting the builder (any elements in the builder are removed,
    /// while any elements not in the builder are added)
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::{UnicodeSetBuilder, UnicodeSet};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![0, 65, 70, (std::char::MAX as u32) + 1]).unwrap();
    /// builder.add_set(&set);
    /// builder.complement();
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('A'));
    /// ```
    pub fn complement(&mut self) {
        if !self.intervals.is_empty() {
            if self.intervals[0] == 0 {
                self.intervals.drain(0..1);
            } else {
                self.intervals.insert(0, 0);
            }
            if self.intervals.last() == Some(&((char::MAX as u32) + 1)) {
                self.intervals.pop();
            } else {
                self.intervals.push((char::MAX as u32) + 1);
            }
        }
    }

    /// Complements the character in the builder, adding it if not in the builder, and removing it otherwise.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='D'));
    /// builder.complement_char('A');
    /// builder.complement_char('E');
    /// let check = builder.build();
    /// assert!(check.contains('E'));
    /// assert!(!check.contains('A'));
    /// ```
    pub fn complement_char(&mut self, c: char) {
        let code_point = c as u32;
        let to_complement = [code_point, code_point + 1];
        self.complement_list(&to_complement);
    }

    /// Complements the range in the builder, adding any elements in the range if not in the builder, and
    /// removing them otherwise.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..='D'));
    /// builder.complement_range(&('C'..='F'));
    /// let check = builder.build();
    /// assert!(check.contains('F'));
    /// assert!(!check.contains('C'));
    /// ```
    pub fn complement_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        let to_complement = [start, end];
        self.complement_list(&to_complement);
    }

    /// Complements the set in the builder, adding any elements in the set if not in the builder, and
    /// removing them otherwise.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_uniset::{UnicodeSetBuilder, UnicodeSet};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 70, 75, 90]).unwrap();
    /// builder.add_range(&('C'..='N')); // 67 - 78
    /// builder.complement_set(&set);
    /// let check = builder.build();
    /// assert!(check.contains('Q')); // 81
    /// assert!(!check.contains('N')); // 78
    /// ```
    pub fn complement_set(&mut self, set: &UnicodeSet) {
        self.complement_list(set.as_inversion_list());
    }
}

#[cfg(test)]
mod tests {
    use super::{UnicodeSet, UnicodeSetBuilder};
    use std::char;

    fn generate_tester(ex: Vec<u32>) -> UnicodeSetBuilder {
        let check = UnicodeSet::from_inversion_list(ex).unwrap();
        let mut builder = UnicodeSetBuilder::new();
        builder.add_set(&check);
        builder
    }

    #[test]
    fn test_new() {
        let ex = UnicodeSetBuilder::new();
        assert!(ex.intervals.is_empty());
    }

    #[test]
    fn test_build() {
        let mut builder = UnicodeSetBuilder::new();
        builder.add(65, 66);
        let check: UnicodeSet = builder.build();
        assert_eq!(check.iter().next(), Some('A'));
    }

    #[test]
    fn test_add_to_empty() {
        let mut builder = UnicodeSetBuilder::new();
        builder.add(0, 10);
        assert_eq!(builder.intervals, vec![0, 10]);
    }

    #[test]
    fn test_add_invalid() {
        let mut builder = UnicodeSetBuilder::new();
        builder.add(0, 0);
        builder.add(5, 0);
        assert_eq!(builder.intervals, vec![]);
    }

    #[test]
    fn test_add_to_start() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(0, 5);
        let expected = vec![0, 5, 10, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_start_overlap() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(0, 15);
        let expected = vec![0, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_end() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(60, 70);
        let expected = vec![10, 20, 40, 50, 60, 70];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_end_overlap() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(45, 70);
        let expected = vec![10, 20, 40, 70];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_middle_no_overlap() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(25, 27);
        let expected = vec![10, 20, 25, 27, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_middle_inside() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(10, 20);
        let expected = vec![10, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_middle_left_overlap() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(15, 25);
        let expected = vec![10, 25, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_middle_right_overlap() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(30, 40);
        let expected = vec![10, 20, 30, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_full_encompass() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(0, 60);
        let expected = vec![0, 60];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_to_partial_encompass() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(0, 35);
        let expected = vec![0, 35, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_aligned_front() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(5, 10);
        let expected = vec![5, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_aligned_back() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(50, 55);
        let expected = vec![10, 20, 40, 55];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_aligned_start_middle() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(20, 25);
        let expected = vec![10, 25, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_aligned_end_middle() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(35, 40);
        let expected = vec![10, 20, 35, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_aligned_in_between_end() {
        let mut builder = generate_tester(vec![10, 20, 30, 40, 50, 60]);
        builder.add(15, 30);
        let expected = vec![10, 40, 50, 60];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_aligned_in_between_start() {
        let mut builder = generate_tester(vec![10, 20, 30, 40, 50, 60]);
        builder.add(20, 35);
        let expected = vec![10, 40, 50, 60];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_adjacent_ranges() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.add(19, 20);
        builder.add(20, 21);
        builder.add(21, 22);
        let expected = vec![10, 22, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_unicodeset() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        let check = UnicodeSet::from_inversion_list(vec![5, 10, 22, 33, 44, 51]).unwrap();
        builder.add_set(&check);
        let expected = vec![5, 20, 22, 33, 40, 51];
        assert_eq!(builder.intervals, expected);
    }
    #[test]
    fn test_add_char() {
        let mut builder = UnicodeSetBuilder::new();
        builder.add_char('a');
        let expected = vec![97, 98];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_range() {
        let mut builder = UnicodeSetBuilder::new();
        builder.add_range(&('A'..='Z'));
        let expected = vec![65, 91];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_add_invalid_range() {
        let mut builder = UnicodeSetBuilder::new();
        builder.add_range(&('Z'..='A'));
        let expected = vec![];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_empty() {
        let mut builder = UnicodeSetBuilder::new();
        builder.remove(0, 10);
        let expected = vec![];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_entire_builder() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(10, 50);
        let expected = vec![];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_entire_range() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(10, 20);
        let expected = vec![40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_partial_range_left() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(10, 45);
        let expected = vec![45, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_ne_range() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(20, 40);
        let expected = vec![10, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_partial_range_right() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(15, 55);
        let expected = vec![10, 15];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_middle_range() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(12, 18);
        let expected = vec![10, 12, 18, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_ne_middle_range() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(25, 27);
        let expected = vec![10, 20, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_encompassed_range() {
        let mut builder = generate_tester(vec![10, 20, 40, 50, 70, 80]);
        builder.remove(25, 55);
        let expected = vec![10, 20, 70, 80];
        assert_eq!(builder.intervals, expected);
    }
    #[test]
    fn test_remove_adjacent_ranges() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.remove(39, 40);
        builder.remove(40, 41);
        builder.remove(41, 42);
        let expected = vec![10, 20, 42, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_char() {
        let mut builder = generate_tester(vec![65, 70]);
        builder.remove_char('A'); // 65
        let expected = vec![66, 70];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_range() {
        let mut builder = generate_tester(vec![65, 90]);
        builder.remove_range(&('A'..'L')); // 65 - 76
        let expected = vec![76, 90];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_remove_set() {
        let mut builder = generate_tester(vec![10, 20, 40, 50, 70, 80]);
        let remove = UnicodeSet::from_inversion_list(vec![10, 20, 45, 75]).unwrap();
        builder.remove_set(&remove);
        let expected = vec![40, 45, 75, 80];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_retain_char() {
        let mut builder = generate_tester(vec![65, 90]);
        builder.retain_char('A'); // 65
        let expected = vec![65, 66];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_retain_range() {
        let mut builder = generate_tester(vec![65, 90]);
        builder.retain_range(&('C'..'F')); // 67 - 70
        let expected = vec![67, 70];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_retain_range_empty() {
        let mut builder = generate_tester(vec![65, 70]);
        builder.retain_range(&('F'..'Z'));
        let expected = vec![];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_retain_set() {
        let mut builder = generate_tester(vec![10, 20, 40, 50, 70, 80]);
        let retain = UnicodeSet::from_inversion_list(vec![15, 20, 25, 55, 79, 81]).unwrap();
        builder.retain_set(&retain);
        let expected = vec![15, 20, 40, 50, 79, 80];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.complement();
        let expected = vec![0, 10, 20, 40, 50, (char::MAX as u32) + 1];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_zero_max() {
        let mut builder = generate_tester(vec![0, 10, 90, (char::MAX as u32) + 1]);
        builder.complement();
        let expected = vec![10, 90];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_interior() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.complement_list(&[15, 20]);
        let expected = vec![10, 15, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_exterior() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.complement_list(&[25, 35]);
        let expected = vec![10, 20, 25, 35, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_larger_list() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.complement_list(&[30, 55, 60, 70]);
        let expected = vec![10, 20, 30, 40, 50, 55, 60, 70];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_char() {
        let mut builder = generate_tester(vec![65, 76]); // A - K
        builder.complement_char('A');
        builder.complement_char('L');
        let expected = vec![66, 77];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_range() {
        let mut builder = generate_tester(vec![70, 76]); // F - K
        builder.complement_range(&('A'..='Z'));
        let expected = vec![65, 70, 76, 91];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_set() {
        let mut builder = generate_tester(vec![67, 78]);
        let set = UnicodeSet::from_inversion_list(vec![65, 70, 75, 90]).unwrap();
        builder.complement_set(&set);
        let expected = vec![65, 67, 70, 75, 78, 90];
        assert_eq!(builder.intervals, expected);
    }
}
