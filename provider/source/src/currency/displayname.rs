// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::currency::displayname::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;

impl DataProvider<CurrencyDisplaynameV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyDisplaynameV1>, DataError> {
        self.check_req::<CurrencyDisplaynameV1>(req)?;

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
            .ok_or_else(|| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("No data for currency")
            })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyDisplayname {
                display_name: Cow::Owned(
                    currency
                        .display_name
                        .as_deref()
                        .ok_or_else(|| {
                            DataErrorKind::IdentifierNotFound
                                .into_error()
                                .with_debug_context("No display name found for the currency")
                        })?
                        .to_string(),
                ),
            }),
        })
    }
}

impl crate::IterableDataProviderCached<CurrencyDisplaynameV1> for SourceDataProvider {
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
            for (currency, patterns) in currencies {
                // If the currency doesn't have a display name, we can not create `CurrencyDisplayname` for it.
                // Therefore, we skip it.
                if patterns.display_name.is_none() {
                    continue;
                }
                if let Ok(attributes) = DataMarkerAttributes::try_from_string(currency.clone()) {
                    result.insert(DataIdentifierCow::from_owned(attributes, locale));
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
    let en: DataPayload<CurrencyDisplaynameV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("en").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;
    let display_name = en.get().to_owned().display_name;
    assert_eq!(display_name, "US Dollar");

    let fr: DataPayload<CurrencyDisplaynameV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("USD"),
                &langid!("fr").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    let display_name = fr.get().to_owned().display_name;
    assert_eq!(display_name, "dollar des États-Unis");
}
