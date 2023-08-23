// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
use icu_provider::DataProvider;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use crate::transform::cldr::cldr_serde;
use crate::DatagenProvider;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_singlenumberformatter::provider::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use tinystr::tinystr;

/// Returns the pattern selection for a currency.
/// For example:
///    if the pattern is ¤#,##0.00 and the symbol is EGP,
///    this means the return value will be PatternSelection::StandardAlphaNextToNumber
///    because the character closes to the number is a letter.
fn currency_pattern_selection(
    provider: &DatagenProvider,
    pattern: String,
    place_holder: String,
) -> Result<PatternSelection, DataError> {
    // TODO(younies): what should we do when the place_holder is empty?
    if place_holder.is_empty() {
        todo!("Handle empty place holders")
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
            place_holder.chars().next_back().unwrap()
        } else if currency_sign_index > last_num_index {
            place_holder.chars().next().unwrap()
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

impl IterableDataProvider<CurrencyEssentialsV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
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
    let mut place_holders = Vec::<&str>::new();
    let mut place_holders_map = HashMap::<&str, u16>::new();
    for (iso, currency_pattern) in currencies {
        let short_place_holder_index: u16;
        let narrow_place_holder_index: u16;

        let short_option = &currency_pattern.short;
        match short_option {
            Some(short_place_holder) => match place_holders_map.get(short_place_holder.as_str()) {
                Some(index) => short_place_holder_index = *index,
                None => {
                    if short_place_holder == iso.try_into_tinystr().unwrap().as_str() {
                        short_place_holder_index = USE_ISO_CODE;
                    } else {
                        short_place_holder_index = place_holders.len() as u16;
                        //TODO(#3900): remove this assert and return an error instead.
                        assert!(short_place_holder_index <= MAX_PLACE_HOLDER_INDEX);
                        place_holders.push(short_place_holder);
                    }
                    place_holders_map.insert(short_place_holder, short_place_holder_index);
                }
            },
            None => short_place_holder_index = NO_PLACE_HOLDER,
        }

        let narrow_option = &currency_pattern.narrow;
        match narrow_option {
            Some(narrow_place_holder) => {
                match place_holders_map.get(narrow_place_holder.as_str()) {
                    Some(index) => narrow_place_holder_index = *index,
                    None => {
                        if narrow_place_holder == iso.try_into_tinystr().unwrap().as_str() {
                            narrow_place_holder_index = USE_ISO_CODE;
                        } else {
                            narrow_place_holder_index = place_holders.len() as u16;
                            //TODO(#3900): remove this assert and return an error instead.
                            assert!(narrow_place_holder_index <= MAX_PLACE_HOLDER_INDEX);
                            place_holders.push(narrow_place_holder);
                        }
                        place_holders_map.insert(narrow_place_holder, narrow_place_holder_index);
                    }
                }
            }
            None => narrow_place_holder_index = NO_PLACE_HOLDER,
        }

        let iso_string = iso.try_into_tinystr().unwrap().to_string();

        let short_pattern_standard: PatternSelection = {
            if standard_alpha_next_to_number.is_empty() {
                PatternSelection::Standard
            } else if short_place_holder_index != USE_ISO_CODE
                && short_place_holder_index < NO_PLACE_HOLDER
            {
                currency_pattern_selection(
                    provider,
                    standard.clone(),
                    place_holders
                        .get(short_place_holder_index as usize)
                        .unwrap()
                        .to_string(),
                )?
            } else {
                // TODO(younies): double check this case
                currency_pattern_selection(provider, standard.clone(), iso_string)?
            }
        };

        let narrow_pattern_standard: PatternSelection = {
            if standard_alpha_next_to_number.is_empty() {
                PatternSelection::Standard
            } else if narrow_place_holder_index != USE_ISO_CODE
                && narrow_place_holder_index != NO_PLACE_HOLDER
            {
                currency_pattern_selection(
                    provider,
                    standard.clone(),
                    place_holders
                        .get(narrow_place_holder_index as usize)
                        .unwrap()
                        .to_string(),
                )?
            } else {
                // TODO(younies): double check this case
                short_pattern_standard
            }
        };

        // TODO(youneis): Check if we can remove also when the patterns equal to
        // PatternSelection::StandardNextToNumber.
        if short_pattern_standard == PatternSelection::Standard
            && narrow_pattern_standard == PatternSelection::Standard
            && short_place_holder_index == NO_PLACE_HOLDER
            && narrow_place_holder_index == NO_PLACE_HOLDER
        {
            continue;
        }

        currency_patterns_map.insert(
            *iso,
            CurrencyPatterns {
                short_pattern_standard,
                narrow_pattern_standard,
                short_place_holder_index,
                narrow_place_holder_index,
            },
        );
    }

    Ok(CurrencyEssentialsV1 {
        currency_patterns_map: ZeroMap::from_iter(currency_patterns_map.iter()),
        standard: standard.to_owned().into(),
        standard_alpha_next_to_number: standard_alpha_next_to_number.to_owned().into(),
        place_holders: VarZeroVec::from(&place_holders),
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
            short_place_holder_index: NO_PLACE_HOLDER,
            narrow_place_holder_index: NO_PLACE_HOLDER,
        };
        let owned = locale.get().to_owned();
        let currency_pattern: CurrencyPatterns = owned
            .currency_patterns_map
            .get_copied(&iso_code)
            .unwrap_or(default);

        let short_place_holder = if currency_pattern.short_place_holder_index == NO_PLACE_HOLDER {
            "".to_string()
        } else if currency_pattern.short_place_holder_index == USE_ISO_CODE {
            iso_code.try_into_tinystr().unwrap().to_string()
        } else {
            place_holders
                .get(currency_pattern.short_place_holder_index as usize)
                .unwrap()
                .to_string()
        };

        let narrow_place_holder = if currency_pattern.narrow_place_holder_index == NO_PLACE_HOLDER {
            "".to_string()
        } else if currency_pattern.short_place_holder_index == USE_ISO_CODE {
            iso_code.try_into_tinystr().unwrap().to_string()
        } else {
            place_holders
                .get(currency_pattern.narrow_place_holder_index as usize)
                .unwrap()
                .to_string()
        };

        (short_place_holder, narrow_place_holder)
    }

    use icu_locid::locale;
    use icu_singlenumberformatter::provider::*;

    let provider = crate::DatagenProvider::latest_tested_offline_subset();

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
