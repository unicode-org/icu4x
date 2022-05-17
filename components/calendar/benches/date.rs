// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_calendar::{AsCalendar, Date, DateDuration};

fn run_calendar_benches<A: AsCalendar>(dates: Vec<&mut Date<A>>) {
    for date in dates {
        // Arithmetic. black_box used to avoid compiler optimization.
        date.add(DateDuration::new(
            black_box(1),
            black_box(2),
            black_box(3),
            black_box(4),
        ));

        // Retrieving vals
        let _ = date.year().number;
        let _ = date.month().number;
        let _ = date.day_of_month().0;

        // Conversion to ISO.
        // let _ = date.to_iso();
    }
}

fn date_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("date");
    let fxs = fixtures::get_dates_fixture().unwrap();

    // General overview is dealing in just ISO.
    group.bench_function("calendar/overview", |b| {
        use icu::calendar::{iso::IsoDay, iso::IsoMonth, iso::IsoYear};
        use std::convert::TryFrom;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantiation from int
                let mut int_instantiated_date_iso =
                    Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();

                // Instantion from ISO
                let iso_year = IsoYear(fx.year);
                let iso_month = IsoMonth::try_from(fx.month).unwrap();
                let iso_day = IsoDay::try_from(fx.day).unwrap();
                let mut iso_insantiated_date_iso =
                    Date::new_iso_date(iso_year, iso_month, iso_day).unwrap();

                run_calendar_benches(vec![
                    &mut iso_insantiated_date_iso,
                    &mut int_instantiated_date_iso,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/buddhist", |b| {
        use icu::calendar::buddhist::Buddhist;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_date_buddhist =
                    Date::new_buddhist_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_buddhist = Date::new_from_iso(date_iso, Buddhist);

                run_calendar_benches(vec![
                    &mut instantiated_date_buddhist,
                    &mut converted_date_buddhist,
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
                let mut instantiated_date_coptic =
                    Date::new_coptic_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_coptic = Date::new_from_iso(date_iso, Coptic);

                run_calendar_benches(vec![
                    &mut instantiated_date_coptic,
                    &mut converted_date_coptic,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/ethiopic", |b| {
        use icu::calendar::ethiopic::Ethiopic;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_date_ethiopic =
                    Date::new_ethiopic_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_ethiopic = Date::new_from_iso(date_iso, Ethiopic);

                run_calendar_benches(vec![
                    &mut instantiated_date_ethiopic,
                    &mut converted_date_ethiopic,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/gregorian", |b| {
        use icu::calendar::gregorian::Gregorian;
        use icu::calendar::{iso::IsoDay, iso::IsoMonth, iso::IsoYear};
        use std::convert::TryFrom;

        b.iter(|| {
            for fx in &fxs.0 {
                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_gregorian = Date::new_from_iso(date_iso, Gregorian);

                // Instantion from ISO
                let iso_year = IsoYear(fx.year);
                let iso_month = IsoMonth::try_from(fx.month).unwrap();
                let iso_day = IsoDay::try_from(fx.day).unwrap();
                let mut iso_insantiated_date_gregorian =
                    Date::new_gregorian_date(iso_year, iso_month, iso_day).unwrap();

                run_calendar_benches(vec![
                    &mut iso_insantiated_date_gregorian,
                    &mut converted_date_gregorian,
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
                let mut instantiated_date_indian =
                    Date::new_indian_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_indian = Date::new_from_iso(date_iso, Indian);

                run_calendar_benches(vec![
                    &mut instantiated_date_indian,
                    &mut converted_date_indian,
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
                let mut instantiated_date_julian =
                    Date::new_julian_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_julian = Date::new_from_iso(date_iso, Julian);

                run_calendar_benches(vec![
                    &mut instantiated_date_julian,
                    &mut converted_date_julian,
                ]);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, date_benches);
criterion_main!(benches);
