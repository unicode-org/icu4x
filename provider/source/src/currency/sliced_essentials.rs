// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::decimal::decimal_pattern::DecimalPattern;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::borrow::Cow;



use icu_pattern::DoublePlaceholderPattern;
use tinystr::TinyAsciiStr;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu_pattern::DoublePlaceholderKey;
use icu_pattern::PatternItemCow;

use icu::experimental::dimension::provider::currency::ule::MAX_PLACEHOLDER_INDEX;
use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
use icu::properties::CodePointMapData;
use icu_provider::DataProvider;

use icu::experimental::dimension::currency::CurrencyCode;
use icu::experimental::dimension::provider::currency::essentials::*;
use icu::locale::subtags::Region;
use icu_provider::prelude::*;


use crate::currency::essentials::extract_currency_essentials;

/// Core currencies that are commonly used worldwide.
pub(crate) const GLOBAL_CORE_CURRENCIES: &[CurrencyCode] = &[
    CurrencyCode(tinystr::tinystr!(3, "USD")), // US Dollar
    CurrencyCode(tinystr::tinystr!(3, "EUR")), // Euro
    CurrencyCode(tinystr::tinystr!(3, "GBP")), // British Pound
    CurrencyCode(tinystr::tinystr!(3, "CHF")), // Swiss Franc
];

/// Describes the slicing strategy for which currencies to include in the data.
/// - `Regional`: Include only those currencies relevant to a given region.
/// - `Core`: Include a set of core currencies that are commonly used worldwide.
/// - `Complete`: Include all the remaining currencies that are not included in the other strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CurrencySlicingType {
    Regional,
    Core,
    Complete,
}

/// Returns true if the currency is currently valid (in use as legal tender).
/// A currency is valid if:
/// - It has a `_from` date (started being used)
/// - It does NOT have a `_to` date (still in use)
/// - It is legal tender: `_tender` is either absent (defaults to true) or not `"false"`
fn is_current_legal_tender(
    data: &cldr_serde::currencies::supplemental::RegionCurrencyData,
) -> bool {
    data.from.is_some() && data.to.is_none() && data.tender.as_deref() != Some("false")
}

/// Extracts all currently valid currencies for a given region from the supplemental data.
/// Returns a list of currency codes that are currently legal tender in the region.
fn extract_current_currencies_for_region(
    region: Region,
    supplemental_resource: &cldr_serde::currencies::supplemental::Resource,
) -> Vec<CurrencyCode> {
    let Some(region_currencies) = supplemental_resource
        .supplemental
        .currency_data
        .region
        .get(&region)
    else {
        return Vec::new();
    };

    region_currencies
        .iter()
        .flat_map(|entry| {
            entry
                .iter()
                .filter(|(_, data)| is_current_legal_tender(data))
                .map(|(code, _)| *code)
        })
        .collect()
}

/// Extracts the needed map of currencies patterns based on the slicing strategy.
///
/// # Arguments
/// * `currency_slicing_type` - The slicing strategy to use
/// * `region` - The region to extract currencies for
/// * `currencies` - The full map of available currency patterns
/// * `supplemental_resource` - The supplemental resource containing region-currency mappings
///
/// # Returns
/// A filtered map of currency patterns based on the slicing strategy:
/// - `Regional`: Only currencies currently valid in the given region
/// - `Core`: Global core currencies (USD, EUR, GBP, CHF) plus regional currencies
/// - `Complete`: All remaining currencies not included in Regional or Core
fn extract_needed_currencies(
    currency_slicing_type: CurrencySlicingType,
    region: Region,
    currencies: &BTreeMap<String, cldr_serde::currencies::data::CurrencyPatterns>,
    supplemental_resource: &cldr_serde::currencies::supplemental::Resource,
) -> BTreeMap<String, cldr_serde::currencies::data::CurrencyPatterns> {
    // Get the regional currencies (used by Regional and Core strategies)
    let regional_currencies = extract_current_currencies_for_region(region, supplemental_resource);

    // Build the set of "core + regional" currencies for filtering in Complete
    let core_and_regional: HashSet<String> = GLOBAL_CORE_CURRENCIES
        .iter()
        .map(|c| c.0.to_string())
        .chain(regional_currencies.iter().map(|c| c.0.to_string()))
        .collect();

    match currency_slicing_type {
        CurrencySlicingType::Regional => {
            // Include only currencies currently valid in the given region
            let regional_set: HashSet<String> =
                regional_currencies.iter().map(|c| c.0.to_string()).collect();
            currencies
                .iter()
                .filter(|(code, _)| regional_set.contains(*code))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
        CurrencySlicingType::Core => {
            // Include the global core currencies plus the regional currencies
            currencies
                .iter()
                .filter(|(code, _)| core_and_regional.contains(*code))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
        CurrencySlicingType::Complete => {
            // Include all remaining currencies NOT in Regional or Core
            currencies
                .iter()
                .filter(|(code, _)| !core_and_regional.contains(*code))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
    }
}

impl DataProvider<CurrencyEssentialsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyEssentialsV1>, DataError> {
        self.check_req::<CurrencyEssentialsV1>(req)?;

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(req.id.locale, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let result = extract_currency_essentials(
            self,
            &currencies_resource.main.value.numbers.currencies,
            numbers_resource,
        );

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<CurrencyEssentialsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}
