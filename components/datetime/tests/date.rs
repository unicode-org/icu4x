use icu_cldr_json_data_provider::{CldrJsonDataProvider, CldrPaths};
use icu_datetime::date::DateTime;
use icu_datetime::options;
use icu_datetime::DateTimeFormat;
use std::fmt::Write;

#[test]
fn it_works() {
    let langid = "en".parse().unwrap();

    let dt = DateTime {
        year: 2020,
        month: 8,
        day: 5,
        ..Default::default()
    };

    let mut cldr_paths = CldrPaths::default();

    cldr_paths.cldr_dates =
        Ok("/Users/zbraniecki/projects/intl-measurements/icu4x/data/cldr/cldr-dates-modern".into());

    let provider = CldrJsonDataProvider::new(&cldr_paths);

    let dtf = DateTimeFormat::try_new(
        langid,
        &provider,
        &options::DateTimeFormatOptions::Style(options::style::Bag {
            date: options::style::Date::Short,
            time: options::style::Time::None,
            ..Default::default()
        }),
    )
    .unwrap();

    let num = dtf.format(&dt);

    let s = num.to_string();
    assert_eq!(s, "8/5/2020");

    let mut s = String::new();
    write!(s, "{}", num).unwrap();
    assert_eq!(s, "8/5/2020");

    let mut s = String::new();
    dtf.format_to_write(&dt, &mut s).unwrap();
    assert_eq!(s, "8/5/2020");

    let s = dtf.format_to_string(&dt);
    assert_eq!(s, "8/5/2020");
}

#[test]
fn display_names() {
    let langid = "en".parse().unwrap();
    let dt = DateTime {
        year: 2020,
        month: 8,
        day: 5,
        ..Default::default()
    };

    let mut cldr_paths = CldrPaths::default();

    cldr_paths.cldr_dates =
        Ok("/Users/zbraniecki/projects/intl-measurements/icu4x/data/cldr/cldr-dates-modern".into());

    let provider = CldrJsonDataProvider::new(&cldr_paths);

    let dtf = DateTimeFormat::try_new(
        langid,
        &provider,
        &options::DateTimeFormatOptions::Style(options::style::Bag {
            date: options::style::Date::Medium,
            time: options::style::Time::Short,
            ..Default::default()
        }),
    )
    .unwrap();

    let s = dtf.format_to_string(&dt);

    assert_eq!(s, "Aug 5, 2020, 0:00 AM");
}
