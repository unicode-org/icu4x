// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::lazy_automaton::LazyAutomaton;
use crate::provider::*;
use crate::ListLength;
#[cfg(feature = "datagen")]
use alloc::borrow::Cow;
#[cfg(feature = "datagen")]
use icu_provider::DataError;
use writeable::{LengthHint, Writeable};

impl<'data> ListFormatterPatternsV1<'data> {
    /// Creates a new [`ListFormatterPatternsV1`] from the given patterns. Fails if any pattern is invalid.
    ///
    /// See [`ListJoinerPattern::from_str`]. `allow_prefix` will be true for `pair` and `end` patterns,
    /// `allow_suffix` for `start` and `pair` patterns.
    #[cfg(feature = "datagen")]
    pub fn try_new(
        [start, middle, end, pair, short_start, short_middle, short_end, short_pair, narrow_start, narrow_middle, narrow_end, narrow_pair]: [&str; 12],
    ) -> Result<Self, DataError> {
        Ok(Self([
            ListJoinerPattern::from_str(start, true, false)?.into(),
            ListJoinerPattern::from_str(middle, false, false)?.into(),
            ListJoinerPattern::from_str(end, false, true)?.into(),
            ListJoinerPattern::from_str(pair, true, true)?.into(),
            ListJoinerPattern::from_str(short_start, true, false)?.into(),
            ListJoinerPattern::from_str(short_middle, false, false)?.into(),
            ListJoinerPattern::from_str(short_end, false, true)?.into(),
            ListJoinerPattern::from_str(short_pair, true, true)?.into(),
            ListJoinerPattern::from_str(narrow_start, true, false)?.into(),
            ListJoinerPattern::from_str(narrow_middle, false, false)?.into(),
            ListJoinerPattern::from_str(narrow_end, false, true)?.into(),
            ListJoinerPattern::from_str(narrow_pair, true, true)?.into(),
        ]))
    }

    /// Adds a special case to all `pattern`s that will evaluate to
    /// `alternative_pattern` when `regex` matches the following element.
    /// The regex is interpreted case-insensitive and anchored to the beginning, but
    /// to improve efficiency does not search for full matches. If a full match is
    /// required, use `$`.
    #[cfg(feature = "datagen")]
    pub fn make_conditional(
        &mut self,
        pattern: &str,
        regex: &SerdeDFA<'static>,
        alternative_pattern: &str,
    ) -> Result<(), DataError> {
        let old = ListJoinerPattern::from_str(pattern, true, true)?;
        for i in 0..12 {
            #[allow(clippy::indexing_slicing)] // self.0 is &[_; 12]
            if self.0[i].default == old {
                self.0[i].special_case = Some(SpecialCasePattern {
                    condition: regex.clone(),
                    pattern: ListJoinerPattern::from_str(
                        alternative_pattern,
                        i % 4 == 0 || i % 4 == 3, // allow_prefix = start or pair
                        i % 4 == 2 || i % 4 == 3, // allow_suffix = end or pair
                    )?,
                });
            }
        }
        Ok(())
    }

    /// The range of the number of bytes required by the list literals to join a
    /// list of length `len`. If none of the patterns are conditional, this is exact.
    pub(crate) fn size_hint(&self, style: ListLength, len: usize) -> LengthHint {
        match len {
            0 | 1 => LengthHint::exact(0),
            2 => self.pair(style).size_hint(),
            n => {
                self.start(style).size_hint()
                    + self.middle(style).size_hint() * (n - 3)
                    + self.end(style).size_hint()
            }
        }
    }
}

type PatternParts<'a> = (&'a str, &'a str, &'a str);

impl<'a> ConditionalListJoinerPattern<'a> {
    pub(crate) fn parts<'b, W: Writeable + ?Sized>(
        &'a self,
        following_value: &'b W,
    ) -> PatternParts<'a> {
        match &self.special_case {
            Some(SpecialCasePattern { condition, pattern })
                if condition.deref().matches_earliest_fwd_lazy(following_value) =>
            {
                pattern.borrow_tuple()
            }
            _ => self.default.borrow_tuple(),
        }
    }

    /// The expected length of this pattern
    fn size_hint(&'a self) -> LengthHint {
        let mut hint = self.default.size_hint();
        if let Some(special_case) = &self.special_case {
            hint |= special_case.pattern.size_hint()
        }
        hint
    }
}

impl<'data> ListJoinerPattern<'data> {
    /// Construct the pattern from a CLDR pattern string
    #[cfg(feature = "datagen")]
    pub fn from_str(
        pattern: &str,
        allow_prefix: bool,
        allow_suffix: bool,
    ) -> Result<Self, DataError> {
        match (pattern.find("{0}"), pattern.find("{1}")) {
            (Some(index_0), Some(index_1))
                if index_0 < index_1
                    && (allow_prefix || index_0 == 0)
                    && (allow_suffix || index_1 == pattern.len() - 3) =>
            {
                if (index_0 > 0 && !cfg!(test)) || index_1 - 3 >= 256 {
                    return Err(DataError::custom(
                        "Found valid pattern that cannot be stored in ListFormatterPatternsV1",
                    )
                    .with_debug_context(pattern));
                }
                #[allow(clippy::indexing_slicing)] // find
                Ok(ListJoinerPattern {
                    string: Cow::Owned(alloc::format!(
                        "{}{}{}",
                        &pattern[0..index_0],
                        &pattern[index_0 + 3..index_1],
                        &pattern[index_1 + 3..]
                    )),
                    index_0: index_0 as u8,
                    index_1: (index_1 - 3) as u8,
                })
            }
            _ => Err(DataError::custom("Invalid list pattern").with_debug_context(pattern)),
        }
    }

