use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;

use icu_datetime::date::DateTime;
use icu_datetime::options::{self, DateTimeFormatOptions};
use icu_datetime::DateTimeFormat;
use icu_fs_data_provider::FsDataProvider;

fn datetime_benches(c: &mut Criterion) {
    let datetimes = vec![
        DateTime::new(2001, 9, 8, 18, 46, 40, 0),
        DateTime::new(2017, 7, 13, 19, 40, 0, 0),
        DateTime::new(2020, 9, 13, 5, 26, 40, 0),
        DateTime::new(2021, 1, 6, 22, 13, 20, 0),
        DateTime::new(2021, 5, 2, 17, 0, 0, 0),
        DateTime::new(2021, 8, 26, 10, 46, 40, 0),
        DateTime::new(2021, 12, 20, 3, 33, 20, 0),
        DateTime::new(2022, 4, 14, 22, 20, 0, 0),
        DateTime::new(2022, 8, 8, 16, 6, 40, 0),
        DateTime::new(2033, 5, 17, 20, 33, 20, 0),
    ];
    let values = &[
        ("pl", options::style::Date::Full, options::style::Time::None),
        ("pl", options::style::Date::Long, options::style::Time::None),
        (
            "pl",
            options::style::Date::Medium,
            options::style::Time::None,
        ),
        (
            "pl",
            options::style::Date::Short,
            options::style::Time::None,
        ),
        ("pl", options::style::Date::None, options::style::Time::Full),
        ("pl", options::style::Date::None, options::style::Time::Long),
        (
            "pl",
            options::style::Date::None,
            options::style::Time::Medium,
        ),
        (
            "pl",
            options::style::Date::None,
            options::style::Time::Short,
        ),
        ("pl", options::style::Date::Full, options::style::Time::Full),
        ("pl", options::style::Date::Long, options::style::Time::Long),
        (
            "pl",
            options::style::Date::Medium,
            options::style::Time::Medium,
        ),
        (
            "pl",
            options::style::Date::Short,
            options::style::Time::Short,
        ),
    ];

    let mut results = vec![];

    for _ in 0..datetimes.len() {
        results.push(String::new());
    }

    let provider = FsDataProvider::try_new(
        "/Users/zbraniecki/projects/intl-measurements/icu4x/data/icu4x/json",
    )
    .expect("Loading file from testdata directory");

    {
        let mut group = c.benchmark_group("datetime");

        group.bench_function("DateTimeFormat/format_to_write", |b| {
            b.iter(|| {
                for value in values {
                    let langid = value.0.parse().unwrap();
                    let options = DateTimeFormatOptions::Style(options::style::Bag {
                        date: value.1,
                        time: value.2,
                        ..Default::default()
                    });
                    let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                    for (dt, result) in datetimes.iter().zip(results.iter_mut()) {
                        result.clear();
                        let _ = dtf.format_to_write(&dt, result);
                    }
                }
            })
        });

        group.bench_function("DateTimeFormat/format_to_string", |b| {
            b.iter(|| {
                for value in values {
                    let langid = value.0.parse().unwrap();
                    let options = DateTimeFormatOptions::Style(options::style::Bag {
                        date: value.1,
                        time: value.2,
                        ..Default::default()
                    });
                    let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                    for dt in &datetimes {
                        let _ = dtf.format_to_string(&dt);
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/format", |b| {
            b.iter(|| {
                for value in values {
                    let langid = value.0.parse().unwrap();
                    let options = DateTimeFormatOptions::Style(options::style::Bag {
                        date: value.1,
                        time: value.2,
                        ..Default::default()
                    });
                    let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                    for (dt, result) in datetimes.iter().zip(results.iter_mut()) {
                        result.clear();
                        let fdt = dtf.format(&dt);
                        write!(result, "{}", fdt).unwrap();
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/to_string", |b| {
            b.iter(|| {
                for value in values {
                    let langid = value.0.parse().unwrap();
                    let options = DateTimeFormatOptions::Style(options::style::Bag {
                        date: value.1,
                        time: value.2,
                        ..Default::default()
                    });
                    let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                    for dt in &datetimes {
                        let fdt = dtf.format(&dt);
                        let _ = fdt.to_string();
                    }
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, datetime_benches,);
criterion_main!(benches);
