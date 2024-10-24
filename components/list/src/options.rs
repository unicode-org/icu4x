// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Represents the style of a list. See the
/// [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns)
/// for an explanation of the different styles.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Default)]
#[non_exhaustive]
pub enum ListLength {
    /// A typical list
    #[default]
    Wide,
    /// A shorter list
    Short,
    /// The shortest type of list
    Narrow,
    // *Important*: When adding a variant here, make sure the code in
    // ListFormatterPatterns::{start, middle, end, pair} stays panic-free!
}
