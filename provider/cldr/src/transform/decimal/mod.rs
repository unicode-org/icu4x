// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::support::KeyedDataProvider;
use crate::CldrPaths;
use icu_decimal::provider::*;
use icu_locid::LanguageIdentifier;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::borrow::Cow;
use std::convert::TryFrom;
use tinystr::TinyStr8;

mod decimal_pattern;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::SYMBOLS_V1, //
];

/// A data provider reading from CLDR JSON plural rule files.
#[derive(PartialEq, Debug)]
pub struct NumbersProvider {
    cldr_numbering_systems_data: cldr_serde::numbering_systems::Resource,
    cldr_numbers_data: LiteMap<LanguageIdentifier, cldr_serde::numbers::LangNumbers>,
}

impl TryFrom<&dyn CldrPaths> for NumbersProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        // Load common numbering system data:
        let cldr_numbering_systems_data: cldr_serde::numbering_systems::Resource = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("numberingSystems.json");
            serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?
        };

        // Load data for each locale:
        let mut cldr_numbers_data = LiteMap::new();
        let path = cldr_paths.cldr_numbers()?.join("main");
        let locale_dirs = get_langid_subdirectories(&path)?;
        for dir in locale_dirs {
            let path = dir.join("numbers.json");
            let resource: cldr_serde::numbers::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            cldr_numbers_data.extend_from_litemap(resource.main.0);
        }

        Ok(Self {
            cldr_numbering_systems_data,
            cldr_numbers_data,
        })
    }
}

impl KeyedDataProvider for NumbersProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        resc_key.match_key(key::SYMBOLS_V1)
    }
}

impl NumbersProvider {
    /// Returns the digits for the given numbering system name.
    fn get_digits_for_numbering_system(&self, nsname: TinyStr8) -> Option<[char; 10]> {
        match self
            .cldr_numbering_systems_data
            .supplemental
            .numbering_systems
            .get(&nsname)
        {
            Some(ns) => match ns.digits.as_ref() {
                Some(digits_str) => {
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
                None => None,
            },
            None => None,
        }
    }
}

impl ResourceProvider<DecimalSymbolsV1Marker> for NumbersProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<DecimalSymbolsV1Marker>, DataError> {
        let langid = req.try_langid(DecimalSymbolsV1Marker::KEY)?;
        let numbers = match self.cldr_numbers_data.get(langid) {
            Some(v) => &v.numbers,
            None => {
                return Err(DataErrorKind::MissingLocale.with_req(DecimalSymbolsV1Marker::KEY, req))
            }
        };
        let nsname = numbers.default_numbering_system;

        let mut result = DecimalSymbolsV1::try_from(numbers)
            .map_err(|s| Error::Custom(s.to_string(), Some(langid.clone())))?;
        result.digits = self
            .get_digits_for_numbering_system(nsname)
            .ok_or_else(|| {
                Error::Custom(
                    format!("Could not process numbering system: {:?}", nsname),
                    Some(langid.clone()),
                )
            })?;

        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(result)),
        })
    }
}

icu_provider::impl_dyn_provider!(NumbersProvider, [DecimalSymbolsV1Marker,], SERDE_SE);

impl IterableProvider for NumbersProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .cldr_numbers_data
            .iter()
            .map(|(l, _)| ResourceOptions {
                variant: None,
                // TODO(#568): Avoid the clone
                langid: Some(l.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
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
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = NumbersProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let ar_decimal: DataPayload<DecimalSymbolsV1Marker> = provider
        .load_resource(&DataRequest {
            options: langid!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(ar_decimal.get().decimal_separator, "٫");
    assert_eq!(ar_decimal.get().digits[0], '٠');
}
