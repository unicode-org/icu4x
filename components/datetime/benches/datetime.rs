// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod fixtures;

use criterion::{criterion_group, criterion_main, Criterion};
use std::fmt::Write;

use icu_datetime::DateTimeFormat;
use icu_datetime::MockDateTime;

fn datetime_benches(c: &mut Criterion) {
    let fxs = fixtures::get_fixture("styles").unwrap();

    let provider = icu_testdata::get_provider();

    let mut group = c.benchmark_group("datetime");

    group.bench_function("overview", |b| {
        b.iter(|| {
            for fx in &fxs.0 {
                let datetimes: Vec<MockDateTime> = fx
                    .values
                    .iter()
                    .map(|value| value.parse().unwrap())
                    .collect();
                for setup in &fx.setups {
                    let langid = setup.locale.parse().unwrap();
                    let options = fixtures::get_options(&setup.options);
                    let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                    let mut result = String::new();

                    for dt in &datetimes {
                        let fdt = dtf.format(dt);
                        write!(result, "{}", fdt).unwrap();
                        result.clear();
                    }
                }
            }
        })
    });
    group.finish();

    #[cfg(feature = "bench")]
    {
        let mut group = c.benchmark_group("datetime");

        group.bench_function("DateTimeFormat/format_to_write", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<MockDateTime> = fx
                        .values
                        .iter()
                        .map(|value| value.parse().unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let langid = setup.locale.parse().unwrap();
                        let options = fixtures::get_options(&setup.options);
                        let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                        let mut result = String::new();

                        for dt in &datetimes {
                            let _ = dtf.format_to_write(&mut result, dt);
                            result.clear();
                        }
                    }
                }
            })
        });

        group.bench_function("DateTimeFormat/format_to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<MockDateTime> = fx
                        .values
                        .iter()
                        .map(|value| value.parse().unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let langid = setup.locale.parse().unwrap();
                        let options = fixtures::get_options(&setup.options);
                        let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                        for dt in &datetimes {
                            let _ = dtf.format_to_string(dt);
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/format", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<MockDateTime> = fx
                        .values
                        .iter()
                        .map(|value| value.parse().unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let langid = setup.locale.parse().unwrap();
                        let options = fixtures::get_options(&setup.options);
                        let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            write!(result, "{}", fdt).unwrap();
                            result.clear();
                        }
                    }
                }
            })
        });

        group.bench_function("FormattedDateTime/to_string", |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<MockDateTime> = fx
                        .values
                        .iter()
                        .map(|value| value.parse().unwrap())
                        .collect();

                    for setup in &fx.setups {
                        let langid = setup.locale.parse().unwrap();
                        let options = fixtures::get_options(&setup.options);
                        let dtf = DateTimeFormat::try_new(langid, &provider, &options).unwrap();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            let _ = fdt.to_string();
                        }
                    }
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, datetime_benches,);
criterion_main!(benches);
