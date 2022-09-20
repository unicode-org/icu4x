// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;

    use core::fmt::Write;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::{AnyCalendar, AnyCalendarKind};

    use crate::errors::ffi::ICU4XError;
    use crate::locale::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;

    /// The various calendar types currently supported by [`ICU4XCalendar`]
    #[diplomat::enum_convert(AnyCalendarKind, needs_wildcard)]
    #[diplomat::rust_link(icu::calendar::AnyCalendarKind, Enum)]
    pub enum ICU4XAnyCalendarKind {
        /// The kind of an Iso calendar
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
    }

    impl ICU4XAnyCalendarKind {
        /// Read the calendar type off of the -u-ca- extension on a locale
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::from_locale, FnInEnum)]
        pub fn from_locale(
            locale: &ICU4XLocale,
        ) -> DiplomatResult<ICU4XAnyCalendarKind, ICU4XError> {
            AnyCalendarKind::from_locale(&locale.0)
                .map(Into::into)
                .map_err(ICU4XError::from)
                .into()
        }

        /// Obtain the calendar type given a BCP-47 -u-ca- extension string
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::from_bcp47, FnInEnum)]
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::from_bcp47_string, FnInEnum, hidden)]
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::from_bcp47_bytes, FnInEnum, hidden)]
        pub fn from_bcp47(s: &str) -> DiplomatResult<ICU4XAnyCalendarKind, ICU4XError> {
            let s = s.as_bytes(); // #2520
            AnyCalendarKind::from_bcp47_bytes(s)
                .map(Into::into)
                .ok_or(ICU4XError::DateTimeUnknownAnyCalendarKindError)
                .into()
        }

        /// Obtain the string suitable for use in the -u-ca- extension in a BCP47 locale
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::as_bcp47_string, FnInEnum)]
        #[diplomat::rust_link(icu::calendar::AnyCalendarKind::as_bcp47_value, FnInEnum, hidden)]
        pub fn bcp47(
            self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let kind = AnyCalendarKind::from(self);
            let result = write
                .write_str(kind.as_bcp47_string())
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    #[diplomat::rust_link(icu::calendar::AnyCalendar, Enum)]
    pub struct ICU4XCalendar(pub Arc<AnyCalendar>);

    impl ICU4XCalendar {
        /// Creates a new [`ICU4XCalendar`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::try_new_for_locale_unstable, FnInEnum)]
        pub fn try_new_for_locale(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> DiplomatResult<Box<ICU4XCalendar>, ICU4XError> {
            let locale = locale.to_datalocale();

            AnyCalendar::try_new_for_locale_unstable(&provider.0, &locale)
                .map(|df| Box::new(ICU4XCalendar(Arc::new(df))))
                .map_err(Into::into)
                .into()
        }

        /// Creates a new [`ICU4XCalendar`] from the specified date and time.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::try_new_unstable, FnInEnum)]
        pub fn try_new_for_kind(
            provider: &ICU4XDataProvider,
            kind: ICU4XAnyCalendarKind,
        ) -> DiplomatResult<Box<ICU4XCalendar>, ICU4XError> {
            AnyCalendar::try_new_unstable(&provider.0, kind.into())
                .map(|df| Box::new(ICU4XCalendar(Arc::new(df))))
                .map_err(Into::into)
                .into()
        }

        /// Returns the kind of this calendar
        #[diplomat::rust_link(icu::calendar::AnyCalendar::kind, FnInEnum)]
        pub fn kind(&self) -> ICU4XAnyCalendarKind {
            self.0.kind().into()
        }
    }
}
