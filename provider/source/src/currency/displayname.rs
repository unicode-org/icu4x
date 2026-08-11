// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use crate::cldr_serde;
use icu::experimental::dimension::provider::currency::displayname::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroCow;

impl DataProvider<CurrencyDisplaynameV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyDisplaynameV1>, DataError> {
        self.check_req::<CurrencyDisplaynameV1>(req)?;

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(req.id.locale, "currencies.json")?;

        let currency_upper = req.id.marker_attributes.as_str().to_ascii_uppercase();

        let currency = currencies_resource
            .main
            .value
            .numbers
            .currencies
            .get(&currency_upper)
            .ok_or_else(|| {
                DataErrorKind::IdentifierNotFound
                    .into_error()
                    .with_debug_context("No currency associated with the auxiliary key")
            })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(VarZeroCow::from_encodeable(
                currency.display_name.as_ref().ok_or_else(|| {
                    DataErrorKind::IdentifierNotFound
                        .into_error()
                        .with_debug_context("No display name found for the currency")
                })?,
            )),
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
            for (iso, currency_data) in currencies {
                // If the currency doesn't have a display name, we cannot create `CurrencyDisplayname` for it.
                // Therefore, we skip it.
                if currency_data.display_name.is_none() {
                    continue;
                }
                let iso_lower = iso.to_ascii_lowercase();
                if let Ok(attributes) = DataMarkerAttributes::try_from_string(iso_lower) {
                    result.insert(DataIdentifierCow::from_owned(attributes, locale));
                }
            }
        }

        Ok(result)
    }
}

#[test]
fn test_basic() {
    use icu::locale::data_locale;

    let provider = SourceDataProvider::new_testing();
    let en: DataPayload<CurrencyDisplaynameV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("usd"),
                &data_locale!("en"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;
    assert_eq!(&**en.get(), "US Dollar");

    let fr: DataPayload<CurrencyDisplaynameV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("usd"),
                &data_locale!("fr"),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(&**fr.get(), "dollar des États-Unis");
}
