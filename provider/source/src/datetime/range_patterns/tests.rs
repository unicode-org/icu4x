// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Tests verifying range pattern consistency and coverage across locales in CLDR.
//!
//! # Overlap Pattern Coverage Invariant
//! Overlap skeletons such as `ej` (`ET`, weekday + time) unify date and time fields into an atomic
//! pattern (e.g. `ccc h:mm a`) without separate datetime glue (`{1}, {0}`). In `icu_datetime`, when
//! an overlap pattern is selected in `DateTimeZonePatternSelectionData::try_new_with_skeleton`,
//! it returns early with `date: DatePatternSelectionData::none()` and `glue: None`.
//!
//! Consequently, when a range formatter (`FixedCalendarDateRangeFormatter` or `DateRangeFormatter`)
//! formats an interval using an overlap skeleton, Case 3 (mixed range where only time differs)
//! cannot decompose into `<date><glue><time_range>`. Instead, it relies on fallback range formatting
//! (`format_fallback`), which combines the single datetime overlap formatting with fallback range
//! glue.
//!
//! To ensure range formatting operates cleanly and without unexpected pattern load failures across
//! all locales, we rely on the critical assumption that any locale in CLDR supporting `ej` in standard
//! date/time patterns ALSO supports `ej` in its interval range pattern data. The tests below enforce
//! this invariant across all 12 supported calendars and all locales in CLDR.

use super::*;
use crate::SourceDataProvider;
use icu::datetime::provider::semantic_skeletons::*;

fn check_ej_coverage<M1, M2>(provider: &SourceDataProvider, calendar: DatagenCalendar)
where
    SourceDataProvider: DataProvider<M1> + DataProvider<M2>,
    M1: DataMarker,
    M2: DataMarker,
{
    let dates = provider.cldr().unwrap().dates(Some(calendar));
    let locales = dates.list_locales().unwrap();

    for locale in locales {
        let req = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("ej"),
                &locale,
            ),
            metadata: Default::default(),
        };

        let has_date_pattern = DataProvider::<M1>::load(provider, req).is_ok();
        let has_range_pattern = DataProvider::<M2>::load(provider, req).is_ok();

        if has_date_pattern {
            assert!(
                has_range_pattern,
                "Locale {locale} ({calendar:?}) has an `ej` date pattern but is missing an `ej` date range pattern"
            );
        }
    }
}

macro_rules! check_all_calendars {
    ($provider:expr) => {
        check_ej_coverage::<DatetimePatternsDateBuddhistV1, DatetimePatternsRangeDateBuddhistV1>(
            $provider,
            DatagenCalendar::Buddhist,
        );
        check_ej_coverage::<DatetimePatternsDateChineseV1, DatetimePatternsRangeDateChineseV1>(
            $provider,
            DatagenCalendar::Chinese,
        );
        check_ej_coverage::<DatetimePatternsDateCopticV1, DatetimePatternsRangeDateCopticV1>(
            $provider,
            DatagenCalendar::Coptic,
        );
        check_ej_coverage::<DatetimePatternsDateDangiV1, DatetimePatternsRangeDateDangiV1>(
            $provider,
            DatagenCalendar::Dangi,
        );
        check_ej_coverage::<DatetimePatternsDateEthiopianV1, DatetimePatternsRangeDateEthiopianV1>(
            $provider,
            DatagenCalendar::Ethiopic,
        );
        check_ej_coverage::<DatetimePatternsDateGregorianV1, DatetimePatternsRangeDateGregorianV1>(
            $provider,
            DatagenCalendar::Gregorian,
        );
        check_ej_coverage::<DatetimePatternsDateHebrewV1, DatetimePatternsRangeDateHebrewV1>(
            $provider,
            DatagenCalendar::Hebrew,
        );
        check_ej_coverage::<DatetimePatternsDateIndianV1, DatetimePatternsRangeDateIndianV1>(
            $provider,
            DatagenCalendar::Indian,
        );
        check_ej_coverage::<DatetimePatternsDateHijriV1, DatetimePatternsRangeDateHijriV1>(
            $provider,
            DatagenCalendar::Hijri,
        );
        check_ej_coverage::<DatetimePatternsDateJapaneseV1, DatetimePatternsRangeDateJapaneseV1>(
            $provider,
            DatagenCalendar::Japanese,
        );
        check_ej_coverage::<DatetimePatternsDatePersianV1, DatetimePatternsRangeDatePersianV1>(
            $provider,
            DatagenCalendar::Persian,
        );
        check_ej_coverage::<DatetimePatternsDateRocV1, DatetimePatternsRangeDateRocV1>(
            $provider,
            DatagenCalendar::Roc,
        );
    };
}

#[test]
fn test_ej_overlap_coverage_testing() {
    let provider = SourceDataProvider::new_testing();
    check_all_calendars!(&provider);
}

/// Runs over the complete downloaded CLDR database across all locales and calendars.
/// Marked as `#[ignore]` because enumerating and checking ~12,000 data request combinations
/// across all 12 calendars and hundreds of locales takes ~46 seconds.
#[test]
#[ignore]
#[cfg(feature = "networking")]
fn test_ej_overlap_coverage_all_locales() {
    let provider = SourceDataProvider::new();
    check_all_calendars!(&provider);
}

#[test]
#[cfg(feature = "networking")]
fn test_inherited_range_pattern_consistency() {
    use icu::locale::data_locale;

    let provider = SourceDataProvider::new();

    let load_packed = |cal: DatagenCalendar, locale: DataLocale, attr: &'static str| {
        provider
            .make_packed_range_data(
                &locale,
                Some(cal),
                DataMarkerAttributes::from_str_or_panic(attr),
                ComponentsType::Date,
            )
            .unwrap()
    };

    let load_elements = |cal: DatagenCalendar, locale: DataLocale, attr: &'static str| {
        load_packed(cal, locale, attr)
            .elements
            .iter()
            .map(|ule| ule.header.0)
            .collect::<Vec<_>>()
    };

    // de Buddhist and Hebrew override single patterns from und without overriding intervalFormats:
    // all lengths are cleared, so make_packed_range_data falls back to und's payload, allowing
    // datagen to deduplicate de to und (and runtime to detect the locale mismatch).
    assert_eq!(
        load_packed(DatagenCalendar::Buddhist, data_locale!("de"), "ym0d"),
        load_packed(DatagenCalendar::Buddhist, data_locale!("und"), "ym0d"),
    );
    assert_eq!(
        load_packed(DatagenCalendar::Hebrew, data_locale!("de"), "ym0d"),
        load_packed(DatagenCalendar::Hebrew, data_locale!("und"), "ym0d"),
    );

    // es Hebrew defines its own intervalFormats matching its single patterns, so range patterns are preserved.
    let es_hebrew = load_elements(DatagenCalendar::Hebrew, data_locale!("es"), "ym0d");
    assert!(es_hebrew.iter().all(|&h| h != 0));
}
