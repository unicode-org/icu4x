// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::extended_currency::*;
use icu::experimental::relativetime::provider::PluralElements;
use icu_provider::prelude::*;
use std::collections::HashSet;

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
                display_names: PluralElements::try_new(
                    currency
                        .other
                        .as_deref()
                        .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?,
                    currency.zero.as_deref(),
                    currency.one.as_deref(),
                    currency.two.as_deref(),
                    currency.few.as_deref(),
                    currency.many.as_deref(),
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
            for (currency, patterns) in currencies {
                // By TR 35 (https://unicode.org/reports/tr35/tr35-numbers.html#Currencies)
                //      If the pattern is not found for the associated `Count`, fall back to the `Count::Other` pattern.
                //      And the `other` pattern must be present.
                //      Therefore, we filter out any currencies that do not have an `other` pattern.
                //      NOTE:
                //          In case of `other` pattern does not exist, File a Jira ticket to CLDR:
                //          https://unicode-org.atlassian.net/browse/CLDR
                if patterns.other.is_none() {
                    continue;
                }
                if let Ok(attributes) = DataMarkerAttributes::try_from_string(currency.clone()) {
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
    use icu::plurals::PluralCategory;
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
    assert_eq!(
        en.get().display_names.get_str(PluralCategory::Zero),
        "US dollars"
    );
    assert_eq!(
        en.get().display_names.get_str(PluralCategory::One),
        "US dollar"
    );
    assert_eq!(
        en.get().display_names.get_str(PluralCategory::Two),
        "US dollars"
    );
    assert_eq!(
        en.get().display_names.get_str(PluralCategory::Few),
        "US dollars"
    );
    assert_eq!(
        en.get().display_names.get_str(PluralCategory::Many),
        "US dollars"
    );
    assert_eq!(
        en.get().display_names.get_str(PluralCategory::Other),
        "US dollars"
    );

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

    assert_eq!(
        fr.get().display_names.get_str(PluralCategory::Zero),
        "dollars des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get_str(PluralCategory::One),
        "dollar des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get_str(PluralCategory::Two),
        "dollars des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get_str(PluralCategory::Few),
        "dollars des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get_str(PluralCategory::Many),
        "dollars des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get_str(PluralCategory::Other),
        "dollars des États-Unis"
    );
}
