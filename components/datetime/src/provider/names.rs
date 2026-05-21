// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for datetime names.

use crate::provider::day_periods::DayPeriodRules;
use crate::size_test_macro::size_test;
use alloc::borrow::Cow;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::marker::ErasedMarker;
use icu_provider::prelude::*;
use icu_time::Hour;
#[cfg(feature = "serde")]
use potential_utf::PotentialUtf8;
use zerovec::ule::vartuple::VarTupleULE;
use zerovec::VarZeroVec;
#[cfg(feature = "serde")]
use zerovec::{ule::tuplevar::Tuple2VarULE, VarZeroCow, VarZeroSlice};

icu_provider::data_marker!(
    /// `DatetimeNamesYearBuddhistV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearBuddhistV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearChineseV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearChineseV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearCopticV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearCopticV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearDangiV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearDangiV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearEthiopianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearEthiopianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearGregorianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearGregorianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearHebrewV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearHebrewV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearIndianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearIndianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearHijriV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearHijriV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearJapaneseV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearJapaneseV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearPersianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearPersianV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesYearRocV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name.
    /// For example, `GGG`/`UUU` both correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesYearRocV1,
    YearNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_year_length"
);

icu_provider::data_marker!(
    /// `DatetimeNamesMonthBuddhistV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthBuddhistV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthChineseV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 1 is "numeric" (only used for months with leap months, like this one)
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthChineseV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthCopticV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthCopticV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthDangiV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 1 is "numeric" (only used for months with leap months, like this one)
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthDangiV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthEthiopianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthEthiopianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthGregorianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthGregorianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthHebrewV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 1 is "numeric" (only used for months with leap months, like this one)
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthHebrewV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthIndianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthIndianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthHijriV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthHijriV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthJapaneseV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthJapaneseV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthPersianV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthPersianV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthRocV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `MMM` corresponds to `3`, and `LLL` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesMonthRocV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);

icu_provider::data_marker!(
    /// `DatetimeNamesWeekdayV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name, plus "s" for "standalone" contexts.
    /// For example, `EEE` corresponds to `3`, and `ccc` to `3s`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    /// - 6 is "short"
    DatetimeNamesWeekdayV1,
    WeekdayNames<'static>,
);
icu_provider::data_marker!(
    /// `DatetimeNamesDayperiodV1`
    ///
    /// This uses a data marker attribute for length. The value is simply the number of
    /// characters in the equivalent CLDR field syntax name. For example, `aaa`/`bbb`/`BBB`
    /// all correspond to `3`.
    ///
    /// The full list is:
    /// - 3 is "abbreviated"
    /// - 4 is "narrow"
    /// - 5 is "wide"
    DatetimeNamesDayperiodV1,
    DayPeriodNames<'static>,
);

size_test!(YearNames, year_names_v1_size, 32);

