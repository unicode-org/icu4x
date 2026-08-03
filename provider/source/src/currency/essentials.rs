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
        super::iter_numsys_pattern_ids(self, |patterns| !patterns.standard.positive.is_empty())
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
            // TODO(#8263): Consider the case of explicit sign characters (`-`/`+`) in
            // currency patterns: they are currently dropped here, and should instead be
            // rendered using the localized plus/minus signs from the decimal symbols data.
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

    let mut tracker = super::UniquePatternsTracker::new();
    let standard_idx = tracker
        .add(Some(create_positive_pattern(standard)?))
        .unwrap();
    let standard_neg_idx = tracker.add(create_negative_pattern(standard)?);
    let standard_alpha_idx = tracker
        .add(
            standard_alpha_next_to_number
                .map(create_positive_pattern)
                .transpose()?,
        )
        .unwrap_or(standard_idx);
    let standard_alpha_neg_idx = match standard_alpha_next_to_number {
        Some(p) => tracker.add(create_negative_pattern(p)?),
        None => None,
    };
    let accounting_pos_idx = tracker
        .add(accounting.map(create_positive_pattern).transpose()?)
        .unwrap_or(standard_idx);
    let accounting_neg_idx = match accounting {
        Some(p) => tracker.add(create_negative_pattern(p)?),
        None => None,
    };
    let accounting_alpha_pos_idx = tracker
        .add(
            accounting_alpha_next_to_number
                .map(create_positive_pattern)
                .transpose()?,
        )
        .unwrap_or(accounting_pos_idx);
    let accounting_alpha_neg_idx = match accounting_alpha_next_to_number {
        Some(p) => tracker.add(create_negative_pattern(p)?),
        None => None,
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
        patterns: tracker.into_var_zero_vec(),
        indices,
    })
}

#[test]
fn test_essentials() {
    use icu::locale::data_locale;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let en: DataPayload<CurrencyEssentialsV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&data_locale!("en")),
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
            id: DataIdentifierBorrowed::for_locale(&data_locale!("ar-EG")),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_writeable_eq!(
        ar_eg.get().get_positive(false, false).interpolate((3, "$")),
        "\u{200f}3\u{a0}$"
    );
}
