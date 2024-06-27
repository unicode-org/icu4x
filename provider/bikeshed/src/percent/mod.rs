// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde;
use crate::DatagenProvider;
use crate::IterableDataProviderCached;

use icu::experimental::dimension::provider::percent::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use tinystr::tinystr;

impl DataProvider<PercentEssentialsV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<PercentEssentialsV1Marker>, DataError> {
        self.check_req::<PercentEssentialsV1Marker>(req)?;
        let langid = req.id.locale.get_langid();

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let result = extract_percent_essentials(numbers_resource);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<PercentEssentialsV1Marker> for DatagenProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l)))
            .collect())
    }
}

fn extract_percent_essentials<'data>(
    numbers_resource: &cldr_serde::numbers::Resource,
) -> Result<PercentEssentialsV1<'data>, DataError> {
    // TODO(#3838): these patterns might be numbering system dependent.
    let percent_patterns = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .percent_patterns
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    // TODO(#3838): these patterns might be numbering system dependent.
    let symbols = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .symbols
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the percent symbol"))?;

    let standard_pattern = &percent_patterns.standard;

    let percent_sign = '%';
    let percent_sign_index = standard_pattern.find(percent_sign).unwrap();
    let first_num_index = standard_pattern.find(['0', '#']).unwrap();
    let last_num_index = standard_pattern.rfind(['0', '#']).unwrap();

    // For the prefix, if the first character is a percent sign, then we have no prefix.
    // If the percent sign is first, then all characters before the percent sign are the prefix.
    // If the percent comes after, then all characters between final number and the percent sign are the prefix.
    let percent_prefix = if percent_sign_index == 0 {
        ""
    } else if percent_sign_index < first_num_index {
        &standard_pattern[0..percent_sign_index]
    } else {
        &standard_pattern[last_num_index + 1..percent_sign_index]
    };

    // For the suffix, if the first character is a percent sign, OR the percent sign is before the first number,
    // then all characters between are the suffix.
    // If the percent sign comes after the first number, then all proceeding characters are the suffix.
    let percent_suffix = if percent_sign_index == 0 || percent_sign_index < first_num_index {
        &standard_pattern[1..first_num_index]
    } else {
        &standard_pattern[percent_sign_index + 1..]
    };

    Ok(PercentEssentialsV1 {
        standard: standard_pattern.to_owned().into(),
        percent_sign_symbol: symbols.percent_sign.to_owned().into(),
        percent_symbol_index: percent_sign_index as u8,
        number_index: first_num_index as u8,
        percent_sign_affixes: PercentAffixesV1 {
            prefix: percent_prefix.to_owned().into(),
            suffix: percent_suffix.to_owned().into(),
        },
    })
}

#[test]
fn test_basic() {
    use icu::experimental::dimension::provider::percent::*;
    use icu::locale::langid;

    let provider = DatagenProvider::new_testing();

    let en: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("en").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    assert_eq!(
        en.payload.get().to_owned(),
        PercentEssentialsV1 {
            standard: "#,##0%".into(),
            percent_sign_symbol: "%".into(),
            percent_symbol_index: 5,
            number_index: 0,
            percent_sign_affixes: PercentAffixesV1 {
                prefix: "".into(),
                suffix: "".into(),
            },
        }
    );

    let fr: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("fr").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    assert_eq!(
        fr.payload.get().to_owned(),
        PercentEssentialsV1 {
            standard: "#,##0\u{a0}%".into(),
            percent_sign_symbol: "%".into(),
            percent_symbol_index: 7,
            number_index: 0,
            percent_sign_affixes: PercentAffixesV1 {
                prefix: "\u{a0}".into(),
                suffix: "".into(),
            },
        }
    );

    let tr: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("tr").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    assert_eq!(
        tr.payload.get().to_owned(),
        PercentEssentialsV1 {
            standard: "%#,##0".into(),
            percent_sign_symbol: "%".into(),
            percent_symbol_index: 0,
            number_index: 1,
            percent_sign_affixes: PercentAffixesV1 {
                prefix: "".into(),
                suffix: "".into(),
            },
        }
    );

    let ar_eg: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("ar-EG").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    assert_eq!(
        ar_eg.payload.get().to_owned().percent_sign_symbol,
        "\u{200e}%\u{200e}" // "٪؜"
    );
}
