// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

impl crate::DatagenProvider {
    fn get_rules_for(&self, key: DataKey) -> Result<&cldr_serde::plurals::Rules, DataError> {
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

macro_rules! implement {
    ($marker:ident) => {
        impl DataProvider<$marker> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: Some(DataPayload::from_owned(PluralRulesV1::from(
                        self.get_rules_for(<$marker>::KEY)?
                            .0
                            .get(&req.locale.get_langid())
                            .ok_or(DataErrorKind::MissingLocale.into_error())?,
                    ))),
                })
            }
        }

        impl IterableDataProvider<$marker> for crate::DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                Ok(self.filter_data_locales(
                    self.get_rules_for(<$marker>::KEY)?
                        .0
                        .keys()
                        // TODO(#568): Avoid the clone
                        .cloned()
                        .map(DataLocale::from)
                        .collect(),
                ))
            }
        }
    };
}

implement!(CardinalV1Marker);
implement!(OrdinalV1Marker);

impl From<&cldr_serde::plurals::LocalePluralRules> for PluralRulesV1<'static> {
    fn from(other: &cldr_serde::plurals::LocalePluralRules) -> Self {
        /// Removes samples from plural rule strings. Takes an owned [`String`] reference and
        /// returns a new [`String`] in a [`Cow::Owned`].
        fn convert(s: &str) -> Rule<'static> {
            s.parse().expect("Rule parsing failed.")
        }
        Self {
            zero: other.zero.as_deref().map(convert),
            one: other.one.as_deref().map(convert),
            two: other.two.as_deref().map(convert),
            few: other.few.as_deref().map(convert),
            many: other.many.as_deref().map(convert),
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid::langid;

    let provider = crate::DatagenProvider::for_test();

    // Spot-check locale 'cs' since it has some interesting entries
    let cs_rules: DataPayload<CardinalV1Marker> = provider
        .load(DataRequest {
            locale: &langid!("cs").into(),
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
