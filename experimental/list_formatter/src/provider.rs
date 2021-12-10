// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

#[cfg(any(test, feature = "provider_transform_internals"))]
use crate::error::Error;
use crate::options::Width;
use crate::string_matcher::StringMatcher;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
#[cfg(any(test, feature = "provider_transform_internals"))]
use std::str::FromStr;

pub mod key {
    //! Resource keys for [`list_formatter`](crate).
    use icu_provider::{resource_key, ResourceKey};

    // Resource key: symbols used for list formatting.
    pub const LIST_FORMAT_AND_V1: ResourceKey = resource_key!(ListFormatter, "and", 1);
    pub const LIST_FORMAT_OR_V1: ResourceKey = resource_key!(ListFormatter, "or", 1);
    pub const LIST_FORMAT_UNIT_V1: ResourceKey = resource_key!(ListFormatter, "unit", 1);
}

/// Symbols and metadata required for [`ListFormatter`](crate::ListFormatter).
/// Absent values follow this fallback structure:
#[icu_provider::data_struct]
#[derive(Debug)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct ListFormatterPatternsV1<'data>(
    #[cfg_attr(
        feature = "provider_serde",
        serde(borrow, with = "crate::deduplicating_array")
    )]
    [ConditionalListJoinerPattern<'data>; 12],
);

impl<'data> ListFormatterPatternsV1<'data> {
    /// The patterns in the order start, middle, end, pair, short_start, short_middle,
    /// short_end, short_pair, narrow_start, narrow_middle, narrow_end, narrow_pair,
    pub fn new(patterns: [ConditionalListJoinerPattern<'data>; 12]) -> Self {
        Self(patterns)
    }

    pub fn replace_patterns(
        &mut self,
        old: ConditionalListJoinerPattern<'data>,
        new: ConditionalListJoinerPattern<'data>,
    ) {
        for i in 0..12 {
            if self.0[i] == old {
                self.0[i] = new.clone();
            }
        }
    }

    pub fn start(&'data self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        &self.0[4 * (width as usize)]
    }

    pub fn middle(&'data self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        &self.0[4 * (width as usize) + 1]
    }

    pub fn end(&'data self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        &self.0[4 * (width as usize) + 2]
    }

    pub fn pair(&'data self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        &self.0[4 * (width as usize) + 3]
    }
}

/// A pattern that can behave conditionally on the next element.
#[derive(Clone, Debug, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct ConditionalListJoinerPattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    default: ListJoinerPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    special_case: Option<SpecialCasePattern<'data>>,
}

/// A pattern that can behave conditionally on the next element.
#[derive(Clone, Debug, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Deserialize, serde::Serialize)
)]
struct SpecialCasePattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    condition: StringMatcher<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pattern: ListJoinerPattern<'data>,
}

/// A pattern containing two numeric placeholders ("{0}, and {1}.")
#[derive(Clone, Debug, PartialEq, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Deserialize, serde::Serialize)
)]
struct ListJoinerPattern<'data> {
    /// The pattern string without the placeholders
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    string: Cow<'data, str>,
    /// The index of the second placeholder
    index_1: u8,
}

pub type PatternParts<'a> = (&'a str, &'a str);

impl<'data> ListJoinerPattern<'data> {
    fn borrow_tuple(&'data self) -> PatternParts<'data> {
        (
            &self.string[0..self.index_1 as usize],
            &self.string[self.index_1 as usize..],
        )
    }
}

#[cfg(any(test, feature = "provider_transform_internals"))]
impl<'data> FromStr for ListJoinerPattern<'data> {
    type Err = Error;
    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
        match (pattern.find("{0}"), pattern.find("{1}")) {
            (Some(0), Some(index_1)) if index_1 - 3 < 256 => Ok(ListJoinerPattern {
                string: Cow::Owned(alloc::format!(
                    "{}{}",
                    &pattern[3..index_1],
                    &pattern[index_1 + 3..]
                )),
                index_1: (index_1 - 3) as u8,
            }),
            _ => Err(Error::IllegalPattern(pattern.into())),
        }
    }
}

