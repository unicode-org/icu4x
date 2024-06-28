// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::cmp::Ordering;
use core::fmt;

pub(crate) struct WriteComparator<'a> {
    code_points: &'a [u8],
    result: Ordering,
}

/// This is an infallible impl. Functions always return Ok, not Err.
impl<'a> fmt::Write for WriteComparator<'a> {
    #[inline]
    fn write_str(&mut self, other: &str) -> fmt::Result {
        if self.result != Ordering::Equal {
            return Ok(());
        }
        let cmp_len = core::cmp::min(other.len(), self.code_points.len());
        let (this, remainder) = self.code_points.split_at(cmp_len);
        self.code_points = remainder;
        self.result = this.cmp(other.as_bytes());
        Ok(())
    }
}

impl<'a> WriteComparator<'a> {
    #[inline]
    pub fn new(code_points: &'a [u8]) -> Self {
        Self {
            code_points,
            result: Ordering::Equal,
        }
    }

    #[inline]
    pub fn finish(self) -> Ordering {
        if matches!(self.result, Ordering::Equal) && !self.code_points.is_empty() {
            // Self is longer than Other
            Ordering::Greater
        } else {
            self.result
        }
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
    fn test_write_char() {
        for a in data::KEBAB_CASE_STRINGS {
            for b in data::KEBAB_CASE_STRINGS {
                let mut wc = WriteComparator::new(a.as_bytes());
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
                let mut wc = WriteComparator::new(a.as_bytes());
                wc.write_str(b).unwrap();
                assert_eq!(a.cmp(b), wc.finish(), "{a} <=> {b}");
            }
        }
    }

    #[test]
    fn test_mixed() {
        for a in data::KEBAB_CASE_STRINGS {
            for b in data::KEBAB_CASE_STRINGS {
                let mut wc = WriteComparator::new(a.as_bytes());
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
