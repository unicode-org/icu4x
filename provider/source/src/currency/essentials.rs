// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::cldr_serde::numbers::NumberPattern;
use crate::cldr_serde::numbers::NumberPatternItem;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::borrow::Cow;

use icu_pattern::DoublePlaceholderPattern;
use tinystr::TinyAsciiStr;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use tinystr::UnvalidatedTinyAsciiStr;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;

use icu_pattern::DoublePlaceholderKey;
use icu_pattern::PatternItemCow;

use icu::experimental::dimension::provider::currency::ule::MAX_PLACEHOLDER_INDEX;
use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
use icu::properties::CodePointMapData;
use icu_provider::DataProvider;

use icu::experimental::dimension::provider::currency::essentials::*;
use icu_provider::prelude::*;

/// Returns the pattern selection for a currency.
/// For example:
///    if the pattern is ¤#,##0.00 and the symbol is EGP,
///    this means the return value will be `PatternSelection::StandardAlphaNextToNumber`
///    because the character closes to the number is a letter.
/// NOTE:
///   `placeholder_value` must not be empty.
fn currency_pattern_selection(
    provider: &SourceDataProvider,
    pattern: &NumberPattern,
    placeholder_value: &str,
) -> Result<PatternSelection, DataError> {
    if placeholder_value.is_empty() {
        return Err(DataError::custom("Place holder value must not be empty"));
    }

    // TODO(#6064): Handle the negative sub pattern.
    let pattern = &pattern.positive;

    let currency_sign_index = pattern
        .iter()
        .position(|i| matches!(i, NumberPatternItem::Currency))
        .unwrap();
    let first_num_index = pattern
        .iter()
        .position(|i| {
            matches!(
                i,
                NumberPatternItem::MandatoryDigit | NumberPatternItem::OptionalDigit
            )
        })
        .unwrap();
    let last_num_index = pattern
        .iter()
        .rposition(|i| {
            matches!(
                i,
                NumberPatternItem::MandatoryDigit | NumberPatternItem::OptionalDigit
            )
        })
        .unwrap();

    let letters_set = CodePointMapData::<GeneralCategory>::try_new_unstable(provider)?
        .as_borrowed()
        .get_set_for_value_group(GeneralCategoryGroup::Letter);

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

        let result = extract_currency_essentials(self, currencies_resource, numbers_resource);

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

fn extract_currency_essentials<'data>(
    provider: &SourceDataProvider,
    currencies_resource: &cldr_serde::currencies::data::Resource,
    numbers_resource: &cldr_serde::numbers::Resource,
) -> Result<CurrencyEssentials<'data>, DataError> {
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
    let standard_alpha_next_to_number = currency_formats
        .standard_alpha_next_to_number
        .as_ref()
        .unwrap_or(standard);

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
                currency_pattern_selection(
                    provider,
                    standard,
                    match placeholder_index {
                        Some(PlaceholderValue::Index(index)) => placeholders[index as usize],
                        Some(PlaceholderValue::ISO) | None => iso.as_str(),
                    },
                )
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
        pattern: &NumberPattern,
    ) -> Result<Cow<'data, DoublePlaceholderPattern>, DataError> {
        // TODO(#4677): Handle the negative sub pattern.
        // TODO: this is wrong - the currency pattern does not necessarily match the decimal pattern with a currency
        // sign and some literals tacked on.
        let pattern_items = pattern.positive.iter().flat_map(|item| match item {
            NumberPatternItem::Currency => {
                Some(PatternItemCow::Placeholder(DoublePlaceholderKey::Place1))
            }
            NumberPatternItem::Literal(s) => Some(PatternItemCow::Literal(Cow::Borrowed(s))),
            NumberPatternItem::DecimalSeparator => {
                Some(PatternItemCow::Placeholder(DoublePlaceholderKey::Place0))
            }
            _ => None,
        });

        DoublePlaceholderPattern::try_from_items(pattern_items.into_iter())
            .map_err(|e| {
                DataError::custom("Could not parse standard pattern").with_display_context(&e)
            })
            .map(Cow::Owned)
    }

    Ok(CurrencyEssentials {
        pattern_config_map: ZeroMap::from_iter(currency_patterns_map.iter()),
        standard_pattern: create_pattern(standard)?,
        standard_alpha_next_to_number_pattern: create_pattern(standard_alpha_next_to_number)?,
        placeholders: VarZeroVec::from(&placeholders),
        default_pattern_config,
    })
}

#[test]
fn test_basic() {
    use tinystr::tinystr;
    use writeable::assert_writeable_eq;

    fn get_placeholders_of_currency(
        iso_code: UnvalidatedTinyAsciiStr<3>,
        locale: &DataResponse<CurrencyEssentialsV1>,
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

    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let en: DataResponse<CurrencyEssentialsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap();

    let en_payload = en.payload.get();

    assert_writeable_eq!(en_payload.standard_pattern.interpolate((3, "$")), "$3");
    assert_writeable_eq!(
        en_payload
            .standard_alpha_next_to_number_pattern
            .interpolate((3, "$")),
        "$\u{a0}3"
    );

    let (en_usd_short, en_usd_narrow) = get_placeholders_of_currency(
        tinystr!(3, "USD").to_unvalidated(),
        &en,
        &en_payload.placeholders,
    );
    assert_eq!(en_usd_short, "$");
    assert_eq!(en_usd_narrow, "$");

    let (en_egp_short, en_egp_narrow) = get_placeholders_of_currency(
        tinystr!(3, "EGP").to_unvalidated(),
        &en,
        &en_payload.placeholders,
    );
    // TODO(#6064)
    assert_eq!(en_egp_short, "EGP");
    assert_eq!(en_egp_narrow, "E£");

    let ar_eg: DataResponse<CurrencyEssentialsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ar-EG").into()),
            ..Default::default()
        })
        .unwrap();

    let ar_eg_payload = ar_eg.payload.get();
    assert_writeable_eq!(
        ar_eg_payload.standard_pattern.interpolate((3, "$")),
        "\u{200f}3\u{a0}$"
    );

    let (ar_eg_egp_short, ar_eg_egp_narrow) = get_placeholders_of_currency(
        tinystr!(3, "EGP").to_unvalidated(),
        &ar_eg,
        &ar_eg_payload.placeholders,
    );
    assert_eq!(ar_eg_egp_short, "ج.م.\u{200f}");
    assert_eq!(ar_eg_egp_narrow, "E£");

    let (ar_eg_usd_short, ar_eg_usd_narrow) = get_placeholders_of_currency(
        tinystr!(3, "USD").to_unvalidated(),
        &ar_eg,
        &ar_eg_payload.placeholders,
    );
    assert_eq!(ar_eg_usd_short, "US$");
    assert_eq!(ar_eg_usd_narrow, "US$");
}
