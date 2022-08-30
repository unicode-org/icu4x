// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::locale;
use icu_provider::{hello_world::*, prelude::*};

fn main() {
    let dp =
        icu_provider_blob::StaticDataProvider::try_new_from_static_blob(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/hello_world.postcard"
        )))
        .unwrap();

    let hello: DataPayload<HelloWorldV1Marker> = dp
        .load(DataRequest {
            locale: &locale!("zh").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();

    println!("{}", hello.get().message);
}
