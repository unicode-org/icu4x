// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::calendar::*;
use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::chinese::Chinese;
use icu_calendar::roc::Roc;
use icu_calendar::AnyCalendar;
use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, dangi::Dangi, ethiopian::Ethiopian, hebrew::Hebrew,
    indian::Indian, islamic::IslamicCivil, islamic::IslamicObservational, islamic::IslamicTabular,
    islamic::IslamicUmmAlQura, japanese::Japanese, japanese::JapaneseExtended, persian::Persian,
    Gregorian,
};
use icu_locid::extensions::unicode::{value, Value};
use icu_provider::prelude::*;
use tinystr::{tinystr, TinyAsciiStr};

#[cfg(any(feature = "datagen", feature = "experimental"))]
use crate::provider::neo::*;
#[cfg(any(feature = "datagen", feature = "experimental"))]
use icu_provider::NeverMarker;

/// The `CldrCalendar` trait is sealed except when the `"experimental"` Cargo
/// feature is enabled. If implementing `CldrCalendar`, you must also
/// implement `UnstableCldrCalendar` and acknowledge the stability policy.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland.
/// </div>
// Exported as `#[cfg(feature = "experimental")]` in lib.rs
// TODO(#4338): Decide what to do with this when we retire this crate's "experimental" feature.
pub trait InternalCldrCalendar {}

/// A calendar that can be found in CLDR
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland.
/// </div>
pub trait CldrCalendar: InternalCldrCalendar {
    /// The Unicode BCP 47 identifier for the calendar's skeleton
    /// If multiple BCP 47 identifiers work, this should be
    /// the default one when no others are provided
    ///
    /// If `is_identifier_allowed_for_calendar()` is set, this only is used for loading skeletons data
    const DEFAULT_BCP_47_IDENTIFIER: Value;

    /// The data marker for loading symbols for this calendar.
    type DateSymbolsV1Marker: KeyedDataMarker<Yokeable = DateSymbolsV1<'static>>;

    /// The data marker for loading length-patterns for this calendar.
    type DateLengthsV1Marker: KeyedDataMarker<Yokeable = DateLengthsV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading year symbols for this calendar.
    type YearNamesV1Marker: KeyedDataMarker<Yokeable = YearNamesV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading month symbols for this calendar.
    type MonthNamesV1Marker: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading a single date pattern for this calendar.
    type DatePatternV1Marker: KeyedDataMarker<Yokeable = DatePatternV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading date-day pattern skeletons for this calendar.
    type DateDaySkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading date-ext pattern skeletons for this calendar.
    type DateExtSkeletonPatternsV1Marker: KeyedDataMarker<Yokeable = PackedSkeletonDataV1<'static>>;

    /// Checks if a given BCP 47 identifier is allowed to be used with this calendar
    ///
    /// By default, just checks against DEFAULT_BCP_47_IDENTIFIER
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == Self::DEFAULT_BCP_47_IDENTIFIER
    }
}

#[cfg(feature = "experimental")]
pub(crate) trait YearNamesV1Provider<M: DataMarker> {
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
}

#[cfg(feature = "experimental")]
impl<M, P> YearNamesV1Provider<M> for P
where
    M: KeyedDataMarker<Yokeable = YearNamesV1<'static>>,
    P: DataProvider<M> + ?Sized,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        DataProvider::<M>::load(self, req)
    }
}

#[cfg(feature = "experimental")]
pub(crate) trait MonthNamesV1Provider<M: DataMarker> {
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
}

#[cfg(feature = "experimental")]
impl<M, P> MonthNamesV1Provider<M> for P
where
    M: KeyedDataMarker<Yokeable = MonthNamesV1<'static>>,
    P: DataProvider<M> + ?Sized,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        DataProvider::<M>::load(self, req)
    }
}

#[cfg(feature = "experimental")]
pub(crate) trait WeekdayNamesV1Provider<M: DataMarker> {
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
}

