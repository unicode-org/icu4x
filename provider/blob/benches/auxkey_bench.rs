// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use icu_locale::fallback::LocaleFallbacker;
use icu_locale_core::{DataLocale, data_locale};
use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::export::*;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::BlobDataProvider;
use icu_provider_blob::export::BlobExporter;
use std::collections::BTreeSet;

icu_provider::data_marker!(MarkerV1, Empty);
icu_provider::data_marker!(MarkerV2, Empty);
icu_provider::data_marker!(MarkerV3, Empty);
icu_provider::data_marker!(MarkerV4, Empty);

#[allow(clippy::exhaustive_structs)]
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
                const LOCALES: &[DataLocale] = &[
                    data_locale!("af"),
                    data_locale!("am"),
                    data_locale!("ar"),
                    data_locale!("as"),
                    data_locale!("ast"),
                    data_locale!("az"),
                    data_locale!("be"),
                    data_locale!("bg"),
                    data_locale!("blo"),
                    data_locale!("bn"),
                    data_locale!("br"),
                    data_locale!("brx"),
                    data_locale!("bs-Cyrl"),
                    data_locale!("bs"),
                    data_locale!("ca"),
                    data_locale!("ceb"),
                    data_locale!("chr"),
                    data_locale!("cs"),
                    data_locale!("cv"),
                    data_locale!("cy"),
                    data_locale!("da"),
                    data_locale!("de"),
                    data_locale!("doi"),
                    data_locale!("dsb"),
                    data_locale!("el"),
                    data_locale!("en-001"),
                    data_locale!("en-AE"),
                    data_locale!("en-AU"),
                    data_locale!("en-BE"),
                    data_locale!("en-BW"),
                    data_locale!("en-BZ"),
                    data_locale!("en-CA"),
                    data_locale!("en-CH"),
                    data_locale!("en-HK"),
                    data_locale!("en-IN"),
                    data_locale!("en-MT"),
                    data_locale!("en-MV"),
                    data_locale!("en-PK"),
                    data_locale!("en-SE"),
                    data_locale!("en-SG"),
                    data_locale!("en-ZA"),
                    data_locale!("en-ZW"),
                    data_locale!("en"),
                    data_locale!("eo"),
                    data_locale!("es-BO"),
                    data_locale!("es-CL"),
                    data_locale!("es-CO"),
                    data_locale!("es-GT"),
                    data_locale!("es-HN"),
                    data_locale!("es-MX"),
                    data_locale!("es-PA"),
                    data_locale!("es-PE"),
                    data_locale!("es-PR"),
                    data_locale!("es-US"),
                    data_locale!("es"),
                    data_locale!("et"),
                    data_locale!("eu"),
                    data_locale!("fa"),
                    data_locale!("ff-Adlm"),
                    data_locale!("fi"),
                    data_locale!("fil"),
                    data_locale!("fo"),
                    data_locale!("fr-BE"),
                    data_locale!("fr-CA"),
                    data_locale!("fr-CH"),
                    data_locale!("fr"),
                    data_locale!("fy"),
                    data_locale!("ga"),
                    data_locale!("gd"),
                    data_locale!("gl"),
                    data_locale!("gu"),
                    data_locale!("ha"),
                    data_locale!("he"),
                    data_locale!("hi-Latn"),
                    data_locale!("hi"),
                    data_locale!("hr-BA"),
                    data_locale!("hr"),
                    data_locale!("hsb"),
                    data_locale!("hu"),
                    data_locale!("hy"),
                    data_locale!("ia"),
                    data_locale!("id"),
                    data_locale!("ig"),
                    data_locale!("is"),
                    data_locale!("it-CH"),
                    data_locale!("it"),
                    data_locale!("ja"),
                    data_locale!("jv"),
                    data_locale!("ka"),
                    data_locale!("kea"),
                    data_locale!("kgp"),
                    data_locale!("kk"),
                    data_locale!("km"),
                    data_locale!("kn"),
                    data_locale!("ko"),
                    data_locale!("kok"),
                    data_locale!("ks-Deva"),
                    data_locale!("ks"),
                    data_locale!("ku"),
                    data_locale!("kxv"),
                    data_locale!("ky"),
                    data_locale!("lb"),
                    data_locale!("lo"),
                    data_locale!("lt"),
                    data_locale!("lv"),
                    data_locale!("mai"),
                    data_locale!("mi"),
                    data_locale!("mk"),
                    data_locale!("ml"),
                    data_locale!("mn"),
                    data_locale!("mni"),
                    data_locale!("mr"),
                    data_locale!("ms-ID"),
                    data_locale!("ms"),
                    data_locale!("mt"),
                    data_locale!("my"),
                    data_locale!("ne"),
                    data_locale!("nl-BE"),
                    data_locale!("nl"),
                    data_locale!("no"),
                    data_locale!("or"),
                    data_locale!("pa"),
                    data_locale!("pcm"),
                    data_locale!("pl"),
                    data_locale!("ps"),
                    data_locale!("pt-PT"),
                    data_locale!("pt"),
                    data_locale!("qu"),
                    data_locale!("rm"),
                    data_locale!("ro"),
                    data_locale!("ru"),
                    data_locale!("sa"),
                    data_locale!("sah"),
                    data_locale!("sat"),
                    data_locale!("sc"),
                    data_locale!("sd-Deva"),
                    data_locale!("sk"),
                    data_locale!("sl"),
                    data_locale!("so"),
                    data_locale!("sq"),
                    data_locale!("sr-Latn"),
                    data_locale!("sr"),
                    data_locale!("su"),
                    data_locale!("sv"),
                    data_locale!("sw"),
                    data_locale!("syr"),
                    data_locale!("ta"),
                    data_locale!("te"),
                    data_locale!("tg"),
                    data_locale!("th"),
                    data_locale!("ti"),
                    data_locale!("tk"),
                    data_locale!("to"),
                    data_locale!("tr"),
                    data_locale!("tt"),
                    data_locale!("ug"),
                    data_locale!("uk"),
                    data_locale!("und"),
                    data_locale!("ur"),
                    data_locale!("uz-Cyrl"),
                    data_locale!("uz"),
                    data_locale!("vec"),
                    data_locale!("vi"),
                    data_locale!("wo"),
                    data_locale!("xh"),
                    data_locale!("xnr"),
                    data_locale!("yo"),
                    data_locale!("yrl"),
                    data_locale!("yue-Hans"),
                    data_locale!("yue"),
                    data_locale!("zh-Hant"),
                    data_locale!("zh-HK"),
                    data_locale!("zh-MO"),
                    data_locale!("zh-SG"),
                    data_locale!("zh"),
                    data_locale!("zu"),
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
                            DataIdentifierBorrowed::for_marker_attributes_and_locale(a, l).as_cow()
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
