// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_locale::LocaleFallbacker;
use icu_locale_core::{langid, LanguageIdentifier};
use icu_provider::datagen::*;
use icu_provider::dynutil::UpcastDataPayload;
use icu_provider::prelude::*;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_blob::export::BlobExporter;
use icu_provider_blob::BlobDataProvider;

#[icu_provider::data_struct(
    marker(MarkerA, "a@1"),
    marker(MarkerB, "b@1"),
    marker(MarkerC, "c@1"),
    marker(MarkerD, "d@1")
)]
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, databake::Bake, PartialEq)]
#[databake(path = crate)]
pub struct Empty;

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
            fn supported_requests(
                &self,
            ) -> Result<std::collections::HashSet<(DataLocale, DataMarkerAttributes)>, DataError>
            {
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
                const ATTRS: &[&str] = &[
                    "a1", "a1e", "de", "gy", "gym0", "gym0d", "gym0de", "m0d", "m0de", "y0w",
                    "ym0", "ym0d", "ym0de",
                ];
                Ok(LOCALES
                    .iter()
                    .flat_map(|l| {
                        ATTRS
                            .iter()
                            .map(|a| (DataLocale::from(l.clone()), a.parse().unwrap()))
                    })
                    .collect())
            }
        }
    };
}

implement!(MarkerA);
implement!(MarkerB);
implement!(MarkerC);
implement!(MarkerD);

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
    put_payloads::<MarkerA>(&mut exporter);
    put_payloads::<MarkerB>(&mut exporter);
    put_payloads::<MarkerC>(&mut exporter);
    put_payloads::<MarkerD>(&mut exporter);
    exporter.close().unwrap();
    drop(exporter);
    assert_eq!(blob.len(), 133578);
    assert!(blob.len() > 100);
    blob
}

fn make_blob_v2() -> Vec<u8> {
    let mut blob: Vec<u8> = Vec::new();
    let mut exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
    put_payloads::<MarkerA>(&mut exporter);
    put_payloads::<MarkerB>(&mut exporter);
    put_payloads::<MarkerC>(&mut exporter);
    put_payloads::<MarkerD>(&mut exporter);
    exporter.close().unwrap();
    drop(exporter);
    assert_eq!(blob.len(), 34306);
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
    println!("benching {version_id}");
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
            &format!("provider/auxkey/fallback/{attr_str}/{locale_str}/{version_id}"),
            |b| {
                b.iter(|| {
                    provider
                        .load_data(black_box(MarkerA::INFO), black_box(req))
                        .unwrap()
                });
            },
        );
    }
}

criterion_group!(benches, auxkey_bench,);
criterion_main!(benches);
