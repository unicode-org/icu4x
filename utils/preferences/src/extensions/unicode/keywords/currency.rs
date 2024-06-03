// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::unicode::errors::Error;
use crate::struct_keyword;
use icu_locale_core::{extensions::unicode::Value, subtags::Subtag};
use tinystr::TinyAsciiStr;

struct_keyword!(
    Currency,
    "cu",
    TinyAsciiStr<3>,
    |input: Value| {
        if let Some(subtag) = input.into_single_subtag() {
            let ts = subtag.as_tinystr();
            if ts.len() == 3 && ts.is_ascii_alphabetic() {
                return Ok(Self(ts.resize()));
            }
        }
        Err(Error::InvalidKeywordValue)
    },
    |input: Currency| {
        icu_locale_core::extensions::unicode::Value::from_subtag(Some(
            Subtag::from_tinystr_unvalidated(input.0.resize()),
        ))
    }
);
