// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::ListStyle;
use alloc::borrow::Cow;
use icu_provider::DataMarker;
use icu_provider::{yoke, zerofrom};
use writeable::{LengthHint, Writeable};

pub use crate::string_matcher::StringMatcher;

/// Symbols and metadata required for [`ListFormatter`](crate::ListFormatter).
#[icu_provider::data_struct(
    AndListV1Marker = "list/and@1",
    OrListV1Marker = "list/or@1",
    UnitListV1Marker = "list/unit@1"
)]
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_list::provider),
)]
pub struct ListFormatterPatternsV1<'data>(
    #[cfg_attr(feature = "datagen", serde(with = "deduplicating_array"))]
    /// The patterns in the order start, middle, end, pair, short_start, short_middle,
    /// short_end, short_pair, narrow_start, narrow_middle, narrow_end, narrow_pair,
    pub [ConditionalListJoinerPattern<'data>; 12],
);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ListFormatterPatternsV1<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[cfg(not(feature = "serde_human"))]
        if deserializer.is_human_readable() {
            use serde::de::Error;
            return Err(D::Error::custom(
                    "Deserializing human-readable ListFormatter data requires the 'serde_human' feature",
                ));
        }

        Ok(ListFormatterPatternsV1(deduplicating_array::deserialize(
            deserializer,
        )?))
    }
}

pub(crate) struct ErasedListV1Marker;

impl DataMarker for ErasedListV1Marker {
    type Yokeable = ListFormatterPatternsV1<'static>;
}

impl<'data> ListFormatterPatternsV1<'data> {
    pub(crate) fn start(&self, style: ListStyle) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize)]
    }

    pub(crate) fn middle(&self, style: ListStyle) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize) + 1]
    }

    pub(crate) fn end(&self, style: ListStyle) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize) + 2]
    }

    pub(crate) fn pair(&self, style: ListStyle) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize) + 3]
    }

    /// The range of the number of bytes required by the list literals to join a
    /// list of length `len`. If none of the patterns are conditional, this is exact.
    pub(crate) fn size_hint(&self, style: ListStyle, len: usize) -> LengthHint {
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

/// A pattern that can behave conditionally on the next element.
#[derive(Clone, Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_list::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ConditionalListJoinerPattern<'data> {
    /// The default pattern
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub default: ListJoinerPattern<'data>,
    /// And optional special case
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub special_case: Option<SpecialCasePattern<'data>>,
}

/// The special case of a [`ConditionalListJoinerPattern`]
#[derive(Clone, Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_list::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SpecialCasePattern<'data> {
    /// The condition on the following element
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub condition: StringMatcher<'data>,
    /// The pattern if the condition matches
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pattern: ListJoinerPattern<'data>,
}

/// A pattern containing two numeric placeholders ("{0}, and {1}.")
#[derive(Clone, Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
pub struct ListJoinerPattern<'data> {
    /// The pattern string without the placeholders
    string: Cow<'data, str>,
    /// The index of the first placeholder. Always <= index_1.
    // Always 0 for CLDR data, so we don't need to serialize it.
    // In-memory we have free space for it as index_1 doesn't
    // fill a word.
    #[cfg_attr(feature = "datagen", serde(skip))]
    index_0: u8,
    /// The index of the second placeholder. Always < string.len().
    index_1: u8,
}

#[cfg(feature = "serde")]
impl<'de: 'data, 'data> serde::Deserialize<'de> for ListJoinerPattern<'data> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Dummy<'data> {
            #[cfg_attr(feature = "serde", serde(borrow))]
            string: Cow<'data, str>,
            index_1: u8,
        }
        let Dummy { string, index_1 } = Dummy::deserialize(deserializer)?;

        if index_1 as usize > string.len() {
            use serde::de::Error;
            Err(D::Error::custom("invalid index_1"))
        } else {
            Ok(ListJoinerPattern {
                string,
                index_0: 0,
                index_1,
            })
        }
    }
}

impl<'a> ListJoinerPattern<'a> {
    /// Constructs a [`ListJoinerPattern`] from raw parts. Used by databake.
    ///
    /// # Safety
    /// index_1 may be at most string.len()
    pub const unsafe fn from_parts_unchecked(string: &'a str, index_1: u8) -> Self {
        Self {
            string: Cow::Borrowed(string),
            index_0: 0,
            index_1,
        }
    }
}

pub(crate) type PatternParts<'a> = (&'a str, &'a str, &'a str);

impl<'a> ConditionalListJoinerPattern<'a> {
    pub(crate) fn parts<'b, W: Writeable + ?Sized>(
        &'a self,
        following_value: &'b W,
    ) -> PatternParts<'a> {
        match &self.special_case {
            Some(SpecialCasePattern { condition, pattern })
                // TODO: Implement lookahead instead of materializing here.
                if condition.test(&*following_value.write_to_string()) =>
            {
                pattern.borrow_tuple()
            }
            _ => self.default.borrow_tuple(),
        }
    }

    /// The expected length of this pattern
    pub fn size_hint(&'a self) -> LengthHint {
        let mut hint = self.default.size_hint();
        if let Some(special_case) = &self.special_case {
            hint |= special_case.pattern.size_hint()
        }
        hint
    }
}

