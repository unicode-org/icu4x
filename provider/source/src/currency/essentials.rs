// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::numbers::NumberPattern;
use crate::cldr_serde::numbers::NumberPatternItem;

use std::borrow::Cow;
use std::collections::HashSet;
use zerovec::VarZeroVec;

use icu_pattern::DoublePlaceholderKey;
use icu_pattern::DoublePlaceholderPattern;
use icu_pattern::PatternItemCow;

use icu_provider::DataProvider;

use icu::experimental::dimension::provider::currency::essentials::*;
use icu_provider::prelude::*;

impl DataProvider<CurrencyEssentialsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyEssentialsV1>, DataError> {
        self.check_req::<CurrencyEssentialsV1>(req)?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let nsname = if !req.id.marker_attributes.is_empty() {
            req.id.marker_attributes.as_str()
        } else {
            &numbers_resource.main.value.numbers.default_numbering_system
        };

        let result = extract_currency_essentials(numbers_resource, nsname);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<CurrencyEssentialsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut ids = HashSet::new();
        for locale in self.cldr()?.numbers().list_locales()? {
            let numbers_resource: &cldr_serde::numbers::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&locale, "numbers.json")?;
            let numbers = &numbers_resource.main.value.numbers;
            let default_numsys = &numbers.default_numbering_system;

            for (nsname, patterns) in &numbers.numsys_data.currency_patterns {
                if patterns.standard.positive.is_empty() {
                    continue;
                }
                if nsname == default_numsys {
                    ids.insert(DataIdentifierCow::from_locale(locale));
                } else {
                    let attr = DataMarkerAttributes::try_from_str(nsname).map_err(|_| {
                        DataError::custom("Invalid numbering system name")
                            .with_display_context(nsname)
                    })?;
                    ids.insert(
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(attr, &locale)
                            .into_owned(),
                    );
                }
            }
        }
        Ok(ids)
    }
}

fn extract_currency_essentials<'data>(
    numbers_resource: &cldr_serde::numbers::Resource,
    numsys_name: &str,
) -> Result<CurrencyEssentials<'data>, DataError> {
    let numbers_block = &numbers_resource.main.value.numbers;
    let currency_formats = numbers_block
        .numsys_data
        .currency_patterns
        .get(numsys_name)
        .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;

    // We only generate CurrencyEssentials for locale/numsys pairs that possess a non-empty standard
    // currency pattern (verified during ID iteration in `iter_ids_cached`). Therefore, if this pattern
    // is missing or empty when `load` is called directly, the requested locale data is unsupported
    // or malformed, so we treat the identifier as not found.
    let standard = &currency_formats.standard;
    if standard.positive.is_empty() {
        return Err(DataErrorKind::IdentifierNotFound.into_error());
    }
    let standard_alpha_next_to_number = currency_formats.standard_alpha_next_to_number.as_ref();
    let accounting = currency_formats.accounting.as_ref();
    let accounting_alpha_next_to_number = currency_formats.accounting_alpha_next_to_number.as_ref();

    fn convert_pattern_items<'a>(
        items: &'a [NumberPatternItem],
    ) -> impl Iterator<Item = PatternItemCow<'a, DoublePlaceholderKey>> + 'a {
        items.iter().flat_map(|item| match item {
            NumberPatternItem::Currency => {
                Some(PatternItemCow::Placeholder(DoublePlaceholderKey::Place1))
            }
            NumberPatternItem::Literal(s) => Some(PatternItemCow::Literal(Cow::Borrowed(s))),
            NumberPatternItem::DecimalSeparator => {
                Some(PatternItemCow::Placeholder(DoublePlaceholderKey::Place0))
            }
            // TODO(#8263): Render the sign characters using the localized plus/minus
            // signs from the decimal symbols data instead of the ASCII characters.
            NumberPatternItem::MinusSign => Some(PatternItemCow::Literal(Cow::Borrowed("-"))),
            NumberPatternItem::PlusSign => Some(PatternItemCow::Literal(Cow::Borrowed("+"))),
            _ => None,
        })
    }

    fn create_positive_pattern<'data>(
        pattern: &NumberPattern,
    ) -> Result<Cow<'data, DoublePlaceholderPattern>, DataError> {
        DoublePlaceholderPattern::try_from_items(convert_pattern_items(&pattern.positive))
            .map_err(|e| {
                DataError::custom("Could not parse positive pattern").with_display_context(&e)
            })
            .map(Cow::Owned)
    }

    fn create_negative_pattern<'data>(
        pattern: &NumberPattern,
    ) -> Result<Option<Cow<'data, DoublePlaceholderPattern>>, DataError> {
        // The negative subpattern is baked as-is when it exists; `None` means the
        // formatter falls back to the positive pattern of the same category and
        // applies the sign itself.
        if let Some(negative_items) = &pattern.negative {
            DoublePlaceholderPattern::try_from_items(convert_pattern_items(negative_items))
                .map_err(|e| {
                    DataError::custom("Could not parse negative pattern").with_display_context(&e)
                })
                .map(Cow::Owned)
                .map(Some)
        } else {
            Ok(None)
        }
    }

    let mut unique_patterns = Vec::<Box<DoublePlaceholderPattern>>::new();

    let mut add_pattern = |opt_cow: Option<Cow<'data, DoublePlaceholderPattern>>| -> Option<u8> {
        opt_cow.map(|cow| {
            let pat: Box<DoublePlaceholderPattern> = cow.into_owned();
            if let Some(idx) = unique_patterns.iter().position(|p| p == &pat) {
                idx as u8
            } else {
                let idx = unique_patterns.len() as u8;
                unique_patterns.push(pat);
                idx
            }
        })
    };

    let standard_idx = add_pattern(Some(create_positive_pattern(standard)?)).unwrap();
    let standard_neg_idx = add_pattern(create_negative_pattern(standard)?);
    let standard_alpha_idx = add_pattern(
        standard_alpha_next_to_number
            .map(create_positive_pattern)
            .transpose()?,
    )
    .unwrap_or(standard_idx);
    let standard_alpha_neg_idx = match standard_alpha_next_to_number {
        Some(p) => add_pattern(create_negative_pattern(p)?),
        // No alpha-next-to-number pattern: the category inherits from standard,
        // including its negative subpattern.
        None => standard_neg_idx,
    };
    let accounting_pos_idx =
        add_pattern(accounting.map(create_positive_pattern).transpose()?).unwrap_or(standard_idx);
    let accounting_neg_idx = match accounting {
        Some(p) => add_pattern(create_negative_pattern(p)?),
        // No accounting pattern: the category inherits from standard,
        // including its negative subpattern.
        None => standard_neg_idx,
    };
    let accounting_alpha_pos_idx = add_pattern(
        accounting_alpha_next_to_number
            .map(create_positive_pattern)
            .transpose()?,
    )
    .unwrap_or(accounting_pos_idx);
    let accounting_alpha_neg_idx = match accounting_alpha_next_to_number {
        Some(p) => add_pattern(create_negative_pattern(p)?),
        // No accounting alpha-next-to-number pattern: the category inherits from
        // accounting, including its negative subpattern.
        None => accounting_neg_idx,
    };

    let indices = PatternIndices {
        standard: standard_idx,
        standard_negative: standard_neg_idx,
        standard_alpha_next_to_number: standard_alpha_idx,
        standard_alpha_next_to_number_negative: standard_alpha_neg_idx,
        accounting_positive: accounting_pos_idx,
        accounting_negative: accounting_neg_idx,
        accounting_alpha_next_to_number_positive: accounting_alpha_pos_idx,
        accounting_alpha_next_to_number_negative: accounting_alpha_neg_idx,
    };

    Ok(CurrencyEssentials {
        patterns: VarZeroVec::from(&unique_patterns),
        indices,
    })
}

