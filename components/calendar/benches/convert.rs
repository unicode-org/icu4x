// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use icu_calendar::{Calendar, Date, Ref};

fn bench_calendar<C: Clone + Calendar>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    calendar: C,
) {
    let iso = Date::try_new_iso_date(2023, 8, 16).unwrap();
    group.bench_function(name, |b| {
        b.iter(|| {
            let converted = black_box(iso).to_calendar(Ref(&calendar));
            let year = black_box(converted.year().number);
            let month = black_box(converted.month().ordinal);
            let day = black_box(converted.day_of_month().0);
            black_box((converted, year, month, day))
        })
    });
}

fn convert_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("convert");

    bench_calendar(&mut group, "calendar/iso", icu::calendar::iso::Iso);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/buddhist",
        icu::calendar::buddhist::Buddhist,
    );

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/coptic", icu::calendar::coptic::Coptic);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/ethiopic",
        icu::calendar::ethiopian::Ethiopian::new(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/indian", icu::calendar::indian::Indian);

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/julian", icu::calendar::julian::Julian);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/chinese_calculating",
        icu::calendar::chinese::Chinese::new_always_calculating(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/chinese_cached",
        icu::calendar::chinese::Chinese::new(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/gregorian",
        icu::calendar::gregorian::Gregorian,
    );

    #[cfg(feature = "bench")]
    bench_calendar(&mut group, "calendar/hebrew", icu::calendar::hebrew::Hebrew);

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/islamic/observational",
        icu::calendar::islamic::IslamicObservational::new_always_calculating(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/islamic/civil",
        icu::calendar::islamic::IslamicCivil::new(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/islamic/ummalqura",
        icu::calendar::islamic::IslamicUmmAlQura::new_always_calculating(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/islamic/tabular",
        icu::calendar::islamic::IslamicTabular::new(),
    );

    group.finish();
}

criterion_group!(benches, convert_benches);
criterion_main!(benches);
