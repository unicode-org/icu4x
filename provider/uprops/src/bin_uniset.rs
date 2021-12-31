// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops_helpers::{self, TomlBinary};

use icu_properties::provider::UnicodePropertyV1;
use icu_properties::provider::UnicodePropertyV1Marker;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;
use std::path::Path;

pub struct BinaryPropertyUnicodeSetDataProvider {
    data: TomlBinary,
}

/// A data provider reading from .toml files produced by the ICU4C icuwriteuprops tool.
impl BinaryPropertyUnicodeSetDataProvider {
    pub fn try_new(root_dir: &Path) -> eyre::Result<Self> {
        let data = uprops_helpers::load_binary_from_dir(root_dir)?;
        Ok(Self { data })
    }
}

impl DataProvider<UnicodePropertyV1Marker> for BinaryPropertyUnicodeSetDataProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyV1Marker>, DataError> {
        let data = &self
            .data
            .get(&req.resource_path.key.sub_category)
            .ok_or_else(|| DataErrorKind::MissingResourceKey.with_req(req))?;

        let mut builder = UnicodeSetBuilder::new();
        for (start, end) in &data.ranges {
            builder.add_range_u32(&(start..=end));
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

icu_provider::impl_dyn_provider!(BinaryPropertyUnicodeSetDataProvider, {
    _ => UnicodePropertyV1Marker,
}, SERDE_SE);

impl IterableProvider for BinaryPropertyUnicodeSetDataProvider {
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
    use icu_properties::provider::key;
    use icu_uniset::UnicodeSet;
    use std::convert::TryInto;

    let root_dir = icu_testdata::paths::uprops_toml_root();
    let provider = BinaryPropertyUnicodeSetDataProvider::try_new(&root_dir)
        .expect("TOML should load successfully");

    let payload: DataPayload<UnicodePropertyV1Marker> = provider
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
