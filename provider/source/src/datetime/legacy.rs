// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO(#5613): Even though these markers are no longer exported, we need them in order to export
//! semantic skeleton data markers. This should be refactored to skip the intermediate data struct.

use alloc::borrow::Cow;
use icu::{calendar::types::MonthCode, datetime::provider::pattern::CoarseHourCycle};
use icu_provider::prelude::*;
use icu::datetime::provider::neo::*;
use potential_utf::PotentialUtf8;
use tinystr::{tinystr, TinyStr4};
use zerovec::ZeroMap;
use alloc::vec;
use icu::datetime::provider::skeleton::*;
use icu::datetime::provider::pattern::runtime;

/// Data struct for date/time patterns broken down by pattern length.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct LengthPatterns<'data> {
    /// A full length date/time pattern.
    pub full: runtime::Pattern<'data>,
    /// A long length date/time pattern.
    pub long: runtime::Pattern<'data>,
    /// A medium length date/time pattern.
    pub medium: runtime::Pattern<'data>,
    /// A short length date/time pattern.
    pub short: runtime::Pattern<'data>,
}

/// Pattern data for dates.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, zerofrom::ZeroFrom, yoke::Yokeable)]
pub struct DateLengths<'data> {
    /// Date pattern data, broken down by pattern length.
    pub date: LengthPatterns<'data>,

    /// Patterns used to combine date and time length patterns into full date_time patterns.
    pub length_combinations: GenericLengthPatterns<'data>,
}

/// Pattern data for times.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct TimeLengths<'data> {
    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    pub time_h11_h12: LengthPatterns<'data>,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    pub time_h23_h24: LengthPatterns<'data>,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: CoarseHourCycle,
}

/// Symbol data for the months, weekdays, and eras needed to format a date.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct DateSymbols<'data> {
    /// Symbol data for months.
    pub months: months::Contexts<'data>,
    /// Symbol data for weekdays.
    pub weekdays: weekdays::Contexts<'data>,
    /// Symbol data for eras.
    pub eras: Eras<'data>,
}

/// Symbol data for the day periods needed to format a time.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct TimeSymbols<'data> {
    /// Symbol data for day periods.
    pub day_periods: day_periods::Contexts<'data>,
}

/// String data for the name, abbreviation, and narrow form of a date's era.
///
/// Keys of the map represent era codes, and the values are the display names.
///
/// Era codes are derived from CLDR data, and are calendar specific.
/// Some examples include: `"be"`, `"0"` / `"1"`, `"bce"` / `"ce"`,
/// `"heisei"` / `"meiji"` / `"reiwa"` / ...  Not all era codes are inherited as-is,
/// such as for the extended Japanese calendar.
///
/// For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct Eras<'data> {
    /// Symbol data for era names.
    ///
    /// Keys are era codes, and values are display names. See [`Eras`].
    pub names: ZeroMap<'data, PotentialUtf8, str>,
    /// Symbol data for era abbreviations.
    ///
    /// Keys are era codes, and values are display names. See [`Eras`].
    pub abbr: ZeroMap<'data, PotentialUtf8, str>,
    /// Symbol data for era narrow forms.
    ///
    /// Keys are era codes, and values are display names. See [`Eras`].
    pub narrow: ZeroMap<'data, PotentialUtf8, str>,
}

