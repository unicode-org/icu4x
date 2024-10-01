// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::unicode::enum_keyword;

enum_keyword!(
    /// TODO
    CurrencyFormatStyle {
        "standard" => Standard,
        "account" => Account
}, "cf");
