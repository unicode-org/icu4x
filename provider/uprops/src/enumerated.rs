// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::upropdump_serde;
use icu_provider::iter::IterableDataProviderCore;
use icu_provider::prelude::*;
use icu_uniset::provider::*;
use icu_uniset::UnicodeSetBuilder;
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

pub struct EnumeratedPropertiesDataProvider {
    root_dir: PathBuf,
}

/// A data provider reading from .toml files produced by the ICU4C upropdump tool.
impl EnumeratedPropertiesDataProvider {
    pub fn new(root_dir: PathBuf) -> Self {
        EnumeratedPropertiesDataProvider { root_dir }
    }
    fn get_toml_data(&self, name: &str) -> Result<upropdump_serde::enumerated::Main, Error> {
        let mut path: PathBuf = self.root_dir.clone().join(name);
        path.set_extension("toml");
        let toml_str = fs::read_to_string(&path).map_err(|e| Error::Io(e, path.clone()))?;
        toml::from_str(&toml_str).map_err(|e| Error::Toml(e, path))
    }
}

fn expand_groupings<'a>(prop_name: &str, prop_val: &'a str) -> Vec<&'a str> {
    match (prop_name, prop_val) {
        // General_Category::CasedLetter
        ("gc", "LC") => vec!["Lu", "Ll", "Lt"],

        // General_Category::Letter
        ("gc", "L") => vec!["Lu", "Ll", "Lt", "Lm", "Lo"],

        // General_Category::Mark
        ("gc", "M") => vec!["Mn", "Mc", "Me"],

        // General_Category::Number
        ("gc", "N") => vec!["Nd", "Nl", "No"],

        // General_Category::Punctuation
        ("gc", "P") => vec!["Pc", "Pd", "Ps", "Pe", "Pi", "Pf", "Po"],

        // General_Category::Symbol
        ("gc", "S") => vec!["Sm", "Sc", "Sk", "So"],

        // General_Category::Separator
        ("gc", "Z") => vec!["Zs", "Zl", "Zp"],

        // General_Category::Control
        ("gc", "C") => vec!["Cc", "Cf", "Cs", "Co", "Cn"],

        (_, prop_val) => vec![prop_val],
    }
}

impl<'data> DataProvider<'data, UnicodePropertyV1Marker> for EnumeratedPropertiesDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'data, UnicodePropertyV1Marker>, DataError> {
        let key = &req.resource_path.key.sub_category;

        // ResourceKey subcategory strings for enumerated properties are
        // of the form "name=value", using the short name for both.
        let (prop_name, prop_value) = {
            let parts = key.split('=').collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(DataError::MissingResourceKey(req.resource_path.key));
            }
            (parts[0], parts[1])
        };

        let toml_data: upropdump_serde::enumerated::Main = self
            .get_toml_data(prop_name)
            .map_err(DataError::new_resc_error)?;

        let valid_names = expand_groupings(prop_name, prop_value);

        let mut builder = UnicodeSetBuilder::new();
        let ranges = toml_data.code_point_map.data.ranges;
        for range in ranges {
            if valid_names.iter().any(|&name| name == range.short_name) {
                builder.add_range_u32(&(range.start..=range.end));
            }
        }
        let uniset = builder.build();

        let name = Cow::from(toml_data.code_point_map.data.name);
        Ok(DataResponse {
            metadata: DataResponseMetadata {
                data_langid: req.resource_path.options.langid.clone(),
            },
            payload: Some(DataPayload::from_owned(UnicodePropertyV1::from_uniset(
                &uniset, name,
            ))),
        })
    }
}

icu_provider::impl_dyn_provider!(EnumeratedPropertiesDataProvider, {
    _ => UnicodePropertyV1Marker,
}, SERDE_SE, 'data);

impl IterableDataProviderCore for EnumeratedPropertiesDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[test]
fn test_general_category() {
    use icu_uniset::UnicodeSet;
    use std::convert::TryInto;

    let root_dir = icu_testdata::paths::data_root().join("uprops");
    let provider = EnumeratedPropertiesDataProvider::new(root_dir);

    let payload: DataPayload<'_, UnicodePropertyV1Marker> = provider
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

    let root_dir = icu_testdata::paths::data_root().join("uprops");
    let provider = EnumeratedPropertiesDataProvider::new(root_dir);

    let payload: DataPayload<'_, UnicodePropertyV1Marker> = provider
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

    let root_dir = icu_testdata::paths::data_root().join("uprops");
    let provider = EnumeratedPropertiesDataProvider::new(root_dir);

    let get_uniset = |key| -> UnicodeSet {
        let payload: DataPayload<'_, UnicodePropertyV1Marker> = provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key,
                    options: ResourceOptions::default(),
                },
            })
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        payload.get().clone().try_into().expect("Valid unicode set")
    };

    let test_group = |category: ResourceKey, subcategories: &[ResourceKey]| {
        let category_set = get_uniset(category);
        let mut builder = UnicodeSetBuilder::new();
        for subcategory in subcategories {
            builder.add_set(&get_uniset(*subcategory));
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

    let root_dir = icu_testdata::paths::data_root().join("uprops");
    let provider = EnumeratedPropertiesDataProvider::new(root_dir);

    let payload: DataPayload<'_, UnicodePropertyV1Marker> = provider
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