// Note: the SymbolsV* struct doc strings metadata are attached to `$name` in the macro invocation to
// avoid macro parsing ambiguity caused by other metadata already attached to `$symbols`.
macro_rules! symbols {
    ($(#[$symbols_attr:meta])*  $name: ident, $field_id: ident, $symbols: item) => {

        $(#[$symbols_attr])*
        #[doc = concat!("Formatting symbols for [`",
                stringify!($field_id),
                "`](crate::provider::fields::FieldSymbol::",
                stringify!($field_id),
                ").\n\n",
                "For more information on date time symbols, see [`FieldSymbol`](crate::provider::fields::FieldSymbol).")]
        pub mod $name {
            use super::*;

            #[derive(Debug, PartialEq, Clone, zerofrom::ZeroFrom, yoke::Yokeable)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("Locale data for ", stringify!($field_id), " corresponding to the symbols.")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            $symbols

            // UTS 35 specifies that `format` widths are mandatory,
            // except for `short`.
            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("Symbol data for the \"format\" style formatting of ", stringify!($field_id),
                ".\n\nThe format style is used in contexts where it is different from the stand-alone form, ex: ",
                "a case inflected form where the stand-alone form is the nominative case.")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            pub struct FormatWidths<'data> {
                #[doc = concat!("Abbreviated length symbol for \"format\" style symbol for ", stringify!($name), ".")]
                pub abbreviated: Symbols<'data>,
                #[doc = concat!("Narrow length symbol for \"format\" style symbol for ", stringify!($name), ".")]
                pub narrow: Symbols<'data>,
                #[doc = concat!("Short length symbol for \"format\" style symbol for ", stringify!($name), ", if present.")]
                pub short: Option<Symbols<'data>>,
                #[doc = concat!("Wide length symbol for \"format\" style symbol for ", stringify!($name), ".")]
                pub wide: Symbols<'data>,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("Symbol data for the \"stand-alone\" style formatting of ", stringify!($field_id),
                ".\n\nThe stand-alone style is used in contexts where the field is displayed by itself.")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            pub struct StandAloneWidths<'data> {
                #[doc = concat!("Abbreviated length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub abbreviated: Option<Symbols<'data>>,
                #[doc = concat!("Narrow length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub narrow: Option<Symbols<'data>>,
                #[doc = concat!("Short length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub short: Option<Symbols<'data>>,
                #[doc = concat!("Wide length symbol for \"stand-alone\" style symbol for ", stringify!($name), ".")]
                pub wide: Option<Symbols<'data>>,
            }

            #[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
            #[yoke(prove_covariance_manually)]
            #[doc = concat!("The struct containing the symbol data for ", stringify!($field_id),
                " that contains the \"format\" style symbol data ([`FormatWidths`]) and \"stand-alone\" style symbol data ([`StandAloneWidths`]).")]
            ///
            /// <div class="stab unstable">
            /// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
            /// including in SemVer minor releases. While the serde representation of data structs is guaranteed
            /// to be stable, their Rust representation might not be. Use with caution.
            /// </div>
            pub struct Contexts<'data> {
                /// The symbol data for "format" style symbols.
                pub format: FormatWidths<'data>,
                /// The symbol data for "stand-alone" style symbols.
                pub stand_alone: Option<StandAloneWidths<'data>>,
            }
        }
    };
}

symbols!(
    months,
    Month,
    #[allow(clippy::large_enum_variant)]
    pub enum Symbols<'data> {
        /// Twelve symbols for a solar calendar
        ///
        /// This is an optimization to reduce data size.
        SolarTwelve(
            [Cow<'data, str>; 12],
        ),
        /// A calendar with an arbitrary number of months, potentially including leap months
        Other(ZeroMap<'data, MonthCode, str>),
    }
);

impl months::Symbols<'_> {
    /// Get the symbol for the given month code
    pub fn get(&self, code: MonthCode) -> Option<&str> {
        match *self {
            Self::SolarTwelve(ref arr) => {
                // The tinystr macro doesn't work in match patterns
                // so we use consts first
                const CODE_1: TinyStr4 = tinystr!(4, "M01");
                const CODE_2: TinyStr4 = tinystr!(4, "M02");
                const CODE_3: TinyStr4 = tinystr!(4, "M03");
                const CODE_4: TinyStr4 = tinystr!(4, "M04");
                const CODE_5: TinyStr4 = tinystr!(4, "M05");
                const CODE_6: TinyStr4 = tinystr!(4, "M06");
                const CODE_7: TinyStr4 = tinystr!(4, "M07");
                const CODE_8: TinyStr4 = tinystr!(4, "M08");
                const CODE_9: TinyStr4 = tinystr!(4, "M09");
                const CODE_10: TinyStr4 = tinystr!(4, "M10");
                const CODE_11: TinyStr4 = tinystr!(4, "M11");
                const CODE_12: TinyStr4 = tinystr!(4, "M12");
                let idx = match code.0 {
                    CODE_1 => 0,
                    CODE_2 => 1,
                    CODE_3 => 2,
                    CODE_4 => 3,
                    CODE_5 => 4,
                    CODE_6 => 5,
                    CODE_7 => 6,
                    CODE_8 => 7,
                    CODE_9 => 8,
                    CODE_10 => 9,
                    CODE_11 => 10,
                    CODE_12 => 11,
                    _ => return None,
                };
                arr.get(idx).map(|x| &**x)
            }
            Self::Other(ref map) => map.get(&code),
        }
    }
}

