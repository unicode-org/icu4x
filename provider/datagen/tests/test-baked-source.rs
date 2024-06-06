// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

#[path = "testutil.rs"]
mod testutil;

use icu_datagen::prelude::*;
use icu_datetime::provider::neo::GregorianDateNeoSkeletonPatternsV1Marker;
use icu_provider::make_exportable_provider;

struct Baked;

const _: () = {
    #[allow(unused_imports)]
    #[path = "data/baked/mod.rs"]
    mod baked_data;
    mod icu {
        pub use icu_datetime as datetime;
    }
    baked_data::make_provider!(Baked);
    baked_data::impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    baked_data::impliterable_datetime_patterns_gregory_skeleton_v1!(Baked);
};

make_exportable_provider!(Baked, [GregorianDateNeoSkeletonPatternsV1Marker,]);

#[test]
fn test() {
    let mut exporter = testutil::TestingExporter::default();
    DatagenDriver::new()
        .with_markers([GregorianDateNeoSkeletonPatternsV1Marker::INFO])
        .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
        .export(&Baked, &mut exporter)
        .unwrap();
    let results = exporter.finish();

    // The test has mostly already passed by getting here successfully.
    // Check only some basic properties of the exported data.
    assert!(!results.is_empty());
    assert!(results.len() > 10);
    assert!(results.len() < 100);
    assert!(results.values().next().unwrap().len() > 10);
}