impl<'data> ListJoinerPattern<'data> {
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
mod datagen {
    #![allow(clippy::indexing_slicing)] // datagen

    use super::*;
    use icu_provider::DataError;

    impl<'data> ListFormatterPatternsV1<'data> {
        /// The patterns in the order start, middle, end, pair, short_start, short_middle,
        /// short_end, short_pair, narrow_start, narrow_middle, narrow_end, narrow_pair,
        pub fn try_new(patterns: [&str; 12]) -> Result<Self, DataError> {
            Ok(Self([
                ListJoinerPattern::from_str(patterns[0], true, false)?.into(),
                ListJoinerPattern::from_str(patterns[1], false, false)?.into(),
                ListJoinerPattern::from_str(patterns[2], false, true)?.into(),
                ListJoinerPattern::from_str(patterns[3], true, true)?.into(),
                ListJoinerPattern::from_str(patterns[4], true, false)?.into(),
                ListJoinerPattern::from_str(patterns[5], false, false)?.into(),
                ListJoinerPattern::from_str(patterns[6], false, true)?.into(),
                ListJoinerPattern::from_str(patterns[7], true, true)?.into(),
                ListJoinerPattern::from_str(patterns[8], true, false)?.into(),
                ListJoinerPattern::from_str(patterns[9], false, false)?.into(),
                ListJoinerPattern::from_str(patterns[10], false, true)?.into(),
                ListJoinerPattern::from_str(patterns[11], true, true)?.into(),
            ]))
        }

        /// Adds a special case to all `pattern`s that will evaluate to
        /// `alternative_pattern` when `regex` matches the following element.
        /// The regex is interpreted case-insensitive and anchored to the beginning, but
        /// to improve efficiency does not search for full matches. If a full match is
        /// required, use `$`.
        pub fn make_conditional(
            &mut self,
            pattern: &str,
            regex: &StringMatcher<'static>,
            alternative_pattern: &str,
        ) -> Result<(), DataError> {
            let old = ListJoinerPattern::from_str(pattern, true, true)?;
            for i in 0..12 {
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
    }

    impl<'data> ListJoinerPattern<'data> {
        /// Construct the pattern from a CLDR pattern string
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
    }

    impl<'data> From<ListJoinerPattern<'data>> for ConditionalListJoinerPattern<'data> {
        fn from(default: ListJoinerPattern<'data>) -> Self {
            Self {
                default,
                special_case: None,
            }
        }
    }

    impl databake::Bake for ListJoinerPattern<'_> {
        fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
            env.insert("icu_list");
            let string = (&*self.string).bake(env);
            let index_1 = self.index_1.bake(env);
            // Safe because our own data is safe
            databake::quote! { unsafe {
                ::icu_list::provider::ListJoinerPattern::from_parts_unchecked(#string, #index_1)
            }}
        }
    }
}

#[cfg(all(test, feature = "datagen"))]
pub(crate) mod test {
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
            .make_conditional("{0}. {1}", &StringMatcher::new("A").unwrap(), "{0} :o {1}")
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
            test_patterns().pair(ListStyle::Wide).parts(""),
            ("$", ";", "+")
        );
    }

    #[test]
    fn produces_correct_parts_conditionally() {
        assert_eq!(
            test_patterns().end(ListStyle::Narrow).parts("A"),
            ("", " :o ", "")
        );
        assert_eq!(
            test_patterns().end(ListStyle::Narrow).parts("a"),
            ("", " :o ", "")
        );
        assert_eq!(
            test_patterns().end(ListStyle::Narrow).parts("ab"),
            ("", " :o ", "")
        );
        assert_eq!(
            test_patterns().end(ListStyle::Narrow).parts("B"),
            ("", ". ", "")
        );
        assert_eq!(
            test_patterns().end(ListStyle::Narrow).parts("BA"),
            ("", ". ", "")
        );
    }

    #[test]
    fn size_hint_works() {
        let pattern = test_patterns();

        assert_eq!(pattern.size_hint(ListStyle::Short, 0), LengthHint::exact(0));
        assert_eq!(pattern.size_hint(ListStyle::Short, 1), LengthHint::exact(0));

        // pair pattern "{0}123{1}456"
        assert_eq!(pattern.size_hint(ListStyle::Short, 2), LengthHint::exact(6));

        // patterns "{0}1{1}", "{0}12{1}" (x197), and "{0}12{1}34"
        assert_eq!(
            pattern.size_hint(ListStyle::Short, 200),
            LengthHint::exact(1 + 2 * 197 + 4)
        );

        // patterns "{0}: {1}", "{0}, {1}" (x197), and "{0} :o {1}" or "{0}. {1}"
        assert_eq!(
            pattern.size_hint(ListStyle::Narrow, 200),
            LengthHint::exact(2 + 197 * 2) + LengthHint::between(2, 4)
        );
    }

    #[test]
    fn databake() {
        databake::test_bake!(
            ListJoinerPattern,
            const: unsafe { crate::provider::ListJoinerPattern::from_parts_unchecked(", ", 2u8) },
            icu_list
        );
    }
}
