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

use icu_pattern::DoublePlaceholderKey;
use icu_pattern::DoublePlaceholderPattern;
use icu_pattern::PatternItemCow;

use icu::experimental::dimension::provider::currency::no_currency::*;
use icu_provider::prelude::*;

impl DataProvider<CurrencyPatternsNoCurrencyV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyPatternsNoCurrencyV1>, DataError> {
        self.check_req::<CurrencyPatternsNoCurrencyV1>(req)?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let nsname = if !req.id.marker_attributes.is_empty() {
            req.id.marker_attributes.as_str()
        } else {
            &numbers_resource.main.value.numbers.default_numbering_system
        };

        let result = extract_currency_no_currency(numbers_resource, nsname);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

/// Returns true if the currency formatting patterns contain a non-empty `standard-noCurrency`
/// pattern or the fallback `standard` pattern.
fn has_no_currency_pattern(patterns: &cldr_serde::numbers::CurrencyFormattingPatterns) -> bool {
    patterns
        .standard_no_currency
        .as_ref()
        .is_some_and(|p| !p.positive.is_empty())
        || !patterns.standard.positive.is_empty()
}

impl IterableDataProviderCached<CurrencyPatternsNoCurrencyV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        super::iter_numsys_pattern_ids(self, has_no_currency_pattern)
    }
}

fn extract_currency_no_currency<'data>(
    numbers_resource: &cldr_serde::numbers::Resource,
    numsys_name: &str,
) -> Result<CurrencyPatternsNoCurrency<'data>, DataError> {
    let numbers_block = &numbers_resource.main.value.numbers;
    let currency_formats = numbers_block
        .numsys_data
        .currency_patterns
        .get(numsys_name)
        .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;

    // We only generate CurrencyPatternsNoCurrency for locale/numsys pairs that possess a non-empty
    // `standard_no_currency` or `standard` pattern (verified during ID iteration in `iter_ids_cached`).
    // If both are missing or empty when `load` is called directly, we treat the identifier as not found.
    if !has_no_currency_pattern(currency_formats) {
        return Err(DataErrorKind::IdentifierNotFound.into_error());
    }

    let (minus_sign, plus_sign) =
        if let Some(symbols) = numbers_block.numsys_data.symbols.get(numsys_name) {
            (symbols.minus_sign.as_str(), symbols.plus_sign.as_str())
        } else {
            ("-", "+")
        };

    // Per UTS #35 (LDML, Part 3: Numbers, §3.2 Currency Formats), noCurrency patterns format
    // a currency value while omitting the currency symbol. Any currency symbol placeholder (`¤` /
    // `NumberPatternItem::Currency`) in the CLDR pattern is stripped (`_ => None`), leaving only the
    // number placeholder (`DoublePlaceholderKey::Place0`).
    fn convert_pattern_items<'a>(
        items: &'a [NumberPatternItem],
        minus_sign: &'a str,
        plus_sign: &'a str,
    ) -> impl Iterator<Item = PatternItemCow<'a, DoublePlaceholderKey>> + 'a {
        items.iter().flat_map(move |item| match item {
            NumberPatternItem::DecimalSeparator => {
                Some(PatternItemCow::Placeholder(DoublePlaceholderKey::Place0))
            }
            NumberPatternItem::Literal(s) => Some(PatternItemCow::Literal(Cow::Borrowed(s))),
            NumberPatternItem::MinusSign => {
                Some(PatternItemCow::Literal(Cow::Borrowed(minus_sign)))
            }
            NumberPatternItem::PlusSign => Some(PatternItemCow::Literal(Cow::Borrowed(plus_sign))),
            _ => None,
        })
    }

    let create_pattern =
        |pattern: &NumberPattern| -> Result<Cow<'data, DoublePlaceholderPattern>, DataError> {
            DoublePlaceholderPattern::try_from_items(convert_pattern_items(
                &pattern.positive,
                minus_sign,
                plus_sign,
            ))
            .map_err(|e| {
                DataError::custom("Could not parse positive pattern").with_display_context(&e)
            })
            .map(Cow::Owned)
        };

    let create_negative_pattern = |pattern: &NumberPattern| -> Result<
        Option<Cow<'data, DoublePlaceholderPattern>>,
        DataError,
    > {
        if let Some(negative_items) = &pattern.negative {
            DoublePlaceholderPattern::try_from_items(convert_pattern_items(
                negative_items,
                minus_sign,
                plus_sign,
            ))
            .map_err(|e| {
                DataError::custom("Could not parse negative pattern").with_display_context(&e)
            })
            .map(Cow::Owned)
            .map(Some)
        } else {
            Ok(None)
        }
    };

    // Per UTS #35 §3.2, if an explicit `alt="noCurrency"` pattern is not supplied in CLDR,
    // we fall back to using the standard pattern for that category with the currency symbol removed.
    let standard_pattern = if let Some(std_nc) = &currency_formats.standard_no_currency {
        std_nc
    } else {
        &currency_formats.standard
    };

    let standard_pos = create_pattern(standard_pattern)?;
    let standard_neg = create_negative_pattern(standard_pattern)?;

    let (accounting_pos, accounting_neg) =
        if let Some(acc_nc) = &currency_formats.accounting_no_currency {
            (
                Some(create_pattern(acc_nc)?),
                create_negative_pattern(acc_nc)?,
            )
        } else if let Some(acc) = &currency_formats.accounting {
            (Some(create_pattern(acc)?), create_negative_pattern(acc)?)
        } else {
            (None, None)
        };

    let mut tracker = super::UniquePatternsTracker::new();
    let standard_idx = tracker.add(Some(standard_pos)).unwrap();
    let standard_neg_idx = tracker.add(standard_neg);
    let accounting_pos_idx = tracker.add(accounting_pos).unwrap_or(standard_idx);
    let accounting_neg_idx = tracker.add(accounting_neg);

    let indices = NoCurrencyPatternIndices {
        standard: standard_idx,
        standard_negative: standard_neg_idx,
        accounting_positive: accounting_pos_idx,
        accounting_negative: accounting_neg_idx,
    };

    Ok(CurrencyPatternsNoCurrency {
        patterns: tracker.into_var_zero_vec(),
        indices,
    })
}
