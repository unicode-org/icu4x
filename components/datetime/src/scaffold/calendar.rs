// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Scaffolding traits and impls for calendars.

use crate::provider::{names::*, packed_pattern::*, semantic_skeletons::*};
use crate::scaffold::UnstableSealed;
use crate::MismatchedCalendarError;
use core::marker::PhantomData;
use icu_calendar::cal;
use icu_calendar::preferences::{CalendarAlgorithm, CalendarPreferences, HijriCalendarAlgorithm};
use icu_calendar::{AnyCalendar, AnyCalendarKind, AsCalendar, Date, IntoAnyCalendar, Ref};
use icu_provider::marker::NeverMarker;
use icu_provider::prelude::*;
use icu_time::{
    zone::{models::TimeZoneModel, UtcOffset},
    DateTime, Time, TimeZoneInfo, ZonedDateTime,
};
#[cfg(feature = "unstable")]
use icu_time::ZonedTime;

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

impl CldrCalendar for cal::Buddhist {
    type YearNamesV1 = DatetimeNamesYearBuddhistV1;
    type MonthNamesV1 = DatetimeNamesMonthBuddhistV1;
    type SkeletaV1 = DatetimePatternsDateBuddhistV1;
}

impl CldrCalendar for cal::ChineseTraditional {
    type YearNamesV1 = DatetimeNamesYearChineseV1;
    type MonthNamesV1 = DatetimeNamesMonthChineseV1;
    type SkeletaV1 = DatetimePatternsDateChineseV1;
}

impl CldrCalendar for cal::Coptic {
    type YearNamesV1 = DatetimeNamesYearCopticV1;
    type MonthNamesV1 = DatetimeNamesMonthCopticV1;
    type SkeletaV1 = DatetimePatternsDateCopticV1;
}

impl CldrCalendar for cal::KoreanTraditional {
    type YearNamesV1 = DatetimeNamesYearDangiV1;
    type MonthNamesV1 = DatetimeNamesMonthDangiV1;
    type SkeletaV1 = DatetimePatternsDateDangiV1;
}

impl CldrCalendar for cal::Ethiopian {
    type YearNamesV1 = DatetimeNamesYearEthiopianV1;
    type MonthNamesV1 = DatetimeNamesMonthEthiopianV1;
    type SkeletaV1 = DatetimePatternsDateEthiopianV1;
}

impl CldrCalendar for cal::Gregorian {
    type YearNamesV1 = DatetimeNamesYearGregorianV1;
    type MonthNamesV1 = DatetimeNamesMonthGregorianV1;
    type SkeletaV1 = DatetimePatternsDateGregorianV1;
}

impl CldrCalendar for cal::Hebrew {
    type YearNamesV1 = DatetimeNamesYearHebrewV1;
    type MonthNamesV1 = DatetimeNamesMonthHebrewV1;
    type SkeletaV1 = DatetimePatternsDateHebrewV1;
}

impl CldrCalendar for cal::Indian {
    type YearNamesV1 = DatetimeNamesYearIndianV1;
    type MonthNamesV1 = DatetimeNamesMonthIndianV1;
    type SkeletaV1 = DatetimePatternsDateIndianV1;
}

/// [`hijri::Rules`](cal::hijri::Rules)-specific formatting options.
///
/// See [`CldrCalendar`].
///
/// The simplest implementation of this uses the same names
/// as some provided [`hijri::Rules`](cal::hijri::Rules):
///
/// ```rust
/// use icu::calendar::cal::hijri;
/// use icu::datetime::scaffold::FormattableHijriRules;
///
/// #[derive(Clone, Debug)]
/// struct MyRules;
///
/// impl icu::calendar::cal::scaffold::UnstableSealed for MyRules {}
/// impl icu::datetime::scaffold::UnstableSealed for MyRules {}
///
/// impl hijri::Rules for MyRules {
///     fn year(&self, _year: i32) -> hijri::HijriYear {
///         todo!()
///     }
/// }
///
/// impl FormattableHijriRules for MyRules {
///     type YearNamesV1 =
///         <hijri::UmmAlQura as FormattableHijriRules>::YearNamesV1;
///     type MonthNamesV1 =
///         <hijri::UmmAlQura as FormattableHijriRules>::MonthNamesV1;
///     type SkeletaV1 = <hijri::UmmAlQura as FormattableHijriRules>::SkeletaV1;
/// }
/// ```
// TODO: default associated types would be nice (https://github.com/rust-lang/rust/issues/29661)
pub trait FormattableHijriRules: cal::hijri::unstable_internal::Rules + UnstableSealed {
    /// The data marker for loading year symbols for this calendar.
    type YearNamesV1: DataMarker<DataStruct = YearNames<'static>>;

