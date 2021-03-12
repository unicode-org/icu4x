// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_langid::CldrLangID;
use crate::error::Error;
use crate::reader::{get_subdirectories, open_reader};
use crate::CldrPaths;
use icu_decimal::provider::*;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryFrom;

mod cldr_serde;
mod decimal_pattern;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 0] = [
    // key::SYMBOLS_V1, //
];

/// A data provider reading from CLDR JSON plural rule files.
#[derive(PartialEq, Debug)]
pub struct NumbersProvider {
    data: Vec<(CldrLangID, cldr_serde::numbers_json::LangNumbers)>,
}

impl TryFrom<&dyn CldrPaths> for NumbersProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let path = cldr_paths.cldr_numbers()?.join("main");

        let locale_dirs = get_subdirectories(&path)?;

        for dir in locale_dirs {
            let path = dir.join("numbers.json");

            let mut resource: cldr_serde::numbers_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.append(&mut resource.main.0);
        }

        Ok(Self { data })
    }
}

impl KeyedDataProvider for NumbersProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::Decimal {
            return Err((&resc_key.category).into());
        }
        if resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'d> DataProvider<'d, DecimalSymbolsV1> for NumbersProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, DecimalSymbolsV1>, DataError> {
        NumbersProvider::supports_key(&req.resource_path.key)?;
        let langid = req.try_langid()?;
        let cldr_langid: CldrLangID = langid.clone().into();
        let numbers = match self
            .data
            .binary_search_by_key(&&cldr_langid, |(lid, _)| lid)
        {
            Ok(idx) => &self.data[idx].1.numbers,
            Err(_) => return Err(DataError::UnavailableResourceOptions(req.clone())),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(Cow::Owned(
                DecimalSymbolsV1::try_from(numbers)
                    .map_err(|s| Error::Custom(s.to_string(), Some(langid.clone())))
                    .map_err(DataError::new_resc_error)?,
            )),
        })
    }
}

icu_provider::impl_erased!(NumbersProvider, 'd);

impl<'d> IterableDataProvider<'d> for NumbersProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        unimplemented!()
    }
}

impl TryFrom<&cldr_serde::numbers_json::Numbers> for DecimalSymbolsV1 {
    // TODO: Use a more expressive error type
    type Error = &'static str;

    fn try_from(other: &cldr_serde::numbers_json::Numbers) -> Result<Self, &'static str> {
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
        let parsed_pattern: decimal_pattern::DecimalPattern =
            formats.standard.parse().map_err(|_| "Could not parse decimal pattern")?;

        Ok(Self {
            minus_sign_affixes: parsed_pattern.localize_sign(&symbols.minus_sign),
            plus_sign_affixes: parsed_pattern.localize_sign(&symbols.plus_sign),
            decimal_separator: symbols.decimal.clone(),
            grouping_separator: symbols.group.clone(),
            grouping_sizes: GroupingSizesV1 {
                primary: parsed_pattern.positive.primary_grouping,
                secondary: parsed_pattern.positive.secondary_grouping,
                min_grouping: other.minimum_grouping_digits,
            },
            zero_digit: '0', // TODO
        })
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;
    use std::borrow::Cow;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = NumbersProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let ar_decimal: Cow<DecimalSymbolsV1> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::SYMBOLS_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("ar")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(ar_decimal.decimal_separator, "٫");
}
