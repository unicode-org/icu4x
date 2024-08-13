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

use crate::rules::runtime::ast::Rule;
use icu_provider::prelude::*;
use icu_provider::DynamicDataMarker;

#[cfg(feature = "compiled_data")]
#[derive(Debug)]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub struct Baked;

#[cfg(feature = "compiled_data")]
#[allow(unused_imports)]
const _: () = {
    use icu_plurals_data::*;
    mod icu {
        pub use crate as plurals;
        pub use icu_plurals_data::icu_locale as locale;
    }

    make_provider!(Baked);
    impl_cardinal_v1_marker!(Baked);
    impl_ordinal_v1_marker!(Baked);
    #[cfg(feature = "experimental")]
    impl_plural_ranges_v1_marker!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[
    CardinalV1Marker::INFO,
    OrdinalV1Marker::INFO,
    #[cfg(feature = "experimental")]
    PluralRangesV1Marker::INFO,
];

/// Plural rule strings conforming to UTS 35 syntax. Includes separate fields for five of the six
/// standard plural forms. If none of the rules match, the "other" category is assumed.
///
/// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(
    CardinalV1Marker = "plurals/cardinal@1",
    OrdinalV1Marker = "plurals/ordinal@1"
)]
#[derive(Default, Clone, PartialEq, Debug)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_plurals::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PluralRulesV1<'data> {
    /// Rule that matches [`PluralCategory::Zero`](super::PluralCategory::Zero), or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub zero: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::One`](super::PluralCategory::One), or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub one: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::Two`](super::PluralCategory::Two), or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub two: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::Few`](super::PluralCategory::Few), or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub few: Option<Rule<'data>>,
    /// Rule that matches [`PluralCategory::Many`](super::PluralCategory::Many), or `None` if not present.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub many: Option<Rule<'data>>,
}

pub(crate) struct ErasedPluralRulesV1Marker;

impl DynamicDataMarker for ErasedPluralRulesV1Marker {
    type DataStruct = PluralRulesV1<'static>;
}

#[cfg(any(feature = "datagen", feature = "experimental"))]
pub use ranges::*;

#[cfg(any(feature = "datagen", feature = "experimental"))]
mod ranges {
    use crate::PluralCategory;
    use icu_provider::prelude::*;
    use zerovec::ZeroMap;

