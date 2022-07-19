// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::SourceData;
use icu_decimal::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use tinystr::TinyStr8;

mod decimal_pattern;

/// A data provider reading from CLDR JSON plural rule files.
#[derive(Debug)]
pub struct NumbersProvider {
    source: SourceData,
}

impl From<&SourceData> for NumbersProvider {
    fn from(source: &SourceData) -> Self {
        NumbersProvider {
            source: source.clone(),
        }
    }
}

impl NumbersProvider {
    /// Returns the digits for the given numbering system name.
    fn get_digits_for_numbering_system(&self, nsname: TinyStr8) -> Result<[char; 10], DataError> {
        let resource: &cldr_serde::numbering_systems::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/numberingSystems.json")?;

        fn digits_str_to_chars(digits_str: &str) -> Option<[char; 10]> {
            let mut chars = digits_str.chars();
            Some([
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
            ])
        }

        match resource.supplemental.numbering_systems.get(&nsname) {
            Some(ns) => ns.digits.as_deref().and_then(digits_str_to_chars),
            None => None,
        }
        .ok_or_else(|| {
            DataError::custom("Could not process numbering system").with_display_context(&nsname)
        })
    }
}

impl DataProvider<DecimalSymbolsV1Marker> for NumbersProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<DecimalSymbolsV1Marker>, DataError> {
        let langid = req.options.get_langid();

        let resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        let numbers = &resource
            .main
            .0
            .get(&langid)
            .expect("CLDR file contains the expected language")
            .numbers;

        let mut result = DecimalSymbolsV1::try_from(numbers).map_err(|s| {
            DataError::custom("Could not create decimal symbols").with_display_context(&s)
        })?;

        result.digits = self.get_digits_for_numbering_system(numbers.default_numbering_system)?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

icu_provider::make_exportable_provider!(NumbersProvider, [DecimalSymbolsV1Marker,]);

impl IterableDataProvider<DecimalSymbolsV1Marker> for NumbersProvider {
    fn supported_options(&self) -> Result<Vec<DataOptions>, DataError> {
        Ok(self
            .source
            .cldr()?
            .numbers()
            .list_langs()?
            .map(Into::<DataOptions>::into)
            .collect())
    }
}

impl TryFrom<&cldr_serde::numbers::Numbers> for DecimalSymbolsV1<'static> {
    type Error = Cow<'static, str>;

    fn try_from(other: &cldr_serde::numbers::Numbers) -> Result<Self, Self::Error> {
        // TODO(#510): Select from non-default numbering systems
        let symbols = other
            .numsys_data
            .symbols
            .get(&other.default_numbering_system)
            .ok_or("Could not find symbols for default numbering system")?;
        let formats = other
            .numsys_data
            .formats
            .get(&other.default_numbering_system)
            .ok_or("Could not find formats for default numbering system")?;
        let parsed_pattern: decimal_pattern::DecimalPattern = formats
            .standard
            .parse()
            .map_err(|s: decimal_pattern::Error| s.to_string())?;

        Ok(Self {
            minus_sign_affixes: parsed_pattern.localize_sign(&symbols.minus_sign),
            plus_sign_affixes: parsed_pattern.localize_sign(&symbols.plus_sign),
            decimal_separator: Cow::Owned(symbols.decimal.clone()),
            grouping_separator: Cow::Owned(symbols.group.clone()),
            grouping_sizes: GroupingSizesV1 {
                primary: parsed_pattern.positive.primary_grouping,
                secondary: parsed_pattern.positive.secondary_grouping,
                min_grouping: other.minimum_grouping_digits,
            },
            digits: Default::default(), // to be filled in
        })
    }
}

#[test]
fn test_basic() {
    use icu_locid::locale;

    let provider = NumbersProvider::from(&SourceData::for_test());

    let ar_decimal: DataPayload<DecimalSymbolsV1Marker> = provider
        .load_resource(&DataRequest {
            options: locale!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(ar_decimal.get().decimal_separator, "٫");
    assert_eq!(ar_decimal.get().digits[0], '٠');
}
