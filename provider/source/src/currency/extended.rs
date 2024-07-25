// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashSet;

use icu::experimental::dimension::provider::extended_currency::Count;

use icu::experimental::dimension::provider::extended_currency::*;
use icu_provider::prelude::*;
use icu_provider::DataProvider;

impl DataProvider<CurrencyExtendedDataV1Marker> for crate::SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyExtendedDataV1Marker>, DataError> {
        self.check_req::<CurrencyExtendedDataV1Marker>(req)?;

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(req.id.locale, "currencies.json")?;

        let currency = currencies_resource
            .main
            .value
            .numbers
            .currencies
            .get(req.id.marker_attributes.as_str())
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

impl crate::IterableDataProviderCached<CurrencyExtendedDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        // TODO: This is a temporary implementation until we have a better way to handle large number of json files.
        let currencies_to_support: HashSet<_> =
            ["USD", "CAD", "EUR", "GBP", "EGP"].into_iter().collect();

        let mut result = HashSet::new();
        let numbers = self.cldr()?.numbers();
        let locales = numbers.list_locales()?;
        for locale in locales {
            let currencies_resource: &cldr_serde::currencies::data::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&locale, "currencies.json")?;

            let currencies = &currencies_resource.main.value.numbers.currencies;
            for key in currencies.keys() {
                if !currencies_to_support.contains(key.as_str()) {
                    continue;
                }

                if let Ok(attributes) = DataMarkerAttributes::try_from_string(key.clone()) {
                    result.insert(DataIdentifierCow::from_owned(attributes, locale.clone()));
                }
            }
        }

        Ok(result)
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();
    let en: DataPayload<CurrencyExtendedDataV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("en").into(),
            ),
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
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("fr").into(),
            ),
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
