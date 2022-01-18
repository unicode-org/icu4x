// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_helpers::{self, TomlEnumerated};

use icu_properties::provider::UnicodePropertyV1;
use icu_properties::provider::UnicodePropertyV1Marker;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;
use std::path::Path;

pub struct EnumeratedPropertyUnicodeSetDataProvider {
    data: TomlEnumerated,
}

/// A data provider reading from .toml files produced by the ICU4C icuwriteuprops tool.
impl EnumeratedPropertyUnicodeSetDataProvider {
    pub fn try_new(root_dir: &Path) -> eyre::Result<Self> {
        let data = uprops_helpers::load_enumerated_from_dir(root_dir)?;
        Ok(Self { data })
    }
}

fn expand_groupings<'a>(prop_name: &str, prop_val: &'a str) -> Vec<&'a str> {
    match prop_name {
        "gc" => match prop_val {
            // GeneralCategoryGroup::CasedLetter
            "LC" => vec!["Lu", "Ll", "Lt"],

            // GeneralCategoryGroup::Letter
            "L" => vec!["Lu", "Ll", "Lt", "Lm", "Lo"],

            // GeneralCategoryGroup::Mark
            "M" => vec!["Mn", "Mc", "Me"],

            // GeneralCategoryGroup::Number
            "N" => vec!["Nd", "Nl", "No"],

            // GeneralCategoryGroup::Punctuation
            "P" => vec!["Pc", "Pd", "Ps", "Pe", "Pi", "Pf", "Po"],

            // GeneralCategoryGroup::Symbol
            "S" => vec!["Sm", "Sc", "Sk", "So"],

            // GeneralCategoryGroup::Separator
            "Z" => vec!["Zs", "Zl", "Zp"],

            // GeneralCategoryGroup::Control
            "C" => vec!["Cc", "Cf", "Cs", "Co", "Cn"],

            _ => vec![prop_val],
        },
        _ => vec![prop_val],
    }
}

impl DataProvider<UnicodePropertyV1Marker> for EnumeratedPropertyUnicodeSetDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyV1Marker>, DataError> {
        let key = &req.resource_path.key.get_last_component_no_version();

        // ResourceKey subcategory strings for enumerated properties are
        // of the form "name=value", using the short name for both.
        let (prop_name, prop_value) = {
            let parts = key.split('=').collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(DataErrorKind::MissingResourceKey.with_req(req));
            }
            (parts[0], parts[1])
        };

        let toml_data = &self
            .data
            .get(prop_name)
            .ok_or_else(|| DataErrorKind::MissingResourceKey.with_req(req))?;

        let valid_names = expand_groupings(&prop_name, prop_value);

        let mut builder = UnicodeSetBuilder::new();
        for range in &toml_data.ranges {
            if valid_names.iter().any(|&name| name == range.name) {
                builder.add_range_u32(&(range.a..=range.b));
            }
        }
        let uniset = builder.build();

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(
                UnicodePropertyV1::from_owned_uniset(uniset),
            )),
        })
    }
}

icu_provider::impl_dyn_provider!(EnumeratedPropertyUnicodeSetDataProvider, {
    _ => UnicodePropertyV1Marker,
}, SERDE_SE);