#[cfg(feature = "experimental")]
impl<M, P> WeekdayNamesV1Provider<M> for P
where
    M: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>,
    P: DataProvider<M> + ?Sized,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        DataProvider::<M>::load(self, req)
    }
}

#[cfg(feature = "experimental")]
pub(crate) trait DayPeriodNamesV1Provider<M: DataMarker> {
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
}

#[cfg(feature = "experimental")]
impl<M, P> DayPeriodNamesV1Provider<M> for P
where
    M: KeyedDataMarker<Yokeable = LinearNamesV1<'static>>,
    P: DataProvider<M> + ?Sized,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        DataProvider::<M>::load(self, req)
    }
}

#[cfg(feature = "experimental")]
pub(crate) trait DatePatternV1Provider<M: DataMarker> {
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError>;
}

#[cfg(feature = "experimental")]
impl<M, P> DatePatternV1Provider<M> for P
where
    M: KeyedDataMarker<Yokeable = DatePatternV1<'static>>,
    P: DataProvider<M> + ?Sized,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        DataProvider::<M>::load(self, req)
    }
}

/// Check if the provided value is of the form `islamic-{subcal}`
fn is_islamic_subcal(value: &Value, subcal: TinyAsciiStr<8>) -> bool {
    if let &[first, second] = value.as_tinystr_slice() {
        first == "islamic" && second == subcal
    } else {
        false
    }
}

/// A calendar that can never exist.
///
/// Used as a substitute for calendar parameters when a calendar is not needed,
/// such as in a time formatter.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
#[cfg(any(feature = "datagen", feature = "experimental"))]
pub enum NeverCalendar {}

#[cfg(any(feature = "datagen", feature = "experimental"))]
impl CldrCalendar for NeverCalendar {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("never");
    type DateSymbolsV1Marker = NeverMarker<DateSymbolsV1<'static>>;
    type DateLengthsV1Marker = NeverMarker<DateLengthsV1<'static>>;
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type DatePatternV1Marker = NeverMarker<DatePatternV1<'static>>;
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
}

impl CldrCalendar for Buddhist {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("buddhist");
    type DateSymbolsV1Marker = BuddhistDateSymbolsV1Marker;
    type DateLengthsV1Marker = BuddhistDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = BuddhistYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = BuddhistMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = BuddhistDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Chinese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("chinese");
    type DateSymbolsV1Marker = ChineseDateSymbolsV1Marker;
    type DateLengthsV1Marker = ChineseDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = ChineseYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = ChineseMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = ChineseDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Coptic {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("coptic");
    type DateSymbolsV1Marker = CopticDateSymbolsV1Marker;
    type DateLengthsV1Marker = CopticDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = CopticYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = CopticMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = CopticDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Dangi {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("dangi");
    type DateSymbolsV1Marker = DangiDateSymbolsV1Marker;
    type DateLengthsV1Marker = DangiDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = DangiYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = DangiMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = DangiDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Ethiopian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("ethiopic");
    type DateSymbolsV1Marker = EthiopianDateSymbolsV1Marker;
    type DateLengthsV1Marker = EthiopianDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = EthiopianYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = EthiopianMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = EthiopianDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("ethiopic") || *value == value!("ethioaa")
    }
}

impl CldrCalendar for Gregorian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("gregory");
    type DateSymbolsV1Marker = GregorianDateSymbolsV1Marker;
    type DateLengthsV1Marker = GregorianDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = GregorianYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = GregorianMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = GregorianDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = GregorianDateDayNeoSkeletonPatternsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = GregorianDateExtNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Hebrew {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("hebrew");
    type DateSymbolsV1Marker = HebrewDateSymbolsV1Marker;
    type DateLengthsV1Marker = HebrewDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = HebrewYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = HebrewMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = HebrewDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Indian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("indian");
    type DateSymbolsV1Marker = IndianDateSymbolsV1Marker;
    type DateLengthsV1Marker = IndianDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = IndianYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = IndianMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = IndianDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for IslamicCivil {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = IslamicDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("islamicc") || is_islamic_subcal(value, tinystr!(8, "civil"))
    }
}

impl CldrCalendar for IslamicObservational {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = IslamicDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for IslamicTabular {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = IslamicDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        is_islamic_subcal(value, tinystr!(8, "tbla"))
    }
}

impl CldrCalendar for IslamicUmmAlQura {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("islamic");
    type DateSymbolsV1Marker = IslamicDateSymbolsV1Marker;
    type DateLengthsV1Marker = IslamicDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = IslamicDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        is_islamic_subcal(value, tinystr!(8, "umalqura"))
    }
}

