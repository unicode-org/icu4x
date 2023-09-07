// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_properties::sets::load_for_general_category_group;
use icu_properties::GeneralCategoryGroup;
use icu_provider::DataProvider;
use icu_singlenumberformatter::ule::MAX_INJECTING_TEXT_INDEX;
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
    pattern: &str,
    injecting_text: &str,
) -> Result<PatternSelection, DataError> {
    // TODO(younies): what should we do when the injecting_text is empty?
    if injecting_text.is_empty() {
        todo!("Handle empty injecting texts")
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
            injecting_text.chars().next_back().unwrap()
        } else if currency_sign_index > last_num_index {
            injecting_text.chars().next().unwrap()
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
    let mut injecting_texts = Vec::<&str>::new();
    // A map to check if the injecting text is already in the injecting_texts vector.
    let mut injecting_texts_checker_map = HashMap::<&str, u16>::new();

    for (iso, currency_pattern) in currencies {
        let short_injecting_text_index =
            currency_pattern.short.as_ref().map(|short_injecting_text| {
                if let Some(&index) = injecting_texts_checker_map.get(short_injecting_text.as_str())
                {
                    InjectingText::Index(index)
                } else if short_injecting_text == iso.try_into_tinystr().unwrap().as_str() {
                    InjectingText::ISO
                } else {
                    let index = injecting_texts.len() as u16;
                    //TODO(#3900): remove this assert and return an error instead.
                    assert!(index <= MAX_INJECTING_TEXT_INDEX);
                    injecting_texts.push(short_injecting_text.as_str());
                    injecting_texts_checker_map.insert(short_injecting_text.as_str(), index);
                    InjectingText::Index(index)
                }
            });

        let narrow_injecting_text_index =
            currency_pattern
                .narrow
                .as_ref()
                .map(|narrow_injecting_text| {
                    if let Some(&index) =
                        injecting_texts_checker_map.get(narrow_injecting_text.as_str())
                    {
                        InjectingText::Index(index)
                    } else if narrow_injecting_text == iso.try_into_tinystr().unwrap().as_str() {
                        InjectingText::ISO
                    } else {
                        let index = injecting_texts.len() as u16;
                        //TODO(#3900): remove this assert and return an error instead.
                        assert!(index <= MAX_INJECTING_TEXT_INDEX);
                        injecting_texts.push(narrow_injecting_text.as_ref());
                        injecting_texts_checker_map.insert(narrow_injecting_text.as_str(), index);
                        InjectingText::Index(index)
                    }
                });

        let iso_string = iso.try_into_tinystr().unwrap().to_string();

        let short_pattern_standard: PatternSelection = if standard_alpha_next_to_number.is_empty() {
            PatternSelection::Standard
        } else {
            match short_injecting_text_index {
                Some(InjectingText::Index(index)) => currency_pattern_selection(
                    provider,
                    standard,
                    injecting_texts.get(index as usize).unwrap(),
                )?,
                Some(InjectingText::ISO) => {
                    currency_pattern_selection(provider, standard, &iso_string)?
                }
                // TODO(younies): double check this case
                None => PatternSelection::Standard,
            }
        };

        let narrow_pattern_standard: PatternSelection = if standard_alpha_next_to_number.is_empty()
        {
            PatternSelection::Standard
        } else {
            match narrow_injecting_text_index {
                Some(InjectingText::Index(index)) => currency_pattern_selection(
                    provider,
                    standard,
                    injecting_texts.get(index as usize).unwrap(),
                )?,
                Some(InjectingText::ISO) => {
                    currency_pattern_selection(provider, standard, &iso_string)?
                }

                // TODO(younies): double check this case
                None => short_pattern_standard,
            }
        };

        // TODO(youneis): Check if we can remove also when the patterns equal to
        // PatternSelection::StandardNextToNumber.
        if short_pattern_standard == PatternSelection::Standard
            && narrow_pattern_standard == PatternSelection::Standard
            && short_injecting_text_index.is_none()
            && narrow_injecting_text_index.is_none()
        {
            continue;
        }

        currency_patterns_map.insert(
            *iso,
            CurrencyPatterns {
                short_pattern_standard,
                narrow_pattern_standard,
                short_injecting_text_index,
                narrow_injecting_text_index,
            },
        );
    }

    Ok(CurrencyEssentialsV1 {
        currency_patterns_map: ZeroMap::from_iter(currency_patterns_map.iter()),
        standard: standard.to_owned().into(),
        standard_alpha_next_to_number: standard_alpha_next_to_number.to_owned().into(),
        injecting_texts: VarZeroVec::from(&injecting_texts),
    })
}

#[test]
fn test_basic() {
    fn get_injecting_texts_of_currency(
        iso_code: UnvalidatedTinyAsciiStr<3>,
        locale: &DataPayload<CurrencyEssentialsV1Marker>,
        injecting_texts: &VarZeroVec<'_, str>,
    ) -> (String, String) {
        let default = CurrencyPatterns {
            short_pattern_standard: PatternSelection::Standard,
            narrow_pattern_standard: PatternSelection::Standard,
            short_injecting_text_index: None,
            narrow_injecting_text_index: None,
        };
        let owned = locale.get().to_owned();
        let currency_pattern: CurrencyPatterns = owned
            .currency_patterns_map
            .get_copied(&iso_code)
            .unwrap_or(default);

        let short_injecting_text = match currency_pattern.short_injecting_text_index {
            Some(InjectingText::Index(index)) => injecting_texts
                .get(index as usize)
                .unwrap_or(&iso_code.try_into_tinystr().unwrap())
                .to_string(),
            Some(InjectingText::ISO) => iso_code.try_into_tinystr().unwrap().to_string(),
            None => "".to_string(),
        };

        let narrow_injecting_text = match currency_pattern.narrow_injecting_text_index {
            Some(InjectingText::Index(index)) => injecting_texts
                .get(index as usize)
                .unwrap_or(&iso_code.try_into_tinystr().unwrap())
                .to_string(),
            Some(InjectingText::ISO) => iso_code.try_into_tinystr().unwrap().to_string(),
            None => "".to_string(),
        };

        (short_injecting_text, narrow_injecting_text)
    }

    use icu_locid::locale;
    use icu_singlenumberformatter::provider::*;

    let provider = crate::DatagenProvider::new_testing();

    let en: DataPayload<CurrencyEssentialsV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("en").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    let en_injecting_texts = &en.get().to_owned().injecting_texts;
    assert_eq!(en.clone().get().to_owned().standard, "¤#,##0.00");
    assert_eq!(
        en.clone().get().to_owned().standard_alpha_next_to_number,
        "¤\u{a0}#,##0.00"
    );

    let (en_usd_short, en_usd_narrow) = get_injecting_texts_of_currency(
        tinystr!(3, "USD").to_unvalidated(),
        &en,
        en_injecting_texts,
    );
    assert_eq!(en_usd_short, "$");
    assert_eq!(en_usd_narrow, "$");

    let (en_egp_short, en_egp_narrow) = get_injecting_texts_of_currency(
        tinystr!(3, "EGP").to_unvalidated(),
        &en,
        en_injecting_texts,
    );
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

    let ar_eg_injecting_texts = &ar_eg.get().to_owned().injecting_texts;

    assert_eq!(
        ar_eg.clone().get().to_owned().standard,
        "‏#,##0.00 ¤;‏-#,##0.00 ¤"
    );
    assert_eq!(
        ar_eg.clone().get().to_owned().standard_alpha_next_to_number,
        ""
    );
    let (ar_eg_egp_short, ar_eg_egp_narrow) = get_injecting_texts_of_currency(
        tinystr!(3, "EGP").to_unvalidated(),
        &ar_eg,
        ar_eg_injecting_texts,
    );
    assert_eq!(ar_eg_egp_short, "ج.م.\u{200f}");
    assert_eq!(ar_eg_egp_narrow, "E£");

    let (ar_eg_usd_short, ar_eg_usd_narrow) = get_injecting_texts_of_currency(
        tinystr!(3, "USD").to_unvalidated(),
        &ar_eg,
        ar_eg_injecting_texts,
    );
    assert_eq!(ar_eg_usd_short, "US$");
    assert_eq!(ar_eg_usd_narrow, "US$");
}
