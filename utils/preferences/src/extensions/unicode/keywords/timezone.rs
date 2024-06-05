// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::unicode::errors::PreferencesParseError;
use crate::struct_keyword;
use icu_locale_core::{extensions::unicode::Value, subtags::Subtag};

struct_keyword!(
    Timezone,
    "tz",
    Subtag,
    |input: Value| {
        input
            .into_single_subtag()
            .map(Self)
            .ok_or(PreferencesParseError::InvalidKeywordValue)
    },
    |input: Timezone| { icu_locale_core::extensions::unicode::Value::from_subtag(Some(input.0)) }
);