impl CldrCalendar for Japanese {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanese");
    type DateSymbolsV1Marker = JapaneseDateSymbolsV1Marker;
    type DateLengthsV1Marker = JapaneseDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = JapaneseYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = JapaneseMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = JapaneseDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for JapaneseExtended {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("japanext");
    type DateSymbolsV1Marker = JapaneseExtendedDateSymbolsV1Marker;
    type DateLengthsV1Marker = JapaneseExtendedDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = JapaneseExtendedYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = JapaneseExtendedMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = JapaneseExtendedDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Persian {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("persian");
    type DateSymbolsV1Marker = PersianDateSymbolsV1Marker;
    type DateLengthsV1Marker = PersianDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = PersianYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = PersianMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = PersianDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

impl CldrCalendar for Roc {
    const DEFAULT_BCP_47_IDENTIFIER: Value = value!("roc");
    type DateSymbolsV1Marker = RocDateSymbolsV1Marker;
    type DateLengthsV1Marker = RocDateLengthsV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type YearNamesV1Marker = RocYearNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type MonthNamesV1Marker = RocMonthNamesV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DatePatternV1Marker = RocDatePatternV1Marker;
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateDaySkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
    #[cfg(any(feature = "datagen", feature = "experimental"))]
    type DateExtSkeletonPatternsV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>; // TODO
}

#[cfg(any(feature = "datagen", feature = "experimental"))]
impl InternalCldrCalendar for NeverCalendar {}
impl InternalCldrCalendar for Buddhist {}
impl InternalCldrCalendar for Chinese {}
impl InternalCldrCalendar for Coptic {}
impl InternalCldrCalendar for Dangi {}
impl InternalCldrCalendar for Ethiopian {}
impl InternalCldrCalendar for Gregorian {}
impl InternalCldrCalendar for Hebrew {}
impl InternalCldrCalendar for Indian {}
impl InternalCldrCalendar for IslamicCivil {}
impl InternalCldrCalendar for IslamicObservational {}
impl InternalCldrCalendar for IslamicTabular {}
impl InternalCldrCalendar for IslamicUmmAlQura {}
impl InternalCldrCalendar for Japanese {}
impl InternalCldrCalendar for JapaneseExtended {}
impl InternalCldrCalendar for Persian {}
impl InternalCldrCalendar for Roc {}

pub(crate) fn load_lengths_for_cldr_calendar<C, P>(
    provider: &P,
    locale: &DataLocale,
) -> Result<DataPayload<ErasedDateLengthsV1Marker>, DataError>
where
    C: CldrCalendar,
    P: DataProvider<<C as CldrCalendar>::DateLengthsV1Marker> + ?Sized,
{
    let payload = provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(payload.cast())
}

pub(crate) fn load_symbols_for_cldr_calendar<C, P>(
    provider: &P,
    locale: &DataLocale,
) -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>
where
    C: CldrCalendar,
    P: DataProvider<<C as CldrCalendar>::DateSymbolsV1Marker> + ?Sized,
{
    let payload = provider
        .load(DataRequest {
            locale,
            metadata: Default::default(),
        })?
        .take_payload()?;
    Ok(payload.cast())
}

pub(crate) fn load_lengths_for_any_calendar_kind<P>(
    provider: &P,
    locale: &DataLocale,
    kind: AnyCalendarKind,
) -> Result<DataPayload<ErasedDateLengthsV1Marker>, DataError>
where
    P: DataProvider<BuddhistDateLengthsV1Marker>
        + DataProvider<ChineseDateLengthsV1Marker>
        + DataProvider<CopticDateLengthsV1Marker>
        + DataProvider<DangiDateLengthsV1Marker>
        + DataProvider<EthiopianDateLengthsV1Marker>
        + DataProvider<GregorianDateLengthsV1Marker>
        + DataProvider<HebrewDateLengthsV1Marker>
        + DataProvider<IndianDateLengthsV1Marker>
        + DataProvider<IslamicDateLengthsV1Marker>
        + DataProvider<JapaneseDateLengthsV1Marker>
        + DataProvider<JapaneseExtendedDateLengthsV1Marker>
        + DataProvider<PersianDateLengthsV1Marker>
        + DataProvider<RocDateLengthsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Dangi => {
            DataProvider::<<Dangi as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicUmmAlQura => DataProvider::<
            <IslamicUmmAlQura as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        _ => {
            return Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&kind),
            )
        }
    };
    Ok(payload)
}

