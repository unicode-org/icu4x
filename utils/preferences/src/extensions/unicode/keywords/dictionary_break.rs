// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::unicode::errors::PreferencesParseError;
use crate::struct_keyword;
use icu_locale_core::extensions::unicode::Value;
use icu_locale_core::subtags::Script;
use std::str::FromStr;

struct_keyword!(
    DictionaryBreak,
    "dx",
    Vec<Script>,
    |input: Value| {
        input
            .into_iter()
            .map(|s| {
                Script::from_str(s.as_str()).map_err(|_| PreferencesParseError::InvalidKeywordValue)
            })
            .collect::<Result<_, _>>()
            .map(Self)
    },
    |input: DictionaryBreak| {
        icu_locale_core::extensions::unicode::Value::from_iter(input.0.into_iter().map(Into::into))
    }
);
