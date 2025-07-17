// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_datetime::{fieldsets, DateTimeFormatter, FixedCalendarDateTimeFormatter};

use icu_calendar::Gregorian;
use icu_locale_core::{locale, Locale};
use icu_time::zone::{IanaParser, ZoneNameTimestamp};
use icu_time::{DateTime, TimeZoneInfo, ZonedDateTime};
use writeable::Writeable;

fn datetime_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");

    let mut bench_neoneo_datetime_with_fixture = |name, file, has_zones| {
        let fx = serde_json::from_str::<fixtures::Fixture>(file).unwrap();

        let datetimes = fx
            .values
            .iter()
            .map(|s| {
                if has_zones {
                    ZonedDateTime::try_lenient_from_str(s, Gregorian, IanaParser::new()).expect(s)
                } else {
                    let DateTime { date, time } = DateTime::try_from_str(s, Gregorian).unwrap();
                    ZonedDateTime {
                        date,
                        time,
                        // zone is unused but we need to make the types match
                        zone: TimeZoneInfo::unknown()
                            .with_zone_name_timestamp(ZoneNameTimestamp::far_in_future()),
                    }
                }
            })
            .collect::<Vec<_>>();

        let setups = fx
            .setups
            .iter()
            .map(|s| {
                (
                    s.locale.parse::<Locale>().unwrap(),
                    s.options.semantic.as_ref().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        let mut result = String::with_capacity(1000);

        group.bench_function(format!("semantic/{name}"), |b| {
            b.iter(|| {
                for &(ref locale, field_set_builder) in &setups {
                    let dtf = {
                        FixedCalendarDateTimeFormatter::<Gregorian, _>::try_new(
                            locale.into(),
                            field_set_builder.clone().build_composite().unwrap(),
                        )
                        .expect("Failed to create FixedCalendarDateTimeFormatter.")
                    };

                    for dt in &datetimes {
                        let fdt = dtf.format(dt);
                        fdt.write_to(&mut result).unwrap();
                        result.clear();
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
