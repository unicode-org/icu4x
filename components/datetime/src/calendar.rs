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
use icu_locale_core::{
    extensions::unicode::{value, Value},
    subtags::{subtag, Subtag},
};
use icu_provider::prelude::*;

#[cfg(any(feature = "datagen", feature = "experimental"))]
use crate::provider::neo::*;
#[cfg(feature = "experimental")]
use core::marker::PhantomData;
#[cfg(any(feature = "datagen", feature = "experimental"))]
use icu_provider::marker::NeverMarker;

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
    type DateSymbolsV1Marker: DataMarker<DataStruct = DateSymbolsV1<'static>>;

    /// The data marker for loading length-patterns for this calendar.
    type DateLengthsV1Marker: DataMarker<DataStruct = DateLengthsV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading year symbols for this calendar.
    type YearNamesV1Marker: DataMarker<DataStruct = YearNamesV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading month symbols for this calendar.
    type MonthNamesV1Marker: DataMarker<DataStruct = MonthNamesV1<'static>>;

    #[cfg(any(feature = "datagen", feature = "experimental"))]
    /// The data marker for loading skeleton patterns for this calendar.
    type SkeletaV1Marker: DataMarker<DataStruct = PackedSkeletonDataV1<'static>>;

    /// Checks if a given BCP 47 identifier is allowed to be used with this calendar
    ///
    /// By default, just checks against DEFAULT_BCP_47_IDENTIFIER
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == Self::DEFAULT_BCP_47_IDENTIFIER
    }
}

