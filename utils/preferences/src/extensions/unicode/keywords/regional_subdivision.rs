// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::Deref;

use crate::extensions::unicode::errors::PreferencesParseError;
use crate::struct_keyword;
use icu_locale_core::{
    extensions::unicode::{SubdivisionId, Value},
    subtags::Subtag,
};

struct_keyword!(
    RegionalSubdivision,
    "sd",
    SubdivisionId,
    |input: Value| {
        input
            .into_single_subtag()
            .and_then(|subtag| subtag.as_str().parse().ok().map(Self))
            .ok_or(PreferencesParseError::InvalidKeywordValue)
    },
    |input: RegionalSubdivision| {
        Value::from_subtag(Some(
            Subtag::try_from_bytes(input.0.to_string().as_bytes()).unwrap(),
        ))
    }
);

impl Deref for RegionalSubdivision {
    type Target = SubdivisionId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use icu_locale_core::extensions::unicode;
    use icu_locale_core::extensions::unicode::subdivision_suffix;
    use icu_locale_core::subtags::region;

    #[test]
    fn region_subdivision_test() {
        let val = unicode::value!("uksct");
        let rg: RegionalSubdivision = val.try_into().unwrap();
        assert_eq!(rg.region, region!("UK"));
        assert_eq!(rg.suffix, subdivision_suffix!("sct"));

        for i in &["4aabel", "a4bel", "ukabcde"] {
            let val = unicode::Value::try_from_bytes(i.as_bytes()).unwrap();
            let rg: Result<RegionalSubdivision, _> = val.try_into();
            assert!(rg.is_err());
        }
    }
}
