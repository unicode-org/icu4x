// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_locid_macros::langid;
use icu_plurals::provider::*;
use icu_provider::prelude::*;
#[cfg(feature = "bench")]
use icu_provider::serde::*;
use icu_provider_fs::FsDataProvider;

fn overview_bench(c: &mut Criterion) {
    // End-to-end JSON test
    c.bench_function("json/overview", |b| {
        b.iter(|| {
            let provider = FsDataProvider::try_new("./tests/testdata/json")
                .expect("Loading file from testdata directory");
            let _: DataPayload<PluralRuleStringsV1Marker> = black_box(&provider)
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key: key::CARDINAL_V1,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(langid!("ru")),
                        },
                    },
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });

    #[cfg(feature = "bench")]
    {
        json_bench(c);
        #[cfg(feature = "bincode")]
        {
            bincode_bench(c);
        }
    }
}

#[cfg(feature = "bench")]
fn json_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/testdata/json")
        .expect("Loading file from testdata directory");

    c.bench_function("json/generic", |b| {
        b.iter(|| {
            let _: DataPayload<PluralRuleStringsV1Marker> = black_box(&provider)
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key: key::CARDINAL_V1,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(langid!("ru")),
                        },
                    },
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });

    c.bench_function("json/erased_serde", |b| {
        b.iter(|| {
            let _: DataPayload<PluralRuleStringsV1Marker> =
                black_box(&provider as &dyn SerdeDeDataProvider)
                    .load_payload(&DataRequest {
                        resource_path: ResourcePath {
                            key: key::CARDINAL_V1,
                            options: ResourceOptions {
                                variant: None,
                                langid: Some(langid!("ru")),
                            },
                        },
                    })
                    .expect("The data should be valid")
                    .take_payload()
                    .expect("Loading was successful");
        });
    });
}

#[cfg(all(feature = "bench", feature = "bincode"))]
fn bincode_bench(c: &mut Criterion) {
    let provider = FsDataProvider::try_new("./tests/testdata/bincode")
        .expect("Loading file from testdata directory");

    c.bench_function("bincode/generic", |b| {
        b.iter(|| {
            let _: DataPayload<PluralRuleStringsV1Marker> = black_box(&provider)
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key: key::CARDINAL_V1,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(langid!("sr")),
                        },
                    },
                })
                .expect("The data should be valid")
                .take_payload()
                .expect("Loading was successful");
        });
    });

    c.bench_function("bincode/erased_serde", |b| {
        b.iter(|| {
            let _: DataPayload<PluralRuleStringsV1Marker> =
                black_box(&provider as &dyn SerdeDeDataProvider)
                    .load_payload(&DataRequest {
                        resource_path: ResourcePath {
                            key: key::CARDINAL_V1,
                            options: ResourceOptions {
                                variant: None,
                                langid: Some(langid!("sr")),
                            },
                        },
                    })
                    .expect("The data should be valid")
                    .take_payload()
                    .expect("Loading was successful");
        });
    });
}

criterion_group!(benches, overview_bench,);
criterion_main!(benches);
