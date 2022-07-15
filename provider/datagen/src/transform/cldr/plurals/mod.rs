// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::SourceData;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;

/// A data provider reading from CLDR JSON plural rule files.
#[derive(Debug)]
pub struct PluralsProvider {
    source: SourceData,
}

impl From<&SourceData> for PluralsProvider {
    fn from(source: &SourceData) -> Self {
        PluralsProvider {
            source: source.clone(),
        }
    }
}

impl PluralsProvider {
    fn get_rules_for(&self, key: ResourceKey) -> Result<&cldr_serde::plurals::Rules, DataError> {
        if key == CardinalV1Marker::KEY {
            self.source
                .cldr()?
                .core()
                .read_and_parse::<cldr_serde::plurals::Resource>("supplemental/plurals.json")?
                .supplemental
                .plurals_type_cardinal
                .as_ref()
        } else if key == OrdinalV1Marker::KEY {
            self.source
                .cldr()?
                .core()
                .read_and_parse::<cldr_serde::plurals::Resource>("supplemental/ordinals.json")?
                .supplemental
                .plurals_type_ordinal
                .as_ref()
        } else {
            None
        }
        .ok_or(DataError::custom("Unknown key for PluralRulesV1"))
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
                    .0
                    .get(&req.options.get_langid())
                    .ok_or(DataErrorKind::MissingLocale.into_error())?,
            ))),
        })
    }
}

icu_provider::make_exportable_provider!(PluralsProvider, [OrdinalV1Marker, CardinalV1Marker,]);

impl<M: ResourceMarker<Yokeable = PluralRulesV1<'static>>> IterableResourceProvider<M>
    for PluralsProvider
{
    fn supported_options(&self) -> Result<Vec<ResourceOptions>, DataError> {
        Ok(self
            .get_rules_for(M::KEY)?
            .0
            .keys()
            // TODO(#568): Avoid the clone
            .cloned()
            .map(ResourceOptions::from)
            .collect())
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

    let provider = PluralsProvider::from(&SourceData::for_test());

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