impl IterableProvider for EnumeratedPropertyUnicodeSetDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_properties::provider::key;

    #[test]
    fn test_general_category() {
        use icu_uniset::UnicodeSet;
        use std::convert::TryInto;

        let root_dir = icu_testdata::paths::uprops_toml_root();
        let provider = EnumeratedPropertyUnicodeSetDataProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<UnicodePropertyV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::GENERAL_CATEGORY_NUMBER_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let digits: UnicodeSet = payload.get().clone().try_into().expect("Valid unicode set");

        assert!(digits.contains('5'));
        assert!(digits.contains('\u{0665}')); // U+0665 ARABIC-INDIC DIGIT FIVE
        assert!(digits.contains('\u{096b}')); // U+0969 DEVANAGARI DIGIT FIVE

        assert!(!digits.contains('A'));
    }

    #[test]
    fn test_script() {
        use icu_uniset::UnicodeSet;
        use std::convert::TryInto;

        let root_dir = icu_testdata::paths::uprops_toml_root();
        let provider = EnumeratedPropertyUnicodeSetDataProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<UnicodePropertyV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::SCRIPT_THAI_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let thai: UnicodeSet = payload.get().clone().try_into().expect("Valid unicode set");

        assert!(thai.contains('\u{0e01}')); // U+0E01 THAI CHARACTER KO KAI
        assert!(thai.contains('\u{0e50}')); // U+0E50 THAI DIGIT ZERO

        assert!(!thai.contains('A'));
        assert!(!thai.contains('\u{0e3f}')); // U+0E50 THAI CURRENCY SYMBOL BAHT
    }

    #[test]
    fn test_gc_groupings() {
        use icu_uniset::{UnicodeSet, UnicodeSetBuilder};
        use std::convert::TryInto;

        fn get_uniset_payload(key: ResourceKey) -> DataPayload<UnicodePropertyV1Marker> {
            let root_dir = icu_testdata::paths::uprops_toml_root();
            let provider = EnumeratedPropertyUnicodeSetDataProvider::try_new(&root_dir)
                .expect("TOML should load successfully");
            let payload: DataPayload<UnicodePropertyV1Marker> = provider
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key,
                        options: ResourceOptions::default(),
                    },
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
            payload
        }

        let test_group = |category: ResourceKey, subcategories: &[ResourceKey]| {
            let category_set_payload = get_uniset_payload(category);
            let category_set: UnicodeSet = category_set_payload
                .get()
                .clone()
                .try_into()
                .expect("Valid unicode set");
            let mut builder = UnicodeSetBuilder::new();
            for subcategory in subcategories {
                builder.add_set(
                    &get_uniset_payload(*subcategory)
                        .get()
                        .clone()
                        .try_into()
                        .expect("Valid unicode set"),
                );
            }
            let combined_set = builder.build();
            println!("{:?} {:?}", category, subcategories);
            assert_eq!(
                category_set.get_inversion_list(),
                combined_set.get_inversion_list()
            );
        };

        test_group(
            key::GENERAL_CATEGORY_LETTER_V1,
            &[
                key::GENERAL_CATEGORY_UPPERCASE_LETTER_V1,
                key::GENERAL_CATEGORY_LOWERCASE_LETTER_V1,
                key::GENERAL_CATEGORY_TITLECASE_LETTER_V1,
                key::GENERAL_CATEGORY_MODIFIER_LETTER_V1,
                key::GENERAL_CATEGORY_OTHER_LETTER_V1,
            ],
        );
        test_group(
            key::GENERAL_CATEGORY_OTHER_V1,
            &[
                key::GENERAL_CATEGORY_CONTROL_V1,
                key::GENERAL_CATEGORY_FORMAT_V1,
                key::GENERAL_CATEGORY_UNASSIGNED_V1,
                key::GENERAL_CATEGORY_PRIVATE_USE_V1,
                key::GENERAL_CATEGORY_SURROGATE_V1,
            ],
        );
        test_group(
            key::GENERAL_CATEGORY_MARK_V1,
            &[
                key::GENERAL_CATEGORY_SPACING_MARK_V1,
                key::GENERAL_CATEGORY_ENCLOSING_MARK_V1,
                key::GENERAL_CATEGORY_NONSPACING_MARK_V1,
            ],
        );
        test_group(
            key::GENERAL_CATEGORY_NUMBER_V1,
            &[
                key::GENERAL_CATEGORY_DIGIT_V1,
                key::GENERAL_CATEGORY_LETTER_NUMBER_V1,
                key::GENERAL_CATEGORY_OTHER_NUMBER_V1,
            ],
        );
        test_group(
            key::GENERAL_CATEGORY_PUNCTUATION_V1,
            &[
                key::GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1,
                key::GENERAL_CATEGORY_DASH_PUNCTUATION_V1,
                key::GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1,
                key::GENERAL_CATEGORY_FINAL_PUNCTUATION_V1,
                key::GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1,
                key::GENERAL_CATEGORY_OTHER_PUNCTUATION_V1,
                key::GENERAL_CATEGORY_OPEN_PUNCTUATION_V1,
            ],
        );
        test_group(
            key::GENERAL_CATEGORY_SYMBOL_V1,
            &[
                key::GENERAL_CATEGORY_CURRENCY_SYMBOL_V1,
                key::GENERAL_CATEGORY_MODIFIER_SYMBOL_V1,
                key::GENERAL_CATEGORY_MATH_SYMBOL_V1,
                key::GENERAL_CATEGORY_OTHER_SYMBOL_V1,
            ],
        );
        test_group(
            key::GENERAL_CATEGORY_SEPARATOR_V1,
            &[
                key::GENERAL_CATEGORY_LINE_SEPARATOR_V1,
                key::GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1,
                key::GENERAL_CATEGORY_SPACE_SEPARATOR_V1,
            ],
        );
    }

    #[test]
    fn test_gc_surrogate() {
        use icu_uniset::UnicodeSet;
        use std::convert::TryInto;

        let root_dir = icu_testdata::paths::uprops_toml_root();
        let provider = EnumeratedPropertyUnicodeSetDataProvider::try_new(&root_dir)
            .expect("TOML should load successfully");

        let payload: DataPayload<UnicodePropertyV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: key::GENERAL_CATEGORY_SURROGATE_V1,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let surrogates: UnicodeSet = payload.get().clone().try_into().expect("Valid unicode set");

        assert!(surrogates.contains_u32(0xd800));
        assert!(surrogates.contains_u32(0xd900));
        assert!(surrogates.contains_u32(0xdfff));

        assert!(!surrogates.contains('A'));
    }
}
