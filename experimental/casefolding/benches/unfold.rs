use criterion::{black_box, criterion_group, criterion_main, Criterion};

use icu_casefolding::{CaseMappingUnfoldData, test_data::UCASE_PROPS_UNFOLD, RawUnfold};

criterion_group!(benches, hit_bench, miss_bench, mix_bench);
criterion_main!(benches);

fn zeromap_bench(zeromap: &CaseMappingUnfoldData, keys: &[String]) -> u32 {
    let mut matches = 0;
    for key in keys {
	if zeromap.get(key.as_ref()).is_some() { matches += 1; }
    }
    matches

}

fn raw_bench(raw: &RawUnfold, keys: &[String]) -> u32 {
    let mut matches = 0;
    for key in keys {
	if raw.get(key.as_ref()).is_some() { matches += 1; }
    }
    matches
}

fn hit_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("unfold_hit");

    let zeromap_unfold = CaseMappingUnfoldData::try_from_raw(&UCASE_PROPS_UNFOLD).unwrap();
    let raw_unfold = RawUnfold::new(&UCASE_PROPS_UNFOLD);

    let good_keys: Vec<String> = zeromap_unfold.map.iter_keys().map(|s| String::from(s)).collect();

    group.bench_function("raw_hit", |b| b.iter(|| raw_bench(&raw_unfold, black_box(&good_keys))));
    group.bench_function("zeromap_hit", |b| b.iter(|| zeromap_bench(&zeromap_unfold, black_box(&good_keys))));
}

fn miss_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("unfold_miss");

    let zeromap_unfold = CaseMappingUnfoldData::try_from_raw(&UCASE_PROPS_UNFOLD).unwrap();
    let raw_unfold = RawUnfold::new(&UCASE_PROPS_UNFOLD);

    let bad_keys: Vec<String> = vec![String::from("foo"),String::from("bar"),String::from("baz")];

    group.bench_function("raw_miss", |b| b.iter(|| raw_bench(&raw_unfold, black_box(&bad_keys))));
    group.bench_function("zeromap_miss", |b| b.iter(|| zeromap_bench(&zeromap_unfold, black_box(&bad_keys))));
}

fn mix_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("unfold_mix");

    let zeromap_unfold = CaseMappingUnfoldData::try_from_raw(&UCASE_PROPS_UNFOLD).unwrap();
    let raw_unfold = RawUnfold::new(&UCASE_PROPS_UNFOLD);

    let mix_keys: Vec<String> = vec![String::from("foo"),String::from("ff")];

    group.bench_function("raw_mix", |b| b.iter(|| raw_bench(&raw_unfold, black_box(&mix_keys))));
    group.bench_function("zeromap_mix", |b| b.iter(|| zeromap_bench(&zeromap_unfold, black_box(&mix_keys))));
}
