// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use crate::fixtures::structs::DateFixture;
use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use icu_calendar::{AsCalendar, Calendar, Date, DateDuration};

fn bench_dates<A: AsCalendar>(dates: Vec<&mut Date<A>>) {
    for date in dates {
        // black_box used to avoid compiler optimization.
        // Arithmetic
        date.add(DateDuration::new(
            black_box(1),
            black_box(2),
            black_box(3),
            black_box(4),
        ));

        // Retrieving vals
        let _ = black_box(date.year().number);
        let _ = black_box(date.month().number);
        let _ = black_box(date.day_of_month().0);

        // Conversion to ISO.
        let _ = black_box(date.to_iso());
    }
}

fn bench_calendar<C: Clone + Calendar>(
    group: &mut BenchmarkGroup<WallTime>,
    name: &str,
    fxs: &DateFixture,
    calendar: C,
    calendar_date_init: impl Fn(i32, u8, u8) -> Date<C>,
) {
    group.bench_function(name, |b| {
        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion from int
                let mut instantiated_date_calendar = calendar_date_init(fx.year, fx.month, fx.day);

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_calendar = Date::new_from_iso(date_iso, calendar.clone());

                bench_dates(vec![
                    &mut instantiated_date_calendar,
                    &mut converted_date_calendar,
                ]);
            }
        })
    });
}

fn date_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("date");
    let fxs = fixtures::get_dates_fixture().unwrap();

    group.bench_function("calendar/overview", |b| {
        // General overview is dealing in just ISO. Abstracted away from `bench_calendar` due
        // to lack of conversion case and use of iso types.
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

                bench_dates(vec![
                    &mut iso_insantiated_date_iso,
                    &mut int_instantiated_date_iso,
                ]);
            }
        })
    });

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/buddhist",
        &fxs,
        icu::calendar::buddhist::Buddhist,
        |y, m, d| Date::new_buddhist_date(y, m, d).unwrap(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/coptic",
        &fxs,
        icu::calendar::coptic::Coptic,
        |y, m, d| Date::new_coptic_date(y, m, d).unwrap(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/ethiopic",
        &fxs,
        icu::calendar::ethiopic::Ethiopic,
        |y, m, d| Date::new_ethiopic_date(y, m, d).unwrap(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/indian",
        &fxs,
        icu::calendar::indian::Indian,
        |y, m, d| Date::new_indian_date(y, m, d).unwrap(),
    );

    #[cfg(feature = "bench")]
    bench_calendar(
        &mut group,
        "calendar/julian",
        &fxs,
        icu::calendar::julian::Julian,
        |y, m, d| Date::new_julian_date(y, m, d).unwrap(),
    );

    #[cfg(feature = "bench")]
    group.bench_function("calendar/gregorian", |b| {
        // Abstracted away from `bench_calendar` due to use of iso types.
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

                bench_dates(vec![
                    &mut iso_insantiated_date_gregorian,
                    &mut converted_date_gregorian,
                ]);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, date_benches);
criterion_main!(benches);
