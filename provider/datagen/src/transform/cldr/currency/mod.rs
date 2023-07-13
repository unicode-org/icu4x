// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
use icu_provider::DataProvider;
use zerovec::maps::MutableZeroVecLike;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use crate::transform::cldr::cldr_serde;
use crate::DatagenProvider;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_singlenumberformatter::provider::*;
use std::collections::HashMap;
use tinystr::tinystr;
use tinystr::TinyAsciiStr;

fn which_currency_pattern(
    provider: &DatagenProvider,
    pattern: String,
    place_holder: String,
) -> Result<bool, DataError> {
    // If the pattern is empty,
    // then return a letter (L) is close to the number.
    // Therefore, we use the standard_alpha_next_to_number pattern.
    if place_holder.is_empty() {
        return Ok(false);
    }

    let currency_sign = '¤';
    let currency_sign_ind = pattern.find(currency_sign).unwrap();
    let first_num_ind = pattern
        .chars()
        .enumerate()
        .find(|(_, c)| c == &'0' || c == &'#')
        .unwrap()
        .0;
    let last_num_ind = pattern.len()
        - pattern
            .chars()
            .rev()
            .enumerate()
            .find(|(_, c)| c == &'0' || c == &'#')
            .unwrap()
            .0
        - 1;

    let letters_set = match load_for_general_category_group(provider, GeneralCategoryGroup::Letter)
    {
        Ok(letters_set) => letters_set,
        Err(icu_properties::PropertiesError::PropDataLoad(e)) => {
            return Err(e);
        }
        Err(_) => unreachable!(),
    };

    let right_letter_test_position = {
        if currency_sign_ind < first_num_ind && currency_sign_ind < last_num_ind {
            true
        } else if currency_sign_ind > first_num_ind && currency_sign_ind > last_num_ind {
            false
        } else {
            panic!("The currency sign is in the middle of the pattern.")
        }
    };

    if right_letter_test_position {
        // Check if the last character has the property of letter (L).
        return Ok(!letters_set
            .as_borrowed()
            .contains(place_holder.chars().last().unwrap()));
    } else {
        // Check if the first character has the property of letter (L).
        return Ok(!letters_set
            .as_borrowed()
            .contains(place_holder.chars().next().unwrap()));
    }
}

impl DataProvider<CurrencyEssentialV1Maker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyEssentialV1Maker>, DataError> {
        let langid = req.locale.get_langid();

        let currencies_resource: &cldr_serde::currencies::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        // TODO: will be used in the next PR.
        // let currency_data: &cldr_serde::currency_data::Resource = self
        //     .source
        //     .cldr()?
        //     .core()
        //     .read_and_parse("supplemental/currencyData.json")?;

        let result = extract_currency_essential(
            self,
            &currencies_resource,
            &numbers_resource,
            // &currency_data,
            &langid,
        );

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}

impl IterableDataProvider<CurrencyEssentialV1Maker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .numbers()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}

