// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::decimal::decimal_pattern::DecimalSubPattern;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::decimal::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroCow;

impl DataProvider<DecimalSymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DecimalSymbolsV1>, DataError> {
        self.check_req::<DecimalSymbolsV1>(req)?;

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = if !req.id.marker_attributes.is_empty() {
            req.id.marker_attributes.as_str()
        } else {
            &numbers.default_numbering_system
        };

        let Some(symbols) = &numbers.numsys_data.symbols.get(nsname) else {
            return Err(DataErrorKind::IdentifierNotFound.with_req(DecimalSymbolsV1::INFO, req));
        };
        let Some(formats) = &numbers.numsys_data.formats.get(nsname) else {
            return Err(DataErrorKind::IdentifierNotFound.with_req(DecimalSymbolsV1::INFO, req));
        };

        let positive = DecimalSubPattern::try_from_items(&formats.standard.positive)?;
        let negative = formats
            .standard
            .negative
            .as_ref()
            .map(|s| DecimalSubPattern::try_from_items(s))
            .transpose()?;

        let affixes = negative
            .as_ref()
            .map(|n| (n.prefix.as_str(), n.suffix.as_str()))
            .unwrap_or_else(|| ("-", ""));

        let strings = DecimalSymbolStrsBuilder {
            minus_sign_prefix: VarZeroCow::new_owned(
                affixes.0.replace('-', &symbols.minus_sign).into_boxed_str(),
            ),
            minus_sign_suffix: VarZeroCow::new_owned(
                affixes.1.replace('-', &symbols.minus_sign).into_boxed_str(),
            ),
            plus_sign_prefix: VarZeroCow::new_owned(
                affixes.0.replace('-', &symbols.plus_sign).into_boxed_str(),
            ),
            plus_sign_suffix: VarZeroCow::new_owned(
                affixes.1.replace('-', &symbols.plus_sign).into_boxed_str(),
            ),
            decimal_separator: VarZeroCow::new_owned(symbols.decimal.clone().into_boxed_str()),
            grouping_separator: VarZeroCow::new_owned(symbols.group.clone().into_boxed_str()),
            numsys: VarZeroCow::new_owned(nsname.to_owned().into_boxed_str()),
        }
        .build();

        if let Some(n) = negative.as_ref() {
            if (
                positive.max_fraction_digits,
                positive.min_fraction_digits,
                positive.primary_grouping,
                positive.secondary_grouping,
            ) != (
                n.max_fraction_digits,
                n.min_fraction_digits,
                n.primary_grouping,
                n.secondary_grouping,
            ) {
                return Err(DataError::custom("positive/negative groupings don't match")
                    .with_req(DecimalSymbolsV1::INFO, req));
            }
        }

        let grouping_sizes = GroupingSizes {
            primary: positive.primary_grouping,
            secondary: positive.secondary_grouping,
            min_grouping: numbers.minimum_grouping_digits,
        };

        // TODO: do something with `numbers.(min/max)_fraction_digits`

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(DecimalSymbols {
                strings,
                grouping_sizes,
            }),
        })
    }
}

impl IterableDataProviderCached<DecimalSymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers_with_locales()
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let ar_decimal: DataResponse<DecimalSymbolsV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("ar-EG").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(ar_decimal.payload.get().decimal_separator(), "٫");
    assert_eq!(ar_decimal.payload.get().numsys(), "arab");
}
