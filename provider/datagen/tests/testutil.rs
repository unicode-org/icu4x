// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use elsa::sync::FrozenMap;
use icu_datagen::prelude::*;
use icu_provider::datagen::ExportMarker;
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};

#[derive(Default)]
pub struct TestingExporter(FrozenMap<(LanguageIdentifier, DataKeyAttributes), Vec<u8>>);

impl DataExporter for &mut TestingExporter {
    fn put_payload(
        &self,
        key: DataKey,
        langid: &LanguageIdentifier,
        key_attributes: &DataKeyAttributes,
        payload: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut serializer = postcard::Serializer {
            output: AllocVec::new(),
        };
        payload.serialize(&mut serializer)?;
        let output = serializer
            .output
            .finalize()
            .expect("Failed to finalize serializer output");
        println!("Putting: {key}/{}/{langid}", key_attributes as &str);
        self.0
            .insert((langid.clone(), key_attributes.clone()), output);
        Ok(())
    }
}

impl TestingExporter {
    pub fn finish(self) -> BTreeMap<String, Vec<u8>> {
        self.0
            .into_tuple_vec()
            .into_iter()
            .map(|((locale, key_attributes), buffer)| {
                (
                    DataRequest {
                        langid: &locale,
                        key_attributes: &key_attributes,
                        ..Default::default()
                    }
                    .legacy_encode(),
                    buffer,
                )
            })
            .collect()
    }
}
