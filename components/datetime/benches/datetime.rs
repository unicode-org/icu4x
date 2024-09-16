// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{criterion_group, criterion_main, Criterion};
use icu_datetime::neo::TypedNeoFormatter;

use icu_calendar::{DateTime, Gregorian};
use icu_locale_core::Locale;
use icu_timezone::FormattableZonedDateTime;
use writeable::TryWriteable;

#[path = "../tests/mock.rs"]
mod mock;

fn datetime_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");

    let mut bench_neoneo_datetime_with_fixture = |name, file, has_zones| {
        let fxs = serde_json::from_str::<fixtures::Fixture>(file).unwrap();
        group.bench_function(&format!("semantic/{name}"), |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<FormattableZonedDateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(move |value| {
                            if has_zones {
                                mock::parse_zoned_gregorian_from_str(value)
                            } else {
                                let DateTime { date, time } = mock::parse_gregorian_from_str(value);
                                FormattableZonedDateTime::new_in_utc(date, time)
                            }
                        })
                        .collect();
                    for setup in &fx.setups {
                        let locale: Locale = setup.locale.parse().expect("Failed to parse locale.");
                        let skeleton = setup.options.semantic.unwrap();

                        let dtf = {
                            TypedNeoFormatter::<Gregorian, _>::try_new_with_components(
                                &locale.into(),
                                skeleton.components,
                                skeleton.length.into(),
                            )
                            .expect("Failed to create TypedNeoFormatter.")
                        };

                        let mut result = String::new();

                        for dt in &datetimes {
                            let fdt = dtf.format(dt);
                            fdt.try_write_to(&mut result)
                                .unwrap()
                                .expect("Failed to write to date time format.");
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

    group.finish();
}

criterion_group!(benches, datetime_benches);
criterion_main!(benches);
