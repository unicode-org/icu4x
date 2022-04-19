// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr::cldr_serde;
use crate::cldr::error::Error;
use crate::cldr::reader::open_reader;
use crate::cldr::CldrPaths;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::sync::RwLock;

/// A data provider reading from CLDR JSON plural rule files.
#[derive(Debug)]
pub struct PluralsProvider {
    cldr_core: PathBuf,
    cardinal_rules: RwLock<Option<Option<cldr_serde::plurals::Rules>>>,
    ordinal_rules: RwLock<Option<Option<cldr_serde::plurals::Rules>>>,
}

impl TryFrom<&dyn CldrPaths> for PluralsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        Ok(PluralsProvider {
            cldr_core: cldr_paths.cldr_core()?,
            cardinal_rules: RwLock::new(None),
            ordinal_rules: RwLock::new(None),
        })
    }
}

impl PluralsProvider {
    fn get_rules_for(
        &self,
        key: ResourceKey,
    ) -> Result<std::sync::RwLockReadGuard<Option<Option<cldr_serde::plurals::Rules>>>, DataError>
    {
        Ok(match key {
            CardinalV1Marker::KEY => {
                #[allow(clippy::unwrap_used)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                if self.cardinal_rules.read().unwrap().is_none() {
                    let path = self.cldr_core.join("supplemental").join("plurals.json");
                    let data: cldr_serde::plurals::Resource =
                        serde_json::from_reader(open_reader(&path)?)
                            .map_err(|e| Error::from((e, path)))?;
                    let _ = self
                        .cardinal_rules
                        .write()
                        .unwrap()
                        .get_or_insert(data.supplemental.plurals_type_cardinal);
                }
                #[allow(clippy::unwrap_used)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                self.cardinal_rules.read().unwrap()
            }
            OrdinalV1Marker::KEY => {
                #[allow(clippy::unwrap_used)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                if self.ordinal_rules.read().unwrap().is_none() {
                    let path = self.cldr_core.join("supplemental").join("ordinals.json");
                    let data: cldr_serde::plurals::Resource =
                        serde_json::from_reader(open_reader(&path)?)
                            .map_err(|e| Error::from((e, path)))?;
                    let _ = self
                        .ordinal_rules
                        .write()
                        .unwrap()
                        .get_or_insert(data.supplemental.plurals_type_ordinal);
                }
                #[allow(clippy::unwrap_used)]
                // TODO(#1668) Clippy exceptions need docs or fixing.
                self.ordinal_rules.read().unwrap()
            }
            _ => return Err(DataError::custom("Unknown key for PluralRulesV1").with_key(key)),
        })
    }
}

impl<M: ResourceMarker<Yokeable = PluralRulesV1<'static>>> ResourceProvider<M> for PluralsProvider {
    fn load_resource(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(PluralRulesV1::from(
                #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
                self.get_rules_for(M::KEY)?
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .ok_or_else(|| DataErrorKind::MissingResourceKey.with_key(M::KEY))?
                    .0
                    .get(&req.options.get_langid())
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(M::KEY, req))?,
            ))),
        })
    }
}

icu_provider::impl_dyn_provider!(
    PluralsProvider,
    [OrdinalV1Marker, CardinalV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

impl<M: ResourceMarker<Yokeable = PluralRulesV1<'static>>> IterableResourceProvider<M>
    for PluralsProvider
{
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(
            #[allow(clippy::unwrap_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
            self.get_rules_for(M::KEY)?
                .as_ref()
                .unwrap()
                .as_ref()
                .ok_or_else(|| DataErrorKind::MissingResourceKey.with_key(M::KEY))?
                .0
                .iter_keys()
                // TODO(#568): Avoid the clone
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .map(Into::<ResourceOptions>::into),
        ))
    }
}

impl From<&cldr_serde::plurals::LocalePluralRules> for PluralRulesV1<'static> {
    fn from(other: &cldr_serde::plurals::LocalePluralRules) -> Self {
        /// Removes samples from plural rule strings. Takes an owned [`String`] reference and
        /// returns a new [`String`] in a [`Cow::Owned`].
        #[allow(clippy::ptr_arg)]
        fn convert(s: &String) -> Rule<'static> {
            #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
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
    use icu_locid::langid;

    let cldr_paths = crate::cldr::cldr_paths::for_test();
    let provider = PluralsProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    // Spot-check locale 'cs' since it has some interesting entries
    let cs_rules: DataPayload<CardinalV1Marker> = provider
        .load_resource(&DataRequest {
            options: langid!("cs").into(),
            metadata: Default::default(),
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
