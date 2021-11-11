// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::options::Width;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};

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
/// ", " - start - middle
///            |-- end - pair
///            |     \ short_end - short_pair
///            |               \ narrow_end - narrow_pair
///             \ short_start - short_middle
///                         \ narrow_start <- narrow_middle
#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ListFormatterPatternsV1<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    start: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    middle: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    end: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pair: Option<ConditionalListJoinerPattern<'data>>,

    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    short_start: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    short_middle: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    short_end: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    short_pair: Option<ConditionalListJoinerPattern<'data>>,

    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    narrow_start: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    narrow_middle: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    narrow_end: Option<ConditionalListJoinerPattern<'data>>,
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    narrow_pair: Option<ConditionalListJoinerPattern<'data>>,
}

macro_rules! fallback {
    ( $from:ident, $to:expr  ) => {
        if $from == $to {
            None
        } else {
            Some($from)
        }
    };
}
macro_rules! or {
    ($from:expr $(,$to:expr)*) => {
        {
            #[allow(unused_mut)]
            let mut res = $from.as_ref();
            $(
                res = res.or($to.as_ref());
            )*
            res.unwrap_or_default()
        }
    };
}
impl<'data> ListFormatterPatternsV1<'data> {
    #[allow(clippy::too_many_arguments)] // same as constructor
    pub fn new(
        start: ConditionalListJoinerPattern<'data>,
        middle: ConditionalListJoinerPattern<'data>,
        end: ConditionalListJoinerPattern<'data>,
        pair: ConditionalListJoinerPattern<'data>,
        short_start: ConditionalListJoinerPattern<'data>,
        short_middle: ConditionalListJoinerPattern<'data>,
        short_end: ConditionalListJoinerPattern<'data>,
        short_pair: ConditionalListJoinerPattern<'data>,
        narrow_start: ConditionalListJoinerPattern<'data>,
        narrow_middle: ConditionalListJoinerPattern<'data>,
        narrow_end: ConditionalListJoinerPattern<'data>,
        narrow_pair: ConditionalListJoinerPattern<'data>,
    ) -> Self {
        Self {
            narrow_pair: fallback!(narrow_pair, narrow_end),
            narrow_end: fallback!(narrow_end, short_end),
            narrow_middle: fallback!(narrow_middle, narrow_start),
            narrow_start: fallback!(narrow_start, short_start),

            short_pair: fallback!(short_pair, short_end),
            short_end: fallback!(short_end, end),
            short_middle: fallback!(short_middle, short_start),
            short_start: fallback!(short_start, start),

            pair: fallback!(pair, end),
            end: fallback!(end, start),
            middle: fallback!(middle, start),
            start: fallback!(start, *<&ConditionalListJoinerPattern>::default()),
        }
    }

    pub fn start(&self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        match width {
            Width::Wide => or!(self.start),
            Width::Short => or!(self.short_start, self.start),
            Width::Narrow => or!(self.narrow_start, self.short_start, self.start),
        }
    }

    pub fn middle(&self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        match width {
            Width::Wide => or!(self.middle, self.start),
            Width::Short => or!(self.short_middle, self.short_start, self.start),
            Width::Narrow => or!(
                self.narrow_middle,
                self.narrow_start,
                self.short_start,
                self.start
            ),
        }
    }

    pub fn end(&self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        match width {
            Width::Wide => or!(self.end, self.start),
            Width::Short => or!(self.short_end, self.end, self.start),
            Width::Narrow => {
                or!(self.narrow_end, self.short_end, self.end, self.start)
            }
        }
    }

    pub fn pair(&self, width: Width) -> &ConditionalListJoinerPattern<'data> {
        match width {
            Width::Wide => or!(self.pair, self.end, self.start),
            Width::Short => {
                or!(self.short_pair, self.short_end, self.end, self.start)
            }
            Width::Narrow => or!(
                self.narrow_pair,
                self.narrow_end,
                self.short_end,
                self.end,
                self.start
            ),
        }
    }
}

/// A pattern that can behave conditionally on the next element.
#[derive(Debug, PartialEq, Eq, Clone, Yokeable, ZeroCopyFrom)]
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

impl<'data> Default for &ConditionalListJoinerPattern<'data> {
    fn default() -> &'static ConditionalListJoinerPattern<'data> {
        &ConditionalListJoinerPattern {
            default: ListJoinerPattern {
                string: Cow::Borrowed(", "),
                index_1: 2,
            },
            special_case: None,
        }
    }
}

/// A pattern that can behave conditionally on the next element.
#[derive(Debug, PartialEq, Eq, Clone, Yokeable, ZeroCopyFrom)]
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
#[derive(Debug, PartialEq, Eq, Clone, Yokeable, ZeroCopyFrom)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
struct ListJoinerPattern<'data> {
    /// The pattern string without the placeholders
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
                (Some(0), Some(index_1)) if index_1 - 3 < 256 => Ok(ListJoinerPattern {
                    string: Cow::Owned(pattern[3..index_1].to_string() + &pattern[index_1 + 3..]),
                    index_1: (index_1 - 3) as u8,
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
        let pattern = ConditionalListJoinerPattern::from_str("{0}a{1}b").unwrap();
        assert_eq!(pattern.parts(""), ("a", "b"));
    }

    #[test]
    fn produces_correct_parts_conditionally() {
        let pattern =
            ConditionalListJoinerPattern::from_regex_and_strs("b.*", "{0}c{1}d", "{0}a{1}b")
                .unwrap();
        assert_eq!(pattern.parts("a"), ("a", "b"));
        assert_eq!(pattern.parts("b"), ("c", "d"));
    }

    #[test]
    fn fallbacks_work() {
        let comma = <&ConditionalListJoinerPattern>::default().clone();
        let period = ConditionalListJoinerPattern::from_str("{0}. {1}").unwrap();
        let semicolon = ConditionalListJoinerPattern::from_str("{0}; {1}").unwrap();
        let colon = ConditionalListJoinerPattern::from_str("{0}: {1}").unwrap();

        // Different fields are returned correctly
        let pattern = ListFormatterPatternsV1::new(
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
        );
        assert_eq!(pattern.start(Width::Wide), &comma);
        assert_eq!(pattern.middle(Width::Wide), &period);
        assert_eq!(pattern.end(Width::Wide), &semicolon);
        assert_eq!(pattern.pair(Width::Wide), &colon);

        // Same fields are returned correctly
        let pattern = ListFormatterPatternsV1::new(
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
            comma.clone(),
        );
        assert_eq!(pattern.start(Width::Wide), &comma);
        assert_eq!(pattern.middle(Width::Wide), &comma);
        assert_eq!(pattern.end(Width::Wide), &comma);
        assert_eq!(pattern.pair(Width::Wide), &comma);

        // Pair/end fallback works correctly
        let pattern = ListFormatterPatternsV1::new(
            comma.clone(),
            comma.clone(),
            period.clone(),
            period.clone(),
            comma.clone(),
            comma.clone(),
            period.clone(),
            period.clone(),
            comma.clone(),
            comma.clone(),
            period.clone(),
            period.clone(),
        );
        assert_eq!(pattern.start(Width::Wide), &comma);
        assert_eq!(pattern.middle(Width::Wide), &comma);
        assert_eq!(pattern.end(Width::Wide), &period);
        assert_eq!(pattern.pair(Width::Wide), &period);
    }
}