fn extract_currency_essential<'data>(
    provider: &DatagenProvider,
    currencies_resource: &cldr_serde::currencies::Resource,
    numbers_resource: &cldr_serde::numbers::Resource,
    // currency_data_resource: &cldr_serde::currency_data::Resource,
    langid: &LanguageIdentifier,
) -> Result<CurrencyEssentialV1<'data>, DataError> {
    let currencies = &currencies_resource
        .main
        .0
        .get(&langid)
        .expect("CLDR file contains the expected language")
        .numbers
        .currencies;

    // TODO: will be used in the next PR.
    // let currency_data = &currency_data_resource
    //     .supplemental
    //     .currency_data
    //     .fractions
    //     .currencies;

    let currency_formats = &&numbers_resource
        .main
        .0
        .get(&langid)
        .expect("CLDR file contains the expected language")
        .numbers
        .numsys_data
        .currency_formats
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    let standard = &currency_formats.standard;
    let standard_alpha_next_to_number = match &currency_formats.standard_alpha_next_to_number {
        Some(standard_alpha_next_to_number) => standard_alpha_next_to_number,
        None => "",
    };

    let mut indices_map: ZeroMap<'_, TinyAsciiStr<3>, CurrencyPatterns> =
        ZeroMap::<TinyAsciiStr<3>, CurrencyPatterns>::new();
    let mut place_holders = VarZeroVec::<str>::new();
    let mut place_holders_map = HashMap::<String, u16>::new();
    for (iso, currency_pattern) in currencies {
        let short_place_holder_index: u16;
        let narrow_place_holder_index: u16;

        let short_option = &currency_pattern.short;
        match short_option {
            Some(short_place_holder) => match place_holders_map.get(short_place_holder) {
                Some(index) => short_place_holder_index = *index,
                None => {
                    short_place_holder_index = place_holders.len() as u16;
                    place_holders_map.insert(short_place_holder.clone(), short_place_holder_index);
                    place_holders.zvl_push(&short_place_holder);
                }
            },
            None => short_place_holder_index = u16::MAX,
        }

        let narrow_option = &currency_pattern.narrow;
        match narrow_option {
            Some(narrow_place_holder) => match place_holders_map.get(narrow_place_holder) {
                Some(index) => narrow_place_holder_index = *index,
                None => {
                    narrow_place_holder_index = place_holders.len() as u16;
                    place_holders_map
                        .insert(narrow_place_holder.clone(), narrow_place_holder_index);
                    place_holders.zvl_push(&narrow_place_holder);
                }
            },
            None => narrow_place_holder_index = u16::MAX,
        }

        let short_pattern_standard: bool = {
            if standard_alpha_next_to_number.is_empty() {
                true
            } else if short_place_holder_index != u16::MAX {
                which_currency_pattern(
                    provider,
                    standard.clone(),
                    place_holders
                        .get(short_place_holder_index as usize)
                        .unwrap()
                        .to_string(),
                )?
            } else {
                which_currency_pattern(provider, standard.clone(), iso.to_string())?
            }
        };

        let narrow_pattern_standard: bool = {
            if standard_alpha_next_to_number.is_empty() {
                true
            } else if narrow_place_holder_index != u16::MAX {
                which_currency_pattern(
                    provider,
                    standard.clone(),
                    place_holders
                        .get(narrow_place_holder_index as usize)
                        .unwrap()
                        .to_string(),
                )?
            } else {
                short_pattern_standard
            }
        };

        if short_pattern_standard
            && narrow_pattern_standard
            && short_place_holder_index == u16::MAX
            && narrow_place_holder_index == u16::MAX
        {
            continue;
        }

        indices_map.insert(
            iso,
            &CurrencyPatterns {
                short_pattern_standard,
                narrow_pattern_standard,
                short_place_holder_index,
                narrow_place_holder_index,
            },
        );
    }

    let result = CurrencyEssentialV1 {
        currency_patterns_map: indices_map,
        standard: standard.to_owned().into(),
        standard_alpha_next_to_number: standard_alpha_next_to_number.to_owned().into(),
        place_holders,
    };

    Ok(result)
}

// TODO(younies): How to not make this function dead?
fn get_place_holders_of_currency(
    iso_code: TinyAsciiStr<3>,
    locale: &DataPayload<CurrencyEssentialV1Maker>,
    place_holders: &VarZeroVec<'_, str>,
) -> (String, String) {
    let default = CurrencyPatternsULE {
        short_pattern_standard: true,
        narrow_pattern_standard: true,
        short_place_holder_index: u16::MAX.into(),
        narrow_place_holder_index: u16::MAX.into(),
    };
    let owned = locale.get().to_owned();
    let currency_pattern: &CurrencyPatternsULE =
        owned.currency_patterns_map.get(&iso_code).unwrap_or(&default);

    let short_place_holder = if currency_pattern.short_place_holder_index == u16::MAX.into() {
        "".to_string()
    } else {
        place_holders
            .get(currency_pattern.short_place_holder_index.as_unsigned_int() as usize)
            .unwrap()
            .to_string()
    };

    let narrow_place_holder = if currency_pattern.narrow_place_holder_index == u16::MAX.into() {
        "".to_string()
    } else {
        place_holders
            .get(currency_pattern.narrow_place_holder_index.as_unsigned_int() as usize)
            .unwrap()
            .to_string()
    };

    (short_place_holder, narrow_place_holder)
}

#[test]
fn test_basic() {
    use icu_locid::locale;
    use icu_singlenumberformatter::provider::*;
    let provider = crate::DatagenProvider::for_test();
    let ar_eg: DataPayload<CurrencyEssentialV1Maker> = provider
        .load(DataRequest {
            locale: &locale!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let place_holders = &ar_eg.get().to_owned().place_holders;

    let (ar_eg_short, ar_eg_narrow) =
        get_place_holders_of_currency(tinystr!(3, "EGP"), &ar_eg, place_holders);

    assert_eq!(
        ar_eg.clone().get().to_owned().standard,
        "‏#,##0.00 ¤;‏-#,##0.00 ¤"
    );
    assert_eq!(
        ar_eg.clone().get().to_owned().standard_alpha_next_to_number,
        ""
    );
    assert_eq!(ar_eg_short, "ج.م.\u{200f}");
    assert_eq!(ar_eg_narrow, "E£");
}