pub(crate) fn load_symbols_for_any_calendar_kind<P>(
    provider: &P,
    locale: &DataLocale,
    kind: AnyCalendarKind,
) -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>
where
    P: DataProvider<BuddhistDateSymbolsV1Marker>
        + DataProvider<ChineseDateSymbolsV1Marker>
        + DataProvider<CopticDateSymbolsV1Marker>
        + DataProvider<DangiDateSymbolsV1Marker>
        + DataProvider<EthiopianDateSymbolsV1Marker>
        + DataProvider<GregorianDateSymbolsV1Marker>
        + DataProvider<HebrewDateSymbolsV1Marker>
        + DataProvider<IndianDateSymbolsV1Marker>
        + DataProvider<IslamicDateSymbolsV1Marker>
        + DataProvider<JapaneseDateSymbolsV1Marker>
        + DataProvider<JapaneseExtendedDateSymbolsV1Marker>
        + DataProvider<PersianDateSymbolsV1Marker>
        + DataProvider<RocDateSymbolsV1Marker>
        + ?Sized,
{
    let req = DataRequest {
        locale,
        metadata: Default::default(),
    };
    let payload = match kind {
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Dangi => {
            DataProvider::<<Dangi as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::IslamicUmmAlQura => DataProvider::<
            <IslamicUmmAlQura as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .take_payload()?
        .cast(),
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .take_payload()?
                .cast()
        }
        _ => {
            return Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&kind),
            )
        }
    };
    Ok(payload)
}

#[cfg(feature = "experimental")]
pub(crate) struct AnyCalendarProvider<'a, P: ?Sized> {
    pub(crate) provider: &'a P,
    pub(crate) kind: AnyCalendarKind,
}

#[cfg(feature = "experimental")]
macro_rules! impl_load_any_calendar {
    ([$(($trait:ident, $erased:ident, $marker:ident)),+], [$($kind_cal:ident),+], [$($kind:ident => $cal:ident),+]) => {
        impl_load_any_calendar!(@expand [$(($trait, $erased, $marker)),+], [$($kind_cal),+], [$($kind => $cal),+]);
    };
    (@expand [$(($trait:ident, $erased:ident, $marker:ident)),+], $tail1:tt, $tail2:tt) => {
        $(impl_load_any_calendar!(@single_impl $trait, $erased, $marker, $tail1, $tail2);)+
    };
    (@single_impl $trait:ident, $erased:ident, $marker:ident, [$($kind_cal:ident),+], [$($kind:ident => $cal:ident),+]) => {
        impl<P> $trait<$erased> for AnyCalendarProvider<'_, P>
        where
            P: ?Sized + $(DataProvider::<<$kind_cal as CldrCalendar>::$marker> +)+
        {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<$erased>, DataError> {
                match self.kind {
                    $(
                        AnyCalendarKind::$kind_cal => DataProvider
                            ::<<$kind_cal as CldrCalendar>::$marker>
                            ::load(self.provider, req)
                            .map(DataResponse::cast),
                    )+
                    $(
                        AnyCalendarKind::$kind => DataProvider
                            ::<<$cal as CldrCalendar>::$marker>
                            ::load(self.provider, req)
                            .map(DataResponse::cast),
                    )+
                    _ => Err(
                        DataError::custom("Don't know how to load data for specified calendar")
                            .with_debug_context(&self.kind)),
                }
            }
        }
    };
}

