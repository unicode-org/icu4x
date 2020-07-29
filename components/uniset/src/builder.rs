use crate::uniset::UnicodeSet;

use std::{char, cmp};

##[derive(Default)]
pub struct UnicodeSetBuilder {
    intervals: Vec<u32>,
}

impl UnicodeSetBuilder {
    pub fn new() -> UnicodeSetBuilder {
        UnicodeSetBuilder { intervals: vec![] }
    }

    pub fn build(self) -> UnicodeSet {
        UnicodeSet::from_inversion_list(self.intervals).unwrap()
    }

    fn append(&mut self, start: u32, end: u32) {
        self.intervals.push(start);
        self.intervals.push(end);
    }

    fn prepend(&mut self, start: u32, end: u32) {
        let mut new = vec![start, end];
        new.append(&mut self.intervals);
        self.intervals = new;
    }

    pub fn add(&mut self, start: u32, end: u32) {
        if start >= end {
            return;
        }
        if self.intervals.is_empty() {
            self.append(start, end);
        } else {
            let last_ind = self.intervals.len() - 1;
            if self.intervals[last_ind - 1] <= start {
                // if the last interval is merged
                if self.intervals[last_ind] < start {
                    self.append(start, end);
                } else {
                    self.intervals[last_ind] = cmp::max(end, self.intervals[last_ind]);
                }
            } else if end <= self.intervals[1] {
                // if the first interval is merged
                if end < self.intervals[0] {
                    // worst case
                    self.prepend(start, end);
                } else {
                    // end > self.intervals[0] && end <= self.intervals[1]
                    self.intervals[0] = cmp::min(self.intervals[0], start);
                }
            } else {
                // the interval is somewhere inbetween everything
                let mut s = self.intervals.binary_search(&start).unwrap_or_else(|x| x);
                let mut e = self.intervals.binary_search(&end).unwrap_or_else(|x| x);
                if s % 2 == 0 {
                    self.intervals[s] = cmp::min(self.intervals[s], start);
                } else {
                    s -= 1; // then we consume it
                }
                if e % 2 == 0 {
                    self.intervals[e - 1] = cmp::max(self.intervals[e - 1], end);
                } else {
                    e += 1;
                }
                self.intervals.drain((s + 1)..(e - 1));
            }
        }
    }

    pub fn remove(&mut self, start: u32, end: u32) {
        if start < end && !self.intervals.is_empty() {
            let s = self.intervals.binary_search(&start).unwrap_or_else(|x| x);
            let last = self.intervals.binary_search(&end);
            let e = last.unwrap_or_else(|x| x - 1) + 1;
            if s % 2 != 0 && e % 2 == 0 {
                let mut insert = vec![start, end];
                if let Ok(_) = last {
                    insert.pop();
                }
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

    pub fn retain(&mut self, ret: Vec<u32>) {
        let mut chunk = vec![0, (char::MAX as u32) + 1];
        chunk.splice(1..1, ret);
        chunk.chunks(2).for_each(|c| self.remove(c[0], c[1]));
    }

    pub fn complement(&mut self) {
        self.intervals.insert(0, 0);
        self.intervals.push((char::MAX as u32) + 1);
    }

    pub fn complement_set(&mut self, set: Vec<u32>) {
        let mut res: Vec<u32> = vec![];
        let mut i = 0;
        let mut j = 0;
        loop {
            if i == self.intervals.len() || j == set.len() {
                break;
            }
            if self.intervals[i] < set[j] {
                res.push(self.intervals[i]);
                i += 1;
            } else if self.intervals[i] > set[j] {
                res.push(set[j]);
                j += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
        while i < self.intervals.len() {
            res.push(self.intervals[i]);
            i += 1;
        }
        while j < set.len() {
            res.push(set[j]);
            j += 1;
        }
        self.intervals = res;
    }
}