/// Check if the provided value is of the form `islamic-{subcal}`
fn is_islamic_subcal(value: &Value, subcal: Subtag) -> bool {
    if let &[first, second] = value.as_subtags_slice() {
        first == *"islamic" && second == subcal
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
    type SkeletaV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
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
    type SkeletaV1Marker = BuddhistDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = ChineseDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = CopticDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = DangiDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = EthiopianDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = GregorianDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = HebrewDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = IndianDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        *value == value!("islamicc") || is_islamic_subcal(value, subtag!("civil"))
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
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        is_islamic_subcal(value, subtag!("tbla"))
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
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
    fn is_identifier_allowed_for_calendar(value: &Value) -> bool {
        is_islamic_subcal(value, subtag!("umalqura"))
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
    type SkeletaV1Marker = JapaneseDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = JapaneseExtendedDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = PersianDateNeoSkeletonPatternsV1Marker;
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
    type SkeletaV1Marker = RocDateNeoSkeletonPatternsV1Marker;
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
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        })?
        .payload;
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
            id: DataIdentifierBorrowed::for_locale(locale),
            ..Default::default()
        })?
        .payload;
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
        id: DataIdentifierBorrowed::for_locale(locale),
        ..Default::default()
    };
    let payload = match kind {
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Dangi => {
            DataProvider::<<Dangi as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::IslamicUmmAlQura => DataProvider::<
            <IslamicUmmAlQura as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateLengthsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateLengthsV1Marker>::load(provider, req)?
                .payload
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
        id: DataIdentifierBorrowed::for_locale(locale),
        ..Default::default()
    };
    let payload = match kind {
        AnyCalendarKind::Buddhist => {
            DataProvider::<<Buddhist as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Chinese => {
            DataProvider::<<Chinese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Coptic => {
            DataProvider::<<Coptic as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Dangi => {
            DataProvider::<<Dangi as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Ethiopian => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::EthiopianAmeteAlem => {
            DataProvider::<<Ethiopian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Gregorian => {
            DataProvider::<<Gregorian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Hebrew => {
            DataProvider::<<Hebrew as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Indian => {
            DataProvider::<<Indian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::IslamicCivil => DataProvider::<
            <IslamicCivil as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::IslamicObservational => DataProvider::<
            <IslamicObservational as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::IslamicTabular => DataProvider::<
            <IslamicTabular as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::IslamicUmmAlQura => DataProvider::<
            <IslamicUmmAlQura as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::Japanese => {
            DataProvider::<<Japanese as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::JapaneseExtended => DataProvider::<
            <JapaneseExtended as CldrCalendar>::DateSymbolsV1Marker,
        >::load(provider, req)?
        .payload
        .cast(),
        AnyCalendarKind::Persian => {
            DataProvider::<<Persian as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
                .cast()
        }
        AnyCalendarKind::Roc => {
            DataProvider::<<Roc as CldrCalendar>::DateSymbolsV1Marker>::load(provider, req)?
                .payload
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
mod private {
    pub trait Sealed {}
}

/// A collection of marker types associated with all calendars.
///
/// This is used to group together the calendar-specific marker types that produce a common
/// [`DynamicDataMarker`]. For example, this trait can be implemented for [`YearNamesV1Marker`].
///
/// This trait serves as a building block for a cross-calendar [`BoundDataProvider`].
#[cfg(feature = "experimental")]
pub trait CalMarkers<M>: private::Sealed
where
    M: DynamicDataMarker,
{
    /// The type for a [`Buddhist`] calendar
    type Buddhist: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Chinese`] calendar
    type Chinese: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Coptic`] calendar
    type Coptic: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Dangi`] calendar
    type Dangi: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`Ethiopian`] calendar, with Amete Mihret era
    type Ethiopian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`Ethiopian`] calendar, with Amete Alem era
    type EthiopianAmeteAlem: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Gregorian`] calendar
    type Gregorian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Hebrew`] calendar
    type Hebrew: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Indian`] calendar
    type Indian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`IslamicCivil`] calendar
    type IslamicCivil: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`IslamicObservational`] calendar
    type IslamicObservational: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`IslamicTabular`] calendar
    type IslamicTabular: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`IslamicUmmAlQura`] calendar
    type IslamicUmmAlQura: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Japanese`] calendar
    type Japanese: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`JapaneseExtended`] calendar
    type JapaneseExtended: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Persian`] calendar
    type Persian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Roc`] calendar
    type Roc: DataMarker<DataStruct = M::DataStruct>;
}

/// Implementation of [`CalMarkers`] that includes data for all calendars.
#[derive(Debug)]
#[cfg(feature = "experimental")]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum FullDataCalMarkers {}

#[cfg(feature = "experimental")]
impl private::Sealed for FullDataCalMarkers {}

/// Implementation of [`CalMarkers`] that includes data for no calendars.
#[derive(Debug)]
#[cfg(feature = "experimental")]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum NoDataCalMarkers {}

#[cfg(feature = "experimental")]
impl private::Sealed for NoDataCalMarkers {}

#[cfg(feature = "experimental")]
impl<M> CalMarkers<M> for NoDataCalMarkers
where
    M: DynamicDataMarker,
{
    type Buddhist = NeverMarker<M::DataStruct>;
    type Chinese = NeverMarker<M::DataStruct>;
    type Coptic = NeverMarker<M::DataStruct>;
    type Dangi = NeverMarker<M::DataStruct>;
    type Ethiopian = NeverMarker<M::DataStruct>;
    type EthiopianAmeteAlem = NeverMarker<M::DataStruct>;
    type Gregorian = NeverMarker<M::DataStruct>;
    type Hebrew = NeverMarker<M::DataStruct>;
    type Indian = NeverMarker<M::DataStruct>;
    type IslamicCivil = NeverMarker<M::DataStruct>;
    type IslamicObservational = NeverMarker<M::DataStruct>;
    type IslamicTabular = NeverMarker<M::DataStruct>;
    type IslamicUmmAlQura = NeverMarker<M::DataStruct>;
    type Japanese = NeverMarker<M::DataStruct>;
    type JapaneseExtended = NeverMarker<M::DataStruct>;
    type Persian = NeverMarker<M::DataStruct>;
    type Roc = NeverMarker<M::DataStruct>;
}

#[cfg(feature = "experimental")]
pub(crate) struct AnyCalendarProvider<H, P> {
    provider: P,
    kind: AnyCalendarKind,
    _helper: PhantomData<H>,
}

#[cfg(feature = "experimental")]
impl<H, P> AnyCalendarProvider<H, P> {
    pub(crate) fn new(provider: P, kind: AnyCalendarKind) -> Self {
        Self {
            provider,
            kind,
            _helper: PhantomData,
        }
    }
}

#[cfg(feature = "experimental")]
impl<M, H, P> BoundDataProvider<M> for AnyCalendarProvider<H, P>
where
    M: DynamicDataMarker,
    H: CalMarkers<M>,
    P: Sized
        + DataProvider<H::Buddhist>
        + DataProvider<H::Chinese>
        + DataProvider<H::Coptic>
        + DataProvider<H::Dangi>
        + DataProvider<H::Ethiopian>
        + DataProvider<H::EthiopianAmeteAlem>
        + DataProvider<H::Gregorian>
        + DataProvider<H::Hebrew>
        + DataProvider<H::Indian>
        + DataProvider<H::IslamicCivil>
        + DataProvider<H::IslamicObservational>
        + DataProvider<H::IslamicTabular>
        + DataProvider<H::IslamicUmmAlQura>
        + DataProvider<H::Japanese>
        + DataProvider<H::JapaneseExtended>
        + DataProvider<H::Persian>
        + DataProvider<H::Roc>,
{
    fn load_bound(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        use AnyCalendarKind::*;
        let p = &self.provider;
        match self.kind {
            Buddhist => H::Buddhist::bind(p).load_bound(req),
            Chinese => H::Chinese::bind(p).load_bound(req),
            Coptic => H::Coptic::bind(p).load_bound(req),
            Dangi => H::Dangi::bind(p).load_bound(req),
            Ethiopian => H::Ethiopian::bind(p).load_bound(req),
            EthiopianAmeteAlem => H::EthiopianAmeteAlem::bind(p).load_bound(req),
            Gregorian => H::Gregorian::bind(p).load_bound(req),
            Hebrew => H::Hebrew::bind(p).load_bound(req),
            Indian => H::Indian::bind(p).load_bound(req),
            IslamicCivil => H::IslamicCivil::bind(p).load_bound(req),
            IslamicObservational => H::IslamicObservational::bind(p).load_bound(req),
            IslamicTabular => H::IslamicTabular::bind(p).load_bound(req),
            IslamicUmmAlQura => H::IslamicUmmAlQura::bind(p).load_bound(req),
            Japanese => H::Japanese::bind(p).load_bound(req),
            JapaneseExtended => H::JapaneseExtended::bind(p).load_bound(req),
            Persian => H::Persian::bind(p).load_bound(req),
            Roc => H::Roc::bind(p).load_bound(req),
            _ => Err(
                DataError::custom("Don't know how to load data for specified calendar")
                    .with_debug_context(&self.kind),
            ),
        }
    }
    fn bound_marker(&self) -> DataMarkerInfo {
        use AnyCalendarKind::*;
        match self.kind {
            Buddhist => H::Buddhist::INFO,
            Chinese => H::Chinese::INFO,
            Coptic => H::Coptic::INFO,
            Dangi => H::Dangi::INFO,
            Ethiopian => H::Ethiopian::INFO,
            EthiopianAmeteAlem => H::EthiopianAmeteAlem::INFO,
            Gregorian => H::Gregorian::INFO,
            Hebrew => H::Hebrew::INFO,
            Indian => H::Indian::INFO,
            IslamicCivil => H::IslamicCivil::INFO,
            IslamicObservational => H::IslamicObservational::INFO,
            IslamicTabular => H::IslamicTabular::INFO,
            IslamicUmmAlQura => H::IslamicUmmAlQura::INFO,
            Japanese => H::Japanese::INFO,
            JapaneseExtended => H::JapaneseExtended::INFO,
            Persian => H::Persian::INFO,
            Roc => H::Roc::INFO,
            _ => NeverMarker::<M::DataStruct>::INFO,
        }
    }
}

#[cfg(feature = "experimental")]
macro_rules! impl_load_any_calendar {
    ([$(($erased:ident, $marker:ident)),+], [$($kind_cal:ident),+], [$($kind:ident => $cal:ident),+]) => {
        impl_load_any_calendar!(@expand [$(($erased, $marker)),+], [$($kind_cal),+], [$($kind => $cal),+]);
    };
    (@expand [$(($erased:ident, $marker:ident)),+], $tail1:tt, $tail2:tt) => {
        $(impl_load_any_calendar!(@single_impl $erased, $marker, $tail1, $tail2);)+
    };
    (@single_impl $erased:ident, $marker:ident, [$($kind_cal:ident),+], [$($kind:ident => $cal:ident),+]) => {
        impl CalMarkers<$erased> for FullDataCalMarkers {
            $(
                type $kind_cal = <$kind_cal as CldrCalendar>::$marker;
            )+
            $(
                type $kind = <$cal as CldrCalendar>::$marker;
            )+
        }
    };
}

#[cfg(feature = "experimental")]
impl_load_any_calendar!([
    (YearNamesV1Marker, YearNamesV1Marker),
    (MonthNamesV1Marker, MonthNamesV1Marker),
    (SkeletaV1Marker, SkeletaV1Marker)
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
