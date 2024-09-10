// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::extended_currency::*;
use icu::plurals::provider::FourBitMetadata;
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use std::borrow::Cow;
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

        let plural_elements = PluralElements::new((
            FourBitMetadata::zero(),
            currency
                .other
                .as_deref()
                .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?,
        ))
        .with_zero_value(
            currency
                .zero
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        )
        .with_one_value(
            currency
                .one
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        )
        .with_two_value(
            currency
                .two
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        )
        .with_few_value(
            currency
                .few
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        )
        .with_many_value(
            currency
                .many
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        )
        .with_explicit_one_value(
            currency
                .explicit_one
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        )
        .with_explicit_zero_value(
            currency
                .explicit_zero
                .as_deref()
                .map(|v| (FourBitMetadata::zero(), v)),
        );

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyExtendedDataV1 {
                display_names: Cow::Owned(zerovec::ule::encode_varule_to_box(&plural_elements)),
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
            for (currency, displaynames) in currencies {
                // By TR 35 (https://unicode.org/reports/tr35/tr35-numbers.html#Currencies)
                //      If the displayname is not found for the associated `Count`, fall back to the `Count::Other` displayname.
                //      And the `other` displayname must be present.
                //      Therefore, we filter out any currencies that do not have an `other` displayname.
                //      NOTE:
                //          In case of `other` displayname does not exist, File a Jira ticket to CLDR:
                //          https://unicode-org.atlassian.net/browse/CLDR
                if displaynames.other.is_none() {
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
    use icu::plurals::PluralRules;
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
    let en_rules =
        PluralRules::try_new_cardinal_unstable(&provider, &langid!("en").into()).unwrap();
    assert_eq!(
        en.get().display_names.get(1.into(), &en_rules).1,
        "US dollar"
    );
    assert_eq!(
        en.get().display_names.get(10.into(), &en_rules).1,
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
    let fr_rules =
        PluralRules::try_new_cardinal_unstable(&provider, &langid!("fr").into()).unwrap();

    assert_eq!(
        fr.get().display_names.get(0.into(), &fr_rules).1,
        "dollar des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get(1.into(), &fr_rules).1,
        "dollar des États-Unis"
    );
    assert_eq!(
        fr.get().display_names.get(10.into(), &fr_rules).1,
        "dollars des États-Unis"
    );
}