    fn borrow_tuple(&'data self) -> PatternParts<'data> {
        #![allow(clippy::indexing_slicing)] // by invariant
        let index_0 = self.index_0 as usize;
        let index_1 = self.index_1 as usize;
        (
            &self.string[0..index_0],
            &self.string[index_0..index_1],
            &self.string[index_1..],
        )
    }

    fn size_hint(&self) -> LengthHint {
        LengthHint::exact(self.string.len())
    }
}

#[cfg(feature = "datagen")]
impl<'data> From<ListJoinerPattern<'data>> for ConditionalListJoinerPattern<'data> {
    fn from(default: ListJoinerPattern<'data>) -> Self {
        Self {
            default,
            special_case: None,
        }
    }
}

#[cfg(all(test, feature = "datagen"))]
pub mod test {
    use super::*;

    pub fn test_patterns() -> ListFormatterPatternsV1<'static> {
        let mut patterns = ListFormatterPatternsV1::try_new([
            // Wide: general
            "@{0}:{1}",
            "{0},{1}",
            "{0}.{1}!",
            "${0};{1}+",
            // Short: different pattern lengths
            "{0}1{1}",
            "{0}12{1}",
            "{0}12{1}34",
            "{0}123{1}456",
            // Narrow: conditionals
            "{0}: {1}",
            "{0}, {1}",
            "{0}. {1}",
            "{0}. {1}",
        ])
        .unwrap();
        patterns
            .make_conditional(
                "{0}. {1}",
                &SerdeDFA::new(Cow::Borrowed("A")).unwrap(),
                "{0} :o {1}",
            )
            .unwrap();
        patterns
    }

    #[test]
    fn rejects_bad_patterns() {
        assert!(ListJoinerPattern::from_str("{0} and", true, true).is_err());
        assert!(ListJoinerPattern::from_str("and {1}", true, true).is_err());
        assert!(ListJoinerPattern::from_str("{1} and {0}", true, true).is_err());
        assert!(ListJoinerPattern::from_str("{1{0}}", true, true).is_err());
        assert!(ListJoinerPattern::from_str("{0\u{202e}} and {1}", true, true).is_err());
        assert!(ListJoinerPattern::from_str("{{0}} {{1}}", true, true).is_ok());

        assert!(ListJoinerPattern::from_str("{0} and {1} ", true, true).is_ok());
        assert!(ListJoinerPattern::from_str("{0} and {1} ", true, false).is_err());
        assert!(ListJoinerPattern::from_str(" {0} and {1}", true, true).is_ok());
        assert!(ListJoinerPattern::from_str(" {0} and {1}", false, true).is_err());
    }

    #[test]
    fn produces_correct_parts() {
        assert_eq!(
            test_patterns().pair(ListLength::Wide).parts(""),
            ("$", ";", "+")
        );
    }

    #[test]
    fn produces_correct_parts_conditionally() {
        assert_eq!(
            test_patterns().end(ListLength::Narrow).parts("A"),
            ("", " :o ", "")
        );
        assert_eq!(
            test_patterns().end(ListLength::Narrow).parts("a"),
            ("", " :o ", "")
        );
        assert_eq!(
            test_patterns().end(ListLength::Narrow).parts("ab"),
            ("", " :o ", "")
        );
        assert_eq!(
            test_patterns().end(ListLength::Narrow).parts("B"),
            ("", ". ", "")
        );
        assert_eq!(
            test_patterns().end(ListLength::Narrow).parts("BA"),
            ("", ". ", "")
        );
    }

    #[test]
    fn size_hint_works() {
        let pattern = test_patterns();

        assert_eq!(
            pattern.size_hint(ListLength::Short, 0),
            LengthHint::exact(0)
        );
        assert_eq!(
            pattern.size_hint(ListLength::Short, 1),
            LengthHint::exact(0)
        );

        // pair pattern "{0}123{1}456"
        assert_eq!(
            pattern.size_hint(ListLength::Short, 2),
            LengthHint::exact(6)
        );

        // patterns "{0}1{1}", "{0}12{1}" (x197), and "{0}12{1}34"
        assert_eq!(
            pattern.size_hint(ListLength::Short, 200),
            LengthHint::exact(1 + 2 * 197 + 4)
        );

        // patterns "{0}: {1}", "{0}, {1}" (x197), and "{0} :o {1}" or "{0}. {1}"
        assert_eq!(
            pattern.size_hint(ListLength::Narrow, 200),
            LengthHint::exact(2 + 197 * 2) + LengthHint::between(2, 4)
        );
    }
}