    /// The data marker for loading month symbols for this calendar.
    type MonthNamesV1: DataMarker<DataStruct = MonthNames<'static>>;

    /// The data marker for loading skeleton patterns for this calendar.
    type SkeletaV1: DataMarker<DataStruct = PackedPatterns<'static>>;
}

impl UnstableSealed for cal::hijri::TabularAlgorithm {}
impl FormattableHijriRules for cal::hijri::TabularAlgorithm {
    type YearNamesV1 = DatetimeNamesYearHijriV1;
    type MonthNamesV1 = DatetimeNamesMonthHijriV1;
    type SkeletaV1 = DatetimePatternsDateHijriV1;
}

impl UnstableSealed for cal::hijri::UmmAlQura {}
impl FormattableHijriRules for cal::hijri::UmmAlQura {
    type YearNamesV1 = DatetimeNamesYearHijriV1;
    type MonthNamesV1 = DatetimeNamesMonthHijriV1;
    type SkeletaV1 = DatetimePatternsDateHijriV1;
}

#[allow(deprecated)]
impl UnstableSealed for cal::hijri::AstronomicalSimulation {}
#[allow(deprecated)]
impl FormattableHijriRules for cal::hijri::AstronomicalSimulation {
    type YearNamesV1 = DatetimeNamesYearHijriV1;
    type MonthNamesV1 = DatetimeNamesMonthHijriV1;
    type SkeletaV1 = DatetimePatternsDateHijriV1;
}

impl<R: FormattableHijriRules> CldrCalendar for cal::Hijri<R> {
    type YearNamesV1 = R::YearNamesV1;
    type MonthNamesV1 = R::MonthNamesV1;
    type SkeletaV1 = R::SkeletaV1;
}

impl CldrCalendar for cal::Japanese {
    type YearNamesV1 = DatetimeNamesYearJapaneseV1;
    type MonthNamesV1 = DatetimeNamesMonthJapaneseV1;
    type SkeletaV1 = DatetimePatternsDateJapaneseV1;
}

impl CldrCalendar for cal::Persian {
    type YearNamesV1 = DatetimeNamesYearPersianV1;
    type MonthNamesV1 = DatetimeNamesMonthPersianV1;
    type SkeletaV1 = DatetimePatternsDatePersianV1;
}

impl CldrCalendar for cal::Roc {
    type YearNamesV1 = DatetimeNamesYearRocV1;
    type MonthNamesV1 = DatetimeNamesMonthRocV1;
    type SkeletaV1 = DatetimePatternsDateRocV1;
}

