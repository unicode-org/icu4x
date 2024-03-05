// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::dimension::ule::MAX_PLACE_HOLDER_INDEX;
use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
use icu_provider::DataProvider;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use crate::provider::IterableDataProviderInternal;
use crate::transform::cldr::cldr_serde;
use crate::DatagenProvider;
use icu_experimental::dimension::provider::*;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use tinystr::tinystr;

/// Returns the pattern selection for a currency.
/// For example:
///    if the pattern is ¤#,##0.00 and the symbol is EGP,
///    this means the return value will be PatternSelection::StandardAlphaNextToNumber
///    because the character closes to the number is a letter.
/// NOTE:
///   place_holder_value must not be empty.
fn currency_pattern_selection(
    provider: &DatagenProvider,
    pattern: &str,
    place_holder_value: &str,
) -> Result<PatternSelection, DataError> {
    if place_holder_value.is_empty() {
        return Err(DataError::custom("Place holder value must not be empty"));
    }

    let currency_sign = '¤';
    let currency_sign_index = pattern.find(currency_sign).unwrap();
    let first_num_index = pattern.find(['0', '#']).unwrap();
    let last_num_index = pattern.rfind(['0', '#']).unwrap();

    let letters_set = match load_for_general_category_group(provider, GeneralCategoryGroup::Letter)
    {
        Ok(letters_set) => letters_set,
        Err(icu_properties::PropertiesError::PropDataLoad(e)) => {
            return Err(e);
        }
        Err(_) => unreachable!("load_for_general_category_group should only return PropDataLoad"),
    };

    let char_closer_to_number = {
        if currency_sign_index < first_num_index {
            place_holder_value.chars().next_back().unwrap()
        } else if currency_sign_index > last_num_index {
            place_holder_value.chars().next().unwrap()
        } else {
            return Err(DataError::custom(
                "Currency sign must be in the middle of the pattern",
            ));
        }
    };

    if letters_set.as_borrowed().contains(char_closer_to_number) {
        Ok(PatternSelection::StandardAlphaNextToNumber)
    } else {
        Ok(PatternSelection::Standard)
    }
}

impl DataProvider<CurrencyEssentialsV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyEssentialsV1Marker>, DataError> {
        self.check_req::<CurrencyEssentialsV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let currencies_resource: &cldr_serde::currencies::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let result = extract_currency_essentials(self, currencies_resource, numbers_resource);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}

impl IterableDataProviderInternal<CurrencyEssentialsV1Marker> for crate::DatagenProvider {
    fn supported_locales_impl(&self) -> Result<HashSet<DataLocale>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}

