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

pub struct BinaryPropertiesDataProvider {
    root_dir: PathBuf,
}

/// A data provider reading from .toml files produced by the ICU4C upropdump tool.
impl BinaryPropertiesDataProvider {
    pub fn new(root_dir: PathBuf) -> Self {
        BinaryPropertiesDataProvider { root_dir }
    }
    fn get_toml_data(&self, name: &str) -> Result<upropdump_serde::binary::Main, Error> {
        let mut path: PathBuf = self.root_dir.clone().join(name);
        path.set_extension("toml");
        let toml_str = fs::read_to_string(&path).map_err(|e| Error::Io(e, path.clone()))?;
        toml::from_str(&toml_str).map_err(|e| Error::Toml(e, path))
    }
}

impl<'d> DataProvider<'d, UnicodePropertyV1Marker> for BinaryPropertiesDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<'d, UnicodePropertyV1Marker>, DataError> {
        let toml_data: upropdump_serde::binary::Main = self
            .get_toml_data(&req.resource_path.key.sub_category)
            .map_err(DataError::new_resc_error)?;

        let mut builder = UnicodeSetBuilder::new();
        for (start, end) in toml_data.unicode_set.data.ranges {
            let (start, end) = match (std::char::from_u32(start), std::char::from_u32(end)) {
                (Some(s), Some(e)) => Ok((s, e)),
                _ => Err(Error::InvalidCharRange(start, end)),
            }
            .map_err(DataError::new_resc_error)?;
            builder.add_range(&(start..=end));
        }
        let uniset = builder.build();

        let name = Cow::from(toml_data.unicode_set.data.name);
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

icu_provider::impl_dyn_provider!(BinaryPropertiesDataProvider, {
    _ => UnicodePropertyV1Marker,
}, SERDE_SE, 'd);

impl IterableDataProviderCore for BinaryPropertiesDataProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = vec![ResourceOptions::default()];
        Ok(Box::new(list.into_iter()))
    }
}

#[test]
fn test_basic() {
    use icu_uniset::UnicodeSet;
    use std::convert::TryInto;

    let root_dir = icu_testdata::paths::data_root().join("uprops");
    let provider = BinaryPropertiesDataProvider::new(root_dir);

    let payload: DataPayload<'_, UnicodePropertyV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::WHITE_SPACE_V1,
                options: ResourceOptions::default(),
            },
        })
        .expect("The data should be valid")
        .take_payload()
        .expect("Loading was successful");

    let whitespace: UnicodeSet = payload.get().clone().try_into().expect("Valid unicode set");

    assert!(whitespace.contains(' '));
    assert!(whitespace.contains('\n'));
    assert!(whitespace.contains('\u{3000}')); // U+3000 IDEOGRAPHIC SPACE

    assert!(!whitespace.contains('A'));
}
