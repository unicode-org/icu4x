// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Scaffolding traits and impls for calendars.

use crate::provider::{neo::*, *};
use crate::scaffold::UnstableSealed;
use crate::{DateTimeFormatterPreferences, MismatchedCalendarError};
use core::marker::PhantomData;
use icu_calendar::cal::Roc;
use icu_calendar::cal::{self, Chinese};
use icu_calendar::cal::{
    Buddhist, Coptic, Dangi, Ethiopian, Gregorian, Hebrew, HijriSimulated, HijriTabular,
    HijriUmmAlQura, Indian, Japanese, JapaneseExtended, Persian,
};
use icu_calendar::{AnyCalendar, AnyCalendarKind, AsCalendar, Date, IntoAnyCalendar, Ref};
use icu_provider::marker::NeverMarker;
use icu_provider::prelude::*;
use icu_time::{
    zone::{models::TimeZoneModel, UtcOffset},
    DateTime, Time, TimeZoneInfo, ZonedDateTime,
};

/// A calendar that can be found in CLDR.
///
/// New implementors of this trait will likely also wish to modify `get_era_code_map()`
/// in the CLDR transformer to support any new era maps.
///
/// <div class="stab unstable">
/// 🚫 This trait is sealed; it cannot be implemented by user code. If an API requests an item that implements this
/// trait, please consider using a type from the implementors listed below.
/// </div>
pub trait CldrCalendar: UnstableSealed {
    /// The data marker for loading year symbols for this calendar.
    type YearNamesV1: DataMarker<DataStruct = YearNames<'static>>;

    /// The data marker for loading month symbols for this calendar.
    type MonthNamesV1: DataMarker<DataStruct = MonthNames<'static>>;

    /// The data marker for loading skeleton patterns for this calendar.
    type SkeletaV1: DataMarker<DataStruct = PackedPatterns<'static>>;
}

impl CldrCalendar for () {
    type YearNamesV1 = NeverMarker<YearNames<'static>>;
    type MonthNamesV1 = NeverMarker<MonthNames<'static>>;
    type SkeletaV1 = NeverMarker<PackedPatterns<'static>>;
}

impl CldrCalendar for Buddhist {
    type YearNamesV1 = DatetimeNamesYearBuddhistV1;
    type MonthNamesV1 = DatetimeNamesMonthBuddhistV1;
    type SkeletaV1 = DatetimePatternsDateBuddhistV1;
}

impl CldrCalendar for Chinese {
    type YearNamesV1 = DatetimeNamesYearChineseV1;
    type MonthNamesV1 = DatetimeNamesMonthChineseV1;
    type SkeletaV1 = DatetimePatternsDateChineseV1;
}

impl CldrCalendar for Coptic {
    type YearNamesV1 = DatetimeNamesYearCopticV1;
    type MonthNamesV1 = DatetimeNamesMonthCopticV1;
    type SkeletaV1 = DatetimePatternsDateCopticV1;
}

impl CldrCalendar for Dangi {
    type YearNamesV1 = DatetimeNamesYearDangiV1;
    type MonthNamesV1 = DatetimeNamesMonthDangiV1;
    type SkeletaV1 = DatetimePatternsDateDangiV1;
}

impl CldrCalendar for Ethiopian {
    type YearNamesV1 = DatetimeNamesYearEthiopianV1;
    type MonthNamesV1 = DatetimeNamesMonthEthiopianV1;
    type SkeletaV1 = DatetimePatternsDateEthiopianV1;
}

impl CldrCalendar for Gregorian {
    type YearNamesV1 = DatetimeNamesYearGregorianV1;
    type MonthNamesV1 = DatetimeNamesMonthGregorianV1;
    type SkeletaV1 = DatetimePatternsDateGregorianV1;
}

impl CldrCalendar for Hebrew {
    type YearNamesV1 = DatetimeNamesYearHebrewV1;
    type MonthNamesV1 = DatetimeNamesMonthHebrewV1;
    type SkeletaV1 = DatetimePatternsDateHebrewV1;
}

