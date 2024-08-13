// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::extended_currency::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::ZeroMap;

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

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyExtendedDataV1 {
                display_names: ZeroMap::from_iter(
                    [
                        (Count::Zero, currency.zero.as_deref()),
                        (Count::One, currency.one.as_deref()),
                        (Count::Two, currency.two.as_deref()),
                        (Count::Few, currency.few.as_deref()),
                        (Count::Many, currency.many.as_deref()),
                        (Count::Other, currency.other.as_deref()),
                        (Count::DisplayName, currency.display_name.as_deref()),
                    ]
                    .into_iter()
                    .filter_map(|(count, pattern)| match (count, pattern) {
                        (Count::DisplayName, Some(p)) => Some((count, p)),
                        (Count::Other, Some(p)) => Some((count, p)),
                        // As per [Unicode TR 35](https://unicode.org/reports/tr35/tr35-numbers.html#Currencies)
                        //      If the pattern is not found for the associated `Count`, fall back to the `Count::Other` pattern.
                        //      Therefore, we filter out any patterns that are the same as the `Count::Other` pattern.
                        (_, p) if p == currency.other.as_deref() => None,
                        _ => Some((count, pattern?)),
                    }),
                ),
            }),
        })
    }
}

impl crate::IterableDataProviderCached<CurrencyExtendedDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
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
