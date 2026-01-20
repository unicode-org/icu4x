// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_locale::LocaleFallbacker;
use icu_locale_core::{langid, LanguageIdentifier};
use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::export::BlobExporter;
use icu_provider_blob::BlobDataProvider;
use std::collections::BTreeSet;

icu_provider::data_marker!(MarkerV1, Empty);
icu_provider::data_marker!(MarkerV2, Empty);
icu_provider::data_marker!(MarkerV3, Empty);
icu_provider::data_marker!(MarkerV4, Empty);

#[derive(
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    databake::Bake,
    PartialEq,
    yoke::Yokeable,
    zerofrom::ZeroFrom,
)]
#[databake(path = crate)]
pub struct Empty;

icu_provider::data_struct!(Empty);

struct Baked;

macro_rules! implement {
    ($marker:ident) => {
        impl DataProvider<$marker> for Baked {
            fn load(&self, _req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(Empty),
                })
            }
        }
        impl IterableDataProvider<$marker> for Baked {
            fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
                const LOCALES: &[LanguageIdentifier] = &[
                    langid!("af"),
                    langid!("am"),
                    langid!("ar"),
                    langid!("as"),
                    langid!("ast"),
                    langid!("az"),
                    langid!("be"),
                    langid!("bg"),
                    langid!("blo"),
                    langid!("bn"),
                    langid!("br"),
                    langid!("brx"),
                    langid!("bs-Cyrl"),
                    langid!("bs"),
                    langid!("ca"),
                    langid!("ceb"),
                    langid!("chr"),
                    langid!("cs"),
                    langid!("cv"),
                    langid!("cy"),
                    langid!("da"),
                    langid!("de"),
                    langid!("doi"),
                    langid!("dsb"),
                    langid!("el"),
                    langid!("en-001"),
                    langid!("en-AE"),
                    langid!("en-AU"),
                    langid!("en-BE"),
                    langid!("en-BW"),
                    langid!("en-BZ"),
                    langid!("en-CA"),
                    langid!("en-CH"),
                    langid!("en-HK"),
                    langid!("en-IN"),
                    langid!("en-MT"),
                    langid!("en-MV"),
                    langid!("en-PK"),
                    langid!("en-SE"),
                    langid!("en-SG"),
                    langid!("en-ZA"),
                    langid!("en-ZW"),
                    langid!("en"),
                    langid!("eo"),
                    langid!("es-BO"),
                    langid!("es-CL"),
                    langid!("es-CO"),
                    langid!("es-GT"),
                    langid!("es-HN"),
                    langid!("es-MX"),
                    langid!("es-PA"),
                    langid!("es-PE"),
                    langid!("es-PR"),
                    langid!("es-US"),
                    langid!("es"),
                    langid!("et"),
                    langid!("eu"),
                    langid!("fa"),
                    langid!("ff-Adlm"),
                    langid!("fi"),
                    langid!("fil"),
                    langid!("fo"),
                    langid!("fr-BE"),
                    langid!("fr-CA"),
                    langid!("fr-CH"),
                    langid!("fr"),
                    langid!("fy"),
                    langid!("ga"),
                    langid!("gd"),
                    langid!("gl"),
                    langid!("gu"),
                    langid!("ha"),
                    langid!("he"),
                    langid!("hi-Latn"),
                    langid!("hi"),
                    langid!("hr-BA"),
                    langid!("hr"),
                    langid!("hsb"),
                    langid!("hu"),
                    langid!("hy"),
                    langid!("ia"),
                    langid!("id"),
                    langid!("ig"),
                    langid!("is"),
                    langid!("it-CH"),
                    langid!("it"),
                    langid!("ja"),
                    langid!("jv"),
                    langid!("ka"),
                    langid!("kea"),
                    langid!("kgp"),
                    langid!("kk"),
                    langid!("km"),
                    langid!("kn"),
                    langid!("ko"),
                    langid!("kok"),
                    langid!("ks-Deva"),
                    langid!("ks"),
                    langid!("ku"),
                    langid!("kxv"),
                    langid!("ky"),
                    langid!("lb"),
                    langid!("lo"),
                    langid!("lt"),
                    langid!("lv"),
                    langid!("mai"),
                    langid!("mi"),
                    langid!("mk"),
                    langid!("ml"),
                    langid!("mn"),
                    langid!("mni"),
                    langid!("mr"),
                    langid!("ms-ID"),
                    langid!("ms"),
                    langid!("mt"),
                    langid!("my"),
                    langid!("ne"),
                    langid!("nl-BE"),
                    langid!("nl"),
                    langid!("no"),
                    langid!("or"),
                    langid!("pa"),
                    langid!("pcm"),
                    langid!("pl"),
                    langid!("ps"),
                    langid!("pt-PT"),
                    langid!("pt"),
                    langid!("qu"),
                    langid!("rm"),
                    langid!("ro"),
                    langid!("ru"),
                    langid!("sa"),
                    langid!("sah"),
                    langid!("sat"),
                    langid!("sc"),
                    langid!("sd-Deva"),
                    langid!("sk"),
                    langid!("sl"),
                    langid!("so"),
                    langid!("sq"),
                    langid!("sr-Latn"),
                    langid!("sr"),
                    langid!("su"),
                    langid!("sv"),
                    langid!("sw"),
                    langid!("syr"),
                    langid!("ta"),
                    langid!("te"),
                    langid!("tg"),
                    langid!("th"),
                    langid!("ti"),
                    langid!("tk"),
                    langid!("to"),
                    langid!("tr"),
                    langid!("tt"),
                    langid!("ug"),
                    langid!("uk"),
                    langid!("und"),
                    langid!("ur"),
                    langid!("uz-Cyrl"),
                    langid!("uz"),
                    langid!("vec"),
                    langid!("vi"),
                    langid!("wo"),
                    langid!("xh"),
                    langid!("xnr"),
                    langid!("yo"),
                    langid!("yrl"),
                    langid!("yue-Hans"),
                    langid!("yue"),
                    langid!("zh-Hant"),
                    langid!("zh-HK"),
                    langid!("zh-MO"),
                    langid!("zh-SG"),
                    langid!("zh"),
                    langid!("zu"),
                ];
                const ATTRS: &[&DataMarkerAttributes] = &[
                    DataMarkerAttributes::from_str_or_panic("a1"),
                    DataMarkerAttributes::from_str_or_panic("a1e"),
                    DataMarkerAttributes::from_str_or_panic("de"),
                    DataMarkerAttributes::from_str_or_panic("gy"),
                    DataMarkerAttributes::from_str_or_panic("gym0"),
                    DataMarkerAttributes::from_str_or_panic("gym0d"),
                    DataMarkerAttributes::from_str_or_panic("gym0de"),
                    DataMarkerAttributes::from_str_or_panic("m0d"),
                    DataMarkerAttributes::from_str_or_panic("m0de"),
                    DataMarkerAttributes::from_str_or_panic("y0w"),
                    DataMarkerAttributes::from_str_or_panic("ym0"),
                    DataMarkerAttributes::from_str_or_panic("ym0d"),
                    DataMarkerAttributes::from_str_or_panic("ym0de"),
                ];
                Ok(LOCALES
                    .iter()
                    .flat_map(|l| {
                        ATTRS.iter().map(|a| {
                            DataIdentifierCow::from_borrowed_and_owned(a, l.clone().into())
                        })
                    })
                    .collect())
            }
        }
    };
}

