// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::transform::cldr::cldr_serde;
use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;
use icu_decimal::provider::*;
use icu_locale_core::{extensions::unicode::key, subtags::Subtag};
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;

impl DataProvider<DecimalSymbolsV1Marker> for DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DecimalSymbolsV1Marker>, DataError> {
        self.check_req::<DecimalSymbolsV1Marker>(req)?;
        let langid = req.locale.get_langid();

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = match req.locale.get_unicode_ext(&key!("nu")) {
            Some(v) => *v
                .as_subtags_slice()
                .first()
                .expect("expecting subtag if key is present"),
            None => Subtag::from_tinystr_unvalidated(numbers.default_numbering_system),
        };

        let mut result =
            DecimalSymbolsV1::try_from(NumbersWithNumsys(numbers, nsname)).map_err(|s| {
                DataError::custom("Could not create decimal symbols")
                    .with_display_context(&s)
                    .with_display_context(&nsname)
            })?;

        result.digits = self.get_digits_for_numbering_system(nsname)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

impl IterableDataProviderCached<DecimalSymbolsV1Marker> for DatagenProvider {
    fn supported_requests_cached(
        &self,
    ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        self.supported_requests_for_numbers()
    }
}

#[derive(Debug)]
struct NumbersWithNumsys<'a>(
    pub(in crate::provider) &'a cldr_serde::numbers::Numbers,
    pub(in crate::provider) Subtag,
);

impl TryFrom<NumbersWithNumsys<'_>> for DecimalSymbolsV1<'static> {
    type Error = Cow<'static, str>;

    fn try_from(other: NumbersWithNumsys<'_>) -> Result<Self, Self::Error> {
        let NumbersWithNumsys(numbers, nsname) = other;
        let symbols = numbers
            .numsys_data
            .symbols
            .get(&nsname.as_tinystr())
            .ok_or("Could not find symbols for numbering system")?;
        let formats = numbers
            .numsys_data
            .formats
            .get(&nsname.as_tinystr())
            .ok_or("Could not find formats for numbering system")?;
        let parsed_pattern: super::decimal_pattern::DecimalPattern = formats
            .standard
            .parse()
            .map_err(|s: super::decimal_pattern::Error| s.to_string())?;

        Ok(Self {
            minus_sign_affixes: parsed_pattern.localize_sign(&symbols.minus_sign),
            plus_sign_affixes: parsed_pattern.localize_sign(&symbols.plus_sign),
            decimal_separator: Cow::Owned(symbols.decimal.clone()),
            grouping_separator: Cow::Owned(symbols.group.clone()),
            grouping_sizes: GroupingSizesV1 {
                primary: parsed_pattern.positive.primary_grouping,
                secondary: parsed_pattern.positive.secondary_grouping,
                min_grouping: numbers.minimum_grouping_digits,
            },
            digits: Default::default(), // to be filled in
        })
    }
}

#[test]
fn test_basic() {
    use icu_locale_core::langid;

    let provider = DatagenProvider::new_testing();

    let ar_decimal: DataPayload<DecimalSymbolsV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("ar-EG").into(),
            ..Default::default()
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(ar_decimal.get().decimal_separator, "٫");
    assert_eq!(ar_decimal.get().digits[0], '٠');
}