impl CldrCalendar for Indian {
    type YearNamesV1 = DatetimeNamesYearIndianV1;
    type MonthNamesV1 = DatetimeNamesMonthIndianV1;
    type SkeletaV1 = DatetimePatternsDateIndianV1;
}

impl CldrCalendar for HijriTabular {
    type YearNamesV1 = DatetimeNamesYearHijriV1;
    type MonthNamesV1 = DatetimeNamesMonthHijriV1;
    type SkeletaV1 = DatetimePatternsDateHijriV1;
}

impl CldrCalendar for HijriSimulated {
    type YearNamesV1 = DatetimeNamesYearHijriV1;
    type MonthNamesV1 = DatetimeNamesMonthHijriV1;
    type SkeletaV1 = DatetimePatternsDateHijriV1;
}

impl CldrCalendar for HijriUmmAlQura {
    type YearNamesV1 = DatetimeNamesYearHijriV1;
    type MonthNamesV1 = DatetimeNamesMonthHijriV1;
    type SkeletaV1 = DatetimePatternsDateHijriV1;
}

impl CldrCalendar for Japanese {
    type YearNamesV1 = DatetimeNamesYearJapaneseV1;
    type MonthNamesV1 = DatetimeNamesMonthJapaneseV1;
    type SkeletaV1 = DatetimePatternsDateJapaneseV1;
}

impl CldrCalendar for JapaneseExtended {
    type YearNamesV1 = DatetimeNamesYearJapanextV1;
    type MonthNamesV1 = DatetimeNamesMonthJapanextV1;
    type SkeletaV1 = DatetimePatternsDateJapanextV1;
}

impl CldrCalendar for Persian {
    type YearNamesV1 = DatetimeNamesYearPersianV1;
    type MonthNamesV1 = DatetimeNamesMonthPersianV1;
    type SkeletaV1 = DatetimePatternsDatePersianV1;
}

impl CldrCalendar for Roc {
    type YearNamesV1 = DatetimeNamesYearRocV1;
    type MonthNamesV1 = DatetimeNamesMonthRocV1;
    type SkeletaV1 = DatetimePatternsDateRocV1;
}

impl UnstableSealed for () {}
impl UnstableSealed for Buddhist {}
impl UnstableSealed for Chinese {}
impl UnstableSealed for Coptic {}
impl UnstableSealed for Dangi {}
impl UnstableSealed for Ethiopian {}
impl UnstableSealed for Gregorian {}
impl UnstableSealed for Hebrew {}
impl UnstableSealed for Indian {}
impl UnstableSealed for HijriTabular {}
impl UnstableSealed for HijriSimulated {}
impl UnstableSealed for HijriUmmAlQura {}
impl UnstableSealed for Japanese {}
impl UnstableSealed for JapaneseExtended {}
impl UnstableSealed for Persian {}
impl UnstableSealed for Roc {}

/// A collection of marker types associated with all formattable calendars.
///
/// This is used to group together the calendar-specific marker types that produce a common
/// [`DynamicDataMarker`]. For example, this trait can be implemented for [`YearNamesV1`].
///
/// This trait serves as a building block for a cross-calendar [`BoundDataProvider`].
///
/// <div class="stab unstable">
/// 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break.
/// </div>
pub trait CalMarkers<M>: UnstableSealed
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
    /// The type for an [`Ethiopian`] calendar (either era style)
    type Ethiopian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Gregorian`] calendar
    type Gregorian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Hebrew`] calendar
    type Hebrew: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Indian`] calendar
    type Indian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for Hirji calendars
    type Hijri: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Japanese`] calendar
    type Japanese: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Persian`] calendar
    type Persian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Roc`] calendar
    type Roc: DataMarker<DataStruct = M::DataStruct>;
}

/// Implementation of [`CalMarkers`] that includes data for all calendars.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum FullDataCalMarkers {}

impl UnstableSealed for FullDataCalMarkers {}

/// Implementation of [`CalMarkers`] that includes data for no calendars.
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // empty enum
pub enum NoDataCalMarkers {}

