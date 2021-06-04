// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

pub type SmallString8 = smallstr::SmallString<[u8; 8]>;

pub mod key {
    //! Resource keys for [`icu_decimal`](crate).
    use icu_provider::{resource_key, ResourceKey};

    /// Resource key: symbols used for basic decimal formatting.
    pub const SYMBOLS_V1: ResourceKey = resource_key!(decimal, "symbols", 1);
}

/// A collection of strings to affix to a decimal number.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct AffixesV1 {
    /// String to prepend before the decimal number.
    pub prefix: Option<SmallString8>,

    /// String to append after the decimal number.
    pub suffix: Option<SmallString8>,
}

/// A collection of settings expressing where to put grouping separators in a decimal number.
/// For example, `1,000,000` has two grouping separators, positioned along every 3 digits.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct GroupingSizesV1 {
    /// The size of the first (lowest-magnitude) group.
    pub primary: u8,

    /// The size of groups after the first group.
    pub secondary: u8,

    /// The minimum number of digits required before the first group. For example, if `primary=3`
    /// and `min_grouping=2`, grouping separators will be present on 10,000 and above.
    pub min_grouping: u8,
}

/// Symbols and metadata required for formatting a [`FixedDecimal`](crate::FixedDecimal).
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DecimalSymbolsV1 {
    /// Prefix and suffix to apply when a negative sign is needed.
    pub minus_sign_affixes: AffixesV1,

    /// Prefix and suffix to apply when a plus sign is needed.
    pub plus_sign_affixes: AffixesV1,

    /// Character used to separate the integer and fraction parts of the number.
    pub decimal_separator: SmallString8,

    /// Character used to separate groups in the integer part of the number.
    pub grouping_separator: SmallString8,

    /// Settings used to determine where to place groups in the integer part of the number.
    pub grouping_sizes: GroupingSizesV1,

    /// Digit characters for the current numbering system. In most systems, these digits are
    /// contiguous, but in some systems, such as *hanidec*, they are not contiguous.
    pub digits: [char; 10],
}

impl Default for DecimalSymbolsV1 {
    fn default() -> Self {
        Self {
            minus_sign_affixes: AffixesV1 {
                prefix: Some("-".into()),
                suffix: None,
            },
            plus_sign_affixes: AffixesV1 {
                prefix: Some("+".into()),
                suffix: None,
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

icu_provider::impl_data_marker_no_lifetime!(
    DecimalSymbolsV1,
    /// Marker type for [`DecimalSymbolsV1`]
    DecimalSymbolsV1_M
);
