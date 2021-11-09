// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::options::{Type, Width};
use alloc::borrow::Cow;
use core::ops::Index;
use icu_provider::yoke::{self, *};

pub mod key {
    //! Resource keys for [`list_formatter`](crate).
    use icu_provider::{resource_key, ResourceKey};

    // Resource key: symbols used for list formatting.
    pub const LIST_FORMAT_V1: ResourceKey = resource_key!(ListFormatter, "list", 1);
}

/// Symbols and metadata required for [`ListFormatter`](crate::ListFormatter).
#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ListFormatterPatternsV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub patterns: PatternTypes<'data>,
}

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PatternTypes<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub and: PatternSizes<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub or: PatternSizes<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub unit: PatternSizes<'data>,
}

impl<'data> Index<Type> for PatternTypes<'data> {
    type Output = PatternSizes<'data>;
    fn index(&self, type_: Type) -> &Self::Output {
        match type_ {
            Type::And => &self.and,
            Type::Or => &self.or,
            Type::Unit => &self.unit,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PatternSizes<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub wide: ListFormatterPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub short: ListFormatterPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub narrow: ListFormatterPattern<'data>,
}

impl<'data> Index<Width> for PatternSizes<'data> {
    type Output = ListFormatterPattern<'data>;
    fn index(&self, width: Width) -> &Self::Output {
        match width {
            Width::Wide => &self.wide,
            Width::Short => &self.short,
            Width::Narrow => &self.narrow,
        }
    }
}

/// A collection of patterns that are needed to join a list
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ListFormatterPattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub start: ConditionalListJoinerPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub middle: ConditionalListJoinerPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub end: ConditionalListJoinerPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub pair: ConditionalListJoinerPattern<'data>,
}

/// A pattern that can behave conditionally on the next element.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ConditionalListJoinerPattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    default: ListJoinerPattern<'data>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    special_case: Option<SpecialCasePattern<'data>>,
}

/// A pattern that can behave conditionally on the next element.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
struct SpecialCasePattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    condition: Cow<'data, str>, // TODO: Serialize a compiled regex instead
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pattern: ListJoinerPattern<'data>,
}

/// A pattern containing two numeric placeholders ("{0}, and {1}.")
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
struct ListJoinerPattern<'data> {
    /// The pattern string without the placeholders
    string: Cow<'data, str>,
    /// The indices of the two placeholders
    insertion_points: [usize; 2],
}

pub type PatternParts<'a> = (&'a str, &'a str, &'a str);

impl<'data> ListJoinerPattern<'data> {
    fn borrow_tuple(&'data self) -> PatternParts<'data> {
        (
            &self.string[0..self.insertion_points[0]],
            &self.string[self.insertion_points[0]..self.insertion_points[1]],
            &self.string[self.insertion_points[1]..],
        )
    }
}

impl<'a> ConditionalListJoinerPattern<'a> {
    pub fn parts(&'a self, following_value: &str) -> PatternParts<'a> {
        match &self.special_case {
            Some(SpecialCasePattern { condition, pattern })
                if regex::Regex::new(&*condition)
                    .unwrap()
                    .is_match(following_value) =>
            {
                pattern.borrow_tuple()
            }
            _ => self.default.borrow_tuple(),
        }
    }
}

#[cfg(any(test, feature = "provider_transform_internals"))]
pub mod pattern_construction {
    use super::*;
    use crate::error::Error;
    use core::str::FromStr;

    impl<'data> FromStr for ListJoinerPattern<'data> {
        type Err = Error;
        fn from_str(pattern: &str) -> Result<Self, Self::Err> {
            match (pattern.find("{0}"), pattern.find("{1}")) {
                (Some(index_0), Some(index_1)) if index_0 + 3 <= index_1 => Ok(ListJoinerPattern {
                    string: Cow::Owned(
                        pattern[0..index_0].to_string()
                            + &pattern[index_0 + 3..index_1]
                            + &pattern[index_1 + 3..],
                    ),
                    insertion_points: [index_0, index_1 - 3],
                }),
                _ => Err(Error::IllegalPattern(pattern.to_string())),
            }
        }
    }

    impl<'data> FromStr for ConditionalListJoinerPattern<'data> {
        type Err = Error;
        fn from_str(pattern: &str) -> Result<Self, Self::Err> {
            Ok(ConditionalListJoinerPattern {
                default: ListJoinerPattern::from_str(pattern)?,
                special_case: None,
            })
        }
    }

    impl<'data> ConditionalListJoinerPattern<'data> {
        pub fn from_regex_and_strs(
            regex: &str,
            then_pattern: &str,
            else_pattern: &str,
        ) -> Result<Self, crate::error::Error> {
            Ok(ConditionalListJoinerPattern {
                default: ListJoinerPattern::from_str(else_pattern)?,
                special_case: Some(SpecialCasePattern {
                    condition: Cow::Owned(regex.to_string()),
                    pattern: ListJoinerPattern::from_str(then_pattern)?,
                }),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn produces_correct_parts() {
        let pattern = ConditionalListJoinerPattern::from_str("a{0}b{1}c").unwrap();
        assert_eq!(pattern.parts(""), ("a", "b", "c"));
    }

    #[test]
    fn produces_correct_parts_conditionally() {
        let pattern =
            ConditionalListJoinerPattern::from_regex_and_strs("b.*", "c{0}d{1}e", "a{0}b{1}c")
                .unwrap();
        assert_eq!(pattern.parts("a"), ("a", "b", "c"));
        assert_eq!(pattern.parts("b"), ("c", "d", "e"));
    }
}
