// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module defines implementations for code-based transliterators that are part of
//! transform rules.

use crate::transliterator::replaceable::{Forward, RepMatcher, Replaceable, Utf8Matcher};
use core::fmt::Write;

/// A transliterator that replaces every character with its `case`-case hexadecimal representation,
/// 0-padded to `min_length`, and surrounded by `prefix` and `suffix`.
#[derive(Debug)]
pub(super) struct HexTransliterator {
    prefix: &'static str,
    suffix: &'static str,
    min_length: u8,
    case: Case,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Case {
    Upper,
    Lower,
}

impl HexTransliterator {
    pub(super) fn new(
        prefix: &'static str,
        suffix: &'static str,
        min_length: u8,
        case: Case,
    ) -> Self {
        Self {
            prefix,
            suffix,
            min_length,
            case,
        }
    }

    pub(super) fn transliterate(&self, mut rep: Replaceable) {
        while !rep.is_finished() {
            let mut matcher = rep.start_match();
            // TODO: ok this is annoying, maybe separate functions are better for Forward vs Reverse matching.
            let c = <RepMatcher<false> as Utf8Matcher<Forward>>::next_char(&matcher);
            // there must always be a char, because we just checked that `rep` is not finished yet.
            let c = c.unwrap();
            <RepMatcher<false> as Utf8Matcher<Forward>>::match_and_consume_char(&mut matcher, c);
            let mut dest = matcher.finish_match();

            let c_u32 = c as u32;
            // computing length for the size hint
            let length = if c_u32 == 0 {
                1
            } else {
                let mut length = 0;
                let mut rem = c_u32;
                while rem != 0 {
                    length += 1;
                    rem >>= 4;
                }
                length
            };
            let length = length.max(self.min_length as u32);
            dest.apply_size_hint(length as usize + self.prefix.len() + self.suffix.len());

            if self.case == Case::Lower {
                write!(
                    dest,
                    "{prefix}{c_u32:0width$x}{suffix}",
                    prefix = self.prefix,
                    width = self.min_length as usize,
                    suffix = self.suffix
                )
                .unwrap();
            } else {
                write!(
                    dest,
                    "{prefix}{c_u32:0width$X}{suffix}",
                    prefix = self.prefix,
                    width = self.min_length as usize,
                    suffix = self.suffix
                )
                .unwrap();
            }
        }
    }
}
