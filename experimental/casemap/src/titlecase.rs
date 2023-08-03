// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Titlecasing-specific types

/// How to handle the rest of the string once the head of the
/// string has been titlecased
#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum TailCasing {
    /// Lowercase the rest of the string ("spoNgEBoB" -> "Spongebob")
    #[default]
    Lowercase,
    /// Preserve the casing of the rest of the string ("spoNgEBoB" -> "SpoNgEBoB")
    PreserveCase,
}

/// Whether to start casing at the beginning of the string or at the first
/// relevant character.
#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub enum HeadAdjustment {
    /// Adjust the string to the first relevant character before beginning to apply casing
    /// ("'twixt" -> "'Twixt")
    #[default]
    Adjust,
    /// Start titlecasing immediately, even if the character is not one that is relevant for casing
    /// ("'twixt" -> "'twixt", "twixt" -> "Twixt")
    NoAdjust,
}

/// Various options for controlling titlecasing
#[non_exhaustive]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct TitlecaseOptions {
    /// How to handle the rest of the string once the head of the
    /// string has been titlecased
    pub tail_casing: TailCasing,
    /// Whether to start casing at the beginning of the string or at the first
    /// relevant character.
    pub head_adjustment: HeadAdjustment,
}