implement!(MarkerV1);
implement!(MarkerV2);
implement!(MarkerV3);
implement!(MarkerV4);

fn put_payloads<M: DataMarker>(exporter: &mut BlobExporter)
where
    Baked: IterableDataProvider<M>,
    ExportMarker: UpcastDataPayload<M>,
{
    for id in &IterableDataProvider::<M>::iter_ids(&Baked).unwrap() {
        let req = DataRequest {
            id: id.as_borrowed(),
            ..Default::default()
        };
        let res = DataProvider::<M>::load(&Baked, req).unwrap();
        exporter
            .put_payload(
                M::INFO,
                id.as_borrowed(),
                &ExportMarker::upcast(res.payload),
            )
            .unwrap();
    }
    exporter.flush(M::INFO, Default::default()).unwrap();
}

fn make_blob_v3() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let mut exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
    put_payloads::<MarkerV1>(&mut exporter);
    put_payloads::<MarkerV2>(&mut exporter);
    put_payloads::<MarkerV3>(&mut exporter);
    put_payloads::<MarkerV4>(&mut exporter);
    exporter.close().unwrap();
    drop(exporter);
    assert_eq!(blob.len(), 32974);
    assert!(blob.len() > 100);
    blob
}

fn auxkey_bench(c: &mut Criterion) {
    let blob_v3 = make_blob_v3();
    auxkey_bench_for_version(c, &blob_v3, "v3");
}

fn auxkey_bench_for_version(c: &mut Criterion, blob: &[u8], version_id: &str) {
    println!("benching {version_id}");
    c.bench_function(&format!("provider/auxkey/construct/{version_id}"), |b| {
        b.iter(|| BlobDataProvider::try_new_from_blob(black_box(blob).into()).unwrap());
    });

    let provider = LocaleFallbackProvider::new(
        BlobDataProvider::try_new_from_blob(black_box(blob).into()).unwrap(),
        LocaleFallbacker::new().static_to_owned(),
    );

    for (locale_str, attr_str) in [("sr-Latn", "ym0d"), ("sr-ME", "ym0d")] {
        let locale = locale_str.parse::<DataLocale>().unwrap();
        let marker_attributes = DataMarkerAttributes::from_str_or_panic(attr_str);
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                marker_attributes,
                &locale,
            ),
            metadata: Default::default(),
        };

        c.bench_function(
            &format!("provider/auxkey/fallback/{attr_str}/{locale_str}/{version_id}"),
            |b| {
                b.iter(|| {
                    provider
                        .load_data(black_box(MarkerV1::INFO), black_box(req))
                        .unwrap()
                });
            },
        );
    }
}

criterion_group!(benches, auxkey_bench,);
criterion_main!(benches);
