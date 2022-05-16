// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_calendar::{Date, DateDuration};

fn date_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("date");
    let fxs = fixtures::get_dates_fixture().unwrap();

    // General overview is dealing in just ISO.
    group.bench_function("calendar/date/overview", |b| {
        use icu::calendar::{iso::IsoDay, iso::IsoMonth, iso::IsoYear};
        use std::convert::TryFrom;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion
                let iso_year = IsoYear(fx.year);
                let iso_month = IsoMonth::try_from(fx.month).unwrap();
                let iso_day = IsoDay::try_from(fx.day).unwrap();
                let mut iso_insantiated_date_iso =
                    Date::new_iso_date(iso_year, iso_month, iso_day).unwrap();
                let mut int_instantiated_date_iso =
                    Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();

                // Arithmetic. black_box used to avoid compiler optimization.
                iso_insantiated_date_iso.add(DateDuration::new(
                    black_box(1),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                ));
                int_instantiated_date_iso.add(DateDuration::new(
                    black_box(1),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                ));

                // Retrieving vals
                let _ = iso_insantiated_date_iso.year().number;
                let _ = iso_insantiated_date_iso.month().number;
                let _ = iso_insantiated_date_iso.day_of_month().0;
                let _ = int_instantiated_date_iso.year().number;
                let _ = int_instantiated_date_iso.month().number;
                let _ = int_instantiated_date_iso.day_of_month().0;
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/date/buddhist", |b| {
        use icu::calendar::buddhist::Buddhist;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion
                let mut instantiated_date_buddhist =
                    Date::new_buddhist_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_buddhist = Date::new_from_iso(date_iso, Buddhist);

                // Arithmetic. black_box used to avoid compiler optimization.
                instantiated_date_buddhist.add(DateDuration::new(
                    black_box(1),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                ));
                converted_date_buddhist.add(DateDuration::new(
                    black_box(1),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                ));

                // Retrieving vals
                let _ = instantiated_date_buddhist.year().number;
                let _ = instantiated_date_buddhist.month().number;
                let _ = instantiated_date_buddhist.day_of_month().0;
                let _ = converted_date_buddhist.year().number;
                let _ = converted_date_buddhist.month().number;
                let _ = converted_date_buddhist.day_of_month().0;

                // Conversion to ISO.
                let _ = instantiated_date_buddhist.to_iso();
                let _ = converted_date_buddhist.to_iso();
            }
        })
    });

    #[cfg(feature = "bench")]
    group.bench_function("calendar/date/julian", |b| {
        use icu::calendar::julian::Julian;

        b.iter(|| {
            for fx in &fxs.0 {
                // Instantion
                let mut instantiated_date_julian =
                    Date::new_julian_date(fx.year, fx.month, fx.day).unwrap();

                // Conversion from ISO
                let date_iso = Date::new_iso_date_from_integers(fx.year, fx.month, fx.day).unwrap();
                let mut converted_date_julian = Date::new_from_iso(date_iso, Julian);

                // Arithmetic. black_box used to avoid compiler optimization.
                instantiated_date_julian.add(DateDuration::new(
                    black_box(1),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                ));
                converted_date_julian.add(DateDuration::new(
                    black_box(1),
                    black_box(2),
                    black_box(3),
                    black_box(4),
                ));

                // Retrieving vals
                let _ = instantiated_date_julian.year().number;
                let _ = instantiated_date_julian.month().number;
                let _ = instantiated_date_julian.day_of_month().0;
                let _ = converted_date_julian.year().number;
                let _ = converted_date_julian.month().number;
                let _ = converted_date_julian.day_of_month().0;

                // Conversion to ISO.
                let _ = instantiated_date_julian.to_iso();
                let _ = converted_date_julian.to_iso();
            }
        })
    });

    // TODO: Run-through example for all calendar types.
    // TODO: Same style done for DateTime.

    group.finish();
}

criterion_group!(benches, date_benches);
criterion_main!(benches);
