// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::experimental::dimension::provider::currency::fractions::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::ZeroMap;

impl DataProvider<CurrencyFractionsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyFractionsV1>, DataError> {
        self.check_req::<CurrencyFractionsV1>(req)?;

        let resource: &cldr_serde::currencies::supplemental::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/currencyData.json")?;

        let cldr_fractions = &resource.supplemental.currency_data.fractions;

        // Parse default values
        let default = parse_rounding_modes(&cldr_fractions.default)?;

        // Parse currency-specific overrides (only include if different from default)
        let mut fractions_map = std::collections::BTreeMap::new();
        for (iso_code, modes) in &cldr_fractions.currencies {
            let info = parse_rounding_modes(modes)?;
            // Only store if different from default
            if info != default {
                fractions_map.insert(iso_code.to_unvalidated(), info);
            }
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyFractions {
                fractions: ZeroMap::from_iter(fractions_map.iter()),
                default,
            }),
        })
    }
}

impl IterableDataProviderCached<CurrencyFractionsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        // Singleton data = one identifier (empty/default)
        Ok(HashSet::from_iter([Default::default()]))
    }
}

fn parse_rounding_modes(
    modes: &cldr_serde::currencies::supplemental::RoundingModes,
) -> Result<FractionInfo, DataError> {
    let digits = modes
        .digits
        .as_ref()
        .map(|s| s.parse::<u8>())
        .transpose()
        .map_err(|_| DataError::custom("Invalid digits value"))?
        .unwrap_or(2);

    let rounding = modes
        .rounding
        .as_ref()
        .map(|s| s.parse::<u8>())
        .transpose()
        .map_err(|_| DataError::custom("Invalid rounding value"))?
        .unwrap_or(0);

    let cash_digits = modes
        .cash_digits
        .as_ref()
        .map(|s| s.parse::<u8>())
        .transpose()
        .map_err(|_| DataError::custom("Invalid cash_digits value"))?;

    let cash_rounding = modes
        .cash_rounding
        .as_ref()
        .map(|s| s.parse::<u8>())
        .transpose()
        .map_err(|_| DataError::custom("Invalid cash_rounding value"))?;

    Ok(FractionInfo {
        digits,
        rounding,
        cash_digits,
        cash_rounding,
    })
}

#[test]
fn test_basic() {
    let provider = SourceDataProvider::new_testing();

    let response: DataResponse<CurrencyFractionsV1> = provider.load(Default::default()).unwrap();
    let data = response.payload.get();

    // Check default values (from CLDR DEFAULT entry: digits=2, rounding=0)
    assert_eq!(data.default.digits, 2);
    assert_eq!(data.default.rounding, 0);
    assert_eq!(data.default.cash_digits, None);
    assert_eq!(data.default.cash_rounding, None);

    // Check a currency with custom values (JPY has 0 digits)
    let jpy = tinystr::tinystr!(3, "JPY").to_unvalidated();
    if let Some(jpy_info) = data.fractions.get(&jpy) {
        let info: FractionInfo = zerovec::ule::AsULE::from_unaligned(*jpy_info);
        assert_eq!(info.digits, 0);
    }

    // Check a currency with cash rounding (CHF has cash_rounding=5)
    let chf = tinystr::tinystr!(3, "CHF").to_unvalidated();
    if let Some(chf_info) = data.fractions.get(&chf) {
        let info: FractionInfo = zerovec::ule::AsULE::from_unaligned(*chf_info);
        assert_eq!(info.cash_rounding, Some(5));
    }
}