impl Default for months::Symbols<'_> {
    fn default() -> Self {
        Self::Other(Default::default())
    }
}

symbols!(
    weekdays,
    Weekday,
    #[derive(Default)]
    pub struct Symbols<'data>(
        pub [Cow<'data, str>; 7],
    );
);

symbols!(
    day_periods,
    DayPeriod,
    #[derive(Default)]
    pub struct Symbols<'data> {
        /// Day period for AM (before noon).
        pub am: Cow<'data, str>,
        /// Day period for PM (after noon).
        pub pm: Cow<'data, str>,
        /// Day period for noon, in locales that support it.
        pub noon: Option<Cow<'data, str>>,
        /// Day period for midnight, in locales that support it.
        pub midnight: Option<Cow<'data, str>>,
    }
);

icu_provider::data_marker!(
    /// `BuddhistDateLengthsV1`
    BuddhistDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `ChineseDateLengthsV1`
    ChineseDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `CopticDateLengthsV1`
    CopticDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `DangiDateLengthsV1`
    DangiDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `EthiopianDateLengthsV1`
    EthiopianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `GregorianDateLengthsV1`
    GregorianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `HebrewDateLengthsV1`
    HebrewDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `IndianDateLengthsV1`
    IndianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `HijriDateLengthsV1`
    HijriDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `JapaneseDateLengthsV1`
    JapaneseDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `JapaneseExtendedDateLengthsV1`
    JapaneseExtendedDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `PersianDateLengthsV1`
    PersianDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `RocDateLengthsV1`
    RocDateLengthsV1,
    DateLengths<'static>
);
icu_provider::data_marker!(
    /// `TimeLengthsV1`
    TimeLengthsV1,
    TimeLengths<'static>
);
icu_provider::data_marker!(
    /// `BuddhistDateSymbolsV1`
    BuddhistDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `ChineseDateSymbolsV1`
    ChineseDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `CopticDateSymbolsV1`
    CopticDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `DangiDateSymbolsV1`
    DangiDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `EthiopianDateSymbolsV1`
    EthiopianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `GregorianDateSymbolsV1`
    GregorianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `HebrewDateSymbolsV1`
    HebrewDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `IndianDateSymbolsV1`
    IndianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `HijriDateSymbolsV1`
    HijriDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `JapaneseDateSymbolsV1`
    JapaneseDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `JapaneseExtendedDateSymbolsV1`
    JapaneseExtendedDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `PersianDateSymbolsV1`
    PersianDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `RocDateSymbolsV1`
    RocDateSymbolsV1,
    DateSymbols<'static>
);
icu_provider::data_marker!(
    /// `TimeSymbolsV1`
    TimeSymbolsV1,
    TimeSymbols<'static>
);

impl<'a> From<&months::Symbols<'a>> for MonthNames<'a> {
    fn from(other: &months::Symbols<'a>) -> Self {
        match other {
            months::Symbols::SolarTwelve(cow_list) => {
                // Can't zero-copy convert a cow list to a VarZeroVec, so we need to allocate
                // a new VarZeroVec. Since VarZeroVec does not implement `from_iter`, first we
                // make a Vec of string references.
                let vec: alloc::vec::Vec<&str> = cow_list.iter().map(|x| &**x).collect();
                MonthNames::Linear((&vec).into())
            }
            months::Symbols::Other(zero_map) => {
                // Only calendar that uses this is hebrew, we can assume it is 12-month
                let mut vec = vec![""; 24];

                for (k, v) in zero_map.iter() {
                    let Some((number, leap)) = MonthCode(*k).parsed() else {
                        debug_assert!(false, "Found unknown month code {k}");
                        continue;
                    };
                    let offset = if leap { 12 } else { 0 };
                    if let Some(entry) = vec.get_mut((number + offset - 1) as usize) {
                        *entry = v;
                    } else {
                        debug_assert!(false, "Found out of bounds hebrew month code {k}")
                    }
                }
                MonthNames::LeapLinear((&vec).into())
            }
        }
    }
}

