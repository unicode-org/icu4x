// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::DatagenProvider;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashSet;

use icu::experimental::dimension::provider::extended_currency::Count;
use tinystr::TinyAsciiStr;

use icu::experimental::dimension::provider::extended_currency::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;

impl DataProvider<CurrencyExtendedDataV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyExtendedDataV1Marker>, DataError> {
        self.check_req::<CurrencyExtendedDataV1Marker>(req)?;

        let langid = req.locale.get_langid();
        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(&langid, "currencies.json")?;

        let aux = req
            .marker_attributes
            .parse::<TinyAsciiStr<3>>()
            .map_err(|_| DataError::custom("failed to parse aux key into tinystr"))?;
        let currency = currencies_resource
            .main
            .value
            .numbers
            .currencies
            .get(&aux.to_unvalidated())
            .ok_or(DataError::custom("No currency associated with the aux key"))?;

        let mut placeholders: BTreeMap<Count, String> = BTreeMap::new();

        fn add_placeholder(
            placeholders: &mut BTreeMap<Count, String>,
            key: Count,
            value: Option<String>,
        ) {
            if let Some(val) = value {
                placeholders.insert(key, val);
            }
        }

        add_placeholder(&mut placeholders, Count::Zero, currency.zero.clone());
        add_placeholder(&mut placeholders, Count::One, currency.one.clone());
        add_placeholder(&mut placeholders, Count::Two, currency.two.clone());
        add_placeholder(&mut placeholders, Count::Few, currency.few.clone());
        add_placeholder(&mut placeholders, Count::Many, currency.many.clone());
        add_placeholder(&mut placeholders, Count::Other, currency.other.clone());
        add_placeholder(
            &mut placeholders,
            Count::DisplayName,
            currency.display_name.clone(),
        );

        let data = CurrencyExtendedDataV1 {
            display_names: placeholders
                .into_iter()
                .map(|(k, v)| (k, Cow::Owned(v)))
                .collect(),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data),
        })
    }
}

impl crate::IterableDataProviderCached<CurrencyExtendedDataV1Marker> for DatagenProvider {
    fn supported_requests_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        // TODO: This is a temporary implementation until we have a better way to handle large number of json files.
        let currencies_to_support: HashSet<_> =
            ["USD", "CAD", "EUR", "GBP", "EGP"].into_iter().collect();

        let mut result = HashSet::new();
        let numbers = self.cldr()?.numbers();
        let langids = numbers.list_langs()?;
        for langid in langids {
            let currencies_resource: &cldr_serde::currencies::data::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&langid, "currencies.json")?;

            let currencies = &currencies_resource.main.value.numbers.currencies;
            for key in currencies.keys() {
                let key_string = key
                    .try_into_tinystr()
                    .map_err(|_| DataError::custom("failed to parse currency code into tinystr"))?
                    .parse::<String>()
                    .map_err(|_| DataError::custom("failed to parse currency code into string"))?;
                if !currencies_to_support.contains(key_string.as_str()) {
                    continue;
                }

                let key = key
                    .try_into_tinystr()
                    .map_err(|_| DataError::custom("failed to parse currency code into tinystr"))?;

                let attributes = DataMarkerAttributes::from_tinystr(key.resize());
                result.insert((DataLocale::from(&langid), attributes));
            }
        }

        Ok(result)
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = DatagenProvider::new_testing();
    let en: DataPayload<CurrencyExtendedDataV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("en").into(),
            marker_attributes: &"USD".parse().unwrap(),
            ..Default::default()
        })
        .unwrap()
        .payload;
    let display_names = en.get().to_owned().display_names;
    assert_eq!(display_names.get(&Count::Zero), None);
    assert_eq!(display_names.get(&Count::One).unwrap(), "US dollar");
    assert_eq!(display_names.get(&Count::Two), None);
    assert_eq!(display_names.get(&Count::Few), None);
    assert_eq!(display_names.get(&Count::Many), None);
    assert_eq!(display_names.get(&Count::Other).unwrap(), "US dollars");
    assert_eq!(display_names.get(&Count::DisplayName).unwrap(), "US Dollar");

    let fr: DataPayload<CurrencyExtendedDataV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("fr").into(),
            marker_attributes: &"USD".parse().unwrap(),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let display_names = fr.get().to_owned().display_names;
    assert_eq!(display_names.get(&Count::Zero), None);
    assert_eq!(
        display_names.get(&Count::One).unwrap(),
        "dollar des États-Unis"
    );
    assert_eq!(display_names.get(&Count::Two), None);
    assert_eq!(display_names.get(&Count::Few), None);
    assert_eq!(display_names.get(&Count::Many), None);
    assert_eq!(
        display_names.get(&Count::Other).unwrap(),
        "dollars des États-Unis"
    );
    assert_eq!(
        display_names.get(&Count::DisplayName).unwrap(),
        "dollar des États-Unis"
    );
}
