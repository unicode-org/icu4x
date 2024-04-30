// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::transform::cldr::cldr_serde;
use crate::provider::IterableDataProviderInternal;

use std::borrow::Cow;

use icu_experimental::dimension::provider::currency::PatternSelection;

use std::collections::HashSet;
use tinystr::tinystr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu_provider::DataProvider;

use icu_experimental::dimension::provider::extended_currency::*;
use icu_provider::prelude::*;

impl DataProvider<CurrencyExtendedDataV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyExtendedDataV1Marker>, DataError> {
        self.check_req::<CurrencyExtendedDataV1Marker>(req)?;

        let langid: icu_locid::LanguageIdentifier = req.locale.get_langid();
        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(&langid, "currencies.json")?;

        // let currencies = &currencies_resource.main.value.numbers.currencies;

        let usd = currencies_resource
            .main
            .value
            .numbers
            .currencies
            .get(&tinystr!(3, "USD").to_unvalidated())
            .ok_or(DataError::custom("remove"))?;

        let extended_placeholders = vec![
            usd.zero.as_deref(),
            usd.one.as_deref(),
            usd.two.as_deref(),
            usd.few.as_deref(),
            usd.many.as_deref(),
            usd.other.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<&str>>();

        let patterns_config = ZeroMap::new();

        let data = CurrencyExtendedDataV1 {
            patterns_config,
            other_pattern_config: ExtendedCurrencyPatternConfig {
                pattern_selection: PatternSelection::Standard,
                placeholder_index: None,
            },
            extended_placeholders: VarZeroVec::from(&extended_placeholders),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

// impl DatagenProvider {
//     fn supported_locales_currencies(
//         &self,
//         currency: Value,
//         keylengths: &'static [Subtag],
//         currencies: &BTreeMap<UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>,
//         lang_id: &icu_locid::LanguageIdentifier,
//     ) -> Result<HashSet<DataLocale>, DataError> {
//         let mut r = HashSet::new();

//         r.extend(currencies.keys().flat_map(|currency_iso| {
//             let locale: Locale = lang_id.clone().into();
//             let mut locale = DataLocale::from(locale);

//             locale.set_aux();
//             locale
//         }));

//         Ok(r)
//     }
// }

impl IterableDataProviderInternal<CurrencyExtendedDataV1Marker> for crate::DatagenProvider {
    fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}
