// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use super::CalendarInner;
    use alloc::boxed::Box;
    use icu_calendar::preferences::CalendarPreferences;

    #[cfg(feature = "buffer_provider")]
    use crate::unstable::errors::ffi::DataError;
    use crate::unstable::locale_core::ffi::Locale;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;

    /// The various calendar types currently supported by [`Calendar`]
    #[diplomat::enum_convert(icu_calendar::AnyCalendarKind, needs_wildcard)]
    #[diplomat::rust_link(icu::calendar::AnyCalendarKind, Enum)]
    #[non_exhaustive]
    pub enum CalendarKind {
        /// The kind of an Iso calendar
        // AnyCalendarKind in Rust doesn't have a default, but it is useful to have one
        // here for consistent behavior.
        #[diplomat::attr(auto, default)]
        Iso = 0,
        /// The kind of a Gregorian calendar
        Gregorian = 1,
        /// The kind of a Buddhist calendar
        Buddhist = 2,
        /// The kind of a Japanese calendar with modern eras
        Japanese = 3,
        /// The kind of a Japanese calendar with modern and historic eras
        JapaneseExtended = 4,
        /// The kind of an Ethiopian calendar, with Amete Mihret era
        Ethiopian = 5,
        /// The kind of an Ethiopian calendar, with Amete Alem era
        EthiopianAmeteAlem = 6,
        /// The kind of a Indian calendar
        Indian = 7,
        /// The kind of a Coptic calendar
        Coptic = 8,
        /// The kind of a Dangi calendar
        Dangi = 9,
        /// The kind of a Chinese calendar
        Chinese = 10,
        /// The kind of a Hebrew calendar
        Hebrew = 11,
        /// The kind of a Hijri tabular, type II leap years, Friday epoch, calendar
        HijriTabularTypeIIFriday = 12,
        /// The kind of a Hijri simulated, Mecca calendar
        #[deprecated]
        HijriSimulatedMecca = 18,
        /// The kind of a Hijri tabular, type II leap years, Thursday epoch, calendar
        HijriTabularTypeIIThursday = 14,
        /// The kind of a Hijri Umm al-Qura calendar
        HijriUmmAlQura = 15,
        /// The kind of a Persian calendar
        Persian = 16,
        /// The kind of a Roc calendar
        Roc = 17,
    }

    impl CalendarKind {
        /// Creates a new [`CalendarKind`] for the specified locale, using compiled data.
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::new, FnInEnum)]
        pub fn create(locale: &Locale) -> Self {
            use icu_calendar::preferences::{CalendarAlgorithm, HijriCalendarAlgorithm};
            let prefs = CalendarPreferences::from(&locale.0);

            match prefs.resolved_algorithm() {
                CalendarAlgorithm::Buddhist => Self::Buddhist,
                CalendarAlgorithm::Iso8601 => Self::Iso,
                CalendarAlgorithm::Gregory => Self::Gregorian,
                CalendarAlgorithm::Japanese => Self::Japanese,
                CalendarAlgorithm::Ethiopic => Self::Ethiopian,
                CalendarAlgorithm::Ethioaa => Self::EthiopianAmeteAlem,
                CalendarAlgorithm::Indian => Self::Indian,
                CalendarAlgorithm::Coptic => Self::Coptic,
                CalendarAlgorithm::Dangi => Self::Dangi,
                CalendarAlgorithm::Chinese => Self::Chinese,
                CalendarAlgorithm::Hebrew => Self::Hebrew,
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura)) => {
                    Self::HijriUmmAlQura
                }
                CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Tbla)) => {
                    Self::HijriTabularTypeIIThursday
                }
                CalendarAlgorithm::Hijri(_) => Self::HijriTabularTypeIIFriday,
                CalendarAlgorithm::Persian => Self::Persian,
                CalendarAlgorithm::Roc => Self::Roc,
                _ => Self::Gregorian,
            }
        }
    }

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    #[diplomat::rust_link(icu::calendar::AnyCalendar, Enum)]
    pub struct Calendar(pub CalendarInner);

    impl Calendar {
        /// Creates a new [`Calendar`] for the specified kind, using compiled data.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::new, FnInEnum)]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create(kind: CalendarKind) -> Box<Calendar> {
            Box::new(Calendar(match kind {
                CalendarKind::Iso => CalendarInner::Iso(icu_calendar::cal::Iso),
                CalendarKind::Gregorian => CalendarInner::Gregorian(icu_calendar::cal::Gregorian),
                CalendarKind::Buddhist => CalendarInner::Buddhist(icu_calendar::cal::Buddhist),
                CalendarKind::Japanese => CalendarInner::Japanese(alloc::sync::Arc::new(
                    icu_calendar::cal::Japanese::new(),
                )),
                CalendarKind::JapaneseExtended => CalendarInner::JapaneseExtended(
                    alloc::sync::Arc::new(icu_calendar::cal::JapaneseExtended::new()),
                ),
                CalendarKind::Ethiopian => {
                    CalendarInner::Ethiopian(icu_calendar::cal::Ethiopian::new())
                }
                CalendarKind::EthiopianAmeteAlem => {
                    CalendarInner::Ethiopian(icu_calendar::cal::Ethiopian::new_with_era_style(
                        icu_calendar::cal::EthiopianEraStyle::AmeteAlem,
                    ))
                }
                CalendarKind::Indian => CalendarInner::Indian(icu_calendar::cal::Indian),
                CalendarKind::Coptic => CalendarInner::Coptic(icu_calendar::cal::Coptic),
                CalendarKind::Dangi => {
                    CalendarInner::Dangi(icu_calendar::cal::KoreanTraditional::new())
                }
                CalendarKind::Chinese => {
                    CalendarInner::Chinese(icu_calendar::cal::ChineseTraditional::new())
                }
                CalendarKind::Hebrew => CalendarInner::Hebrew(icu_calendar::cal::Hebrew),
                #[allow(deprecated)]
                CalendarKind::HijriTabularTypeIIFriday | CalendarKind::HijriSimulatedMecca => {
                    CalendarInner::HijriTabular(icu_calendar::cal::Hijri::new_tabular(
                        icu_calendar::cal::HijriTabularLeapYears::TypeII,
                        icu_calendar::cal::HijriTabularEpoch::Friday,
                    ))
                }
                CalendarKind::HijriTabularTypeIIThursday => {
                    CalendarInner::HijriTabular(icu_calendar::cal::Hijri::new_tabular(
                        icu_calendar::cal::HijriTabularLeapYears::TypeII,
                        icu_calendar::cal::HijriTabularEpoch::Thursday,
                    ))
                }
                CalendarKind::HijriUmmAlQura => {
                    CalendarInner::HijriUmmAlQura(icu_calendar::cal::Hijri::new_umm_al_qura())
                }
                CalendarKind::Persian => CalendarInner::Persian(icu_calendar::cal::Persian),
                CalendarKind::Roc => CalendarInner::Roc(icu_calendar::cal::Roc),
            }))
        }

        /// Creates a new [`Calendar`] for the specified kind, using a particular data source.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::new, FnInEnum)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "new_with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
            kind: CalendarKind,
        ) -> Result<Box<Calendar>, DataError> {
            Ok(Box::new(Calendar(match kind {
                CalendarKind::Iso => CalendarInner::Iso(icu_calendar::cal::Iso),
                CalendarKind::Gregorian => CalendarInner::Gregorian(icu_calendar::cal::Gregorian),
                CalendarKind::Buddhist => CalendarInner::Buddhist(icu_calendar::cal::Buddhist),
                CalendarKind::Japanese => CalendarInner::Japanese(alloc::sync::Arc::new(
                    icu_calendar::cal::Japanese::try_new_with_buffer_provider(provider.get()?)?,
                )),
                CalendarKind::JapaneseExtended => {
                    CalendarInner::JapaneseExtended(alloc::sync::Arc::new(
                        icu_calendar::cal::JapaneseExtended::try_new_with_buffer_provider(
                            provider.get()?,
                        )?,
                    ))
                }
                CalendarKind::Ethiopian => {
                    CalendarInner::Ethiopian(icu_calendar::cal::Ethiopian::new())
                }
                CalendarKind::EthiopianAmeteAlem => {
                    CalendarInner::Ethiopian(icu_calendar::cal::Ethiopian::new_with_era_style(
                        icu_calendar::cal::EthiopianEraStyle::AmeteAlem,
                    ))
                }
                CalendarKind::Indian => CalendarInner::Indian(icu_calendar::cal::Indian),
                CalendarKind::Coptic => CalendarInner::Coptic(icu_calendar::cal::Coptic),
                CalendarKind::Dangi => {
                    CalendarInner::Dangi(icu_calendar::cal::KoreanTraditional::new())
                }
                CalendarKind::Chinese => {
                    CalendarInner::Chinese(icu_calendar::cal::ChineseTraditional::new())
                }
                CalendarKind::Hebrew => CalendarInner::Hebrew(icu_calendar::cal::Hebrew),
                #[allow(deprecated)]
                CalendarKind::HijriTabularTypeIIFriday | CalendarKind::HijriSimulatedMecca => {
                    CalendarInner::HijriTabular(icu_calendar::cal::Hijri::new_tabular(
                        icu_calendar::cal::HijriTabularLeapYears::TypeII,
                        icu_calendar::cal::HijriTabularEpoch::Friday,
                    ))
                }
                CalendarKind::HijriTabularTypeIIThursday => {
                    CalendarInner::HijriTabular(icu_calendar::cal::Hijri::new_tabular(
                        icu_calendar::cal::HijriTabularLeapYears::TypeII,
                        icu_calendar::cal::HijriTabularEpoch::Thursday,
                    ))
                }
                CalendarKind::HijriUmmAlQura => {
                    CalendarInner::HijriUmmAlQura(icu_calendar::cal::Hijri::new_umm_al_qura())
                }
                CalendarKind::Persian => CalendarInner::Persian(icu_calendar::cal::Persian),
                CalendarKind::Roc => CalendarInner::Roc(icu_calendar::cal::Roc),
            })))
        }

        /// Returns the kind of this calendar
        #[diplomat::rust_link(icu::calendar::AnyCalendar::kind, FnInEnum)]
        #[diplomat::attr(auto, getter)]
        #[diplomat::attr(demo_gen, disable)] // this just returns the single constructor argument
        pub fn kind(&self) -> CalendarKind {
            use icu_calendar::IntoAnyCalendar;
            self.0.kind().into()
        }
    }
}

