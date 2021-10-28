// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::reader::open_reader;
use crate::CldrPaths;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::marker::PhantomData;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 2] = [
    key::CARDINAL_V1, //
    key::ORDINAL_V1,  //
];

/// A data provider reading from CLDR JSON plural rule files.
#[derive(PartialEq, Debug)]
pub struct PluralsProvider<'data> {
    cardinal_rules: Option<cldr_json::Rules>,
    ordinal_rules: Option<cldr_json::Rules>,
    _phantom: PhantomData<&'data ()>, // placeholder for when we need the lifetime param
}

impl TryFrom<&dyn CldrPaths> for PluralsProvider<'_> {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let cardinal_rules = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("plurals.json");
            let data: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            let mut rules = data.supplemental.plurals_type_cardinal;
            if let Some(v) = rules.as_mut() {
                v.0.sort()
            }
            rules
        };
        let ordinal_rules = {
            let path = cldr_paths
                .cldr_core()?
                .join("supplemental")
                .join("ordinals.json");
            let data: cldr_json::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            let mut rules = data.supplemental.plurals_type_ordinal;
            if let Some(v) = rules.as_mut() {
                v.0.sort()
            }
            rules
        };
        Ok(PluralsProvider {
            cardinal_rules,
            ordinal_rules,
            _phantom: PhantomData,
        })
    }
}

impl<'data> KeyedDataProvider for PluralsProvider<'data> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        if resc_key.category != ResourceCategory::Plurals || resc_key.version != 1 {
            return Err(resc_key.into());
        }
        Ok(())
    }
}

impl<'data> PluralsProvider<'data> {
    fn get_rules_for(&self, resc_key: &ResourceKey) -> Result<&cldr_json::Rules, DataError> {
        PluralsProvider::supports_key(resc_key)?;
        match *resc_key {
            key::CARDINAL_V1 => self.cardinal_rules.as_ref(),
            key::ORDINAL_V1 => self.ordinal_rules.as_ref(),
            _ => return Err(resc_key.into()),
        }
        .ok_or_else(|| resc_key.into())
    }
}

impl<'data> DataProvider<'data, PluralRulesV1Marker> for PluralsProvider<'data> {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, PluralRulesV1Marker>, DataError> {
        let cldr_rules = self.get_rules_for(&req.resource_path.key)?;
        // TODO: Implement language fallback?
        let langid = req.try_langid()?;
        let (_, r) = match cldr_rules.0.binary_search_by_key(&langid, |(l, _)| l) {
            Ok(idx) => &cldr_rules.0[idx],
            Err(_) => return Err(req.clone().into()),
        };
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(PluralRulesV1::from(r))),
        })
    }
}

icu_provider::impl_dyn_provider!(PluralsProvider<'data>, {
    _ => PluralRulesV1Marker,
}, SERDE_SE, 'data);

impl<'data> IterableDataProviderCore for PluralsProvider<'data> {
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

impl From<&cldr_json::LocalePluralRules> for PluralRulesV1<'static> {
    fn from(other: &cldr_json::LocalePluralRules) -> Self {
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

/// Serde structs for the CLDR JSON plurals files.
pub(self) mod cldr_json {
    use icu_locid::LanguageIdentifier;
    use serde::Deserialize;

    // TODO: Use Serde Borrow throughout these structs. Blocked by:
    // https://stackoverflow.com/q/63201624/1407170

    #[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Deserialize)]
    pub struct LocalePluralRules {
        #[serde(rename = "pluralRule-count-zero")]
        pub zero: Option<String>,
        #[serde(rename = "pluralRule-count-one")]
        pub one: Option<String>,
        #[serde(rename = "pluralRule-count-two")]
        pub two: Option<String>,
        #[serde(rename = "pluralRule-count-few")]
        pub few: Option<String>,
        #[serde(rename = "pluralRule-count-many")]
        pub many: Option<String>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Rules(
        #[serde(with = "tuple_vec_map")] pub(crate) Vec<(LanguageIdentifier, LocalePluralRules)>,
    );

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Supplemental {
        #[serde(rename = "plurals-type-cardinal")]
        pub plurals_type_cardinal: Option<Rules>,
        #[serde(rename = "plurals-type-ordinal")]
        pub plurals_type_ordinal: Option<Rules>,
    }

    #[derive(PartialEq, Debug, Deserialize)]
    pub struct Resource {
        pub supplemental: Supplemental,
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;
    use std::borrow::Borrow;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = PluralsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    // Spot-check locale 'cs' since it has some interesting entries
    let cs_rules: DataPayload<PluralRuleStringsV1Marker> = provider
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
        Some("i = 1 and v = 0"),
        cs_rules.get().one.as_ref().map(|v| v.borrow())
    );
    assert_eq!(None, cs_rules.get().two);
    assert_eq!(
        Some("i = 2..4 and v = 0"),
        cs_rules.get().few.as_ref().map(|v| v.borrow())
    );
    assert_eq!(
        Some("v != 0"),
        cs_rules.get().many.as_ref().map(|v| v.borrow())
    );
}
