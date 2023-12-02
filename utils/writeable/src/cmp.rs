// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::Ordering;
use core::fmt;

pub(crate) struct WriteComparator<'a> {
    string: &'a str,
    result: Ordering,
}

/// This is an infallible impl. Functions always return Ok, not Err.
impl<'a> fmt::Write for WriteComparator<'a> {
    fn write_char(&mut self, other: char) -> fmt::Result {
        if self.result != Ordering::Equal {
            return Ok(());
        }
        let Some(this) = self.pop_front() else {
            // Self is shorter than Other
            self.result = Ordering::Less;
            return Ok(());
        };
        self.result = this.cmp(&other);
        Ok(())
    }

    fn write_str(&mut self, other: &str) -> fmt::Result {
        if self.result != Ordering::Equal {
            return Ok(());
        }
        let this = if self.string.is_char_boundary(other.len()) {
            let (this, remainder) = self.string.split_at(other.len());
            self.string = remainder;
            this
        } else {
            let tmp = self.string;
            self.string = "";
            tmp
        };
        self.result = this.cmp(other);
        Ok(())
    }
}

impl<'a> WriteComparator<'a> {
    #[inline]
    pub fn new(string: &'a str) -> Self {
        Self {
            string,
            result: Ordering::Equal,
        }
    }

    #[inline]
    pub fn finish(self) -> Ordering {
        if matches!(self.result, Ordering::Equal) && !self.string.is_empty() {
            // Self is longer than Other
            Ordering::Greater
        } else {
            self.result
        }
    }

    #[inline]
    pub(crate) fn pop_front(&mut self) -> Option<char> {
        let mut chars = self.string.chars();
        let option_this = chars.next();
        self.string = chars.as_str();
        option_this
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Write;

    mod data {
        include!("../tests/data/data.rs");
    }

    #[test]
    fn test_pop_front() {
        let s = "aéi🧺øü";
        let mut wc = WriteComparator::new(s);
        assert_eq!(wc.pop_front(), Some('a'));
        assert_eq!(wc.pop_front(), Some('é'));
        assert_eq!(wc.pop_front(), Some('i'));
        assert_eq!(wc.pop_front(), Some('🧺'));
        assert_eq!(wc.pop_front(), Some('ø'));
        assert_eq!(wc.pop_front(), Some('ü'));
        assert_eq!(wc.pop_front(), None);
    }

    #[test]
    fn test_write_char() {
        for a in data::KEBAB_CASE_STRINGS {
            for b in data::KEBAB_CASE_STRINGS {
                let mut wc = WriteComparator::new(a);
                for ch in b.chars() {
                    wc.write_char(ch).unwrap();
                }
                assert_eq!(a.cmp(b), wc.finish(), "{a} <=> {b}");
            }
        }
    }

    #[test]
    fn test_write_str() {
        for a in data::KEBAB_CASE_STRINGS {
            for b in data::KEBAB_CASE_STRINGS {
                let mut wc = WriteComparator::new(a);
                wc.write_str(b).unwrap();
                assert_eq!(a.cmp(b), wc.finish(), "{a} <=> {b}");
            }
        }
    }

    #[test]
    fn test_mixed() {
        for a in data::KEBAB_CASE_STRINGS {
            for b in data::KEBAB_CASE_STRINGS {
                let mut wc = WriteComparator::new(a);
                let mut first = true;
                for substr in b.split('-') {
                    if first {
                        first = false;
                    } else {
                        wc.write_char('-').unwrap();
                    }
                    wc.write_str(substr).unwrap();
                }
                assert_eq!(a.cmp(b), wc.finish(), "{a} <=> {b}");
            }
        }
    }
}