impl UnstableSealed for () {}
impl UnstableSealed for cal::Buddhist {}
impl UnstableSealed for cal::ChineseTraditional {}
impl UnstableSealed for cal::Coptic {}
impl UnstableSealed for cal::KoreanTraditional {}
impl UnstableSealed for cal::Ethiopian {}
impl UnstableSealed for cal::Gregorian {}
impl UnstableSealed for cal::Hebrew {}
impl UnstableSealed for cal::Indian {}
impl<R: cal::hijri::unstable_internal::Rules> UnstableSealed for cal::Hijri<R> {}
impl UnstableSealed for cal::Japanese {}
impl UnstableSealed for cal::Persian {}
impl UnstableSealed for cal::Roc {}

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
    /// The type for a [`Buddhist`](cal::Buddhist) calendar
    type Buddhist: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Chinese`](cal::Chinese) calendar
    type Chinese: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Coptic`](cal::Coptic) calendar
    type Coptic: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Dangi`](cal::Dangi) calendar
    type Dangi: DataMarker<DataStruct = M::DataStruct>;
    /// The type for an [`Ethiopian`](cal::Ethiopian) calendar (either era style)
    type Ethiopian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Gregorian`](cal::Gregorian) calendar
    type Gregorian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Hebrew`](cal::Hebrew) calendar
    type Hebrew: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Indian`](cal::Indian) calendar
    type Indian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for Hirji calendars
    type Hijri: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Japanese`](cal::Japanese) calendar
    type Japanese: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Persian`](cal::Persian) calendar
    type Persian: DataMarker<DataStruct = M::DataStruct>;
    /// The type for a [`Roc`](cal::Roc) calendar
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
impl IntoFormattableAnyCalendar for cal::Buddhist {}
impl IntoFormattableAnyCalendar for cal::ChineseTraditional {}
impl IntoFormattableAnyCalendar for cal::Coptic {}
impl IntoFormattableAnyCalendar for cal::KoreanTraditional {}
impl IntoFormattableAnyCalendar for cal::Ethiopian {}
impl IntoFormattableAnyCalendar for cal::Gregorian {}
impl IntoFormattableAnyCalendar for cal::Hebrew {}
impl IntoFormattableAnyCalendar for cal::Indian {}
impl IntoFormattableAnyCalendar for cal::Hijri<cal::hijri::TabularAlgorithm> {}
#[allow(deprecated)]
impl IntoFormattableAnyCalendar for cal::Hijri<cal::hijri::AstronomicalSimulation> {}
impl IntoFormattableAnyCalendar for cal::Hijri<cal::hijri::UmmAlQura> {}
// _NOT_ Hijri<S>
impl IntoFormattableAnyCalendar for cal::Japanese {}
impl IntoFormattableAnyCalendar for cal::Persian {}
impl IntoFormattableAnyCalendar for cal::Roc {}

// keep in sync with IntoFormattableAnyCalendar
#[derive(Debug, Clone, Copy, PartialEq)]
enum FormattableAnyCalendarKind {
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
    Persian,
    Roc,
}

impl FormattableAnyCalendarKind {
    fn try_from_any_calendar(cal: &AnyCalendar) -> Option<Self> {
        use AnyCalendarKind::*;
        let res = match cal.kind() {
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
            #[allow(deprecated)]
            Japanese | JapaneseExtended => Self::Japanese,
            Persian => Self::Persian,
            Roc => Self::Roc,
            _ => {
                debug_assert!(false, "cross-crate exhaustive match");
                return None;
            }
        };
        Some(res)
    }
}

#[test]
fn test_calendar_fallback() {
    use icu_locale_core::{locale, Locale};
    assert_eq!(
        FormattableAnyCalendar::try_new(locale!("en-TH-u-ca-iso8601").into()),
        FormattableAnyCalendar::try_new(locale!("und-u-ca-buddhist").into()),
    );
    assert_eq!(
        FormattableAnyCalendar::try_new(locale!("en-TH").into()),
        FormattableAnyCalendar::try_new(locale!("und-u-ca-buddhist").into()),
    );
    assert_eq!(
        FormattableAnyCalendar::try_new(locale!("en-SA-u-ca-islamic").into()),
        FormattableAnyCalendar::try_new(
            Locale::try_from_str("und-u-ca-islamic-umalqura")
                .unwrap()
                .into()
        ),
    );
    assert_eq!(
        FormattableAnyCalendar::try_new(locale!("en-IL-u-ca-islamic").into()),
        FormattableAnyCalendar::try_new(
            Locale::try_from_str("und-u-ca-islamic-civil")
                .unwrap()
                .into()
        ),
    );
}

/// A version of [`AnyCalendar`] for the calendars supported in the any-calendar formatter.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FormattableAnyCalendar {
    any_calendar: AnyCalendar,
}

impl FormattableAnyCalendar {
    pub(crate) fn from_calendar(calendar: impl IntoFormattableAnyCalendar) -> Self {
        Self {
            any_calendar: calendar.to_any(),
        }
    }

