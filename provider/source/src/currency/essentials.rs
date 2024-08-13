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

use icu_pattern::DoublePlaceholder;
use icu_pattern::DoublePlaceholderKey;
use icu_pattern::Pattern;
use icu_pattern::PatternItemCow;

use icu::experimental::dimension::provider::ule::MAX_PLACEHOLDER_INDEX;
use icu::properties::sets::load_for_general_category_group;
use icu::properties::GeneralCategoryGroup;
use icu_provider::DataProvider;

use icu::experimental::dimension::provider::currency::*;
use icu_provider::prelude::*;

/// Returns the pattern selection for a currency.
/// For example:
///    if the pattern is ¤#,##0.00 and the symbol is EGP,
///    this means the return value will be PatternSelection::StandardAlphaNextToNumber
///    because the character closes to the number is a letter.
/// NOTE:
///   placeholder_value must not be empty.
fn currency_pattern_selection(
    provider: &SourceDataProvider,
    pattern: &str,
    placeholder_value: &str,
) -> Result<PatternSelection, DataError> {
    if placeholder_value.is_empty() {
        return Err(DataError::custom("Place holder value must not be empty"));
    }

    let currency_sign = '¤';
    let currency_sign_index = pattern.find(currency_sign).unwrap();
    let first_num_index = pattern.find(['0', '#']).unwrap();
    let last_num_index = pattern.rfind(['0', '#']).unwrap();

    let letters_set = load_for_general_category_group(provider, GeneralCategoryGroup::Letter)?;

    let char_closer_to_number = if currency_sign_index < first_num_index {
        placeholder_value.chars().next_back().unwrap()
    } else if currency_sign_index > last_num_index {
        placeholder_value.chars().next().unwrap()
    } else {
        return Err(DataError::custom(
            "Currency sign must not be in the middle of the pattern",
        ));
    };

    Ok(
        if letters_set.as_borrowed().contains(char_closer_to_number) {
            PatternSelection::StandardAlphaNextToNumber
        } else {
            PatternSelection::Standard
        },
    )
}

impl DataProvider<CurrencyEssentialsV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyEssentialsV1Marker>, DataError> {
        self.check_req::<CurrencyEssentialsV1Marker>(req)?;