icu_calendar::make_any_calendar!(
    #[non_exhaustive]
    CalendarInner,
    #[non_exhaustive]
    CalendarInnerDateInner,
    #[cfg(all())],
    Buddhist(icu_calendar::cal::Buddhist),
    Chinese(icu_calendar::cal::ChineseTraditional),
    Coptic(icu_calendar::cal::Coptic),
    Dangi(icu_calendar::cal::KoreanTraditional),
    Ethiopian(icu_calendar::cal::Ethiopian),
    Gregorian(icu_calendar::cal::Gregorian),
    Hebrew(icu_calendar::cal::Hebrew),
    HijriTabular(icu_calendar::cal::Hijri<icu_calendar::cal::hijri::TabularAlgorithm>),
    HijriUmmAlQura(icu_calendar::cal::Hijri<icu_calendar::cal::hijri::UmmAlQura>),
    Indian(icu_calendar::cal::Indian),
    Iso(icu_calendar::cal::Iso),
    Japanese(alloc::sync::Arc<icu_calendar::cal::Japanese>),
    JapaneseExtended(alloc::sync::Arc<icu_calendar::cal::JapaneseExtended>),
    Persian(icu_calendar::cal::Persian),
    Roc(icu_calendar::cal::Roc),
);

use icu_calendar::IntoAnyCalendar;

