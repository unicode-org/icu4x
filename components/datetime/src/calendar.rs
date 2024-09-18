// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::any_calendar::AnyCalendarKind;
use icu_calendar::chinese::Chinese;
use icu_calendar::roc::Roc;
use icu_calendar::{
    buddhist::Buddhist, coptic::Coptic, dangi::Dangi, ethiopian::Ethiopian, hebrew::Hebrew,
    indian::Indian, islamic::IslamicCivil, islamic::IslamicObservational, islamic::IslamicTabular,
    islamic::IslamicUmmAlQura, japanese::Japanese, japanese::JapaneseExtended, persian::Persian,
    Gregorian,
};
use icu_provider::prelude::*;

use crate::provider::neo::*;
use core::marker::PhantomData;
use icu_provider::marker::NeverMarker;

/// The `CldrCalendar` trait is sealed except when the `"experimental"` Cargo
/// feature is enabled. If implementing `CldrCalendar`, you must also
/// implement `UnstableCldrCalendar` and acknowledge the stability policy.
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland.
/// </div>
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
    /// The data marker for loading year symbols for this calendar.
    type YearNamesV1Marker: DataMarker<DataStruct = YearNamesV1<'static>>;

    /// The data marker for loading month symbols for this calendar.
    type MonthNamesV1Marker: DataMarker<DataStruct = MonthNamesV1<'static>>;

    /// The data marker for loading skeleton patterns for this calendar.
    type SkeletaV1Marker: DataMarker<DataStruct = PackedSkeletonDataV1<'static>>;
}

/// A calendar that can never exist.
///
/// Used as a substitute for calendar parameters when a calendar is not needed,
/// such as in a time formatter.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum NeverCalendar {}

impl CldrCalendar for NeverCalendar {
    type YearNamesV1Marker = NeverMarker<YearNamesV1<'static>>;
    type MonthNamesV1Marker = NeverMarker<MonthNamesV1<'static>>;
    type SkeletaV1Marker = NeverMarker<PackedSkeletonDataV1<'static>>;
}

impl CldrCalendar for Buddhist {
    type YearNamesV1Marker = BuddhistYearNamesV1Marker;
    type MonthNamesV1Marker = BuddhistMonthNamesV1Marker;
    type SkeletaV1Marker = BuddhistDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Chinese {
    type YearNamesV1Marker = ChineseYearNamesV1Marker;
    type MonthNamesV1Marker = ChineseMonthNamesV1Marker;
    type SkeletaV1Marker = ChineseDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Coptic {
    type YearNamesV1Marker = CopticYearNamesV1Marker;
    type MonthNamesV1Marker = CopticMonthNamesV1Marker;
    type SkeletaV1Marker = CopticDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Dangi {
    type YearNamesV1Marker = DangiYearNamesV1Marker;
    type MonthNamesV1Marker = DangiMonthNamesV1Marker;
    type SkeletaV1Marker = DangiDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Ethiopian {
    type YearNamesV1Marker = EthiopianYearNamesV1Marker;
    type MonthNamesV1Marker = EthiopianMonthNamesV1Marker;
    type SkeletaV1Marker = EthiopianDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Gregorian {
    type YearNamesV1Marker = GregorianYearNamesV1Marker;
    type MonthNamesV1Marker = GregorianMonthNamesV1Marker;
    type SkeletaV1Marker = GregorianDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Hebrew {
    type YearNamesV1Marker = HebrewYearNamesV1Marker;
    type MonthNamesV1Marker = HebrewMonthNamesV1Marker;
    type SkeletaV1Marker = HebrewDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Indian {
    type YearNamesV1Marker = IndianYearNamesV1Marker;
    type MonthNamesV1Marker = IndianMonthNamesV1Marker;
    type SkeletaV1Marker = IndianDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for IslamicCivil {
    // this value is not actually a valid identifier for this calendar,
    // however since we are overriding is_identifier_allowed_for_calendar we are using
    // this solely for its effects on skeleton data loading
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for IslamicObservational {
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for IslamicTabular {
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for IslamicUmmAlQura {
    type YearNamesV1Marker = IslamicYearNamesV1Marker;
    type MonthNamesV1Marker = IslamicMonthNamesV1Marker;
    type SkeletaV1Marker = IslamicDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Japanese {
    type YearNamesV1Marker = JapaneseYearNamesV1Marker;
    type MonthNamesV1Marker = JapaneseMonthNamesV1Marker;
    type SkeletaV1Marker = JapaneseDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for JapaneseExtended {
    type YearNamesV1Marker = JapaneseExtendedYearNamesV1Marker;
    type MonthNamesV1Marker = JapaneseExtendedMonthNamesV1Marker;
    type SkeletaV1Marker = JapaneseExtendedDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Persian {
    type YearNamesV1Marker = PersianYearNamesV1Marker;
    type MonthNamesV1Marker = PersianMonthNamesV1Marker;
    type SkeletaV1Marker = PersianDateNeoSkeletonPatternsV1Marker;
}

impl CldrCalendar for Roc {
    type YearNamesV1Marker = RocYearNamesV1Marker;
    type MonthNamesV1Marker = RocMonthNamesV1Marker;
    type SkeletaV1Marker = RocDateNeoSkeletonPatternsV1Marker;
}

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

mod private {
    pub trait Sealed {}
}

/// A collection of marker types associated with all calendars.
///
/// This is used to group together the calendar-specific marker types that produce a common
/// [`DynamicDataMarker`]. For example, this trait can be implemented for [`YearNamesV1Marker`].
///
/// This trait serves as a building block for a cross-calendar [`BoundDataProvider`].
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
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum FullDataCalMarkers {}

impl private::Sealed for FullDataCalMarkers {}

/// Implementation of [`CalMarkers`] that includes data for no calendars.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum NoDataCalMarkers {}

impl private::Sealed for NoDataCalMarkers {}

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

pub(crate) struct AnyCalendarProvider<H, P> {
    provider: P,
    kind: AnyCalendarKind,
    _helper: PhantomData<H>,
}

impl<H, P> AnyCalendarProvider<H, P> {
    pub(crate) fn new(provider: P, kind: AnyCalendarKind) -> Self {
        Self {
            provider,
            kind,
            _helper: PhantomData,
        }
    }
}

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