        let currencies_resource: &cldr_serde::currencies::data::Resource =
            self.cldr()?
                .numbers()
                .read_and_parse(req.id.locale, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let result = extract_currency_essentials(self, currencies_resource, numbers_resource);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<CurrencyEssentialsV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

fn extract_currency_essentials<'data>(
    provider: &SourceDataProvider,
    currencies_resource: &cldr_serde::currencies::data::Resource,
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
        .get("latn")
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    let standard = &currency_formats.standard;
    let standard_alpha_next_to_number = match &currency_formats.standard_alpha_next_to_number {
        Some(standard_alpha_next_to_number) => standard_alpha_next_to_number,
        None => "",
    };

    let mut currency_patterns_map =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>::new();
    let mut currency_patterns_standard_none =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>::new();
    let mut currency_patterns_standard_next_to_num =
        BTreeMap::<UnvalidatedTinyAsciiStr<3>, CurrencyPatternConfig>::new();
    let mut placeholders = Vec::<&str>::new();
    // A map to check if the place holder is already in the placeholders vector.
    let mut placeholders_checker_map = HashMap::<&str, u16>::new();

    for (iso, currency_pattern) in currencies {
        let short_placeholder_value = currency_pattern.short.as_ref().map(|short_placeholder| {
            if let Some(&index) = placeholders_checker_map.get(short_placeholder.as_str()) {
                PlaceholderValue::Index(index)
            } else if short_placeholder == iso {
                PlaceholderValue::ISO
            } else {
                let index = placeholders.len() as u16;
                placeholders.push(short_placeholder.as_str());
                placeholders_checker_map.insert(short_placeholder.as_str(), index);
                PlaceholderValue::Index(index)
            }
        });

        let narrow_placeholder_value = currency_pattern.narrow.as_ref().map(|narrow_placeholder| {
            if let Some(&index) = placeholders_checker_map.get(narrow_placeholder.as_str()) {
                PlaceholderValue::Index(index)
            } else if narrow_placeholder == iso {
                PlaceholderValue::ISO
            } else {
                let index = placeholders.len() as u16;
                placeholders.push(narrow_placeholder.as_ref());
                placeholders_checker_map.insert(narrow_placeholder.as_str(), index);
                PlaceholderValue::Index(index)
            }
        });

        // Ensure that short_placeholder_value and narrow_placeholder_value do not exceed MAX_PLACEHOLDER_INDEX.
        if let Some(PlaceholderValue::Index(index)) = short_placeholder_value {
            if index > MAX_PLACEHOLDER_INDEX {
                return Err(DataError::custom(
                    "short_placeholder_value exceeded MAX_PLACEHOLDER_INDEX",
                ));
            }
        }
        if let Some(PlaceholderValue::Index(index)) = narrow_placeholder_value {
            if index > MAX_PLACEHOLDER_INDEX {
                return Err(DataError::custom(
                    "narrow_placeholder_value exceeded MAX_PLACEHOLDER_INDEX",
                ));
            }
        }

        let determine_pattern_selection =
            |placeholder_index: Option<PlaceholderValue>| -> Result<PatternSelection, DataError> {
                if standard_alpha_next_to_number.is_empty() {
                    Ok(PatternSelection::Standard)
                } else {
                    let placeholder_value = match placeholder_index {
                        Some(PlaceholderValue::Index(index)) => {
                            placeholders.get(index as usize).unwrap()
                        }
                        Some(PlaceholderValue::ISO) | None => iso.as_str(),
                    };
                    currency_pattern_selection(provider, standard, placeholder_value)
                }
            };

        let short_pattern_selection: PatternSelection =
            determine_pattern_selection(short_placeholder_value)?;
        let narrow_pattern_selection: PatternSelection =
            determine_pattern_selection(narrow_placeholder_value)?;

        let currency_patterns = CurrencyPatternConfig {
            short_pattern_selection,
            narrow_pattern_selection,
            short_placeholder_value,
            narrow_placeholder_value,
        };

        let iso = TinyAsciiStr::try_from_str(iso).unwrap().to_unvalidated();
        match (short_pattern_selection, narrow_pattern_selection) {
            (PatternSelection::Standard, PatternSelection::Standard)
                if short_placeholder_value.is_none() && narrow_placeholder_value.is_none() =>
            {
                currency_patterns_standard_none.insert(iso, currency_patterns);
            }
            (
                PatternSelection::StandardAlphaNextToNumber,
                PatternSelection::StandardAlphaNextToNumber,
            ) if short_placeholder_value.is_none() && narrow_placeholder_value.is_none() => {
                currency_patterns_standard_next_to_num.insert(iso, currency_patterns);
            }
            _ => {
                currency_patterns_map.insert(iso, currency_patterns);
            }
        }
    }

    let default_pattern_config =
        if currency_patterns_standard_none.len() <= currency_patterns_standard_next_to_num.len() {
            currency_patterns_map.extend(currency_patterns_standard_none);
            CurrencyPatternConfig {
                short_pattern_selection: PatternSelection::StandardAlphaNextToNumber,
                narrow_pattern_selection: PatternSelection::StandardAlphaNextToNumber,
                short_placeholder_value: None,
                narrow_placeholder_value: None,
            }
        } else {
            currency_patterns_map.extend(currency_patterns_standard_next_to_num);
            CurrencyPatternConfig {
                short_pattern_selection: PatternSelection::Standard,
                narrow_pattern_selection: PatternSelection::Standard,
                short_placeholder_value: None,
                narrow_placeholder_value: None,
            }
        };

    /// Create a `DoublePlaceholderPattern` from a string pattern.
    fn create_pattern<'data>(
        pattern: &str,
    ) -> Result<Option<DoublePlaceholderPattern<Cow<'data, str>>>, DataError> {
        if pattern.is_empty() {
            return Ok(None);
        }

        let decimal_pattern = DecimalPattern::from_str(pattern).map_err(|e| {
            DataError::custom("Could not parse the pattern").with_display_context(&e)
        })?;

        // TODO(#4677): Handle the negative sub pattern.
        let pattern_items = decimal_pattern
            .positive
            .to_pattern_items()
            .into_iter()
            .flat_map(|item| match item {
                PatternItemCow::Placeholder(_) => vec![item],
                PatternItemCow::Literal(s) if s.contains('¤') => {
                    itertools::Itertools::intersperse(
                        s.split('¤')
                            .map(|s| PatternItemCow::Literal(s.to_string().into())),
                        PatternItemCow::Placeholder(DoublePlaceholderKey::Place1),
                    )
                    .collect()
                }
                PatternItemCow::Literal(s) => vec![PatternItemCow::Literal(s)],
            });

        let pattern = Pattern::<DoublePlaceholder, _>::try_from_items(pattern_items.into_iter())
            .map_err(|e| {
                DataError::custom("Could not parse standard pattern").with_display_context(&e)
            })?;