impl<'a> From<&weekdays::Symbols<'a>> for LinearNames<'a> {
    fn from(other: &weekdays::Symbols<'a>) -> Self {
        // Input is a cow array of length 7. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = other.0.iter().map(|x| &**x).collect();
        LinearNames {
            names: (&vec).into(),
        }
    }
}

impl<'a> From<&day_periods::Symbols<'a>> for LinearNames<'a> {
    fn from(other: &day_periods::Symbols<'a>) -> Self {
        // Input is a struct with four fields. Need to make it a VarZeroVec.
        let vec: alloc::vec::Vec<&str> = match (other.noon.as_ref(), other.midnight.as_ref()) {
            (Some(noon), Some(midnight)) => vec![&other.am, &other.pm, &noon, &midnight],
            (Some(noon), None) => vec![&other.am, &other.pm, &noon],
            (None, Some(midnight)) => vec![&other.am, &other.pm, "", &midnight],
            (None, None) => vec![&other.am, &other.pm],
        };
        LinearNames {
            names: (&vec).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SourceDataProvider;
    use icu_provider::prelude::*;
    use icu::datetime::provider::neo::*;
    use icu::locale::langid;
    use super::*;

    mod key_attr_consts {
        use super::*;

        pub const STADLN_ABBR: &DataMarkerAttributes =
            DataMarkerAttributes::from_str_or_panic("3s");
        pub const STADLN_WIDE: &DataMarkerAttributes =
            DataMarkerAttributes::from_str_or_panic("4s");
        pub const STADLN_NARW: &DataMarkerAttributes =
            DataMarkerAttributes::from_str_or_panic("5s");
        pub const STADLN_SHRT: &DataMarkerAttributes =
            DataMarkerAttributes::from_str_or_panic("6s");
        pub const FORMAT_ABBR: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("3");
        pub const FORMAT_WIDE: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("4");
        pub const FORMAT_NARW: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("5");
        pub const FORMAT_SHRT: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("6");

        /// Used for matching
        pub const STADLN_ABBR_STR: &str = STADLN_ABBR.as_str();
        pub const STADLN_WIDE_STR: &str = STADLN_WIDE.as_str();
        pub const STADLN_NARW_STR: &str = STADLN_NARW.as_str();
        pub const STADLN_SHRT_STR: &str = STADLN_SHRT.as_str();
        pub const FORMAT_ABBR_STR: &str = FORMAT_ABBR.as_str();
        pub const FORMAT_WIDE_STR: &str = FORMAT_WIDE.as_str();
        pub const FORMAT_NARW_STR: &str = FORMAT_NARW.as_str();
        pub const FORMAT_SHRT_STR: &str = FORMAT_SHRT.as_str();
    }

    fn month_symbols_map_project_cloned<M, P>(
        payload: &DataPayload<M>,
        req: DataRequest,
    ) -> Result<DataResponse<P>, DataError>
    where
        M: DataMarker<DataStruct = DateSymbols<'static>>,
        P: DataMarker<DataStruct = MonthNames<'static>>,
    {
        let new_payload = payload.try_map_project_cloned(|payload, _| {
            use key_attr_consts::*;
            let result = match req.id.marker_attributes.as_str() {
                STADLN_ABBR_STR => payload
                    .months
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.abbreviated.as_ref()),
                STADLN_WIDE_STR => payload
                    .months
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.wide.as_ref()),
                STADLN_NARW_STR => payload
                    .months
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.narrow.as_ref()),
                _ => None,
            };
            if let Some(result) = result {
                return Ok(result.into());
            }
            let result = match req.id.marker_attributes.as_str() {
                STADLN_ABBR_STR | FORMAT_ABBR_STR => &payload.months.format.abbreviated,
                STADLN_WIDE_STR | FORMAT_WIDE_STR => &payload.months.format.wide,
                STADLN_NARW_STR | FORMAT_NARW_STR => &payload.months.format.narrow,
                _ => {
                    return Err(DataError::custom("Unknown marker attribute")
                        .with_marker(M::INFO)
                        .with_display_context(req.id.marker_attributes.as_str()))
                }
            };
            Ok(result.into())
        })?;
        Ok(DataResponse {
            payload: new_payload,
            metadata: Default::default(),
        })
    }

    fn weekday_symbols_map_project_cloned<M, P>(
        payload: &DataPayload<M>,
        req: DataRequest,
    ) -> Result<DataResponse<P>, DataError>
    where
        M: DataMarker<DataStruct = DateSymbols<'static>>,
        P: DataMarker<DataStruct = LinearNames<'static>>,
    {
        let new_payload = payload.try_map_project_cloned(|payload, _| {
            use key_attr_consts::*;
            let result = match req.id.marker_attributes.as_str() {
                STADLN_ABBR_STR => payload
                    .weekdays
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.abbreviated.as_ref()),
                STADLN_WIDE_STR => payload
                    .weekdays
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.wide.as_ref()),
                STADLN_NARW_STR => payload
                    .weekdays
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.narrow.as_ref()),
                STADLN_SHRT_STR => payload
                    .weekdays
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.short.as_ref()),
                _ => None,
            };
            if let Some(result) = result {
                return Ok(result.into());
            }
            let result = match req.id.marker_attributes.as_str() {
                STADLN_SHRT_STR | FORMAT_SHRT_STR => payload.weekdays.format.short.as_ref(),
                _ => None,
            };
            if let Some(result) = result {
                return Ok(result.into());
            }
            let result = match req.id.marker_attributes.as_str() {
                STADLN_ABBR_STR | FORMAT_ABBR_STR | STADLN_SHRT_STR | FORMAT_SHRT_STR => {
                    &payload.weekdays.format.abbreviated
                }
                STADLN_WIDE_STR | FORMAT_WIDE_STR => &payload.weekdays.format.wide,
                STADLN_NARW_STR | FORMAT_NARW_STR => &payload.weekdays.format.narrow,
                _ => {
                    return Err(DataError::custom("Unknown marker attribute")
                        .with_marker(M::INFO)
                        .with_display_context(req.id.marker_attributes.as_str()))
                }
            };
            Ok(result.into())
        })?;
        Ok(DataResponse {
            payload: new_payload,
            metadata: Default::default(),
        })
    }

    fn dayperiod_symbols_map_project_cloned<M, P>(
        payload: &DataPayload<M>,
        req: DataRequest,
    ) -> Result<DataResponse<P>, DataError>
    where
        M: DataMarker<DataStruct = TimeSymbols<'static>>,
        P: DataMarker<DataStruct = LinearNames<'static>>,
    {
        let new_payload = payload.try_map_project_cloned(|payload, _| {
            use key_attr_consts::*;
            let result = match req.id.marker_attributes.as_str() {
                STADLN_ABBR_STR => payload
                    .day_periods
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.abbreviated.as_ref()),
                STADLN_WIDE_STR => payload
                    .day_periods
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.wide.as_ref()),
                STADLN_NARW_STR => payload
                    .day_periods
                    .stand_alone
                    .as_ref()
                    .and_then(|x| x.narrow.as_ref()),
                _ => None,
            };
            if let Some(result) = result {
                return Ok(result.into());
            }
            let result = match req.id.marker_attributes.as_str() {
                STADLN_ABBR_STR | FORMAT_ABBR_STR => &payload.day_periods.format.abbreviated,
                STADLN_WIDE_STR | FORMAT_WIDE_STR => &payload.day_periods.format.wide,
                STADLN_NARW_STR | FORMAT_NARW_STR => &payload.day_periods.format.narrow,
                _ => {
                    return Err(DataError::custom("Unknown marker attribute")
                        .with_marker(M::INFO)
                        .with_display_context(req.id.marker_attributes.as_str()))
                }
            };
            Ok(result.into())
        })?;
        Ok(DataResponse {
            payload: new_payload,
            metadata: Default::default(),
        })
    }

    trait Convert<M: DataMarker> {
        fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
    }

    macro_rules! impl_data_provider_adapter {
        ($old_ty:ty, $new_ty:ty, $cnv:ident) => {
            impl Convert<$new_ty> for DataPayload<$old_ty> {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$new_ty>, DataError> {
                    $cnv(self, req)
                }
            }
        };
    }

    impl_data_provider_adapter!(
        BuddhistDateSymbolsV1,
        DatetimeNamesMonthBuddhistV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        ChineseDateSymbolsV1,
        DatetimeNamesMonthChineseV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        CopticDateSymbolsV1,
        DatetimeNamesMonthCopticV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        DangiDateSymbolsV1,
        DatetimeNamesMonthDangiV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        EthiopianDateSymbolsV1,
        DatetimeNamesMonthEthiopianV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        GregorianDateSymbolsV1,
        DatetimeNamesMonthGregorianV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        HebrewDateSymbolsV1,
        DatetimeNamesMonthHebrewV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        IndianDateSymbolsV1,
        DatetimeNamesMonthIndianV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        HijriDateSymbolsV1,
        DatetimeNamesMonthHijriV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        JapaneseDateSymbolsV1,
        DatetimeNamesMonthJapaneseV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        JapaneseExtendedDateSymbolsV1,
        DatetimeNamesMonthJapanextV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        PersianDateSymbolsV1,
        DatetimeNamesMonthPersianV1,
        month_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        RocDateSymbolsV1,
        DatetimeNamesMonthRocV1,
        month_symbols_map_project_cloned
    );

    impl_data_provider_adapter!(
        BuddhistDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        ChineseDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        CopticDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        DangiDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        EthiopianDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        GregorianDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        HebrewDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        IndianDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        HijriDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        JapaneseDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        JapaneseExtendedDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        PersianDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        RocDateSymbolsV1,
        WeekdayNamesV1,
        weekday_symbols_map_project_cloned
    );
    impl_data_provider_adapter!(
        TimeSymbolsV1,
        DayPeriodNamesV1,
        dayperiod_symbols_map_project_cloned
    );

    #[test]
    fn test_adapter_months_numeric() {
        let symbols: DataPayload<GregorianDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_month_abbreviated: DataPayload<DatetimeNamesMonthGregorianV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "Linear([\"Jan\", \"Feb\", \"Mar\", \"Apr\", \"May\", \"Jun\", \"Jul\", \"Aug\", \"Sep\", \"Oct\", \"Nov\", \"Dec\"])"
        );
    }

    #[test]
    fn test_adapter_months_map() {
        let symbols: DataPayload<HebrewDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_month_abbreviated: DataPayload<DatetimeNamesMonthHebrewV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_month_abbreviated:?}"),
            "LeapLinear([\"Tishri\", \"Heshvan\", \"Kislev\", \"Tevet\", \"Shevat\", \"Adar\", \"Nisan\", \"Iyar\", \"Sivan\", \"Tamuz\", \"Av\", \"Elul\", \"\", \"\", \"\", \"\", \"Adar I\", \"Adar II\", \"\", \"\", \"\", \"\", \"\", \"\"])"
        );
    }

    #[test]
    fn test_adapter_weekdays_abbreviated() {
        let symbols: DataPayload<HebrewDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_weekdays_abbreviated: DataPayload<WeekdayNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_weekdays_abbreviated:?}"),
            "LinearNames { names: [\"Sun\", \"Mon\", \"Tue\", \"Wed\", \"Thu\", \"Fri\", \"Sat\"] }"
        );
    }

    #[test]
    fn test_adapter_weekdays_short() {
        let symbols: DataPayload<HebrewDateSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_weekdays_short: DataPayload<WeekdayNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("6s"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_weekdays_short:?}"),
            "LinearNames { names: [\"Su\", \"Mo\", \"Tu\", \"We\", \"Th\", \"Fr\", \"Sa\"] }"
        );
    }

    #[test]
    fn test_adapter_dayperiods() {
        let symbols: DataPayload<TimeSymbolsV1> = SourceDataProvider::new_testing()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let neo_dayperiods_abbreviated: DataPayload<DayPeriodNamesV1> = symbols
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("3s"),
                    &"en".parse().unwrap(),
                ),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            format!("{neo_dayperiods_abbreviated:?}"),
            "LinearNames { names: [\"AM\", \"PM\", \"noon\", \"midnight\"] }"
        );
    }
}

