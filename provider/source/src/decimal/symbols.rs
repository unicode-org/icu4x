// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::decimal::provider::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;

impl DataProvider<DecimalSymbolsV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DecimalSymbolsV2>, DataError> {
        self.check_req::<DecimalSymbolsV2>(req)?;

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

        let result = DecimalSymbols::try_from(NumbersWithNumsys(numbers, nsname)).map_err(|s| {
            DataError::custom("Could not create decimal symbols")
                .with_display_context(&s)
                .with_display_context(nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl IterableDataProviderCached<DecimalSymbolsV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers_with_locales()
    }
}

#[derive(Debug)]
struct NumbersWithNumsys<'a>(
    pub(crate) &'a cldr_serde::numbers::Numbers,
    pub(crate) &'a str,
);

impl TryFrom<NumbersWithNumsys<'_>> for DecimalSymbols<'static> {
    type Error = Cow<'static, str>;

    fn try_from(other: NumbersWithNumsys<'_>) -> Result<Self, Self::Error> {
        let NumbersWithNumsys(numbers, nsname) = other;
        let symbols = numbers
            .numsys_data
            .symbols
            .get(nsname)
            .ok_or("Could not find symbols for numbering system")?;
        let formats = numbers
            .numsys_data
            .formats
            .get(nsname)
            .ok_or("Could not find formats for numbering system")?;
        let parsed_pattern: super::decimal_pattern::DecimalPattern = formats
            .standard
            .parse()
            .map_err(|s: super::decimal_pattern::Error| s.to_string())?;

        let minus_sign_affixes = parsed_pattern.localize_sign(&symbols.minus_sign);
        let plus_sign_affixes = parsed_pattern.localize_sign(&symbols.plus_sign);
        let numsys = nsname
            .parse()
            .map_err(|_| format!("Numbering system {nsname} should not be more than 8 bytes!"))?;

        let strings = DecimalSymbolStrsBuilder {
            minus_sign_prefix: minus_sign_affixes.0.into(),
            minus_sign_suffix: minus_sign_affixes.1.into(),
            plus_sign_prefix: plus_sign_affixes.0.into(),
            plus_sign_suffix: plus_sign_affixes.1.into(),
            decimal_separator: Cow::Owned(symbols.decimal.clone()),
            grouping_separator: Cow::Owned(symbols.group.clone()),
            numsys: Cow::Owned(numsys),
        };

        Ok(Self {
            strings: strings.build(),
            grouping_sizes: GroupingSizes {
                primary: parsed_pattern.positive.primary_grouping,
                secondary: parsed_pattern.positive.secondary_grouping,
                min_grouping: numbers.minimum_grouping_digits,
            },
        })
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let ar_decimal: DataResponse<DecimalSymbolsV2> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("ar-EG").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(ar_decimal.payload.get().decimal_separator(), "٫");
    assert_eq!(ar_decimal.payload.get().numsys(), "arab");
}
