use std::{char, cmp, cmp::Ordering, ops::RangeBounds};

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

    /// Adds the range to the end of the builder
    fn append(&mut self, start: u32, end: u32) {
        self.intervals.push(start);
        self.intervals.push(end);
    }

    /// Adds the range to the beginning of the builder in a single memory move
    fn prepend(&mut self, start: u32, end: u32) {
        let mut new = vec![start, end];
        new.append(&mut self.intervals);
        self.intervals = new;
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
            self.append(start, end);
        } else {
            let last_ind = self.intervals.len() - 1;
            if self.intervals[last_ind - 1] <= start {
                if self.intervals[last_ind] < start {
                    self.append(start, end);
                } else {
                    self.intervals[last_ind] = cmp::max(end, self.intervals[last_ind]);
                }
            } else if end <= self.intervals[1] {
                if end < self.intervals[0] {
                    self.prepend(start, end);
                } else {
                    self.intervals[0] = cmp::min(self.intervals[0], start);
                }
            } else {
                let s = self.intervals.binary_search(&start);
                let e = self.intervals.binary_search(&end);
                let mut s_index = s.unwrap_or_else(|x| x);
                let mut e_index = e.unwrap_or_else(|x| x);
                let s_even: bool = s_index % 2 == 0;
                let e_even: bool = e_index % 2 == 0;

                if s_even && e_even && s.is_err() && e.is_err() && s_index == e_index {
                    self.intervals.splice(s_index..e_index, vec![start, end]);
                } else {
                    let s_equals_e = s_index != e_index;
                    if s_even {
                        self.intervals[s_index] = start;
                        s_index += 1;
                    }
                    if e_even && s_equals_e {
                        e_index -= 1;
                        self.intervals[e_index] = end;
                    }
                    if s_index < e_index {
                        self.intervals.drain((s_index)..(e_index));
                    }
                }
            }
        }
    }

    /// Add the character to the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSetBuilder;
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
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'Z'));
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
    /// use icu_unicodeset::{UnicodeSet, UnicodeSetBuilder};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 76]).unwrap();
    /// builder.add_set(&set);
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('A'));
    /// ```
    pub fn add_set(&mut self, set: &UnicodeSet) {
        for range in set.ranges() {
            self.add(range[0], range[1]);
        }
    }

    /// Removes the range from the UnicodeSetBuilder
    ///
    /// Performs binary search to find start and end affected intervals, then removes in an `O(N)` fashion
    /// where `N` is the number of endpoints, with in-place memory.
    fn remove(&mut self, start: u32, end: u32) {
        if start < end && !self.intervals.is_empty() {
            if start <= self.intervals[0] && end >= *self.intervals.last().unwrap() {
                self.intervals = vec![];
            } else {
                let start_res = self.intervals.binary_search(&start);
                let end_res = self.intervals.binary_search(&end);
                let mut start_ind = start_res.unwrap_or_else(|x| x);
                let mut end_ind = end_res.unwrap_or_else(|x| x);
                let start_odd = start_ind % 2 != 0;
                let end_odd = end_ind % 2 != 0;
                if start_res.is_err() && end_res.is_err() && start_ind == end_ind && start_odd {
                    self.intervals.splice(start_ind..end_ind, vec![start, end]);
                } else {
                    if start_odd {
                        if start_res.is_err() {
                            self.intervals[start_ind] = start;
                        }
                        start_ind += 1;
                    }
                    if end_odd {
                        if end_res.is_err() {
                            end_ind -= 1;
                            self.intervals[end_ind] = end;
                        } else {
                            end_ind += 1;
                        }
                    }
                    if start_ind < end_ind {
                        self.intervals.drain(start_ind..end_ind);
                    }
                }
            }
        }
    }

    /// Remove the character from the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'Z'));
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
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'Z'));
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
    /// use icu_unicodeset::{UnicodeSet, UnicodeSetBuilder};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 70]).unwrap();
    /// builder.add_range(&('A'..'Z'));
    /// builder.remove_set(&set); // rermoves 'A'..'F'
    /// let check = builder.build();
    /// assert_eq!(check.iter().next(), Some('F'));
    pub fn remove_set(&mut self, set: &UnicodeSet) {
        for range in set.ranges() {
            self.remove(range[0], range[1]);
        }
    }

    /// Retain the elements in the UnicodeSetBuilder that are in the parameter
    ///
    /// Performs this in an `O(R)` amortized run-time, where `R` is the number of ranges
    fn retain(&mut self, ret: Vec<u32>) {
        let mut chunk = vec![0];
        chunk.extend(ret);
        chunk.push((char::MAX as u32) + 1);
        chunk.chunks(2).for_each(|c| self.remove(c[0], c[1]));
    }

    /// Retain the specified character in the UnicodeSetBuilder if it exists
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'Z'));
    /// builder.retain_char('A');
    /// let set = builder.build();
    /// let mut check = set.iter();
    /// assert_eq!(check.next(), Some('A'));
    /// assert_eq!(check.next(), None);
    /// ```
    pub fn retain_char(&mut self, c: char) {
        let code_point = c as u32;
        let to_retain = vec![code_point, code_point + 1];
        self.retain(to_retain);
    }

    /// Retain the range of characters located within the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'Z'));
    /// builder.retain_range(&('A'..='B'));
    /// let set = builder.build();
    /// let mut check = set.iter();
    /// assert_eq!(check.next(), Some('A'));
    /// assert_eq!(check.next(), Some('B'));
    /// assert_eq!(check.next(), None);
    /// ```
    pub fn retain_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        let to_retain = vec![start, end];
        self.retain(to_retain);
    }

    /// Retain the elements in the specified set within the UnicodeSetBuilder
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::{UnicodeSetBuilder, UnicodeSet};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 70]).unwrap();
    /// builder.add_range(&('A'..'Z'));
    /// builder.retain_set(&set); // retains 'A'..'F'
    /// let check = builder.build();
    /// assert!(check.contains('A'));
    /// assert!(!check.contains('G'));
    /// ```
    pub fn retain_set(&mut self, set: &UnicodeSet) {
        self.retain(set.ranges().collect::<Vec<&[u32]>>().concat());
    }

    /// Computes the complement of the argument, adding any elements that do not yet exist in the builder,
    /// and removing any elements that already exist in the builder. See public functions for examples.
    ///
    /// Performs in `O(B + S)`, where `B` is the number of endpoints in the Builder, and `S` is the number
    /// of endpoints in the argument.
    fn complement_list(&mut self, set: Vec<u32>) {
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
    /// use icu_unicodeset::{UnicodeSetBuilder, UnicodeSet};
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
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'D'));
    /// builder.complement_char('A');
    /// builder.complement_char('E');
    /// let check = builder.build();
    /// assert!(check.contains('E'));
    /// assert!(!check.contains('A'));
    /// ```
    pub fn complement_char(&mut self, c: char) {
        let code_point = c as u32;
        let to_complement = vec![code_point, code_point + 1];
        self.complement_list(to_complement);
    }

    /// Complements the range in the builder, adding any elements in the range if not in the builder, and
    /// removing them otherwise.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_range(&('A'..'D'));
    /// builder.complement_range(&('C'..='F'));
    /// let check = builder.build();
    /// assert!(check.contains('F'));
    /// assert!(!check.contains('C'));
    /// ```
    pub fn complement_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        let to_complement = vec![start, end];
        self.complement_list(to_complement);
    }

    /// Complements the set in the builder, adding any elements in the set if not in the builder, and
    /// removing them otherwise.
    ///
    /// # Example:
    ///
    /// ```
    /// use icu_unicodeset::{UnicodeSetBuilder, UnicodeSet};
    /// let mut builder = UnicodeSetBuilder::new();
    /// let set = UnicodeSet::from_inversion_list(vec![65, 70, 75, 90]).unwrap();
    /// builder.add_range(&('C'..'N')); // 67 - 78
    /// builder.complement_set(&set);
    /// let check = builder.build();
    /// assert!(check.contains('Q')); // 81
    /// assert!(!check.contains('M')); // 77
    /// ```
    pub fn complement_set(&mut self, set: &UnicodeSet) {
        self.complement_list(set.ranges().collect::<Vec<&[u32]>>().concat());
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
    fn test_append() {
        let mut ex = UnicodeSetBuilder::new();
        ex.append(10, 20);
        assert_eq!(ex.intervals, vec![10, 20]);
    }

    #[test]
    fn test_prepend() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.prepend(0, 5);
        let check = vec![0, 5, 10, 20, 40, 50];
        assert_eq!(builder.intervals, check);
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
    fn test_add_unicodeset() {
        let ex = vec![10, 20, 40, 50];
        let check = UnicodeSet::from_inversion_list(ex.clone()).unwrap();
        let mut builder = UnicodeSetBuilder::new();
        builder.add_set(&check);
        assert_eq!(builder.intervals, ex);
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
    fn test_retain() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.retain(vec![0, 15, 45, 100]);
        let expected = vec![10, 15, 45, 50];
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
        builder.complement_list(vec![15, 20]);
        let expected = vec![10, 15, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_exterior() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.complement_list(vec![25, 35]);
        let expected = vec![10, 20, 25, 35, 40, 50];
        assert_eq!(builder.intervals, expected);
    }

    #[test]
    fn test_complement_larger_list() {
        let mut builder = generate_tester(vec![10, 20, 40, 50]);
        builder.complement_list(vec![30, 55, 60, 70]);
        let expected = vec![10, 20, 30, 40, 50, 55, 60, 70];
        assert_eq!(builder.intervals, expected);
    }
}
