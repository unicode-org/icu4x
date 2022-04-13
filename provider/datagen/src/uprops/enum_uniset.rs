// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops::uprops_helpers::{self, get_last_component_no_version, TomlEnumerated};

use crate::SourceData;
use icu_properties::provider::UnicodePropertyV1;
use icu_properties::provider::UnicodePropertyV1Marker;
use icu_provider::datagen::IterableDynProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;
use std::sync::RwLock;

pub struct EnumeratedPropertyUnicodeSetDataProvider {
    source: SourceData,
    data: RwLock<Option<TomlEnumerated>>,
}

impl From<&SourceData> for EnumeratedPropertyUnicodeSetDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
            data: RwLock::new(None),
        }
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

impl DynProvider<UnicodePropertyV1Marker> for EnumeratedPropertyUnicodeSetDataProvider {
    fn load_payload(
        &self,
        key: ResourceKey,
        _: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyV1Marker>, DataError> {
        let key_str = get_last_component_no_version(key);

        // ResourceKey subcategory strings for enumerated properties are
        // of the form "name=value", using the short name for both.
        let (prop_name, prop_value) = {
            let parts = key_str.split('=').collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(DataErrorKind::MissingResourceKey.into_error());
            }
            #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
            (parts[0], parts[1])
        };

        if self.data.read().unwrap().is_none() {
            let data = uprops_helpers::load_enumerated_from_dir(self.source.get_uprops_root()?)?;
            *self.data.write().unwrap() = Some(data);
        }

        let guard = self.data.read().unwrap();

        let toml_data = &guard
            .as_ref()
            .unwrap()
            .get(prop_name)
            .ok_or_else(|| DataErrorKind::MissingResourceKey.into_error())?;

        let valid_names = expand_groupings(prop_name, prop_value);

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
    _k if _k.get_path().starts_with("props/") && _k.get_path().contains('=') => UnicodePropertyV1Marker,
}, SERDE_SE, ITERABLE_SERDE_SE, DATA_CONVERTER);

impl IterableDynProvider<UnicodePropertyV1Marker> for EnumeratedPropertyUnicodeSetDataProvider {
    fn supported_options_for_key(
        &self,
        _: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}
