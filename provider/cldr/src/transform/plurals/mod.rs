// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::open_reader;
use crate::CldrPaths;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 2] = [
    key::CARDINAL_V1, //
    key::ORDINAL_V1,  //
];

/// A data provider reading from CLDR JSON plural rule files.
#[derive(PartialEq, Debug)]
pub struct PluralsProvider {
    cardinal_rules: Option<cldr_serde::plurals::Rules>,
    ordinal_rules: Option<cldr_serde::plurals::Rules>,
}

impl TryFrom<&dyn CldrPaths> for PluralsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let cardinal_rules = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("plurals.json");
            let data: cldr_serde::plurals::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.supplemental.plurals_type_cardinal
        };
        let ordinal_rules = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("ordinals.json");
            let data: cldr_serde::plurals::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.supplemental.plurals_type_ordinal
        };
        Ok(PluralsProvider {
            cardinal_rules,
            ordinal_rules,
        })
    }
}

impl KeyedDataProvider for PluralsProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::Plurals || resc_key.version != 1 {
            return Err(DataErrorKind::MissingResourceKey.with_key(*resc_key));
        }
        Ok(())
    }
}

impl PluralsProvider {
    fn get_rules_for(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<&cldr_serde::plurals::Rules, DataError> {
        PluralsProvider::supports_key(resc_key)?;
        match *resc_key {
            key::CARDINAL_V1 => self.cardinal_rules.as_ref(),
            key::ORDINAL_V1 => self.ordinal_rules.as_ref(),
            _ => return Err(DataErrorKind::MissingResourceKey.with_key(*resc_key)),
        }
        .ok_or_else(|| DataErrorKind::MissingResourceKey.with_key(*resc_key))
    }
}

impl DataProvider<PluralRulesV1Marker> for PluralsProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<PluralRulesV1Marker>, DataError> {
        let cldr_rules = self.get_rules_for(&req.resource_path.key)?;
        // TODO: Implement language fallback?
        let langid = req.try_langid()?;
        let r = match cldr_rules.0.get(langid) {
            Some(v) => v,
            None => return Err(DataErrorKind::MissingLocale.with_req(req)),
        };
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(PluralRulesV1::from(r))),
        })
    }
}

icu_provider::impl_dyn_provider!(PluralsProvider, {
    _ => PluralRulesV1Marker,
}, SERDE_SE);

impl IterableDataProviderCore for PluralsProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let cldr_rules = self.get_rules_for(resc_key)?;
        let list: Vec<ResourceOptions> = cldr_rules
            .0
            .iter()
            .map(|(l, _)| ResourceOptions {
                variant: None,
                // TODO: Avoid the clone
                langid: Some(l.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

impl From<&cldr_serde::plurals::LocalePluralRules> for PluralRulesV1<'static> {
    fn from(other: &cldr_serde::plurals::LocalePluralRules) -> Self {
        /// Removes samples from plural rule strings. Takes an owned [`String`] reference and
        /// returns a new [`String`] in a [`Cow::Owned`].
        #[allow(clippy::ptr_arg)]
        fn convert(s: &String) -> Rule<'static> {
            s.parse().expect("Rule parsing failed.")
        }
        Self {
            zero: other.zero.as_ref().map(convert),
            one: other.one.as_ref().map(convert),
            two: other.two.as_ref().map(convert),
            few: other.few.as_ref().map(convert),
            many: other.many.as_ref().map(convert),
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = PluralsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    // Spot-check locale 'cs' since it has some interesting entries
    let cs_rules: DataPayload<PluralRulesV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::CARDINAL_V1,
                options: ResourceOptions {
                    variant: None,
                    langid: Some(langid!("cs")),
                },
            },
        })
        .unwrap()
        .take_payload()
        .unwrap();

    assert_eq!(None, cs_rules.get().zero);
    assert_eq!(
        Some("i = 1 and v = 0".parse().expect("Failed to parse rule")),
        cs_rules.get().one
    );
    assert_eq!(None, cs_rules.get().two);
    assert_eq!(
        Some("i = 2..4 and v = 0".parse().expect("Failed to parse rule")),
        cs_rules.get().few
    );
    assert_eq!(
        Some("v != 0".parse().expect("Failed to parse rule")),
        cs_rules.get().many
    );
}
