// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_datetime::{fieldsets, DateTimeFormatter, FixedCalendarDateTimeFormatter};

use icu_calendar::{Date, Gregorian};
use icu_locale_core::{locale, Locale};
use icu_time::{zone::TimeZoneVariant, DateTime, Time, TimeZoneInfo, ZonedDateTime};
use writeable::Writeable;

#[path = "../tests/mock.rs"]
mod mock;

fn datetime_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");

    let mut bench_neoneo_datetime_with_fixture = |name, file, has_zones| {
        let fxs = serde_json::from_str::<fixtures::Fixture>(file).unwrap();
        group.bench_function(format!("semantic/{name}"), |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<ZonedDateTime<Gregorian, _>> = fx
                        .values
                        .iter()
                        .map(move |value| {
                            if has_zones {
                                mock::parse_zoned_gregorian_from_str(value)
                            } else {
                                let DateTime { date, time } =
                                    DateTime::try_from_str(value, Gregorian).unwrap();
                                ZonedDateTime {
                                    date,
                                    time,
                                    // zone is unused but we need to make the types match
                                    zone: TimeZoneInfo::utc()
                                        .at_time((
                                            Date::try_new_iso(2024, 1, 1).unwrap(),
                                            Time::start_of_day(),
                                        ))
                                        .with_zone_variant(TimeZoneVariant::Standard),
                                }
                            }
                        })
                        .collect();
                    for setup in &fx.setups {
                        let locale: Locale = setup.locale.parse().expect("Failed to parse locale.");
                        let fset = setup
                            .options
                            .semantic
                            .clone()
                            .unwrap()
                            .build_composite()
                            .unwrap();

                        let dtf = {
                            FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
                                locale.into(),
                                fset,
                            )
                            .expect("Failed to create FixedCalendarDateTimeFormatter.")
                        };

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            fdt.write_to(&mut result).unwrap();
                            result.clear();
                        }
                    }
                }
            })
        });
    };

    bench_neoneo_datetime_with_fixture(
        "lengths",
        include_str!("fixtures/tests/lengths.json"),
        false,
    );

    bench_neoneo_datetime_with_fixture(
        "components",
        include_str!("fixtures/tests/components.json"),
        false,
    );

    bench_neoneo_datetime_with_fixture(
        "lengths_with_zones",
        include_str!("fixtures/tests/lengths_with_zones.json"),
        true,
    );

    let ten_cases = [
        "2001-09-08T18:46:40.000[u-ca=gregory]",
        "2017-07-13T19:40:00.000[u-ca=gregory]",
        "2020-09-13T05:26:40.000[u-ca=gregory]",
        "2021-01-06T22:13:20.000[u-ca=gregory]",
        "2021-05-02T17:00:00.000[u-ca=gregory]",
        "2021-08-26T10:46:40.000[u-ca=gregory]",
        "2021-11-20T03:33:20.000[u-ca=gregory]",
        "2022-04-14T22:20:00.000[u-ca=gregory]",
        "2022-08-08T16:06:40.000[u-ca=gregory]",
        "2033-05-17T20:33:20.000[u-ca=gregory]",
    ]
    .map(|s| DateTime::try_from_str(s, Gregorian).unwrap());

    #[inline]
    fn construct_any_ymd_short() -> DateTimeFormatter<fieldsets::YMD> {
        DateTimeFormatter::try_new(locale!("fr").into(), fieldsets::YMD::short()).unwrap()
    }

    #[inline]
    fn construct_fixed_ymd_short() -> FixedCalendarDateTimeFormatter<Gregorian, fieldsets::YMD> {
        FixedCalendarDateTimeFormatter::try_new(locale!("fr").into(), fieldsets::YMD::short())
            .unwrap()
    }

    #[inline]
    fn construct_fixed_ymd_long() -> FixedCalendarDateTimeFormatter<Gregorian, fieldsets::YMD> {
        FixedCalendarDateTimeFormatter::try_new(locale!("fr").into(), fieldsets::YMD::long())
            .unwrap()
    }

    group.bench_function("ymd_short/any/construct_and_format/10_cases", |b| {
        let mut buffer = String::with_capacity(1000);
        b.iter(|| {
            for datetime in black_box(&ten_cases).iter() {
                buffer.clear();
                let formatter = construct_any_ymd_short();
                formatter.format(datetime).write_to(&mut buffer).unwrap();
            }
        });
    });

    group.bench_function("ymd_short/any/format_only/10_cases", |b| {
        let formatter = construct_any_ymd_short();
        let mut buffer = String::with_capacity(1000);
        b.iter(|| {
            for datetime in black_box(&ten_cases).iter() {
                buffer.clear();
                black_box(&formatter)
                    .format(datetime)
                    .write_to(&mut buffer)
                    .unwrap();
            }
        });
    });

    group.bench_function("ymd_short/any/format_to_string/10_cases", |b| {
        let formatter = construct_any_ymd_short();
        b.iter(|| {
            let mut counter = 0usize; // make sure the loop is not DCE'd
            for datetime in black_box(&ten_cases).iter() {
                let n = black_box(&formatter)
                    .format(datetime)
                    .write_to_string()
                    .len();
                counter = counter.wrapping_add(n);
            }
            counter
        });
    });

    group.bench_function("ymd_short/fixed/construct_and_format/10_cases", |b| {
        let mut buffer = String::with_capacity(1000);
        b.iter(|| {
            for datetime in black_box(&ten_cases).iter() {
                buffer.clear();
                let formatter = construct_fixed_ymd_short();
                formatter.format(datetime).write_to(&mut buffer).unwrap();
            }
        });
    });

    group.bench_function("ymd_short/fixed/format_only/10_cases", |b| {
        let formatter = construct_fixed_ymd_short();
        let mut buffer = String::with_capacity(1000);
        b.iter(|| {
            for datetime in black_box(&ten_cases).iter() {
                buffer.clear();
                black_box(&formatter)
                    .format(datetime)
                    .write_to(&mut buffer)
                    .unwrap();
            }
        });
    });

    group.bench_function("ymd_short/fixed/format_to_string/10_cases", |b| {
        let formatter = construct_fixed_ymd_short();
        b.iter(|| {
            let mut counter = 0usize; // make sure the loop is not DCE'd
            for datetime in black_box(&ten_cases).iter() {
                let n = black_box(&formatter)
                    .format(datetime)
                    .write_to_string()
                    .len();
                counter = counter.wrapping_add(n);
            }
            counter
        });
    });

    group.bench_function("ymd_long/fixed/construct_and_format/10_cases", |b| {
        let mut buffer = String::with_capacity(1000);
        b.iter(|| {
            for datetime in black_box(&ten_cases).iter() {
                buffer.clear();
                let formatter = construct_fixed_ymd_long();
                formatter.format(datetime).write_to(&mut buffer).unwrap();
            }
        });
    });

    group.bench_function("ymd_long/fixed/format_only/10_cases", |b| {
        let formatter = construct_fixed_ymd_long();
        let mut buffer = String::with_capacity(1000);
        b.iter(|| {
            for datetime in black_box(&ten_cases).iter() {
                buffer.clear();
                black_box(&formatter)
                    .format(datetime)
                    .write_to(&mut buffer)
                    .unwrap();
            }
        });
    });

    group.bench_function("ymd_long/fixed/format_to_string/10_cases", |b| {
        let formatter = construct_fixed_ymd_long();
        b.iter(|| {
            let mut counter = 0usize; // make sure the loop is not DCE'd
            for datetime in black_box(&ten_cases).iter() {
                let n = black_box(&formatter)
                    .format(datetime)
                    .write_to_string()
                    .len();
                counter = counter.wrapping_add(n);
            }
            counter
        });
    });

    group.finish();
}

criterion_group!(benches, datetime_benches);
criterion_main!(benches);