#[cfg(feature = "experimental")]
impl_load_any_calendar!([
    (DatePatternV1Provider, ErasedDatePatternV1Marker, DatePatternV1Marker),
    (YearNamesV1Provider, ErasedYearNamesV1Marker, YearNamesV1Marker),
    (MonthNamesV1Provider, ErasedMonthNamesV1Marker, MonthNamesV1Marker)
], [
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopian,
    Gregorian,
    Hebrew,
    Indian,
    IslamicCivil,
    IslamicObservational,
    IslamicTabular,
    IslamicUmmAlQura,
    Japanese,
    JapaneseExtended,
    Persian,
    Roc
], [
    EthiopianAmeteAlem => Ethiopian
]);

/// Converts a date to the correct calendar if necessary
///
/// Returns `Err` if the date is not ISO or compatible with the current calendar, returns `Ok(None)`
/// if the date is compatible with the current calendar and doesn't need conversion
pub(crate) fn convert_if_necessary<'a>(
    any_calendar: &'a AnyCalendar,
    value: &impl crate::input::DateInput<Calendar = AnyCalendar>,
) -> Result<
    Option<icu_calendar::Date<icu_calendar::Ref<'a, AnyCalendar>>>,
    crate::MismatchedCalendarError,
> {
    let this_kind = any_calendar.kind();
    let date_kind = value.any_calendar_kind();

    if Some(this_kind) != date_kind {
        if date_kind != Some(AnyCalendarKind::Iso) {
            return Err(crate::MismatchedCalendarError {
                this_kind,
                date_kind,
            });
        }
        let date = value.to_iso().to_any();
        let converted = any_calendar.convert_any_date(&date);
        Ok(Some(converted))
    } else {
        Ok(None)
    }
}

/// Converts a date to the correct calendar if necessary
///
/// Returns `Err` if the date is not ISO or compatible with the current calendar, returns `Ok(None)`
/// if the date is compatible with the current calendar and doesn't need conversion
pub(crate) fn convert_datetime_if_necessary<'a>(
    any_calendar: &'a AnyCalendar,
    value: &impl crate::input::DateTimeInput<Calendar = AnyCalendar>,
) -> Result<
    Option<icu_calendar::DateTime<icu_calendar::Ref<'a, AnyCalendar>>>,
    crate::MismatchedCalendarError,
> {
    let this_kind = any_calendar.kind();
    let date_kind = value.any_calendar_kind();

    if Some(this_kind) != date_kind {
        if date_kind != Some(AnyCalendarKind::Iso) {
            return Err(crate::MismatchedCalendarError {
                this_kind,
                date_kind,
            });
        }
        let date = value.to_iso();
        let time = icu_calendar::Time::new(
            value.hour().unwrap_or_default(),
            value.minute().unwrap_or_default(),
            value.second().unwrap_or_default(),
            value.nanosecond().unwrap_or_default(),
        );
        let datetime = icu_calendar::DateTime::new(date, time).to_any();
        let converted = any_calendar.convert_any_datetime(&datetime);
        Ok(Some(converted))
    } else {
        Ok(None)
    }
}
