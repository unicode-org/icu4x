// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::currency::essentials::extract_currency_essentials;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::HashSet;

use icu_provider::DataProvider;

use icu::experimental::dimension::provider::currency::essentials::*;
use icu_provider::prelude::*;

impl DataProvider<CurrencyEssentialsRootV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyEssentialsRootV1>, DataError> {
        self.check_req::<CurrencyEssentialsRootV1>(req)?;

        let root_locale = self.cldr()?.script_locale_group(req.id.locale)?;

        println!("root_locale: {:?}", root_locale);
        println!("req.id.locale: {:?}", req.id.locale);

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(&root_locale, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&root_locale, "numbers.json")?;

        let result = extract_currency_essentials(self, currencies_resource, numbers_resource);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<CurrencyEssentialsRootV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}