impl UnstableSealed for NoDataCalMarkers {}

impl<M> CalMarkers<M> for NoDataCalMarkers
where
    M: DynamicDataMarker,
{
    type Buddhist = NeverMarker<M::DataStruct>;
    type Chinese = NeverMarker<M::DataStruct>;
    type Coptic = NeverMarker<M::DataStruct>;
    type Dangi = NeverMarker<M::DataStruct>;
    type Ethiopian = NeverMarker<M::DataStruct>;
    type Gregorian = NeverMarker<M::DataStruct>;
    type Hebrew = NeverMarker<M::DataStruct>;
    type Indian = NeverMarker<M::DataStruct>;
    type Hijri = NeverMarker<M::DataStruct>;
    type Japanese = NeverMarker<M::DataStruct>;
    type Persian = NeverMarker<M::DataStruct>;
    type Roc = NeverMarker<M::DataStruct>;
}

/// A calendar type that is supported by [`DateTimeFormatter`](crate::DateTimeFormatter).
///
/// [`FixedCalendarDateTimeFormatter`](crate::FixedCalendarDateTimeFormatter) might support additional calendars.
pub trait IntoFormattableAnyCalendar: CldrCalendar + IntoAnyCalendar {}

// keep in sync with FormattableAnyCalendarKind
impl IntoFormattableAnyCalendar for Buddhist {}
impl IntoFormattableAnyCalendar for Chinese {}
impl IntoFormattableAnyCalendar for Coptic {}
impl IntoFormattableAnyCalendar for Dangi {}
impl IntoFormattableAnyCalendar for Ethiopian {}
impl IntoFormattableAnyCalendar for Gregorian {}
impl IntoFormattableAnyCalendar for Hebrew {}
impl IntoFormattableAnyCalendar for Indian {}
impl IntoFormattableAnyCalendar for HijriTabular {}
impl IntoFormattableAnyCalendar for HijriSimulated {}
impl IntoFormattableAnyCalendar for HijriUmmAlQura {}
impl IntoFormattableAnyCalendar for Japanese {}
// _NOT_ JapaneseExtended
impl IntoFormattableAnyCalendar for Persian {}
impl IntoFormattableAnyCalendar for Roc {}

// keep in sync with IntoFormattableAnyCalendar
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum FormattableAnyCalendarKind {
    Buddhist,
    Chinese,
    Coptic,
    Dangi,
    Ethiopian,
    EthiopianAmeteAlem,
    Gregorian,
    Hebrew,
    Indian,
    HijriTabularTypeIIFriday,
    // _NOT_ HijriSimulatedMecca
    HijriTabularTypeIIThursday,
    HijriUmmAlQura,
    Japanese,
    // _NOT_ JapaneseExtended
    Persian,
    Roc,
}

impl FormattableAnyCalendarKind {
    pub(crate) fn try_from_any_calendar_kind(kind: AnyCalendarKind) -> Option<Self> {
        use AnyCalendarKind::*;
        let res = match kind {
            Buddhist => Self::Buddhist,
            Chinese => Self::Chinese,
            Coptic => Self::Coptic,
            Dangi => Self::Dangi,
            Ethiopian => Self::Ethiopian,
            EthiopianAmeteAlem => Self::EthiopianAmeteAlem,
            Gregorian => Self::Gregorian,
            Hebrew => Self::Hebrew,
            Indian => Self::Indian,
            HijriTabularTypeIIFriday => Self::HijriTabularTypeIIFriday,
            HijriSimulatedMecca => return None,
            HijriTabularTypeIIThursday => Self::HijriTabularTypeIIThursday,
            HijriUmmAlQura => Self::HijriUmmAlQura,
            Iso => return None,
            Japanese => Self::Japanese,
            JapaneseExtended => return None,
            Persian => Self::Persian,
            Roc => Self::Roc,
            _ => {
                debug_assert!(false, "cross-crate exhaustive match");
                return None;
            }
        };
        Some(res)
    }

