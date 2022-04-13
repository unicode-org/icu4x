// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::uprops::uprops_helpers::{self, get_last_component_no_version, TomlBinary};

use crate::SourceData;
use icu_properties::provider::UnicodePropertyV1;
use icu_properties::provider::UnicodePropertyV1Marker;
use icu_provider::datagen::IterableDynProvider;
use icu_provider::prelude::*;
use icu_uniset::UnicodeSetBuilder;
use std::sync::RwLock;

/// A data provider reading from TOML files produced by the ICU4C icuwriteuprops tool.
pub struct BinaryPropertyUnicodeSetDataProvider {
    source: SourceData,
    data: RwLock<Option<TomlBinary>>,
}

impl From<&SourceData> for BinaryPropertyUnicodeSetDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
            data: RwLock::new(None),
        }
    }
}

impl DynProvider<UnicodePropertyV1Marker> for BinaryPropertyUnicodeSetDataProvider {
    fn load_payload(
        &self,
        key: ResourceKey,
        _: &DataRequest,
    ) -> Result<DataResponse<UnicodePropertyV1Marker>, DataError> {
        if self.data.read().unwrap().is_none() {
            let data = uprops_helpers::load_binary_from_dir(self.source.get_uprops_root()?)?;
            *self.data.write().unwrap() = Some(data);
        }

        let guard = self.data.read().unwrap();

        let data = guard
            .as_ref()
            .unwrap()
            .get(get_last_component_no_version(key))
            .ok_or_else(|| DataErrorKind::MissingResourceKey.into_error())?;

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
    _k if _k.get_path().starts_with("props/") => UnicodePropertyV1Marker,
}, SERDE_SE, ITERABLE_SERDE_SE, DATA_CONVERTER);

impl IterableDynProvider<UnicodePropertyV1Marker> for BinaryPropertyUnicodeSetDataProvider {
    fn supported_options_for_key(
        &self,
        _: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

#[test]
fn test_basic() {
    use icu_properties::provider::key;
    use icu_uniset::UnicodeSet;
    use std::convert::TryInto;

    let provider = BinaryPropertyUnicodeSetDataProvider::from(&SourceData::for_test());

    let payload: DataPayload<UnicodePropertyV1Marker> = provider
        .load_payload(key::WHITE_SPACE_V1, &DataRequest::default())
        .expect("The data should be valid")
        .take_payload()
        .expect("Loading was successful");

    let whitespace: UnicodeSet = payload.get().clone().try_into().expect("Valid unicode set");

    assert!(whitespace.contains(' '));
    assert!(whitespace.contains('\n'));
    assert!(whitespace.contains('\u{3000}')); // U+3000 IDEOGRAPHIC SPACE

    assert!(!whitespace.contains('A'));
}
