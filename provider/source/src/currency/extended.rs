// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::extended_currency::*;
use icu::plurals::PluralCategory;
use icu_provider::prelude::*;
use std::borrow::Cow;
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

        let other_display_name = currency.other.clone().ok_or_else(|| {
            DataErrorKind::IdentifierNotFound
                .into_error()
                .with_debug_context(req.id.locale)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyExtendedDataV1 {
                categorized_display_names: ZeroMap::from_iter(
                    [
                        (PluralCategory::Zero, currency.zero.as_deref()),
                        (PluralCategory::One, currency.one.as_deref()),
                        (PluralCategory::Two, currency.two.as_deref()),
                        (PluralCategory::Few, currency.few.as_deref()),
                        (PluralCategory::Many, currency.many.as_deref()),
                    ]
                    .into_iter()
                    .filter_map(|(count, pattern)| match (count, pattern) {
                        // As per [Unicode TR 35](https://unicode.org/reports/tr35/tr35-numbers.html#Currencies)
                        //      If the pattern is not found for the associated `Count`, fall back to the `Count::Other` pattern.
                        //      Therefore, we filter out any patterns that are the same as the `Count::Other` pattern.
                        (_, p) if p == currency.other.as_deref() => None,
                        _ => Some((count, pattern?)),
                    }),
                ),
                other_display_name: Cow::Owned(other_display_name.to_owned()),
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
    let extended_en = en.get().to_owned();
    let categorized_display_names = extended_en.categorized_display_names;
    assert_eq!(categorized_display_names.get(&PluralCategory::Zero), None);
    assert_eq!(
        categorized_display_names.get(&PluralCategory::One).unwrap(),
        "US dollar"
    );
    assert_eq!(categorized_display_names.get(&PluralCategory::Two), None);
    assert_eq!(categorized_display_names.get(&PluralCategory::Few), None);
    assert_eq!(categorized_display_names.get(&PluralCategory::Many), None);
    assert_eq!(extended_en.other_display_name, "US dollars");

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

    let extended_fr = fr.get().to_owned();
    let categorized_display_names = extended_fr.categorized_display_names;
    assert_eq!(categorized_display_names.get(&PluralCategory::Zero), None);
    assert_eq!(
        categorized_display_names.get(&PluralCategory::One),
        Some("dollar des États-Unis")
    );
    assert_eq!(categorized_display_names.get(&PluralCategory::Two), None);
    assert_eq!(categorized_display_names.get(&PluralCategory::Few), None);
    assert_eq!(categorized_display_names.get(&PluralCategory::Many), None);
    assert_eq!(extended_fr.other_display_name, "dollars des États-Unis");
}