fn extract_currency_essentials<'data>(
    provider: &DatagenProvider,
    currencies_resource: &cldr_serde::currencies::Resource,
    numbers_resource: &cldr_serde::numbers::Resource,
) -> Result<CurrencyEssentialsV1<'data>, DataError> {
    let currencies = &currencies_resource.main.value.numbers.currencies;

    // TODO(#3838): these patterns might be numbering system dependent.
    let currency_formats = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .currency_patterns
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    let standard = &currency_formats.standard;
    let standard_alpha_next_to_number = match &currency_formats.standard_alpha_next_to_number {
        Some(standard_alpha_next_to_number) => standard_alpha_next_to_number,
        None => "",
    };

    let mut currency_patterns_map = BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>::new();
    let mut currency_patterns_standard_none =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>::new();
    let mut currency_patterns_standard_next_to_num =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatterns>::new();
    let mut place_holders = Vec::<&str>::new();
    // A map to check if the place holder is already in the place_holders vector.
    let mut place_holders_checker_map = HashMap::<&str, u16>::new();

    for (iso, currency_pattern) in currencies {
        let short_place_holder_index = currency_pattern.short.as_ref().map(|short_place_holder| {
            if let Some(&index) = place_holders_checker_map.get(short_place_holder.as_str()) {
                PlaceholderValue::Index(index)
            } else if short_place_holder == iso.try_into_tinystr().unwrap().as_str() {
                PlaceholderValue::ISO
            } else {
                let index = place_holders.len() as u16;
                place_holders.push(short_place_holder.as_str());
                place_holders_checker_map.insert(short_place_holder.as_str(), index);
                PlaceholderValue::Index(index)
            }
        });

        let narrow_place_holder_index =
            currency_pattern.narrow.as_ref().map(|narrow_place_holder| {
                if let Some(&index) = place_holders_checker_map.get(narrow_place_holder.as_str()) {
                    PlaceholderValue::Index(index)
                } else if narrow_place_holder == iso.try_into_tinystr().unwrap().as_str() {
                    PlaceholderValue::ISO
                } else {
                    let index = place_holders.len() as u16;
                    place_holders.push(narrow_place_holder.as_ref());
                    place_holders_checker_map.insert(narrow_place_holder.as_str(), index);
                    PlaceholderValue::Index(index)
                }
            });

        // Ensure that short_place_holder_index and narrow_place_holder_index do not exceed MAX_PLACE_HOLDER_INDEX.
        if let Some(PlaceholderValue::Index(index)) = short_place_holder_index {
            if index > MAX_PLACE_HOLDER_INDEX {
                return Err(DataError::custom(
                    "short_place_holder_index exceeded MAX_PLACE_HOLDER_INDEX",
                ));
            }
        }
        if let Some(PlaceholderValue::Index(index)) = narrow_place_holder_index {
            if index > MAX_PLACE_HOLDER_INDEX {
                return Err(DataError::custom(
                    "narrow_place_holder_index exceeded MAX_PLACE_HOLDER_INDEX",
                ));
            }
        }

        let iso_string = iso.try_into_tinystr().unwrap().to_string();

        let short_pattern_standard: PatternSelection = if standard_alpha_next_to_number.is_empty() {
            PatternSelection::Standard
        } else {
            match short_place_holder_index {
                Some(PlaceholderValue::Index(index)) => currency_pattern_selection(
                    provider,
                    standard,
                    place_holders.get(index as usize).unwrap(),
                )?,
                Some(PlaceholderValue::ISO) => {
                    currency_pattern_selection(provider, standard, iso_string.as_str())?
                }
                // Based on UTS-35: https://www.unicode.org/reports/tr35/tr35-numbers.html#Currencies
                // If the place holder value is empty, then use the currency code (ISO code).
                None => currency_pattern_selection(provider, standard, iso_string.as_str())?,
            }
        };

        let narrow_pattern_standard: PatternSelection = if standard_alpha_next_to_number.is_empty()
        {
            PatternSelection::Standard
        } else {
            match narrow_place_holder_index {
                Some(PlaceholderValue::Index(index)) => currency_pattern_selection(
                    provider,
                    standard,
                    place_holders.get(index as usize).unwrap(),
                )?,
                Some(PlaceholderValue::ISO) => {
                    currency_pattern_selection(provider, standard, &iso_string)?
                }

                // Based on UTS-35: https://www.unicode.org/reports/tr35/tr35-numbers.html#Currencies
                // If the place holder value is empty, then use the currency code (ISO code).
                None => currency_pattern_selection(provider, standard, &iso_string)?,
            }
        };

        let currency_patterns = CurrencyPatterns {
            short_pattern_standard,
            narrow_pattern_standard,
            short_place_holder_index,
            narrow_place_holder_index,
        };

        match (short_pattern_standard, narrow_pattern_standard) {
            (PatternSelection::Standard, PatternSelection::Standard)
                if short_place_holder_index.is_none() && narrow_place_holder_index.is_none() =>
            {
                currency_patterns_standard_none.insert(*iso, currency_patterns);
            }
            (
                PatternSelection::StandardAlphaNextToNumber,
                PatternSelection::StandardAlphaNextToNumber,
            ) if short_place_holder_index.is_none() && narrow_place_holder_index.is_none() => {
                currency_patterns_standard_next_to_num.insert(*iso, currency_patterns);
            }
            _ => {
                currency_patterns_map.insert(*iso, currency_patterns);
            }
        }
    }

    let default_pattern =
        if currency_patterns_standard_none.len() <= currency_patterns_standard_next_to_num.len() {
            currency_patterns_map.extend(currency_patterns_standard_none);
            CurrencyPatterns {
                short_pattern_standard: PatternSelection::StandardAlphaNextToNumber,
                narrow_pattern_standard: PatternSelection::StandardAlphaNextToNumber,
                short_place_holder_index: None,
                narrow_place_holder_index: None,
            }
        } else {
            currency_patterns_map.extend(currency_patterns_standard_next_to_num);
            CurrencyPatterns {
                short_pattern_standard: PatternSelection::Standard,
                narrow_pattern_standard: PatternSelection::Standard,
                short_place_holder_index: None,
                narrow_place_holder_index: None,
            }
        };

    Ok(CurrencyEssentialsV1 {
        currency_patterns_map: ZeroMap::from_iter(currency_patterns_map.iter()),
        standard: standard.to_owned().into(),
        standard_alpha_next_to_number: standard_alpha_next_to_number.to_owned().into(),
        place_holders: VarZeroVec::from(&place_holders),
        default_pattern,
    })
}