/// Names used for representing the year.
///
#[doc = year_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum YearNames<'data> {
    /// This calendar has a small, fixed set of eras.
    ///
    /// See [`era_index`](icu_calendar::types::EraYear::era_index) for how this is keyed.
    FixedEras(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
    /// This calendar has a variable set of eras with numeric years, this stores the era names mapped from
    /// era code to the name.
    #[cfg(feature = "serde")]
    VariableEras(#[cfg_attr(feature = "serde", serde(borrow))] YearNamesMap<'data>),
    /// This calendar is cyclic (Chinese, Dangi), so it uses cyclic year names without any eras
    Cyclic(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),
}

#[cfg(feature = "serde")]
impl serde::Serialize for YearNames<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use alloc::vec::Vec;

        #[derive(serde::Serialize)]
        enum Raw<'a> {
            FixedEras(&'a VarZeroVec<'a, str>),
            VariableEras(&'a YearNamesMap<'a>),
            Cyclic(&'a VarZeroVec<'a, str>),
        }

        let x: YearNamesMap;

        match self {
            // Japanese eras are now generated as `FixedEras`, but we want to keep serializing
            // them as VariableEras. It's the only calendar with 7 eras.
            Self::FixedEras(e) if e.len() == 7 => {
                let mut kvs = [
                    PotentialUtf8::from_str("bce"),
                    PotentialUtf8::from_str("ce"),
                    PotentialUtf8::from_str("meiji"),
                    PotentialUtf8::from_str("taisho"),
                    PotentialUtf8::from_str("showa"),
                    PotentialUtf8::from_str("heisei"),
                    PotentialUtf8::from_str("reiwa"),
                ]
                .into_iter()
                .zip(e.iter())
                .collect::<Vec<_>>();
                kvs.sort_unstable();
                let (ks, vs) = kvs.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
                x = VarZeroCow::from_encodeable(&(ks, vs));
                Raw::VariableEras(&x)
            }
            Self::FixedEras(e) => Raw::FixedEras(e),
            Self::VariableEras(e) => Raw::VariableEras(e),
            Self::Cyclic(c) => Raw::Cyclic(c),
        }
        .serialize(serializer)
    }
}

icu_provider::data_struct!(
    YearNames<'_>,
    #[cfg(feature = "datagen")]
);

#[cfg(feature = "serde")]
type YearNamesMap<'data> =
    VarZeroCow<'data, Tuple2VarULE<VarZeroSlice<PotentialUtf8>, VarZeroSlice<str>>>;

#[cfg(feature = "serde")]
pub(crate) fn get_year_name_from_map<'a>(
    map: &'a YearNamesMap<'_>,
    year: &PotentialUtf8,
) -> Option<&'a str> {
    let idx = map.a().binary_search_by(|x| x.cmp(year)).ok()?;
    map.b().get(idx)
}

size_test!(MonthNames, month_names_v1_size, 32);

/// Names used for representing the month.
///
#[doc = month_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub enum MonthNames<'data> {
    /// Month codes M01, M02, M03, .. (can allow for M13 onwards)
    ///
    /// Found for solar and pure lunar calendars
    Linear(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),

    #[cfg(feature = "serde")]
    /// Month codes M01, M02, M03, .. M01L, M02L, ...
    ///
    /// Empty entries for non-present month codes. Will have an equal number of leap and non-leap
    /// entries.
    ///
    /// Found for lunisolar and lunisidereal calendars
    ///
    /// Not used anymore, but kept around for serde stabililty.
    LeapLinear(#[cfg_attr(feature = "serde", serde(borrow))] VarZeroVec<'data, str>),

    /// This represents the formatting to apply to numeric values to produce the corresponding
    /// leap month symbol.
    ///
    /// For numeric formatting only, on calendars with leap months
    LeapNumeric(
        #[cfg_attr(
            feature = "serde",
            serde(
                borrow,
                deserialize_with = "icu_pattern::deserialize_borrowed_cow::<icu_pattern::SinglePlaceholder, _>"
            )
        )]
        Cow<'data, SinglePlaceholderPattern>,
    ),

    /// This represents the formatting to apply to numeric values to produce the corresponding
    /// leap month symbol.
    ///
    /// The VZV contains two elements, the pattern for leap months and the pattern for base months. The
    /// associated `i8` is the offset to apply to the month number before interpolating it into the pattern.
    ///
    /// For numeric formatting only, on calendars with leap months.
    LeapNumericWithBase(
        #[cfg_attr(feature = "serde", serde(borrow))]
        VarZeroVec<'data, VarTupleULE<i8, SinglePlaceholderPattern>>,
    ),

    /// Numeric only
    Numeric,

    /// This represents the formatting to apply to calendars with leap months.
    /// The last two elements are patterns:
    /// * N-2: `SinglePlaceholderPattern` for leap months
    /// * N-1: `SinglePlaceholderPattern` for leap base months
    LeapPattern(VarZeroVec<'data, str>),
}

