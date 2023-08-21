// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::ListLength;
use alloc::borrow::Cow;
use icu_provider::prelude::*;
use icu_provider::DataMarker;

mod serde_dfa;
pub use serde_dfa::SerdeDFA;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
pub struct Baked;

#[cfg(feature = "compiled_data")]
const _: () = {
    pub mod icu {
        pub use crate as list;
        pub use icu_locid_transform as locid_transform;
    }
    icu_list_data::impl_list_and_v1!(Baked);
    icu_list_data::impl_list_or_v1!(Baked);
    icu_list_data::impl_list_unit_v1!(Baked);
};

/// Symbols and metadata required for [`ListFormatter`](crate::ListFormatter).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    AndListV1Marker = "list/and@1",
    OrListV1Marker = "list/or@1",
    UnitListV1Marker = "list/unit@1"
)]
#[derive(Clone, Debug, PartialEq)]
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
    pub(crate) fn start(&self, style: ListLength) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize)]
    }

    pub(crate) fn middle(&self, style: ListLength) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize) + 1]
    }

    pub(crate) fn end(&self, style: ListLength) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize) + 2]
    }

    pub(crate) fn pair(&self, style: ListLength) -> &ConditionalListJoinerPattern<'data> {
        #![allow(clippy::indexing_slicing)] // style as usize < 3
        &self.0[4 * (style as usize) + 3]
    }
}

/// A pattern that can behave conditionally on the next element.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
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
    #[cfg_attr(
        feature = "serde",
        serde(borrow, deserialize_with = "SpecialCasePattern::deserialize_option")
    )]
    pub special_case: Option<SpecialCasePattern<'data>>,
}

/// The special case of a [`ConditionalListJoinerPattern`]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_list::provider),
)]
pub struct SpecialCasePattern<'data> {
    /// The condition on the following element
    pub condition: SerdeDFA<'data>,
    /// The pattern if the condition matches
    pub pattern: ListJoinerPattern<'data>,
}

#[cfg(feature = "serde")]
impl<'data> SpecialCasePattern<'data> {
    // If the condition doesn't deserialize, the whole special case becomes `None`
    fn deserialize_option<'de: 'data, D>(deserializer: D) -> Result<Option<Self>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::Deserialize;

        #[derive(Deserialize)]
        struct SpecialCasePatternOptionalDfa<'data> {
            #[cfg_attr(
                feature = "serde",
                serde(borrow, deserialize_with = "SerdeDFA::maybe_deserialize")
            )]
            pub condition: Option<SerdeDFA<'data>>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            pub pattern: ListJoinerPattern<'data>,
        }

        Ok(
            match Option::<SpecialCasePatternOptionalDfa<'data>>::deserialize(deserializer)? {
                Some(SpecialCasePatternOptionalDfa {
                    condition: Some(condition),
                    pattern,
                }) => Some(SpecialCasePattern { condition, pattern }),
                _ => None,
            },
        )
    }
}

/// A pattern containing two numeric placeholders ("{0}, and {1}.")
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, Debug, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
pub struct ListJoinerPattern<'data> {
    /// The pattern string without the placeholders
    pub(crate) string: Cow<'data, str>,
    /// The index of the first placeholder. Always <= index_1.
    // Always 0 for CLDR data, so we don't need to serialize it.
    // In-memory we have free space for it as index_1 doesn't
    // fill a word.
    #[cfg_attr(feature = "datagen", serde(skip))]
    pub(crate) index_0: u8,
    /// The index of the second placeholder. Always < string.len().
    pub(crate) index_1: u8,
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
    /// # Panics
    /// If `string[..index_1]` panics.
    pub const fn from_parts(string: &'a str, index_1: u8) -> Self {
        assert!(string.len() <= 255 && index_1 <= string.len() as u8);
        Self {
            string: Cow::Borrowed(string),
            index_0: 0,
            index_1,
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for ListJoinerPattern<'_> {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        env.insert("icu_list");
        let string = (&*self.string).bake(env);
        let index_1 = self.index_1.bake(env);
        databake::quote! {
            icu_list::provider::ListJoinerPattern::from_parts(#string, #index_1)
        }
    }
}

#[cfg(all(test, feature = "datagen"))]
#[test]
fn databake() {
    databake::test_bake!(
        ListJoinerPattern,
        const: crate::provider::ListJoinerPattern::from_parts(", ", 2u8),
        icu_list
    );
}