    pub(crate) fn from_preferences(mut prefs: DateTimeFormatterPreferences) -> Self {
        if let Some(res) = Self::try_from_any_calendar_kind(AnyCalendarKind::new((&prefs).into())) {
            return res;
        }

        // Calendar not supported by DateTimeFormatter
        // Currently this is CalendarAlgorithm::Iso8601, CalendarAlgorithm::Hijri(Rgsa)
        // Let AnyCalendarKind constructor select an appropriate fallback
        prefs.calendar_algorithm = None;
        if let Some(res) = Self::try_from_any_calendar_kind(AnyCalendarKind::new((&prefs).into())) {
            return res;
        }

        debug_assert!(false, "all locale-default calendars are supported");
        // fall back to something non-Gregorian to make errors more obvious
        FormattableAnyCalendarKind::Coptic
    }
}

#[test]
fn test_calendar_fallback() {
    use icu_locale_core::locale;
    assert_eq!(
        FormattableAnyCalendarKind::from_preferences(locale!("en-TH-u-ca-iso8601").into()),
        FormattableAnyCalendarKind::Buddhist
    );
    assert_eq!(
        FormattableAnyCalendarKind::from_preferences(locale!("en-TH").into()),
        FormattableAnyCalendarKind::Buddhist
    );
    assert_eq!(
        FormattableAnyCalendarKind::from_preferences(locale!("en-SA-u-ca-islamic").into()),
        FormattableAnyCalendarKind::HijriUmmAlQura
    );
    assert_eq!(
        FormattableAnyCalendarKind::from_preferences(locale!("en-IL-u-ca-islamic").into()),
        FormattableAnyCalendarKind::Gregorian
    );
}

/// A version of [`AnyCalendar`] for the calendars supported in the any-calendar formatter.
#[derive(Debug, Clone)]
pub(crate) struct FormattableAnyCalendar {
    any_calendar: AnyCalendar,
    kind: FormattableAnyCalendarKind,
}

impl FormattableAnyCalendar {
    pub(crate) fn from_calendar(calendar: impl IntoFormattableAnyCalendar) -> Self {
        let any_calendar = calendar.to_any();
        let kind = any_calendar.kind();
        let kind = FormattableAnyCalendarKind::try_from_any_calendar_kind(any_calendar.kind())
            .unwrap_or_else(|| {
                debug_assert!(false, "{kind:?} is not a FormattableAnyCalendarKind");
                FormattableAnyCalendarKind::Coptic
            });
        Self { any_calendar, kind }
    }

    pub(crate) fn try_from_any_calendar(any_calendar: AnyCalendar) -> Option<Self> {
        let kind = FormattableAnyCalendarKind::try_from_any_calendar_kind(any_calendar.kind())?;
        Some(Self { any_calendar, kind })
    }

