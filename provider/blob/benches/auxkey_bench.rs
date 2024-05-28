// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_datagen::prelude::*;
use icu_datetime::provider::neo::*;
use icu_locid_transform::LocaleFallbacker;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::export::BlobExporter;
use icu_provider_blob::BlobDataProvider;

struct Baked;

const _: () = {
    pub mod icu {
        pub use icu_datetime as datetime;
        #[allow(unused_imports)] // baked data may or may not need this
        pub use icu_locid_transform as locid_transform;
    }
    icu_datetime_data::make_provider!(Baked);

    icu_datetime_data::impl_datetime_patterns_buddhist_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_chinese_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_coptic_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_dangi_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_ethiopic_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_hebrew_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_indian_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_islamic_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_japanese_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_japanext_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_persian_skeleton_v1!(Baked);
    icu_datetime_data::impl_datetime_patterns_roc_skeleton_v1!(Baked);

    icu_datetime_data::impliterable_datetime_patterns_buddhist_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_chinese_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_coptic_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_dangi_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_ethiopic_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_gregory_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_hebrew_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_indian_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_islamic_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_japanese_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_japanext_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_persian_skeleton_v1!(Baked);
    icu_datetime_data::impliterable_datetime_patterns_roc_skeleton_v1!(Baked);
};

macro_rules! skeleton_markers {
    ($cb:ident) => {
        $cb! {[
            BuddhistDateNeoSkeletonPatternsV1Marker,
            ChineseDateNeoSkeletonPatternsV1Marker,
            CopticDateNeoSkeletonPatternsV1Marker,
            DangiDateNeoSkeletonPatternsV1Marker,
            EthiopianDateNeoSkeletonPatternsV1Marker,
            GregorianDateNeoSkeletonPatternsV1Marker,
            HebrewDateNeoSkeletonPatternsV1Marker,
            IndianDateNeoSkeletonPatternsV1Marker,
            IslamicDateNeoSkeletonPatternsV1Marker,
            JapaneseDateNeoSkeletonPatternsV1Marker,
            JapaneseExtendedDateNeoSkeletonPatternsV1Marker,
            PersianDateNeoSkeletonPatternsV1Marker,
            RocDateNeoSkeletonPatternsV1Marker,
        ]}
    };
}

macro_rules! make_exportable_provider_cb {
    ([$($marker:path,)+]) => {
        icu_provider::make_exportable_provider!(Baked, [$($marker,)+]);
    };
}

macro_rules! key_array_cb {
    ([$($marker:path,)+]) => {
        [$(<$marker>::KEY,)+]
    };
}

skeleton_markers!(make_exportable_provider_cb);

fn make_blob_v1() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    DatagenDriver::new()
        .with_keys(skeleton_markers!(key_array_cb))
        .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
        .export(&Baked, exporter)
        .unwrap();
    assert_eq!(blob.len(), 684197);
    assert!(blob.len() > 100);
    blob
}

fn make_blob_v2() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    DatagenDriver::new()
        .with_keys(skeleton_markers!(key_array_cb))
        .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
        .export(&Baked, exporter)
        .unwrap();
    assert_eq!(blob.len(), 308744);
    assert!(blob.len() > 100);
    blob
}

fn auxkey_bench(c: &mut Criterion) {
    let blob_v1 = make_blob_v1();
    auxkey_bench_for_version(c, &blob_v1, "v1");
    let blob_v2 = make_blob_v2();
    auxkey_bench_for_version(c, &blob_v2, "v2");
}

fn auxkey_bench_for_version(c: &mut Criterion, blob: &[u8], version_id: &str) {
    c.bench_function(&format!("provider/auxkey/construct/{version_id}"), |b| {
        b.iter(|| BlobDataProvider::try_new_from_blob(black_box(blob).into()).unwrap());
    });

    let provider = LocaleFallbackProvider::new_with_fallbacker(
        BlobDataProvider::try_new_from_blob(black_box(blob).into()).unwrap(),
        LocaleFallbacker::new().static_to_owned(),
    );

    for locale_str in ["sr-Latn-x-ym0d", "sr-ME-x-ym0d"] {
        let locale = locale_str.parse::<DataLocale>().unwrap();

        c.bench_function(
            &format!("provider/auxkey/fallback/{locale_str}/{version_id}"),
            |b| {
                b.iter(|| {
                    assert!(
                        DataProvider::<GregorianDateNeoSkeletonPatternsV1Marker>::load(
                            &provider.as_deserializing(),
                            DataRequest {
                                locale: black_box(&locale),
                                metadata: Default::default()
                            }
                        )
                        .is_ok()
                    )
                });
            },
        );
    }
}

criterion_group!(benches, auxkey_bench,);
criterion_main!(benches);
