// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_datetime::provider::neo::*;
use icu_locale::LocaleFallbacker;
use icu_provider::datagen::*;
use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::export::BlobExporter;
use icu_provider_blob::BlobDataProvider;

struct Baked;

#[allow(unused_imports)]
const _: () = {
    use icu_datetime_data::*;
    pub mod icu {
        pub use icu_datetime as datetime;
        pub use icu_datetime_data::icu_locale as locale;
    }
    make_provider!(Baked);

    impl_datetime_patterns_buddhist_skeleton_v1!(Baked);
    impl_datetime_patterns_chinese_skeleton_v1!(Baked);
    impl_datetime_patterns_coptic_skeleton_v1!(Baked);
    impl_datetime_patterns_dangi_skeleton_v1!(Baked);
    impl_datetime_patterns_ethiopic_skeleton_v1!(Baked);
    impl_datetime_patterns_gregory_skeleton_v1!(Baked);
    impl_datetime_patterns_hebrew_skeleton_v1!(Baked);
    impl_datetime_patterns_indian_skeleton_v1!(Baked);
    impl_datetime_patterns_islamic_skeleton_v1!(Baked);
    impl_datetime_patterns_japanese_skeleton_v1!(Baked);
    impl_datetime_patterns_japanext_skeleton_v1!(Baked);
    impl_datetime_patterns_persian_skeleton_v1!(Baked);
    impl_datetime_patterns_roc_skeleton_v1!(Baked);

    impliterable_datetime_patterns_buddhist_skeleton_v1!(Baked);
    impliterable_datetime_patterns_chinese_skeleton_v1!(Baked);
    impliterable_datetime_patterns_coptic_skeleton_v1!(Baked);
    impliterable_datetime_patterns_dangi_skeleton_v1!(Baked);
    impliterable_datetime_patterns_ethiopic_skeleton_v1!(Baked);
    impliterable_datetime_patterns_gregory_skeleton_v1!(Baked);
    impliterable_datetime_patterns_hebrew_skeleton_v1!(Baked);
    impliterable_datetime_patterns_indian_skeleton_v1!(Baked);
    impliterable_datetime_patterns_islamic_skeleton_v1!(Baked);
    impliterable_datetime_patterns_japanese_skeleton_v1!(Baked);
    impliterable_datetime_patterns_japanext_skeleton_v1!(Baked);
    impliterable_datetime_patterns_persian_skeleton_v1!(Baked);
    impliterable_datetime_patterns_roc_skeleton_v1!(Baked);
};

fn put_payloads<M: DataMarker>(exporter: &mut BlobExporter)
where
    Baked: IterableDataProvider<M>,
    ExportMarker: UpcastDataPayload<M>,
{
    for (locale, marker_attributes) in
        &IterableDataProvider::<M>::supported_requests(&Baked).unwrap()
    {
        let req = DataRequest {
            locale,
            marker_attributes,
            ..Default::default()
        };
        let res = DataProvider::<M>::load(&Baked, req).unwrap();
        exporter
            .put_payload(
                M::INFO,
                locale,
                marker_attributes,
                &ExportMarker::upcast(res.payload),
            )
            .unwrap();
    }
    exporter.flush(M::INFO).unwrap();
}

fn make_blob_v1() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let mut exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    put_payloads::<GregorianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<BuddhistDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<ChineseDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<CopticDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<DangiDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<EthiopianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<GregorianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<HebrewDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<IndianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<IslamicDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<JapaneseDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<JapaneseExtendedDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<PersianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<RocDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    exporter.close().unwrap();
    drop(exporter);
    assert_eq!(blob.len(), 450725);
    assert!(blob.len() > 100);
    blob
}

fn make_blob_v2() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let mut exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    put_payloads::<GregorianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<BuddhistDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<ChineseDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<CopticDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<DangiDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<EthiopianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<GregorianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<HebrewDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<IndianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<IslamicDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<JapaneseDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<JapaneseExtendedDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<PersianDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    put_payloads::<RocDateNeoSkeletonPatternsV1Marker>(&mut exporter);
    exporter.close().unwrap();
    drop(exporter);
    assert_eq!(blob.len(), 241278);
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

    for (locale_str, attr_str) in [("sr-Latn", "ym0d"), ("sr-ME", "ym0d")] {
        let locale = locale_str.parse::<DataLocale>().unwrap();
        let attrs = attr_str.parse::<DataMarkerAttributes>().unwrap();
        let req = DataRequest {
            locale: &locale,
            marker_attributes: &attrs,
            metadata: Default::default(),
        };

        c.bench_function(
            &format!("provider/auxkey/fallback/{locale_str}/{version_id}"),
            |b| {
                b.iter(|| {
                    provider
                        .load_data(GregorianDateNeoSkeletonPatternsV1Marker::INFO, req)
                        .unwrap()
                });
            },
        );
    }
}

criterion_group!(benches, auxkey_bench,);
criterion_main!(benches);