#[cfg(any())] // these tests only test serde
mod obsolete_tests {
    fn serialize_date() -> Vec<u8> {
        let months = [
            (&MonthCode(tinystr!(4, "M01")), "January"),
            (&MonthCode(tinystr!(4, "M02")), "February"),
            (&MonthCode(tinystr!(4, "M03")), "March"),
            (&MonthCode(tinystr!(4, "M04")), "April"),
            (&MonthCode(tinystr!(4, "M05")), "May"),
            (&MonthCode(tinystr!(4, "M06")), "June"),
            (&MonthCode(tinystr!(4, "M07")), "July"),
            (&MonthCode(tinystr!(4, "M08")), "August"),
            (&MonthCode(tinystr!(4, "M09")), "September"),
            (&MonthCode(tinystr!(4, "M10")), "October"),
            (&MonthCode(tinystr!(4, "M11")), "November"),
            (&MonthCode(tinystr!(4, "M12")), "December"),
        ];
        let months = months::Symbols::Other(months.iter().copied().collect());

        let weekdays = weekdays::Symbols([
            Cow::Borrowed("Monday"),
            Cow::Borrowed("Tuesday"),
            Cow::Borrowed("Wednesday"),
            Cow::Borrowed("Thursday"),
            Cow::Borrowed("Friday"),
            Cow::Borrowed("Saturday"),
            Cow::Borrowed("Sunday"),
        ]);

        bincode::serialize(&DateSymbols {
            months: months::Contexts {
                format: months::FormatWidths {
                    abbreviated: months.clone(),
                    narrow: months.clone(),
                    short: Some(months.clone()),
                    wide: months.clone(),
                },
                stand_alone: Some(months::StandAloneWidths {
                    abbreviated: Some(months.clone()),
                    narrow: Some(months.clone()),
                    short: Some(months.clone()),
                    wide: Some(months.clone()),
                }),
            },
            weekdays: weekdays::Contexts {
                format: weekdays::FormatWidths {
                    abbreviated: weekdays.clone(),
                    narrow: weekdays.clone(),
                    short: Some(weekdays.clone()),
                    wide: weekdays.clone(),
                },
                stand_alone: Some(weekdays::StandAloneWidths {
                    abbreviated: Some(weekdays.clone()),
                    narrow: Some(weekdays.clone()),
                    short: Some(weekdays.clone()),
                    wide: Some(weekdays.clone()),
                }),
            },
            eras: Eras {
                names: ZeroMap::new(),
                abbr: ZeroMap::new(),
                narrow: ZeroMap::new(),
            },
        })
        .unwrap()
    }