    /// [`PluralCategory`] but serializable as provider data.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
    #[cfg_attr(
        feature = "datagen",
        derive(serde::Serialize, databake::Bake),
        databake(path = icu_plurals::provider),
    )]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    #[zerovec::make_ule(RawPluralCategoryULE)]
    #[repr(u8)]
    #[cfg_attr(
        any(feature = "datagen", feature = "serde"),
        serde(rename_all = "lowercase")
    )]
    pub enum RawPluralCategory {
        /// CLDR "other" plural category.
        Other = 0,
        /// CLDR "zero" plural category.
        Zero = 1,
        /// CLDR "one" plural category.
        One = 2,
        /// CLDR "two" plural category.
        Two = 3,
        /// CLDR "few" plural category.
        Few = 4,
        /// CLDR "many" plural category.
        Many = 5,
    }

    impl RawPluralCategory {
        /// Gets the corresponding variant string of this `RawPluralCategory`.
        #[cfg(any(feature = "datagen", feature = "serde"))]
        const fn as_str(self) -> &'static str {
            match self {
                Self::Other => "other",
                Self::Zero => "zero",
                Self::One => "one",
                Self::Two => "two",
                Self::Few => "few",
                Self::Many => "many",
            }
        }
    }

    impl From<RawPluralCategory> for PluralCategory {
        fn from(value: RawPluralCategory) -> Self {
            match value {
                RawPluralCategory::Other => PluralCategory::Other,
                RawPluralCategory::Zero => PluralCategory::Zero,
                RawPluralCategory::One => PluralCategory::One,
                RawPluralCategory::Two => PluralCategory::Two,
                RawPluralCategory::Few => PluralCategory::Few,
                RawPluralCategory::Many => PluralCategory::Many,
            }
        }
    }

    impl From<PluralCategory> for RawPluralCategory {
        fn from(value: PluralCategory) -> Self {
            match value {
                PluralCategory::Zero => RawPluralCategory::Zero,
                PluralCategory::One => RawPluralCategory::One,
                PluralCategory::Two => RawPluralCategory::Two,
                PluralCategory::Few => RawPluralCategory::Few,
                PluralCategory::Many => RawPluralCategory::Many,
                PluralCategory::Other => RawPluralCategory::Other,
            }
        }
    }

    /// An `u8` that is expected to be a plural range, but does not enforce this invariant.
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
    #[cfg_attr(
        feature = "datagen",
        derive(databake::Bake),
        databake(path = icu_plurals::provider),
    )]
    #[zerovec::make_ule(UnvalidatedPluralRangeULE)]
    pub struct UnvalidatedPluralRange(pub u8);

    impl UnvalidatedPluralRange {
        /// Creates a new `UnvalidatedPluralRange` from a category range.
        pub fn from_range(start: RawPluralCategory, end: RawPluralCategory) -> Self {
            let start = start as u8;
            let end = end as u8;

            debug_assert!(start < 16);
            debug_assert!(end < 16);

            let range = (start << 4) | end;

            Self(range)
        }
    }

    #[cfg(feature = "datagen")]
    impl serde::Serialize for UnvalidatedPluralRange {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::Error;

            struct PrettyPrinter(RawPluralCategory, RawPluralCategory);

            impl core::fmt::Display for PrettyPrinter {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str(self.0.as_str())?;
                    f.write_str("--")?;
                    f.write_str(self.1.as_str())
                }
            }

            if serializer.is_human_readable() {
                let start = RawPluralCategory::new_from_u8(self.0 >> 4)
                    .ok_or_else(|| S::Error::custom("invalid tag in UnvalidatedPluralRange"))?;
                let end = RawPluralCategory::new_from_u8(self.0 & 0x0F)
                    .ok_or_else(|| S::Error::custom("invalid tag in UnvalidatedPluralRange"))?;
                serializer.collect_str(&PrettyPrinter(start, end))
            } else {
                self.0.serialize(serializer)
            }
        }
    }

    #[cfg(feature = "serde")]
    impl<'de> serde::Deserialize<'de> for UnvalidatedPluralRange {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use serde::de::{Error, Visitor};

            struct HumanReadableVisitor;

            impl<'de> Visitor<'de> for HumanReadableVisitor {
                type Value = UnvalidatedPluralRange;

                fn expecting(&self, formatter: &mut alloc::fmt::Formatter) -> alloc::fmt::Result {
                    write!(
                        formatter,
                        "a plural range of the form <PluralCategory>-<PluralCategory>",
                    )
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    const VARIANTS: [&str; 6] = [
                        RawPluralCategory::Other.as_str(),
                        RawPluralCategory::Zero.as_str(),
                        RawPluralCategory::One.as_str(),
                        RawPluralCategory::Two.as_str(),
                        RawPluralCategory::Few.as_str(),
                        RawPluralCategory::Many.as_str(),
                    ];

                    let (start, end) = v
                        .split_once("--")
                        .ok_or_else(|| E::custom("expected token `--` in plural range"))?;

                    let start = PluralCategory::get_for_cldr_string(start)
                        .ok_or_else(|| E::unknown_variant(start, &VARIANTS))?;
                    let end = PluralCategory::get_for_cldr_string(end)
                        .ok_or_else(|| E::unknown_variant(end, &VARIANTS))?;

                    Ok(UnvalidatedPluralRange::from_range(start.into(), end.into()))
                }
            }

            if deserializer.is_human_readable() {
                deserializer.deserialize_str(HumanReadableVisitor)
            } else {
                Ok(Self(<u8>::deserialize(deserializer)?))
            }
        }
    }

    /// Plural categories for ranges.
    ///
    /// Obtains the plural category of a range from the categories of its endpoints. It is required that
    /// the start value must be strictly less than the end value, and both values must be strictly positive.
    ///
    /// More information: <https://unicode.org/reports/tr35/tr35-numbers.html#Plural_Ranges>
    ///
    /// <div class="stab unstable">
    /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
    /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
    /// to be stable, their Rust representation might not be. Use with caution.
    /// </div>
    #[icu_provider::data_struct(PluralRangesV1Marker = "plurals/ranges@1")]
    #[derive(Clone, PartialEq, Debug)]
    #[cfg_attr(
        feature = "datagen",
        derive(serde::Serialize, databake::Bake),
        databake(path = icu_plurals::provider),
    )]
    #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
    #[yoke(prove_covariance_manually)]
    pub struct PluralRangesV1<'data> {
        /// Map between the categories of the endpoints of a range and its corresponding
        /// category.
        ///
        /// This is roughly equivalent to a `BTreeMap<(PluralCategory, PluralCategory), PluralCategory>`,
        /// where the key is `(start category, end category)`.
        #[cfg_attr(feature = "serde", serde(borrow))]
        pub ranges: ZeroMap<'data, UnvalidatedPluralRange, RawPluralCategory>,
    }
}