#[test]
fn test_basic() {
    fn get_place_holders_of_currency(
        iso_code: UnvalidatedTinyAsciiStr<3>,
        locale: &DataPayload<CurrencyEssentialsV1Marker>,
        place_holders: &VarZeroVec<'_, str>,
    ) -> (String, String) {
        let default = CurrencyPatterns {
            short_pattern_standard: PatternSelection::Standard,
            narrow_pattern_standard: PatternSelection::Standard,
            short_place_holder_index: None,
            narrow_place_holder_index: None,
        };
        let owned = locale.get().to_owned();
        let currency_pattern: CurrencyPatterns = owned
            .currency_patterns_map
            .get_copied(&iso_code)
            .unwrap_or(default);

        let short_place_holder = match currency_pattern.short_place_holder_index {
            Some(PlaceholderValue::Index(index)) => place_holders
                .get(index as usize)
                .unwrap_or(&iso_code.try_into_tinystr().unwrap())
                .to_string(),
            Some(PlaceholderValue::ISO) => iso_code.try_into_tinystr().unwrap().to_string(),
            None => "".to_string(),
        };

        let narrow_place_holder = match currency_pattern.narrow_place_holder_index {
            Some(PlaceholderValue::Index(index)) => place_holders
                .get(index as usize)
                .unwrap_or(&iso_code.try_into_tinystr().unwrap())
                .to_string(),
            Some(PlaceholderValue::ISO) => iso_code.try_into_tinystr().unwrap().to_string(),
            None => "".to_string(),
        };

        (short_place_holder, narrow_place_holder)
    }

    use icu_experimental::dimension::provider::*;
    use icu_locid::locale;

    let provider = crate::DatagenProvider::new_testing();

    let en: DataPayload<CurrencyEssentialsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("en").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let en_place_holders = &en.get().to_owned().place_holders;
    assert_eq!(en.clone().get().to_owned().standard, "¤#,##0.00");
    assert_eq!(
        en.clone().get().to_owned().standard_alpha_next_to_number,
        "¤\u{a0}#,##0.00"
    );

    let (en_usd_short, en_usd_narrow) =
        get_place_holders_of_currency(tinystr!(3, "USD").to_unvalidated(), &en, en_place_holders);
    assert_eq!(en_usd_short, "$");
    assert_eq!(en_usd_narrow, "$");

    let (en_egp_short, en_egp_narrow) =
        get_place_holders_of_currency(tinystr!(3, "EGP").to_unvalidated(), &en, en_place_holders);
    assert_eq!(en_egp_short, "");
    assert_eq!(en_egp_narrow, "E£");

    let ar_eg: DataPayload<CurrencyEssentialsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let ar_eg_place_holders = &ar_eg.get().to_owned().place_holders;

    assert_eq!(
        ar_eg.clone().get().to_owned().standard,
        "‏#,##0.00 ¤;‏-#,##0.00 ¤"
    );
    assert_eq!(
        ar_eg.clone().get().to_owned().standard_alpha_next_to_number,
        ""
    );
    let (ar_eg_egp_short, ar_eg_egp_narrow) = get_place_holders_of_currency(
        tinystr!(3, "EGP").to_unvalidated(),
        &ar_eg,
        ar_eg_place_holders,
    );
    assert_eq!(ar_eg_egp_short, "ج.م.\u{200f}");
    assert_eq!(ar_eg_egp_narrow, "E£");

    let (ar_eg_usd_short, ar_eg_usd_narrow) = get_place_holders_of_currency(
        tinystr!(3, "USD").to_unvalidated(),
        &ar_eg,
        ar_eg_place_holders,
    );
    assert_eq!(ar_eg_usd_short, "US$");
    assert_eq!(ar_eg_usd_narrow, "US$");
}