        let pattern_store = pattern.take_store();
        let borrowed_pattern: Pattern<DoublePlaceholder, Cow<'_, str>> =
            Pattern::from_store_unchecked(Cow::Owned(pattern_store));

        Ok(Some(borrowed_pattern.to_owned()))
    }

    Ok(CurrencyEssentialsV1 {
        pattern_config_map: ZeroMap::from_iter(currency_patterns_map.iter()),
        standard_pattern: create_pattern(standard.as_str())?,
        standard_alpha_next_to_number_pattern: create_pattern(standard_alpha_next_to_number)?,
        placeholders: VarZeroVec::from(&placeholders),
        default_pattern_config,
    })
}

#[test]
fn test_basic() {
    use tinystr::tinystr;
    fn get_placeholders_of_currency(
        iso_code: UnvalidatedTinyAsciiStr<3>,
        locale: &DataResponse<CurrencyEssentialsV1Marker>,
        placeholders: &VarZeroVec<'_, str>,
    ) -> (String, String) {
        let default = CurrencyPatternConfig {
            short_pattern_selection: PatternSelection::Standard,
            narrow_pattern_selection: PatternSelection::Standard,
            short_placeholder_value: None,
            narrow_placeholder_value: None,
        };
        let owned = locale.payload.get().to_owned();
        let currency_pattern: CurrencyPatternConfig = owned
            .pattern_config_map
            .get_copied(&iso_code)
            .unwrap_or(default);

        let short_placeholder = match currency_pattern.short_placeholder_value {
            Some(PlaceholderValue::Index(index)) => placeholders
                .get(index as usize)
                .unwrap_or(&iso_code.try_into_tinystr().unwrap())
                .to_string(),
            Some(PlaceholderValue::ISO) => iso_code.try_into_tinystr().unwrap().to_string(),
            None => "".to_string(),
        };

        let narrow_placeholder = match currency_pattern.narrow_placeholder_value {
            Some(PlaceholderValue::Index(index)) => placeholders
                .get(index as usize)
                .unwrap_or(&iso_code.try_into_tinystr().unwrap())
                .to_string(),
            Some(PlaceholderValue::ISO) => iso_code.try_into_tinystr().unwrap().to_string(),
            None => "".to_string(),
        };

        (short_placeholder, narrow_placeholder)
    }

    use icu::experimental::dimension::provider::currency::*;
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let en: DataResponse<CurrencyEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap();

    let en_placeholders = &en.payload.get().to_owned().placeholders;
    assert_eq!(
        en.payload
            .get()
            .to_owned()
            .standard_pattern
            .unwrap()
            .take_store(),
        "\u{3}\u{2}"
    );
    assert_eq!(
        en.payload
            .get()
            .to_owned()
            .standard_alpha_next_to_number_pattern
            .unwrap()
            .take_store(),
        "\u{3}\u{6}\u{a0}"
    );

    let (en_usd_short, en_usd_narrow) =
        get_placeholders_of_currency(tinystr!(3, "USD").to_unvalidated(), &en, en_placeholders);
    assert_eq!(en_usd_short, "$");
    assert_eq!(en_usd_narrow, "$");

    let (en_egp_short, en_egp_narrow) =
        get_placeholders_of_currency(tinystr!(3, "EGP").to_unvalidated(), &en, en_placeholders);
    assert_eq!(en_egp_short, "");
    assert_eq!(en_egp_narrow, "E£");

    let ar_eg: DataResponse<CurrencyEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ar-EG").into()),
            ..Default::default()
        })
        .unwrap();

    let ar_eg_placeholders = &ar_eg.payload.get().to_owned().placeholders;

    assert_eq!(
        ar_eg
            .payload
            .get()
            .to_owned()
            .standard_pattern
            .unwrap()
            .take_store(),
        "\u{8}\r\u{200f}\u{a0}"
    );
    assert!(ar_eg
        .payload
        .get()
        .to_owned()
        .standard_alpha_next_to_number_pattern
        .is_none());

    let (ar_eg_egp_short, ar_eg_egp_narrow) = get_placeholders_of_currency(
        tinystr!(3, "EGP").to_unvalidated(),
        &ar_eg,
        ar_eg_placeholders,
    );
    assert_eq!(ar_eg_egp_short, "ج.م.\u{200f}");
    assert_eq!(ar_eg_egp_narrow, "E£");

    let (ar_eg_usd_short, ar_eg_usd_narrow) = get_placeholders_of_currency(
        tinystr!(3, "USD").to_unvalidated(),
        &ar_eg,
        ar_eg_placeholders,
    );
    assert_eq!(ar_eg_usd_short, "US$");
    assert_eq!(ar_eg_usd_narrow, "US$");
}