// Stability, don't want to serialize ::LeapPattern
#[cfg(feature = "serde")]
impl serde::Serialize for MonthNames<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        enum Raw<'a> {
            Linear(&'a VarZeroVec<'a, str>),
            LeapLinear(&'a VarZeroVec<'a, str>),
            LeapNumeric(&'a Cow<'a, SinglePlaceholderPattern>),
            LeapNumericWithBase(&'a VarZeroVec<'a, VarTupleULE<i8, SinglePlaceholderPattern>>),
            Numeric,
        }

        let z;

        match self {
            Self::Numeric => Raw::Numeric,
            Self::Linear(l) => Raw::Linear(l),
            Self::LeapLinear(l) => Raw::LeapLinear(l),
            Self::LeapNumeric(l) => Raw::LeapNumeric(l),
            Self::LeapNumericWithBase(l) => Raw::LeapNumericWithBase(l),
            Self::LeapPattern(l) => {
                use alloc::string::String;
                use alloc::vec::Vec;
                use zerovec::vecs::VarZeroVecOwned;

                let leap_pattern = l.get(l.len() - 2).unwrap_or_default();
                let leap_base_pattern = l.get(l.len() - 1).unwrap_or_default();

                let normal_names = l.iter().take(l.len() - 2);

                let r = if leap_pattern.starts_with('\0') {
                    // The leap pattern is not actually a pattern (no placeholder) - this means it's Hebrew
                    normal_names
                        .map(String::from)
                        .chain([
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            SinglePlaceholderPattern::from_ref_store(leap_pattern)
                                .unwrap_or(SinglePlaceholderPattern::PASS_THROUGH)
                                .interpolate_to_string([l.get(5).unwrap_or_default()]),
                            SinglePlaceholderPattern::from_ref_store(leap_base_pattern)
                                .unwrap_or(SinglePlaceholderPattern::PASS_THROUGH)
                                .interpolate_to_string([l.get(5).unwrap_or_default()]),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                            String::new(),
                        ])
                        .collect()
                } else {
                    normal_names
                        .clone()
                        .map(String::from)
                        .chain(normal_names.map(|m| {
                            SinglePlaceholderPattern::from_ref_store(leap_pattern)
                                .unwrap_or(SinglePlaceholderPattern::PASS_THROUGH)
                                .interpolate_to_string([m])
                        }))
                        .collect::<Vec<_>>()
                };

                #[allow(clippy::unwrap_used)] // small enough
                {
                    z = VarZeroVecOwned::try_from_elements(&r).unwrap().into();
                }
                Raw::LeapLinear(&z)
            }
        }
        .serialize(serializer)
    }
}

icu_provider::data_struct!(
    MonthNames<'_>,
    #[cfg(feature = "datagen")]
);

size_test!(WeekdayNames, linear_names_v1_size, 24);

/// Names used for representing the weekday.
///
#[doc = linear_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct WeekdayNames<'data> {
    /// Element 0 is Sunday.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: VarZeroVec<'data, str>,
}

icu_provider::data_struct!(
    WeekdayNames<'_>,
    #[cfg(feature = "datagen")]
);

impl WeekdayNames<'_> {
    /// Creates a new [`WeekdayNames`] from an iterator of weekday names.
    #[cfg(feature = "datagen")]
    pub fn new<'a>(names: impl Iterator<Item = (icu_calendar::types::Weekday, &'a str)>) -> Self {
        let mut v = [""; 7];
        for (day, name) in names {
            use icu_calendar::types::Weekday::*;
            *match day {
                Sunday => &mut v[0],
                Monday => &mut v[1],
                Tuesday => &mut v[2],
                Wednesday => &mut v[3],
                Thursday => &mut v[4],
                Friday => &mut v[5],
                Saturday => &mut v[6],
            } = name;
        }
        Self { names: (&v).into() }
    }

    /// Returns the name for a weekday, if possible.
    pub fn get(&self, day: icu_calendar::types::Weekday) -> Option<&str> {
        use icu_calendar::types::Weekday::*;
        self.names.get(match day {
            Sunday => 0,
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
        })
    }
}

