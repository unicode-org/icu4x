// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::BTreeMap;

use elsa::sync::FrozenMap;
use icu_provider::export::ExportMarker;
use icu_provider::prelude::*;
use icu_provider_export::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};

#[derive(Default)]
pub struct TestingExporter(FrozenMap<DataIdentifierCow<'static>, Vec<u8>>);

impl DataExporter for &mut TestingExporter {
    fn put_payload(
        &self,
        marker: DataMarkerInfo,
        id: DataIdentifierBorrowed,
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
        println!(
            "Putting: {marker:?}/{marker_attributes}/{locale}",
            marker_attributes = id.marker_attributes.as_str(),
            locale = id.locale
        );
        self.0.insert(id.into_owned(), output);
        Ok(())
    }
}

impl TestingExporter {
    pub fn finish(self) -> BTreeMap<String, Vec<u8>> {
        self.0
            .into_tuple_vec()
            .into_iter()
            .map(|(id, buffer)| {
                let mut string = id.locale.to_string();
                if !id.marker_attributes.is_empty() {
                    string.push('/');
                    string.push_str(id.marker_attributes.as_str());
                }
                (string, buffer)
            })
            .collect()
    }
}