    pub(crate) fn try_from_any_calendar(any_calendar: AnyCalendar) -> Result<Self, AnyCalendar> {
        match FormattableAnyCalendarKind::try_from_any_calendar(&any_calendar) {
            Some(_) => Ok(Self { any_calendar }),
            None => Err(any_calendar),
        }
    }

    pub(crate) fn any_calendar(&self) -> &AnyCalendar {
        &self.any_calendar
    }

    fn kind(&self) -> FormattableAnyCalendarKind {
        FormattableAnyCalendarKind::try_from_any_calendar(&self.any_calendar).unwrap_or_else(|| {
            debug_assert!(false, "unreachable by invariant");
            // fall back to something non-Gregorian to make errors more obvious
            FormattableAnyCalendarKind::Coptic
        })
    }

    pub(crate) fn take_any_calendar(self) -> AnyCalendar {
        self.any_calendar
    }

    #[cfg(feature = "compiled_data")]
    pub(crate) fn try_new(prefs: CalendarPreferences) -> Result<Self, DataError> {
        Self::try_new_unstable(&icu_calendar::provider::Baked, prefs)
    }

    #[cfg(feature = "serde")]
    pub(crate) fn try_new_with_buffer_provider<P>(
        provider: &P,
        prefs: CalendarPreferences,
    ) -> Result<Self, DataError>
    where
        P: ?Sized + BufferProvider,
    {
        Self::try_new_unstable(&provider.as_deserializing(), prefs)
    }

    pub(crate) fn try_new_unstable<P>(
        provider: &P,
        mut prefs: CalendarPreferences,
    ) -> Result<Self, DataError>
    where
        P: ?Sized + DataProvider<icu_calendar::provider::CalendarJapaneseModernV1>,
    {
        use CalendarAlgorithm::*;
        let any_calendar = match prefs.resolved_algorithm() {
            Buddhist => AnyCalendar::Buddhist(cal::Buddhist),
            Chinese => AnyCalendar::Chinese(cal::ChineseTraditional::new()),
            Coptic => AnyCalendar::Coptic(cal::Coptic),
            Dangi => AnyCalendar::Dangi(cal::KoreanTraditional::new()),
            Ethiopic => AnyCalendar::Ethiopian(cal::Ethiopian::new()),
            Ethioaa => AnyCalendar::Ethiopian(cal::Ethiopian::new_with_era_style(
                cal::EthiopianEraStyle::AmeteAlem,
            )),
            Gregory => AnyCalendar::Gregorian(cal::Gregorian),
            Hebrew => AnyCalendar::Hebrew(cal::Hebrew),
            Indian => AnyCalendar::Indian(cal::Indian),
            Hijri(Some(HijriCalendarAlgorithm::Civil)) => {
                AnyCalendar::HijriTabular(cal::Hijri::new_tabular(
                    cal::hijri::TabularAlgorithmLeapYears::TypeII,
                    cal::hijri::TabularAlgorithmEpoch::Friday,
                ))
            }
            Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                AnyCalendar::HijriTabular(cal::Hijri::new_tabular(
                    cal::hijri::TabularAlgorithmLeapYears::TypeII,
                    cal::hijri::TabularAlgorithmEpoch::Thursday,
                ))
            }
            Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => {
                AnyCalendar::HijriUmmAlQura(cal::Hijri::new_umm_al_qura())
            }
            Japanese => AnyCalendar::Japanese(cal::Japanese::try_new_unstable(provider)?),
            Persian => AnyCalendar::Persian(cal::Persian),
            Roc => AnyCalendar::Roc(cal::Roc),
            Iso8601 | Hijri(_) => {
                // unsupported
                prefs.calendar_algorithm = None;
                return Self::try_new_unstable(provider, prefs);
            }
            _ => {
                // unknown
                AnyCalendar::Gregorian(cal::Gregorian)
            }
        };
        Ok(Self { any_calendar })
    }
}

pub(crate) struct FormattableAnyCalendarNamesLoader<'a, H, P> {
    provider: P,
    calendar: &'a FormattableAnyCalendar,
    _helper: PhantomData<H>,
}

impl<'a, H, P> FormattableAnyCalendarNamesLoader<'a, H, P> {
    pub(crate) fn new(provider: P, calendar: &'a FormattableAnyCalendar) -> Self {
        Self {
            provider,
            calendar,
            _helper: PhantomData,
        }
    }
}