size_test!(DayPeriodNames, day_period_names_v1_size, 24);

/// Names used for representing the day period.
///
#[doc = day_period_names_v1_size!()]
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::names))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodNames<'data> {
    /// The elements are in order: AM, PM, (noon), (midnight), where the latter two are optional.
    /// In the case noon is missing but midnight is present, the noon value can be the empty string. This is unlikely.
    /// If the locale has flexible day periods, the day period rules are encoded as the first 4 bytes of the 5th string,
    /// and the remainder of the 5th string and any remaining strings contain the flexible period names. Noon and
    /// midnight might be empty in this case.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub names: VarZeroVec<'data, str>,
}

icu_provider::data_struct!(
    DayPeriodNames<'_>,
    #[cfg(feature = "datagen")]
);

impl DayPeriodNames<'_> {
    /// Gets the 'am' name.
    pub(crate) fn am(&self) -> Option<&str> {
        self.names.get(0)
    }
    /// Gets the 'pm' name.
    pub(crate) fn pm(&self) -> Option<&str> {
        self.names.get(1)
    }
    /// Gets the 'noon' name.
    pub(crate) fn noon(&self) -> Option<&str> {
        self.names.get(2).filter(|s| !s.is_empty())
    }
    /// Gets the 'midnight' name.
    pub(crate) fn midnight(&self) -> Option<&str> {
        self.names.get(3).filter(|s| !s.is_empty())
    }
    /// Gets the name for a flexible day period.
    pub(crate) fn flexible_day_period(&self, hour: Hour) -> Option<&str> {
        let (rules, first_name) = self.names.get(4)?.split_at_checked(4)?;
        let offset = DayPeriodRules::decode_from_str(rules)?.name_offset(hour);
        if offset == 0 {
            Some(first_name)
        } else {
            self.names.get(4 + offset)
        }
    }
}

/// Calendar-agnostic year name data marker
pub type YearNamesV1 = ErasedMarker<YearNames<'static>>;

/// Calendar-agnostic month name data marker
pub type MonthNamesV1 = ErasedMarker<MonthNames<'static>>;

/// Re-export of weekday names marker for more consistency
pub use DatetimeNamesWeekdayV1 as WeekdayNamesV1;

/// Re-export of day period names marker for more consistency
pub use DatetimeNamesDayperiodV1 as DayPeriodNamesV1;

#[test]
fn test_dayperiod_names() {
    let names_zh = DataProvider::<DayPeriodNamesV1>::load(
        &crate::provider::Baked,
        DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("5"),
                &icu_locale::langid!("zh").into(),
            ),
            ..Default::default()
        },
    )
    .unwrap()
    .payload;

    // Simplified Chinese (zh) does not map non-B skeletons to B in CLDR, so it only has standard names
    assert_eq!(names_zh.get().am(), Some("上午"));
    assert_eq!(names_zh.get().pm(), Some("下午"));
    assert_eq!(
        names_zh.get().flexible_day_period(12u8.try_into().unwrap()),
        None
    );

    let names_zh_hant = DataProvider::<DayPeriodNamesV1>::load(
        &crate::provider::Baked,
        DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("5"),
                &icu_locale::langid!("zh-Hant").into(),
            ),
            ..Default::default()
        },
    )
    .unwrap()
    .payload;

    // Traditional Chinese (zh-Hant) maps non-B skeletons to B, so it has flexible names
    assert_eq!(names_zh_hant.get().am(), Some("上午"));
    assert_eq!(names_zh_hant.get().pm(), Some("下午"));
    assert_eq!(
        names_zh_hant
            .get()
            .flexible_day_period(19u8.try_into().unwrap()),
        Some("晚上")
    );
}