    pub(crate) fn kind(&self) -> FormattableAnyCalendarKind {
        self.kind
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new(kind: FormattableAnyCalendarKind) -> Result<Self, DataError> {
        use FormattableAnyCalendarKind::*;
        let any_calendar = match kind {
            Buddhist => AnyCalendar::Buddhist(cal::Buddhist),
            Chinese => AnyCalendar::Chinese(cal::Chinese::new()),
            Coptic => AnyCalendar::Coptic(cal::Coptic),
            Dangi => AnyCalendar::Dangi(cal::Dangi::new()),
            Ethiopian => AnyCalendar::Ethiopian(cal::Ethiopian::new()),
            EthiopianAmeteAlem => AnyCalendar::Ethiopian(cal::Ethiopian::new_with_era_style(
                cal::EthiopianEraStyle::AmeteAlem,
            )),
            Gregorian => AnyCalendar::Gregorian(cal::Gregorian),
            Hebrew => AnyCalendar::Hebrew(cal::Hebrew),
            Indian => AnyCalendar::Indian(cal::Indian),
            HijriTabularTypeIIFriday => AnyCalendar::HijriTabular(cal::HijriTabular::new(
                cal::HijriTabularLeapYears::TypeII,
                cal::HijriTabularEpoch::Friday,
            )),
            HijriTabularTypeIIThursday => AnyCalendar::HijriTabular(cal::HijriTabular::new(
                cal::HijriTabularLeapYears::TypeII,
                cal::HijriTabularEpoch::Thursday,
            )),
            HijriUmmAlQura => AnyCalendar::HijriUmmAlQura(cal::HijriUmmAlQura::new()),
            Japanese => AnyCalendar::Japanese(cal::Japanese::new()),
            Persian => AnyCalendar::Persian(cal::Persian),
            Roc => AnyCalendar::Roc(cal::Roc),
        };
        Ok(Self { any_calendar, kind })
    }

    #[cfg(feature = "serde")]
    pub(crate) fn try_new_with_buffer_provider<P>(
        provider: &P,
        kind: FormattableAnyCalendarKind,
    ) -> Result<Self, DataError>
    where
        P: ?Sized + BufferProvider,
    {
        use FormattableAnyCalendarKind::*;
        let any_calendar = match kind {
            Buddhist => AnyCalendar::Buddhist(cal::Buddhist),
            Chinese => AnyCalendar::Chinese(cal::Chinese::new()),
            Coptic => AnyCalendar::Coptic(cal::Coptic),
            Dangi => AnyCalendar::Dangi(cal::Dangi::new()),
            Ethiopian => AnyCalendar::Ethiopian(cal::Ethiopian::new()),
            EthiopianAmeteAlem => AnyCalendar::Ethiopian(cal::Ethiopian::new_with_era_style(
                cal::EthiopianEraStyle::AmeteAlem,
            )),
            Gregorian => AnyCalendar::Gregorian(cal::Gregorian),
            Hebrew => AnyCalendar::Hebrew(cal::Hebrew),
            Indian => AnyCalendar::Indian(cal::Indian),
            HijriTabularTypeIIFriday => AnyCalendar::HijriTabular(cal::HijriTabular::new(
                cal::HijriTabularLeapYears::TypeII,
                cal::HijriTabularEpoch::Friday,
            )),
            HijriTabularTypeIIThursday => AnyCalendar::HijriTabular(cal::HijriTabular::new(
                cal::HijriTabularLeapYears::TypeII,
                cal::HijriTabularEpoch::Thursday,
            )),
            HijriUmmAlQura => AnyCalendar::HijriUmmAlQura(cal::HijriUmmAlQura::new()),
            Japanese => {
                AnyCalendar::Japanese(cal::Japanese::try_new_with_buffer_provider(provider)?)
            }
            Persian => AnyCalendar::Persian(cal::Persian),
            Roc => AnyCalendar::Roc(cal::Roc),
        };
        Ok(Self { any_calendar, kind })
    }

    pub(crate) fn try_new_unstable<P>(
        provider: &P,
        kind: FormattableAnyCalendarKind,
    ) -> Result<Self, DataError>
    where
        P: ?Sized + DataProvider<icu_calendar::provider::CalendarJapaneseModernV1>,
    {
        use FormattableAnyCalendarKind::*;
        let any_calendar = match kind {
            Buddhist => AnyCalendar::Buddhist(cal::Buddhist),
            Chinese => AnyCalendar::Chinese(cal::Chinese::new()),
            Coptic => AnyCalendar::Coptic(cal::Coptic),
            Dangi => AnyCalendar::Dangi(cal::Dangi::new()),
            Ethiopian => AnyCalendar::Ethiopian(cal::Ethiopian::new()),
            EthiopianAmeteAlem => AnyCalendar::Ethiopian(cal::Ethiopian::new_with_era_style(
                cal::EthiopianEraStyle::AmeteAlem,
            )),
            Gregorian => AnyCalendar::Gregorian(cal::Gregorian),
            Hebrew => AnyCalendar::Hebrew(cal::Hebrew),
            Indian => AnyCalendar::Indian(cal::Indian),
            HijriTabularTypeIIFriday => AnyCalendar::HijriTabular(cal::HijriTabular::new(
                cal::HijriTabularLeapYears::TypeII,
                cal::HijriTabularEpoch::Friday,
            )),
            HijriTabularTypeIIThursday => AnyCalendar::HijriTabular(cal::HijriTabular::new(
                cal::HijriTabularLeapYears::TypeII,
                cal::HijriTabularEpoch::Thursday,
            )),
            HijriUmmAlQura => AnyCalendar::HijriUmmAlQura(cal::HijriUmmAlQura::new()),
            Japanese => AnyCalendar::Japanese(cal::Japanese::try_new_unstable(provider)?),
            Persian => AnyCalendar::Persian(cal::Persian),
            Roc => AnyCalendar::Roc(cal::Roc),
        };
        Ok(Self { any_calendar, kind })
    }

    pub(crate) fn into_untagged(self) -> UntaggedFormattableAnyCalendar {
        UntaggedFormattableAnyCalendar {
            any_calendar: self.any_calendar,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UntaggedFormattableAnyCalendar {
    // Invariant: the kind must be representable as an FormattableAnyCalendarKind
    any_calendar: AnyCalendar,
}

/// A version of [`FormattableAnyCalendar`] that is smaller on the stack.
impl UntaggedFormattableAnyCalendar {
    pub(crate) fn into_tagged(self) -> FormattableAnyCalendar {
        let kind = FormattableAnyCalendarKind::try_from_any_calendar_kind(self.any_calendar.kind())
            .unwrap_or_else(|| {
                debug_assert!(false, "unreachable by invariant");
                // fall back to something non-Gregorian to make errors more obvious
                FormattableAnyCalendarKind::Coptic
            });
        FormattableAnyCalendar {
            any_calendar: self.any_calendar,
            kind,
        }
    }

    pub(crate) fn any_calendar(&self) -> &AnyCalendar {
        &self.any_calendar
    }

    pub(crate) fn take_any_calendar(self) -> AnyCalendar {
        self.any_calendar
    }
}

pub(crate) struct FormattableAnyCalendarNamesLoader<H, P> {
    provider: P,
    kind: FormattableAnyCalendarKind,
    _helper: PhantomData<H>,
}

impl<H, P> FormattableAnyCalendarNamesLoader<H, P> {
    pub(crate) fn new(provider: P, kind: FormattableAnyCalendarKind) -> Self {
        Self {
            provider,
            kind,
            _helper: PhantomData,
        }
    }
}

impl<M, H, P> BoundDataProvider<M> for FormattableAnyCalendarNamesLoader<H, P>
where
    M: DynamicDataMarker,
    H: CalMarkers<M>,
    P: Sized
        + DataProvider<H::Buddhist>
        + DataProvider<H::Chinese>
        + DataProvider<H::Coptic>
        + DataProvider<H::Dangi>
        + DataProvider<H::Ethiopian>
        + DataProvider<H::Gregorian>
        + DataProvider<H::Hebrew>
        + DataProvider<H::Indian>
        + DataProvider<H::Hijri>
        + DataProvider<H::Japanese>
        + DataProvider<H::Persian>
        + DataProvider<H::Roc>,
{
    fn load_bound(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        use FormattableAnyCalendarKind::*;
        let p = &self.provider;
        match self.kind {
            Buddhist => H::Buddhist::bind(p).load_bound(req),
            Chinese => H::Chinese::bind(p).load_bound(req),
            Coptic => H::Coptic::bind(p).load_bound(req),
            Dangi => H::Dangi::bind(p).load_bound(req),
            Ethiopian | EthiopianAmeteAlem => H::Ethiopian::bind(p).load_bound(req),
            Gregorian => H::Gregorian::bind(p).load_bound(req),
            Hebrew => H::Hebrew::bind(p).load_bound(req),
            Indian => H::Indian::bind(p).load_bound(req),
            HijriTabularTypeIIFriday | HijriTabularTypeIIThursday | HijriUmmAlQura => {
                H::Hijri::bind(p).load_bound(req)
            }
            Japanese => H::Japanese::bind(p).load_bound(req),
            Persian => H::Persian::bind(p).load_bound(req),
            Roc => H::Roc::bind(p).load_bound(req),
        }
    }
    fn bound_marker(&self) -> DataMarkerInfo {
        use FormattableAnyCalendarKind::*;
        match self.kind {
            Buddhist => H::Buddhist::INFO,
            Chinese => H::Chinese::INFO,
            Coptic => H::Coptic::INFO,
            Dangi => H::Dangi::INFO,
            Ethiopian | EthiopianAmeteAlem => H::Ethiopian::INFO,
            Gregorian => H::Gregorian::INFO,
            Hebrew => H::Hebrew::INFO,
            Indian => H::Indian::INFO,
            HijriTabularTypeIIFriday | HijriTabularTypeIIThursday | HijriUmmAlQura => {
                H::Hijri::INFO
            }
            Japanese => H::Japanese::INFO,
            Persian => H::Persian::INFO,
            Roc => H::Roc::INFO,
        }
    }
}

impl CalMarkers<YearNamesV1> for FullDataCalMarkers {
    type Buddhist = <Buddhist as CldrCalendar>::YearNamesV1;
    type Chinese = <Chinese as CldrCalendar>::YearNamesV1;
    type Coptic = <Coptic as CldrCalendar>::YearNamesV1;
    type Dangi = <Dangi as CldrCalendar>::YearNamesV1;
    type Ethiopian = <Ethiopian as CldrCalendar>::YearNamesV1;
    type Gregorian = <Gregorian as CldrCalendar>::YearNamesV1;
    type Hebrew = <Hebrew as CldrCalendar>::YearNamesV1;
    type Indian = <Indian as CldrCalendar>::YearNamesV1;
    type Hijri = <HijriUmmAlQura as CldrCalendar>::YearNamesV1;
    type Japanese = <Japanese as CldrCalendar>::YearNamesV1;
    type Persian = <Persian as CldrCalendar>::YearNamesV1;
    type Roc = <Roc as CldrCalendar>::YearNamesV1;
}

impl CalMarkers<MonthNamesV1> for FullDataCalMarkers {
    type Buddhist = <Buddhist as CldrCalendar>::MonthNamesV1;
    type Chinese = <Chinese as CldrCalendar>::MonthNamesV1;
    type Coptic = <Coptic as CldrCalendar>::MonthNamesV1;
    type Dangi = <Dangi as CldrCalendar>::MonthNamesV1;
    type Ethiopian = <Ethiopian as CldrCalendar>::MonthNamesV1;
    type Gregorian = <Gregorian as CldrCalendar>::MonthNamesV1;
    type Hebrew = <Hebrew as CldrCalendar>::MonthNamesV1;
    type Indian = <Indian as CldrCalendar>::MonthNamesV1;
    type Hijri = <HijriUmmAlQura as CldrCalendar>::MonthNamesV1;
    type Japanese = <Japanese as CldrCalendar>::MonthNamesV1;
    type Persian = <Persian as CldrCalendar>::MonthNamesV1;
    type Roc = <Roc as CldrCalendar>::MonthNamesV1;
}

impl CalMarkers<ErasedPackedPatterns> for FullDataCalMarkers {
    type Buddhist = <Buddhist as CldrCalendar>::SkeletaV1;
    type Chinese = <Chinese as CldrCalendar>::SkeletaV1;
    type Coptic = <Coptic as CldrCalendar>::SkeletaV1;
    type Dangi = <Dangi as CldrCalendar>::SkeletaV1;
    type Ethiopian = <Ethiopian as CldrCalendar>::SkeletaV1;
    type Gregorian = <Gregorian as CldrCalendar>::SkeletaV1;
    type Hebrew = <Hebrew as CldrCalendar>::SkeletaV1;
    type Indian = <Indian as CldrCalendar>::SkeletaV1;
    type Hijri = <HijriUmmAlQura as CldrCalendar>::SkeletaV1;
    type Japanese = <Japanese as CldrCalendar>::SkeletaV1;
    type Persian = <Persian as CldrCalendar>::SkeletaV1;
    type Roc = <Roc as CldrCalendar>::SkeletaV1;
}

/// A type that can be converted into a specific calendar system.
// This trait is implementable
pub trait ConvertCalendar {
    /// The converted type. This can be the same as the receiver type.
    type Converted<'a>: Sized;
    /// Converts `self` to the specified [`AnyCalendar`].
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a>;
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> ConvertCalendar for Date<A> {
    type Converted<'a> = Date<Ref<'a, AnyCalendar>>;
    #[inline]
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        self.to_calendar(Ref(calendar))
    }
}

impl ConvertCalendar for Time {
    type Converted<'a> = Time;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> ConvertCalendar for DateTime<A> {
    type Converted<'a> = DateTime<Ref<'a, AnyCalendar>>;
    #[inline]
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        DateTime {
            date: self.date.to_calendar(Ref(calendar)),
            time: self.time,
        }
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>, Z: Copy> ConvertCalendar
    for ZonedDateTime<A, Z>
{
    type Converted<'a> = ZonedDateTime<Ref<'a, AnyCalendar>, Z>;
    #[inline]
    fn to_calendar<'a>(&self, calendar: &'a AnyCalendar) -> Self::Converted<'a> {
        ZonedDateTime {
            date: self.date.to_calendar(Ref(calendar)),
            time: self.time,
            zone: self.zone,
        }
    }
}

impl<O: TimeZoneModel> ConvertCalendar for TimeZoneInfo<O> {
    type Converted<'a> = TimeZoneInfo<O>;
    #[inline]
    fn to_calendar<'a>(&self, _: &'a AnyCalendar) -> Self::Converted<'a> {
        *self
    }
}

/// An input that may be associated with a specific runtime calendar.
// This trait is implementable
pub trait InSameCalendar {
    /// Checks whether this type is compatible with the given calendar.
    ///
    /// Types that are agnostic to calendar systems should return `Ok(())`.
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError>;
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> InSameCalendar for Date<A> {
    #[inline]
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        if self.calendar().kind() == any_calendar_kind {
            Ok(())
        } else {
            Err(MismatchedCalendarError {
                this_kind: any_calendar_kind,
                date_kind: Some(self.calendar().kind()),
            })
        }
    }
}

impl InSameCalendar for Time {
    #[inline]
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>> InSameCalendar for DateTime<A> {
    #[inline]
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        self.date.check_any_calendar_kind(any_calendar_kind)
    }
}

impl<C: IntoAnyCalendar, A: AsCalendar<Calendar = C>, Z> InSameCalendar for ZonedDateTime<A, Z> {
    #[inline]
    fn check_any_calendar_kind(
        &self,
        any_calendar_kind: AnyCalendarKind,
    ) -> Result<(), MismatchedCalendarError> {
        self.date.check_any_calendar_kind(any_calendar_kind)
    }
}

impl InSameCalendar for UtcOffset {
    #[inline]
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

impl<O: TimeZoneModel> InSameCalendar for TimeZoneInfo<O> {
    #[inline]
    fn check_any_calendar_kind(&self, _: AnyCalendarKind) -> Result<(), MismatchedCalendarError> {
        Ok(())
    }
}

/// An input associated with a fixed, static calendar.
// This trait is implementable
pub trait InFixedCalendar<C> {}

impl<C: CldrCalendar, A: AsCalendar<Calendar = C>> InFixedCalendar<C> for Date<A> {}

impl<C> InFixedCalendar<C> for Time {}

impl<C: CldrCalendar, A: AsCalendar<Calendar = C>> InFixedCalendar<C> for DateTime<A> {}

impl<C: CldrCalendar, A: AsCalendar<Calendar = C>, Z> InFixedCalendar<C> for ZonedDateTime<A, Z> {}

impl<C> InFixedCalendar<C> for UtcOffset {}

impl<C, O: TimeZoneModel> InFixedCalendar<C> for TimeZoneInfo<O> {}
