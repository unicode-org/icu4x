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

use icu_pattern::PatternItemCow;
use icu_pattern::SinglePlaceholderKey;
use icu_pattern::SinglePlaceholderPattern;

use icu::experimental::dimension::provider::currency::no_currency::*;
use icu_provider::prelude::*;

impl DataProvider<CurrencyNoCurrencyPatternsV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyNoCurrencyPatternsV1>, DataError> {
        self.check_req::<CurrencyNoCurrencyPatternsV1>(req)?;

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

impl IterableDataProviderCached<CurrencyNoCurrencyPatternsV1> for SourceDataProvider {
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
                if patterns.standard.positive.is_empty() && patterns.standard_no_currency.is_none()
                {
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

fn extract_currency_no_currency<'data>(
    numbers_resource: &cldr_serde::numbers::Resource,
    numsys_name: &str,
) -> Result<CurrencyNoCurrencyPatterns<'data>, DataError> {
    let numbers_block = &numbers_resource.main.value.numbers;
    let currency_formats = numbers_block
        .numsys_data
        .currency_patterns
        .get(numsys_name)
        .ok_or_else(|| DataErrorKind::IdentifierNotFound.into_error())?;

    fn convert_pattern_items<'a>(
        items: &'a [NumberPatternItem],
    ) -> impl Iterator<Item = PatternItemCow<'a, SinglePlaceholderKey>> + 'a {
        items.iter().flat_map(|item| match item {
            NumberPatternItem::DecimalSeparator => {
                Some(PatternItemCow::Placeholder(SinglePlaceholderKey::Singleton))
            }
            NumberPatternItem::Literal(s) => Some(PatternItemCow::Literal(Cow::Borrowed(s))),
            _ => None,
        })
    }

    fn create_pattern<'data>(
        pattern: &NumberPattern,
    ) -> Result<Cow<'data, SinglePlaceholderPattern>, DataError> {
        SinglePlaceholderPattern::try_from_items(convert_pattern_items(&pattern.positive))
            .map_err(|e| {
                DataError::custom("Could not parse positive pattern").with_display_context(&e)
            })
            .map(Cow::Owned)
    }

    fn create_negative_pattern<'data>(
        pattern: &NumberPattern,
    ) -> Result<Option<Cow<'data, SinglePlaceholderPattern>>, DataError> {
        if let Some(negative_items) = &pattern.negative {
            SinglePlaceholderPattern::try_from_items(convert_pattern_items(negative_items))
                .map_err(|e| {
                    DataError::custom("Could not parse negative pattern").with_display_context(&e)
                })
                .map(Cow::Owned)
                .map(Some)
        } else {
            Ok(None)
        }
    }

    let standard_pattern = if let Some(std_nc) = &currency_formats.standard_no_currency {
        std_nc
    } else {
        &currency_formats.standard
    };

    let standard = create_pattern(standard_pattern)?;
    let standard_negative = create_negative_pattern(standard_pattern)?;

    let (accounting_positive, accounting_negative) =
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

    Ok(CurrencyNoCurrencyPatterns {
        standard,
        standard_negative,
        accounting_positive,
        accounting_negative,
    })
}
