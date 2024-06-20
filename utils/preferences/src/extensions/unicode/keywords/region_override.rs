// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::extensions::unicode::errors::PreferencesParseError;
use crate::struct_keyword;
use icu_locale_core::{
    extensions::unicode::{SubdivisionId, Value},
    subtags::Subtag,
};

struct_keyword!(
    RegionOverride,
    "rg",
    SubdivisionId,
    |input: Value| {
        input
            .into_single_subtag()
            .and_then(|subtag| subtag.as_str().parse().ok().map(Self))
            .ok_or(PreferencesParseError::InvalidKeywordValue)
    },
    |input: RegionOverride| {
        Value::from_subtag(Some(Subtag::try_from_str(&input.0.to_string()).unwrap()))
    }
);

#[cfg(test)]
mod test {
    use super::*;
    use icu_locale_core::extensions::unicode;
    use icu_locale_core::extensions::unicode::subdivision_suffix;
    use icu_locale_core::subtags::region;

    #[test]
    fn region_override_test() {
        let val = unicode::value!("uksct");
        let rg: RegionOverride = val.try_into().unwrap();
        assert_eq!(rg.0.region, region!("UK"));
        assert_eq!(rg.0.suffix, subdivision_suffix!("sct"));

        let val = unicode::value!("usca");
        let rg: RegionOverride = val.try_into().unwrap();
        assert_eq!(rg.0.region, region!("US"));
        assert_eq!(rg.0.suffix, subdivision_suffix!("ca"));

        let val = unicode::value!("419bel");
        let rg: RegionOverride = val.try_into().unwrap();
        assert_eq!(rg.0.region, region!("419"));
        assert_eq!(rg.0.suffix, subdivision_suffix!("bel"));

        let val = unicode::value!("uszzzz");
        let rg: RegionOverride = val.try_into().unwrap();
        assert_eq!(rg.0.region, region!("us"));
        assert_eq!(rg.0.suffix, subdivision_suffix!("zzzz"));

        for i in &["4aabel", "a4bel", "ukabcde"] {
            let val = unicode::Value::try_from_str(i).unwrap();
            let rg: Result<RegionOverride, _> = val.try_into();
            assert!(rg.is_err());
        }
    }
}
