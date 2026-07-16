// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use tinystr::TinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;
use zerovec::ule::vartuple::VarTuple;

use icu::experimental::dimension::provider::currency::symbols::*;
use icu::experimental::dimension::provider::currency::ule::MAX_SYMBOL_INDEX;
use icu::properties::CodePointMapData;
use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
use icu_provider::DataProvider;
use icu_provider::prelude::*;

impl DataProvider<CurrencySymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencySymbolsV1>, DataError> {
        self.check_req::<CurrencySymbolsV1>(req)?;

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(req.id.locale, "currencies.json")?;

        let symbols = extract_currency_symbols(self, currencies_resource)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(symbols),
        })
    }
}

impl IterableDataProviderCached<CurrencySymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers_with_locales()
    }
}

fn extract_currency_symbols<'data>(
    provider: &SourceDataProvider,
    currencies_resource: &cldr_serde::currencies::data::Resource,
) -> Result<CurrencySymbols<'data>, DataError> {
    let currencies = &currencies_resource.main.value.numbers.currencies;

    let letters_set = CodePointMapData::<GeneralCategory>::try_new_unstable(provider)?
        .as_borrowed()
        .get_set_for_value_group(GeneralCategoryGroup::Letter);

    let mut currency_patterns_map = BTreeMap::new();
    let mut symbols = Vec::new();
    let mut symbols_checker_map = HashMap::new();

    fn intern_symbol<'a>(
        symbol: &'a str,
        iso: &str,
        symbols: &mut Vec<&'a str>,
        symbols_checker_map: &mut HashMap<&'a str, u16>,
    ) -> Result<CurrencySymbolIndex, DataError> {
        if let Some(&index) = symbols_checker_map.get(symbol) {
            Ok(CurrencySymbolIndex::Index(index))
        } else if symbol == iso {
            Ok(CurrencySymbolIndex::ISO)
        } else {
            let index = symbols.len() as u16;
            if index > MAX_SYMBOL_INDEX {
                return Err(DataError::custom("symbol index exceeded MAX_SYMBOL_INDEX"));
            }
            symbols.push(symbol);
            symbols_checker_map.insert(symbol, index);
            Ok(CurrencySymbolIndex::Index(index))
        }
    }

    for (iso, currency_pattern) in currencies {
        let short_symbol = currency_pattern
            .short
            .as_ref()
            .map(|p| intern_symbol(p.as_str(), iso, &mut symbols, &mut symbols_checker_map))
            .transpose()?;

        let narrow_symbol = currency_pattern
            .narrow
            .as_ref()
            .map(|p| intern_symbol(p.as_str(), iso, &mut symbols, &mut symbols_checker_map))
            .transpose()?;

        if short_symbol.is_some() || narrow_symbol.is_some() {
            currency_patterns_map.insert(
                TinyAsciiStr::try_from_str(iso).unwrap().to_unvalidated(),
                CurrencyPatternConfig {
                    short_symbol,
                    narrow_symbol,
                },
            );
        }
    }

    let symbols = symbols
        .into_iter()
        .map(|s| {
            // TODO: This is not entirely correct. We need to look at the first/last grapheme cluster.
            let starts_with_letter = letters_set
                .as_borrowed()
                .contains(s.chars().next().unwrap());
            let ends_with_letter = letters_set
                .as_borrowed()
                .contains(s.chars().next_back().unwrap());
            VarTuple {
                sized: (starts_with_letter as u8) << 1 | (ends_with_letter as u8),
                variable: s,
            }
        })
        .collect::<Vec<_>>();

    Ok(CurrencySymbols {
        pattern_config_map: ZeroMap::from_iter(currency_patterns_map),
        symbols: VarZeroVec::from(&symbols),
    })
}

#[test]
fn test_symbols() {
    use icu::experimental::dimension::currency::CurrencyCode;
    use icu::experimental::dimension::provider::currency::symbols::Width;
    use icu::locale::langid;
    use tinystr::tinystr;

    const USD: CurrencyCode = CurrencyCode(tinystr!(3, "USD"));
    const EGP: CurrencyCode = CurrencyCode(tinystr!(3, "EGP"));
    let provider = SourceDataProvider::new_testing();

    let en: DataPayload<CurrencySymbolsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(
        en.get().get(Width::Short, &USD),
        CurrencySymbol {
            symbol: "$",
            starts_with_letter: false,
            ends_with_letter: false
        }
    );
    assert_eq!(
        en.get().get(Width::Narrow, &USD),
        CurrencySymbol {
            symbol: "$",
            starts_with_letter: false,
            ends_with_letter: false
        }
    );

    // TODO(#6064)
    assert_eq!(
        en.get().get(Width::Short, &EGP),
        CurrencySymbol {
            symbol: "EGP",
            starts_with_letter: true,
            ends_with_letter: true
        }
    );
    assert_eq!(
        en.get().get(Width::Narrow, &EGP),
        CurrencySymbol {
            symbol: "E£",
            starts_with_letter: true,
            ends_with_letter: false
        }
    );

    let ar_eg: DataPayload<CurrencySymbolsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ar-EG").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(
        ar_eg.get().get(Width::Short, &EGP),
        CurrencySymbol {
            symbol: "ج.م.\u{200f}",
            starts_with_letter: false,
            ends_with_letter: false
        }
    );
    assert_eq!(
        ar_eg.get().get(Width::Narrow, &EGP),
        CurrencySymbol {
            symbol: "E£",
            starts_with_letter: true,
            ends_with_letter: false
        }
    );

    assert_eq!(
        ar_eg.get().get(Width::Short, &USD),
        CurrencySymbol {
            symbol: "US$",
            starts_with_letter: true,
            ends_with_letter: false
        }
    );
    assert_eq!(
        ar_eg.get().get(Width::Narrow, &USD),
        CurrencySymbol {
            symbol: "US$",
            starts_with_letter: true,
            ends_with_letter: false
        }
    );
}
