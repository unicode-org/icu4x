// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::provider::IterableDataProviderInternal;
use crate::transform::cldr::cldr_serde;

use icu_dimension::provider::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use tinystr::tinystr;

impl DataProvider<PercentEssentialsV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<PercentEssentialsV1Marker>, DataError> {
        self.check_req::<PercentEssentialsV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let result = extract_percent_essentials(numbers_resource);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}

impl IterableDataProviderInternal<PercentEssentialsV1Marker> for crate::DatagenProvider {
    fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}

fn extract_percent_essentials<'data>(
    numbers_resource: &cldr_serde::numbers::Resource,
) -> Result<PercentEssentialsV1<'data>, DataError> {
    let percent_formats = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .percent_patterns
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    let standard = &percent_formats.standard;

    Ok(PercentEssentialsV1 {
        standard: standard.to_owned().into(),
    })
}

#[test]
fn test_basic() {
    use icu_dimension::provider::*;
    use icu_locid::locale;

    let provider = crate::DatagenProvider::new_testing();

    let en: DataPayload<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("en").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(en.clone().get().to_owned().standard, "#,##0%");

    let fr: DataPayload<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("fr").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(fr.clone().get().to_owned().standard, "#,##0\u{a0}%");
}
