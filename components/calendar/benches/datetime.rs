// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_calendar::{types::Time, AsCalendar, DateDuration, DateTime};

fn run_calendar_benches<A: AsCalendar>(datetimes: Vec<&mut DateTime<A>>) {
    for datetime in datetimes {
        // Arithmetic. black_box used to avoid compiler optimization.
        datetime.date.add(DateDuration::new(
            black_box(1),
            black_box(2),
            black_box(3),
            black_box(4),
        ));
        datetime.time = Time::try_new(black_box(14), black_box(30), black_box(0), black_box(0))
            .expect("Failed to initialize Time instance.");

        // Retrieving vals
        let _ = datetime.date.year().number;
        let _ = datetime.date.month().number;
        let _ = datetime.date.day_of_month().0;
        let _ = datetime.time.hour;
        let _ = datetime.time.minute;
        let _ = datetime.time.second;

        // Conversion to ISO.
        // let _ = datetime.to_iso();
    }
}

fn datetime_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");
    let fxs = fixtures::get_dates_fixture().unwrap();

    // General overview is dealing in just ISO.
    group.bench_function("calendar/overview", |b| {
        b.iter(|| {
            for fx in &fxs.0 {
                // Instantiation from int
                let mut int_instantiated_datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();

                run_calendar_benches(vec![&mut int_instantiated_datetime_iso]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/buddhist", |b| {
        use icu::calendar::buddhist::Buddhist;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_datetime_buddhist = DateTime::new_buddhist_datetime(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();

                // Conversion from ISO
                let datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();
                let mut converted_datetime_buddhist =
                    DateTime::new_from_iso(datetime_iso, Buddhist);

                run_calendar_benches(vec![
                    &mut instantiated_datetime_buddhist,
                    &mut converted_datetime_buddhist,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/coptic", |b| {
        use icu::calendar::coptic::Coptic;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_datetime_coptic = DateTime::new_coptic_datetime(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();

                // Conversion from ISO
                let datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();
                let mut converted_datetime_coptic = DateTime::new_from_iso(datetime_iso, Coptic);

                run_calendar_benches(vec![
                    &mut instantiated_datetime_coptic,
                    &mut converted_datetime_coptic,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/ethiopic", |b| {
        use icu::calendar::ethiopic::Ethiopic;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int. Version with nanoseconds, so supplying nanosecond
                let mut instantiated_datetime_ethiopic = DateTime::new_ethiopic_datetime(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second, 0,
                )
                .unwrap();

                // Conversion from ISO
                let datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();
                let mut converted_datetime_ethiopic =
                    DateTime::new_from_iso(datetime_iso, Ethiopic);

                run_calendar_benches(vec![
                    &mut instantiated_datetime_ethiopic,
                    &mut converted_datetime_ethiopic,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/gregorian", |b| {
        use icu::calendar::gregorian::Gregorian;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int. Version with nanoseconds, so supplying nanosecond
                let mut instantiated_datetime_gregorian =
                    DateTime::new_gregorian_datetime_from_integers(
                        fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second, 0
                    )
                    .unwrap();

                // Conversion from ISO
                let datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second
                )
                .unwrap();
                let mut converted_datetime_gregorian =
                    DateTime::new_from_iso(datetime_iso, Gregorian);

                run_calendar_benches(vec![
                    &mut instantiated_datetime_gregorian,
                    &mut converted_datetime_gregorian,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/indian", |b| {
        use icu::calendar::indian::Indian;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_datetime_indian = DateTime::new_indian_datetime(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();

                // Conversion from ISO
                let datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();
                let mut converted_datetime_indian = DateTime::new_from_iso(datetime_iso, Indian);

                run_calendar_benches(vec![
                    &mut instantiated_datetime_indian,
                    &mut converted_datetime_indian,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/julian", |b| {
        use icu::calendar::julian::Julian;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_datetime_julian = DateTime::new_julian_datetime(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();

                // Conversion from ISO
                let datetime_iso = DateTime::new_iso_datetime_from_integers(
                    fx.year, fx.month, fx.day, fx.hour, fx.minute, fx.second,
                )
                .unwrap();
                let mut converted_datetime_julian = DateTime::new_from_iso(datetime_iso, Julian);

                run_calendar_benches(vec![
                    &mut instantiated_datetime_julian,
                    &mut converted_datetime_julian,
                ]);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, datetime_benches);
criterion_main!(benches);
