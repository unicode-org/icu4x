use std::{char, cmp, ops::RangeBounds};

use crate::uniset::UnicodeSet;
use crate::{utils::deconstruct_range};

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
    /// in between with inplace memory. Performs O(1) operation if adding to end of list, and O(n) otherwise.
    fn add(&mut self, start: u32, end: u32) -> Self{
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
                let mut s = self.intervals.binary_search(&start).unwrap_or_else(|x| x);
                let mut e = self.intervals.binary_search(&end).unwrap_or_else(|x| x);
                if s % 2 == 0 {
                    self.intervals[s] = cmp::min(self.intervals[s], start);
                } else {
                    s -= 1; 
                }
                if e % 2 == 0 {
                    self.intervals[e - 1] = cmp::max(self.intervals[e - 1], end);
                } else {
                    e += 1;
                }
                self.intervals.drain((s + 1)..(e - 1));
            }
        }
        self
    }

    /// Add the character to the UnicodeSetBuilder
    /// 
    /// # Example:
    /// 
    /// ```
    /// use icu_unicodeset::UnicodeSetBuilder;
    /// let mut builder = UnicodeSetBuilder::new();
    /// builder.add_char('a');
    /// ```
    pub fn add_char(&mut self, c: char) {
        let to_add = c as u32;
        self.add(to_add, to_add + 1);
    }

    /// Add the range of characters to the UnicodeSetBuilder
    pub fn add_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        self.add(start, end);
    }

    /// Add the UnicodeSet to the UnicodeSetBuilder
    pub fn add_set(&mut self, set: &UnicodeSet) {
        set.get_invlist().chunks(2).for_each(|range| self.add(range[0], range[1]));
    }

    /// Removes the range from the UnicodeSetBuilder
    fn remove(&mut self, start: u32, end: u32) {
        if start < end && !self.intervals.is_empty() {
            let s = self.intervals.binary_search(&start).unwrap_or_else(|x| x);
            let last = self.intervals.binary_search(&end);
            let e = last.unwrap_or_else(|x| x - 1) + 1;
            if s % 2 != 0 && e % 2 == 0 {
                let mut insert = vec![start, end];
                if last.is_ok() { insert.pop(); }
                self.intervals.splice(s..e, insert.iter().cloned());
            } else {
                self.intervals.drain(s..e);
                if s % 2 != 0 {
                    self.intervals.insert(s, start);
                } else if e % 2 != 0 {
                    self.intervals.insert(s, end);
                }
            }
        }
    }

    /// Remove the character from the UnicodeSetBuilder
    pub fn remove_char(&mut self, c: char) {
        let to_remove = c as u32;
        self.remove(to_remove, to_remove + 1);
    }

    /// Remove the range of characters from the UnicodeSetBuilder
    pub fn remove_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        self.remove(start, end);
    }

    /// Remove the UnicodeSet from the UnicodeSetBuilder
    pub fn remove_set(&mut self, set: &UnicodeSet) {
        set.get_invlist().chunks(2).for_each(|range| self.remove(range[0], range[1]));
    }

    /// Retain the elements in the UnicodeSetBuilder that are in the parameter
    fn retain(&mut self, ret: Vec<u32>) {
        let mut chunk = vec![0, (char::MAX as u32) + 1];
        chunk.splice(1..1, ret);
        chunk.chunks(2).for_each(|c| self.remove(c[0], c[1]));
    }

    /// Retain the specified character in the UnicodeSetBuilder if it exists 
    pub fn retain_char(&mut self, c: char) {
        let code_point = c as u32;
        let to_retain = vec![code_point, code_point + 1];
        self.retain(to_retain);
    }

    /// Retain the range of characters located within the UnicodeSetBuilder 
    pub fn retain_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        let to_retain = vec![start, end];
        self.retain(to_retain);
    }

    /// Retain the elements in the specified set within the UnicodeSetBuilder
    pub fn retain_set(&mut self, set: &UnicodeSet) {
        self.retain(set.get_invlist());
    }

    fn complement_list(&mut self, set: Vec<u32>) {
        let mut res: Vec<u32> = vec![]; // not the biggest fan of having to allocate new memory
        let mut ai = self.intervals.iter();
        let mut bi = set.iter();
        let mut a = ai.next();
        let mut b = bi.next();
        while let (Some(c), Some(d)) = (a, b) {
            if c < d {
                res.push(*c);
                a = ai.next();
            }
            else if d < c {
                res.push(*d);
                b = bi.next();
            }
            else {
                a = ai.next();
                b = bi.next();
            }
        }
        if let Some(c) = a { res.push(*c) }
        if let Some(d) = b { res.push(*d) }
        res.extend(ai);
        res.extend(bi);
        self.intervals = res;
    }

    pub fn complement(&mut self) {
        self.intervals.insert(0, 0);
        self.intervals.push((char::MAX as u32) + 1);
    }

    pub fn complement_char(&mut self, c: char) {
        let code_point = c as u32;
        let to_complement = vec![code_point, code_point + 1];
        self.complement_list(to_complement);
    }

    pub fn complement_range(&mut self, range: &impl RangeBounds<char>) {
        let (start, end) = deconstruct_range(range);
        let to_complement = vec![start, end];
        self.complement_list(to_complement);
    }

    pub fn complement_set(&mut self, set: &UnicodeSet) {
        self.complement_list(set.get_invlist());
    }
}

#[cfg(test)]
mod tests {
    use super::{UnicodeSetBuilder, UnicodeSet};

    #[test]
    fn test_new() {
        let ex = UnicodeSetBuilder::new();
        assert!(ex.intervals.is_empty());
    }

    #[test]
    fn test_append() {
        let mut ex = UnicodeSetBuilder::new();
        ex.append(10, 20);
        assert_eq!(ex.intervals, vec![10, 20]);
    }

    #[test]
    fn test_prepend()  {
        let ex = vec![10, 20, 40, 50];
        let check = UnicodeSet::from_inversion_list(ex.clone()).unwrap();
        let mut builder = UnicodeSetBuilder::new();
        builder.add_set(&check);
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
}