// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::transform::cldr::cldr_serde;
use crate::DatagenProvider;

use std::borrow::Cow;
use std::str::FromStr;

use icu_experimental::dimension::currency::formatter::CurrencyCode;
use icu_experimental::dimension::provider::currency::PatternSelection;
use icu_provider::datagen::IterableDataProvider;
use tinystr::TinyAsciiStr;
use tinystr::UnvalidatedTinyAsciiStr;

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

        let langid = req.locale.get_langid();
        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(&langid, "currencies.json")?;

        // let currencies = &currencies_resource.main.value.numbers.currencies;

        let aux = req
            .key_attributes
            .parse::<TinyAsciiStr<3>>()
            .map_err(|_| DataError::custom("failed to parse aux key into tinystr"))?;
        let usd = currencies_resource
            .main
            .value
            .numbers
            .currencies
            .get(&aux.to_unvalidated())
            .ok_or(DataError::custom("remove"))?;

        let extended_placeholders = vec![
            usd.display_name.as_deref(),
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

impl IterableDataProvider<CurrencyExtendedDataV1Marker> for DatagenProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
        let mut result = HashSet::new();
        let numbers = self.cldr()?.numbers();
        let langids = numbers.list_langs()?;
        for langid in langids {
            let currencies_resource: &cldr_serde::currencies::data::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&langid, "currencies.json")?;

            let currencies = &currencies_resource.main.value.numbers.currencies;
            for (key, _) in currencies {
                let key = key
                    .try_into_tinystr()
                    .map_err(|_| DataError::custom("failed to parse currency code into tinystr"))?;

                let attributes = DataKeyAttributes::from_tinystr(key.resize());
                result.insert((DataLocale::from(&langid), attributes));
            }
        }

        Ok(result)
    }
}