#[cfg(any(test, feature = "provider_transform_internals"))]
impl<'data> FromStr for ConditionalListJoinerPattern<'data> {
    type Err = Error;
    fn from_str(pattern: &str) -> Result<Self, Self::Err> {
        Ok(ConditionalListJoinerPattern {
            default: ListJoinerPattern::from_str(pattern)?,
            special_case: None,
        })
    }
}

impl<'a> ConditionalListJoinerPattern<'a> {
    /// Creates a conditional list joiner that will evaluate to the `then_pattern` when
    /// `regex` matches the following element, and to `else_pattern` otherwise.
    /// The regex is interpreted case-insensitive and anchored to the beginning, but
    /// to improve efficiency does not search for full matches. If a full match is
    /// required, use `$`.
    #[cfg(any(test, feature = "provider_transform_internals"))]
    pub fn from_regex_and_strs(
        regex: &str,
        then_pattern: &str,
        else_pattern: &str,
    ) -> Result<Self, crate::error::Error> {
        Ok(ConditionalListJoinerPattern {
            default: ListJoinerPattern::from_str(else_pattern)?,
            special_case: Some(SpecialCasePattern {
                condition: StringMatcher::new(regex)?,
                pattern: ListJoinerPattern::from_str(then_pattern)?,
            }),
        })
    }

    pub fn parts(&'a self, following_value: &str) -> PatternParts<'a> {
        match &self.special_case {
            Some(SpecialCasePattern { condition, pattern }) if condition.test(following_value) => {
                pattern.borrow_tuple()
            }
            _ => self.default.borrow_tuple(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn produces_correct_parts() {
        let pattern = ConditionalListJoinerPattern::from_str("{0}a{1}b").unwrap();
        assert_eq!(pattern.parts(""), ("a", "b"));
    }

    #[test]
    fn produces_correct_parts_conditionally() {
        let pattern =
            ConditionalListJoinerPattern::from_regex_and_strs("b", "{0}c{1}d", "{0}a{1}b").unwrap();
        // Only matches at the beginning of the string
        assert_eq!(pattern.parts("ab"), ("a", "b"));
        // Doesn't require a full match
        assert_eq!(pattern.parts("ba"), ("c", "d"));
    }

    #[test]
    fn fallbacks_work() {
        let comma = ConditionalListJoinerPattern::from_str("{0}, {1}").unwrap();
        let period = ConditionalListJoinerPattern::from_str("{0}. {1}").unwrap();
        let semicolon = ConditionalListJoinerPattern::from_str("{0}; {1}").unwrap();
        let colon = ConditionalListJoinerPattern::from_str("{0}: {1}").unwrap();

        // Different fields are returned correctly
        let pattern = ListFormatterPatternsV1::new([
            comma.clone(),
            period.clone(),
            semicolon.clone(),
            colon.clone(),
            comma.clone(),
            period.clone(),
            semicolon.clone(),
            colon.clone(),
            comma.clone(),
            period.clone(),
            semicolon.clone(),
            colon.clone(),
        ]);
        assert_eq!(pattern.start(Width::Wide), &comma);
        assert_eq!(pattern.start(Width::Short), &comma);
        assert_eq!(pattern.start(Width::Narrow), &comma);
        assert_eq!(pattern.middle(Width::Wide), &period);
        assert_eq!(pattern.middle(Width::Short), &period);
        assert_eq!(pattern.middle(Width::Narrow), &period);
        assert_eq!(pattern.end(Width::Wide), &semicolon);
        assert_eq!(pattern.end(Width::Short), &semicolon);
        assert_eq!(pattern.end(Width::Narrow), &semicolon);
        assert_eq!(pattern.pair(Width::Wide), &colon);
        assert_eq!(pattern.pair(Width::Short), &colon);
        assert_eq!(pattern.pair(Width::Narrow), &colon);
    }
}
