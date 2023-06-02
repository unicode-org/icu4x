// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_locid_transform::provider::*;

use icu_locid_transform::Direction;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;

impl DataProvider<DirectionalityV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DirectionalityV1Marker>, DataError> {
        // We treat searching for `und` as a request for all data. Other requests
        // are not currently supported.
        if !req.locale.is_empty() {
            return Err(DataErrorKind::ExtraneousLocale.into_error());
        }

        let data: &cldr_serde::directionality::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("scriptMetadata.json")?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(DirectionalityV1::from(data))),
        })
    }
}

impl IterableDataProvider<DirectionalityV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl From<&cldr_serde::directionality::Resource> for DirectionalityV1<'_> {
    fn from(other: &cldr_serde::directionality::Resource) -> Self {
        let mut map = BTreeMap::new();
        for (script, metadata) in &other.script_metadata {
            let rtl = match metadata.rtl {
                cldr_serde::directionality::Rtl::Yes => Direction::RightToLeft,
                cldr_serde::directionality::Rtl::No => Direction::LeftToRight,
                cldr_serde::directionality::Rtl::Unknown => Direction::Unknown,
            };
            map.insert(script.to_unvalidated(), rtl);
        }
        Self {
            rtl: map.into_iter().collect(),
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid::subtags_script as script;
    use zerovec::ule::AsULE;

    let provider = crate::DatagenProvider::for_test();
    let data: DataPayload<DirectionalityV1Marker> = provider
        .load(Default::default())
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(
        data.get()
            .rtl
            .get(&script!("Avst").into_tinystr().to_unvalidated())
            .unwrap(),
        &Direction::RightToLeft.to_unaligned()
    );

    assert_eq!(
        data.get()
            .rtl
            .get(&script!("Latn").into_tinystr().to_unvalidated())
            .unwrap(),
        &Direction::LeftToRight.to_unaligned()
    );

    assert_eq!(
        data.get()
            .rtl
            .get(&script!("Brai").into_tinystr().to_unvalidated())
            .unwrap(),
        &Direction::Unknown.to_unaligned()
    );
}
