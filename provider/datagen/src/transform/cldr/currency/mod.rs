// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
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
    let first_num_index = pattern.find(|c| c == '0' || c == '#').unwrap();
    let last_num_index = pattern.rfind(|c: char| c == '0' || c == '#').unwrap();

    let letters_set = match load_for_general_category_group(provider, GeneralCategoryGroup::Letter)
    {
        Ok(letters_set) => letters_set,
        Err(icu_properties::PropertiesError::PropDataLoad(e)) => {
            return Err(e);
        }
        Err(_) => unreachable!("load_for_general_category_group should only return PropDataLoad"),
    };

    #[derive(PartialEq)]
    enum PlaceHolderCharCloserToNumber {
        First,
        Last,
    }

    let char_closer_to_number = {
        if currency_sign_index < first_num_index {
            PlaceHolderCharCloserToNumber::Last
        } else if currency_sign_index > last_num_index {
            PlaceHolderCharCloserToNumber::First
        } else {
            return Err(DataError::custom(
                "Currency sign must be in the middle of the pattern",
            ));
        }
    };

    if char_closer_to_number == PlaceHolderCharCloserToNumber::Last {
        match letters_set
            .as_borrowed()
            .contains(place_holder.chars().last().unwrap())
        {
            true => Ok(PatternSelection::StandardAlphaNextToNumber),
            false => Ok(PatternSelection::Standard),
        }
    } else {
        match letters_set
            .as_borrowed()
            .contains(place_holder.chars().next().unwrap())
        {
            true => Ok(PatternSelection::StandardAlphaNextToNumber),
            false => Ok(PatternSelection::Standard),
        }
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
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let result =
            extract_currency_essentials(self, currencies_resource, numbers_resource, &langid);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}

impl IterableDataProvider<CurrencyEssentialsV1Marker> for crate::DatagenProvider {
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

fn extract_currency_essentials<'data>(
    provider: &DatagenProvider,
    currencies_resource: &cldr_serde::currencies::Resource,
    numbers_resource: &cldr_serde::numbers::Resource,
    langid: &LanguageIdentifier,
) -> Result<CurrencyEssentialsV1<'data>, DataError> {
    let currencies = &currencies_resource
        .main
        .0
        .get(langid)
        .expect("CLDR file contains the expected language")
        .numbers
        .currencies;

    let currency_formats = &&numbers_resource
        .main
        .0
        .get(langid)
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
                    short_place_holder_index = place_holders.len() as u16;
                    place_holders_map.insert(short_place_holder, short_place_holder_index);
                    place_holders.push(short_place_holder);
                }
            },
            None => short_place_holder_index = u16::MAX,
        }

        let narrow_option = &currency_pattern.narrow;
        match narrow_option {
            Some(narrow_place_holder) => {
                match place_holders_map.get(narrow_place_holder.as_str()) {
                    Some(index) => narrow_place_holder_index = *index,
                    None => {
                        narrow_place_holder_index = place_holders.len() as u16;
                        place_holders_map.insert(narrow_place_holder, narrow_place_holder_index);
                        place_holders.push(narrow_place_holder);
                    }
                }
            }
            None => narrow_place_holder_index = u16::MAX,
        }

        let iso_string = iso.try_into_tinystr().unwrap().to_string();

        let short_pattern_standard: PatternSelection = {
            if standard_alpha_next_to_number.is_empty() {
                PatternSelection::Standard
            } else if short_place_holder_index != u16::MAX {
                currency_pattern_selection(
                    provider,
                    standard.clone(),
                    place_holders
                        .get(short_place_holder_index as usize)
                        .unwrap()
                        .to_string(),
                )?
            } else {
                currency_pattern_selection(provider, standard.clone(), iso_string)?
            }
        };

        let narrow_pattern_standard: PatternSelection = {
            if standard_alpha_next_to_number.is_empty() {
                PatternSelection::Standard
            } else if narrow_place_holder_index != u16::MAX {
                currency_pattern_selection(
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

        // TODO(youneis): Check if we can remove also when the patterns equal to
        // PatternSelection::StandardNextToNumber.
        if short_pattern_standard == PatternSelection::Standard
            && narrow_pattern_standard == PatternSelection::Standard
            && short_place_holder_index == u16::MAX
            && narrow_place_holder_index == u16::MAX
        {
            continue;
        }

        currency_patterns_map.insert(
            *iso,
            CurrencyPatterns {
                short_pattern_standard: short_pattern_standard as u8,
                narrow_pattern_standard: narrow_pattern_standard as u8,
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
        let default = CurrencyPatternsULE {
            short_pattern_standard: PatternSelection::Standard as u8,
            narrow_pattern_standard: PatternSelection::Standard as u8,
            short_place_holder_index: u16::MAX.into(),
            narrow_place_holder_index: u16::MAX.into(),
        };
        let owned = locale.get().to_owned();
        let currency_pattern: &CurrencyPatternsULE = owned
            .currency_patterns_map
            .get(&iso_code)
            .unwrap_or(&default);

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

    use icu_locid::locale;
    use icu_singlenumberformatter::provider::*;

    let provider = crate::DatagenProvider::for_test();

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