// For when we need to get the `AnyCalendarKind`
impl IntoAnyCalendar for CalendarInner {
    fn kind(&self) -> icu_calendar::AnyCalendarKind {
        match self {
            CalendarInner::Buddhist(c) => c.kind(),
            CalendarInner::Chinese(c) => c.kind(),
            CalendarInner::Coptic(c) => c.kind(),
            CalendarInner::Dangi(c) => c.kind(),
            CalendarInner::Ethiopian(c) => c.kind(),
            CalendarInner::Gregorian(c) => c.kind(),
            CalendarInner::Hebrew(c) => c.kind(),
            CalendarInner::HijriTabular(c) => c.kind(),
            CalendarInner::HijriUmmAlQura(c) => c.kind(),
            CalendarInner::Indian(c) => c.kind(),
            CalendarInner::Iso(c) => c.kind(),
            CalendarInner::Japanese(c) => c.kind(),
            CalendarInner::JapaneseExtended(c) => c.kind(),
            CalendarInner::Persian(c) => c.kind(),
            CalendarInner::Roc(c) => c.kind(),
        }
    }

    fn date_to_any(
        &self,
        _: &Self::DateInner,
    ) -> <icu_calendar::AnyCalendar as icu_calendar::Calendar>::DateInner {
        unreachable!()
    }

    fn from_any(_: icu_calendar::AnyCalendar) -> Result<Self, icu_calendar::AnyCalendar> {
        unreachable!()
    }

    fn from_any_ref(_: &icu_calendar::AnyCalendar) -> Option<&Self> {
        unreachable!()
    }

    fn to_any(self) -> icu_calendar::AnyCalendar {
        unreachable!()
    }
}