    fn serialize_time() -> Vec<u8> {
        let day_periods = day_periods::Symbols {
            am: Cow::Borrowed("am"),
            pm: Cow::Borrowed("pm"),
            noon: Some(Cow::Borrowed("noon")),
            midnight: None,
        };

        bincode::serialize(&TimeSymbols {
            day_periods: day_periods::Contexts {
                format: day_periods::FormatWidths {
                    abbreviated: day_periods.clone(),
                    narrow: day_periods.clone(),
                    short: Some(day_periods.clone()),
                    wide: day_periods.clone(),
                },
                stand_alone: Some(day_periods::StandAloneWidths {
                    abbreviated: Some(day_periods.clone()),
                    narrow: Some(day_periods.clone()),
                    short: Some(day_periods.clone()),
                    wide: Some(day_periods.clone()),
                }),
            },
        })
        .unwrap()
    }

    #[test]
    fn weekdays_borrows() {
        let bytes = serialize_date();
        let de = bincode::deserialize::<DateSymbols>(&bytes).unwrap();

        assert!(matches!(de.weekdays.format.narrow.0[2], Cow::Borrowed(_)));
        assert!(matches!(
            de.weekdays.format.short.as_ref().unwrap().0[4],
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn day_periods_borrows() {
        let bytes = serialize_time();
        let de = bincode::deserialize::<TimeSymbols>(&bytes).unwrap();

        assert!(matches!(
            de.day_periods.format.narrow.noon,
            Some(Cow::Borrowed(_))
        ));
        assert!(matches!(
            de.day_periods.format.short.as_ref().unwrap().noon,
            Some(Cow::Borrowed(_))
        ));

        assert!(matches!(de.day_periods.format.narrow.am, Cow::Borrowed(_)));
        assert!(matches!(
            de.day_periods.format.short.as_ref().unwrap().am,
            Cow::Borrowed(_)
        ));
    }
}