#[test]
fn test_essentials() {
    use icu::locale::langid;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let en: DataPayload<CurrencyEssentialsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_writeable_eq!(
        en.get().get_positive(false, false).interpolate((3, "$")),
        "$3"
    );

    assert_writeable_eq!(
        en.get().get_positive(true, true).interpolate((3, "USD")),
        "USD\u{a0}3"
    );
    assert_writeable_eq!(
        en.get()
            .get_positive_accounting(false, false)
            .interpolate((3, "$")),
        "$3"
    );
    assert_writeable_eq!(
        en.get()
            .get_negative_accounting(false, false)
            .unwrap()
            .interpolate((3, "$")),
        "($3)"
    );
    assert_writeable_eq!(
        en.get()
            .get_positive_accounting(true, true)
            .interpolate((3, "USD")),
        "USD\u{a0}3"
    );
    assert_writeable_eq!(
        en.get()
            .get_negative_accounting(true, true)
            .unwrap()
            .interpolate((3, "USD")),
        "(USD\u{a0}3)"
    );

    let ar_eg: DataPayload<CurrencyEssentialsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ar-EG").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_writeable_eq!(
        ar_eg.get().get_positive(false, false).interpolate((3, "$")),
        "\u{200f}3\u{a0}$"
    );

    // The `latn` patterns for `ar-EG` have an explicit standard negative subpattern
    // (`\u{200f}#,##0.00\u{a0}\u{a4};\u{200f}-#,##0.00\u{a0}\u{a4}`), which is baked
    // as-is, sign character included.
    let ar_eg_latn: DataPayload<CurrencyEssentialsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("latn"),
                &langid!("ar-EG").into(),
            ),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_writeable_eq!(
        ar_eg_latn
            .get()
            .get_negative(false, false)
            .unwrap()
            .interpolate((3, "$")),
        "\u{200f}-3\u{a0}$"
    );

    // `en` has no explicit standard negative subpattern, so `get_negative` is `None`
    // and the formatter falls back to the positive pattern, applying the sign itself.
    assert!(en.get().get_negative(false, false).is_none());
}
