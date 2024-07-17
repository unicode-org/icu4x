// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod fixtures;

use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(feature = "experimental")]
use icu_datetime::neo::TypedNeoFormatter;
#[cfg(feature = "experimental")]
use icu_datetime::neo_skeleton::{NeoDateSkeleton, NeoSkeletonLength, NeoTimeComponents};
#[cfg(feature = "experimental")]
use icu_datetime::neo_skeleton::{NeoDateTimeComponents, NeoDateTimeSkeleton};
#[cfg(feature = "experimental")]
use icu_datetime::options::length;

use icu_calendar::{DateTime, Gregorian};
#[cfg(feature = "experimental")]
use icu_datetime::DateTimeFormatterOptions;
use icu_locale_core::Locale;
#[cfg(feature = "experimental")]
use writeable::TryWriteable;

#[path = "../tests/mock.rs"]
mod mock;

fn datetime_benches(c: &mut Criterion) {
    let mut group = c.benchmark_group("datetime");

    #[cfg(feature = "experimental")]
    let mut bench_neoneo_datetime_with_fixture = |name, file| {
        let fxs = serde_json::from_str::<fixtures::Fixture>(file).unwrap();
        group.bench_function(&format!("neoneo/datetime_{name}"), |b| {
            b.iter(|| {
                for fx in &fxs.0 {
                    let datetimes: Vec<DateTime<Gregorian>> = fx
                        .values
                        .iter()
                        .map(|value| mock::parse_gregorian_from_str(value))
                        .collect();
                    for setup in &fx.setups {
                        let locale: Locale = setup.locale.parse().expect("Failed to parse locale.");
                        let options = fixtures::get_options(&setup.options).unwrap();

                        let (neo_components, length) = match options {
                            DateTimeFormatterOptions::Length(length::Bag {
                                date: Some(date),
                                time: Some(time),
                                ..
                            }) => {
                                let neo_skeleton =
                                    NeoDateTimeSkeleton::from_date_time_length(date, time);
                                (neo_skeleton.components, neo_skeleton.length)
                            }
                            DateTimeFormatterOptions::Length(length::Bag {
                                date: Some(date),
                                time: None,
                                ..
                            }) => {
                                let neo_skeleton = NeoDateSkeleton::from_date_length(date);
                                (
                                    NeoDateTimeComponents::Date(neo_skeleton.components),
                                    NeoSkeletonLength::Short,
                                )
                            }
                            DateTimeFormatterOptions::Length(length::Bag {
                                date: None,
                                time: Some(time),
                                ..
                            }) => {
                                let neo_time_components = NeoTimeComponents::from_time_length(time);
                                (
                                    NeoDateTimeComponents::Time(neo_time_components),
                                    NeoSkeletonLength::Short,
                                )
                            }
                            _ => todo!(), // Err(LoadError::UnsupportedOptions),
                        };

                        let dtf = {
                            TypedNeoFormatter::<Gregorian, _>::try_new_with_components(
                                &locale.into(),
                                neo_components,
                                length,
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

    #[cfg(feature = "experimental")]
    bench_neoneo_datetime_with_fixture("lengths", include_str!("fixtures/tests/lengths.json"));

    // TODO: Add back benches that read the other fixture files

    group.finish();
}

criterion_group!(benches, datetime_benches);
criterion_main!(benches);
