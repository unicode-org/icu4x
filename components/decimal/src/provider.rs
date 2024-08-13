// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

// Provider structs must be stable
#![allow(clippy::exhaustive_structs)]
#![allow(clippy::exhaustive_enums)]

use alloc::borrow::Cow;
use icu_provider::prelude::*;

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
    use icu_decimal_data::*;
    pub mod icu {
        pub use crate as decimal;
        pub use icu_decimal_data::icu_locale as locale;
    }
    make_provider!(Baked);
    impl_decimal_symbols_v1_marker!(Baked);
};

#[cfg(feature = "datagen")]
/// The latest minimum set of markers required by this component.
pub const MARKERS: &[DataMarkerInfo] = &[DecimalSymbolsV1Marker::INFO];

/// A collection of strings to affix to a decimal number.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_decimal::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct AffixesV1<'data> {
    /// String to prepend before the decimal number.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub prefix: Cow<'data, str>,

    /// String to append after the decimal number.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub suffix: Cow<'data, str>,
}

/// A collection of settings expressing where to put grouping separators in a decimal number.
/// For example, `1,000,000` has two grouping separators, positioned along every 3 digits.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, Copy, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_decimal::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct GroupingSizesV1 {
    /// The size of the first (lowest-magnitude) group.
    ///
    /// If 0, grouping separators will never be shown.
    pub primary: u8,

    /// The size of groups after the first group.
    ///
    /// If 0, defaults to be the same as `primary`.
    pub secondary: u8,

    /// The minimum number of digits required before the first group. For example, if `primary=3`
    /// and `min_grouping=2`, grouping separators will be present on 10,000 and above.
    pub min_grouping: u8,
}

/// Symbols and metadata required for formatting a [`FixedDecimal`](crate::FixedDecimal).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(DecimalSymbolsV1Marker = "decimal/symbols@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_decimal::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DecimalSymbolsV1<'data> {
    /// Prefix and suffix to apply when a negative sign is needed.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub minus_sign_affixes: AffixesV1<'data>,

    /// Prefix and suffix to apply when a plus sign is needed.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub plus_sign_affixes: AffixesV1<'data>,

    /// Character used to separate the integer and fraction parts of the number.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub decimal_separator: Cow<'data, str>,

    /// Character used to separate groups in the integer part of the number.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub grouping_separator: Cow<'data, str>,

    /// Settings used to determine where to place groups in the integer part of the number.
    pub grouping_sizes: GroupingSizesV1,

    /// Digit characters for the current numbering system. In most systems, these digits are
    /// contiguous, but in some systems, such as *hanidec*, they are not contiguous.
    pub digits: [char; 10],
}

impl Default for DecimalSymbolsV1<'static> {
    fn default() -> Self {
        Self {
            minus_sign_affixes: AffixesV1 {
                prefix: Cow::Borrowed("-"),
                suffix: Cow::Borrowed(""),
            },
            plus_sign_affixes: AffixesV1 {
                prefix: Cow::Borrowed("+"),
                suffix: Cow::Borrowed(""),
            },
            decimal_separator: ".".into(),
            grouping_separator: ",".into(),
            grouping_sizes: GroupingSizesV1 {
                primary: 3,
                secondary: 3,
                min_grouping: 1,
            },
            digits: ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        }
    }
}
