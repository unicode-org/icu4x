// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::IterableDataProviderInternal;
use crate::transform::cldr::cldr_serde;

use crate::transform::cldr::cldr_serde::currencies;
use crate::transform::cldr::decimal::decimal_pattern::DecimalPattern;

use crate::DatagenProvider;

use std::borrow::Cow;

use icu_pattern::DoublePlaceholderPattern;
use rayon::iter::Map;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use tinystr::tinystr;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu_pattern::DoublePlaceholder;
use icu_pattern::DoublePlaceholderKey;
use icu_pattern::Pattern;
use icu_pattern::PatternItemCow;

use icu_experimental::dimension::ule::MAX_PLACEHOLDER_INDEX;
use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
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
            extended_placeholders: VarZeroVec::from(&extended_placeholders),
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

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
