// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use std::collections::BTreeMap;

use elsa::sync::FrozenMap;
use icu_datagen::prelude::*;
use icu_datagen::DeduplicationStrategy;
use icu_datagen::FallbackOptions;
use icu_locid_transform::provider::*;
use icu_provider::datagen::ExportMarker;
use icu_provider::datagen::*;
use icu_provider::hello_world::*;
use icu_provider::make_exportable_provider;
use icu_provider::prelude::*;
use postcard::ser_flavors::{AllocVec, Flavor};
use writeable::Writeable;

/*

struct Baked;

#[allow(unused_imports)]
mod baked_data {
    include!("data/baked/mod.rs");
}

const _: () = {
    pub mod icu {
        pub use icu_datetime as datetime;
    }
    baked_data::make_provider!(Baked);
    baked_data::impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    baked_data::impliterable_datetime_patterns_gregory_skeleton_v1!(Baked);
};

#[derive(Default)]
struct TestingExporter(FrozenMap<DataLocale, String>);

impl DataExporter for &mut TestingExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
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
        println!("Putting: {key}/{locale}");
        self.0.insert(
            locale.clone(),
            postcard::from_bytes::<HelloWorldV1>(&output)
                .unwrap()
                .message
                .to_string(),
        );
        Ok(())
    }
}

*/
