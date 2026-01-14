// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::currency::extended::*;
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use std::collections::HashSet;

impl DataProvider<CurrencyExtendedDataV1> for crate::SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyExtendedDataV1>, DataError> {
        self.check_req::<CurrencyExtendedDataV1>(req)?;

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
            payload: DataPayload::from_owned(CurrencyExtendedData {
                display_names: PluralElements::new(
                    currency
                        .other
                        .as_deref()
                        .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?,
                )
                .with_zero_value(currency.zero.as_deref())
                .with_one_value(currency.one.as_deref())
                .with_two_value(currency.two.as_deref())
                .with_few_value(currency.few.as_deref())
                .with_many_value(currency.many.as_deref())
                .with_explicit_one_value(currency.explicit_one.as_deref())
                .with_explicit_zero_value(currency.explicit_zero.as_deref())
                .into(),
            }),
        })
    }
}

impl crate::IterableDataProviderCached<CurrencyExtendedDataV1> for SourceDataProvider {
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
                    result.insert(DataIdentifierCow::from_owned(attributes, locale));
                }
            }
        }

        Ok(result)
    }
}