impl<M, H, P> BoundDataProvider<M> for FormattableAnyCalendarNamesLoader<'_, H, P>
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
        match self.calendar.kind() {
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
        match self.calendar.kind() {
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
    type Buddhist = <cal::Buddhist as CldrCalendar>::YearNamesV1;
    type Chinese = <cal::ChineseTraditional as CldrCalendar>::YearNamesV1;
    type Coptic = <cal::Coptic as CldrCalendar>::YearNamesV1;
    type Dangi = <cal::KoreanTraditional as CldrCalendar>::YearNamesV1;
    type Ethiopian = <cal::Ethiopian as CldrCalendar>::YearNamesV1;
    type Gregorian = <cal::Gregorian as CldrCalendar>::YearNamesV1;
    type Hebrew = <cal::Hebrew as CldrCalendar>::YearNamesV1;
    type Indian = <cal::Indian as CldrCalendar>::YearNamesV1;
    type Hijri = <cal::Hijri<cal::hijri::UmmAlQura> as CldrCalendar>::YearNamesV1;
    type Japanese = <cal::Japanese as CldrCalendar>::YearNamesV1;
    type Persian = <cal::Persian as CldrCalendar>::YearNamesV1;
    type Roc = <cal::Roc as CldrCalendar>::YearNamesV1;
}

impl CalMarkers<MonthNamesV1> for FullDataCalMarkers {
    type Buddhist = <cal::Buddhist as CldrCalendar>::MonthNamesV1;
    type Chinese = <cal::ChineseTraditional as CldrCalendar>::MonthNamesV1;
    type Coptic = <cal::Coptic as CldrCalendar>::MonthNamesV1;
    type Dangi = <cal::KoreanTraditional as CldrCalendar>::MonthNamesV1;
    type Ethiopian = <cal::Ethiopian as CldrCalendar>::MonthNamesV1;
    type Gregorian = <cal::Gregorian as CldrCalendar>::MonthNamesV1;
    type Hebrew = <cal::Hebrew as CldrCalendar>::MonthNamesV1;
    type Indian = <cal::Indian as CldrCalendar>::MonthNamesV1;
    type Hijri = <cal::Hijri<cal::hijri::UmmAlQura> as CldrCalendar>::MonthNamesV1;
    type Japanese = <cal::Japanese as CldrCalendar>::MonthNamesV1;
    type Persian = <cal::Persian as CldrCalendar>::MonthNamesV1;
    type Roc = <cal::Roc as CldrCalendar>::MonthNamesV1;
}

impl CalMarkers<ErasedPackedPatterns> for FullDataCalMarkers {
    type Buddhist = <cal::Buddhist as CldrCalendar>::SkeletaV1;
    type Chinese = <cal::ChineseTraditional as CldrCalendar>::SkeletaV1;
    type Coptic = <cal::Coptic as CldrCalendar>::SkeletaV1;
    type Dangi = <cal::KoreanTraditional as CldrCalendar>::SkeletaV1;
    type Ethiopian = <cal::Ethiopian as CldrCalendar>::SkeletaV1;
    type Gregorian = <cal::Gregorian as CldrCalendar>::SkeletaV1;
    type Hebrew = <cal::Hebrew as CldrCalendar>::SkeletaV1;
    type Indian = <cal::Indian as CldrCalendar>::SkeletaV1;
    type Hijri = <cal::Hijri<cal::hijri::UmmAlQura> as CldrCalendar>::SkeletaV1;
    type Japanese = <cal::Japanese as CldrCalendar>::SkeletaV1;
    type Persian = <cal::Persian as CldrCalendar>::SkeletaV1;
    type Roc = <cal::Roc as CldrCalendar>::SkeletaV1;
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

#[cfg(feature = "unstable")]
impl<C, Z> InFixedCalendar<C> for ZonedTime<Z> {}

impl<C> InFixedCalendar<C> for UtcOffset {}

impl<C, O: TimeZoneModel> InFixedCalendar<C> for TimeZoneInfo<O> {}
