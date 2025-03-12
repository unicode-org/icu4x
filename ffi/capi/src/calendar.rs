// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::sync::Arc;

    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::errors::ffi::DataError;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::locale_core::ffi::Locale;
    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;

    /// The various calendar types currently supported by [`Calendar`]
    #[diplomat::enum_convert(icu_calendar::AnyCalendarKind, needs_wildcard)]
    #[diplomat::rust_link(icu::calendar::AnyCalendarKind, Enum)]
    pub enum AnyCalendarKind {
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
        /// The kind of a Dangi calendar
        Dangi = 9,
        /// The kind of a Chinese calendar
        Chinese = 10,
        /// The kind of a Hebrew calendar
        Hebrew = 11,
        /// The kind of a Hijri civil calendar
        HijriCivil = 12,
        /// The kind of a Hijri observational calendar
        HijriObservational = 13,
        /// The kind of a Hijri tabular calendar
        HijriTabular = 14,
        /// The kind of a Hijri Umm al-Qura calendar
        HijriUmmAlQura = 15,
        /// The kind of a Persian calendar
        Persian = 16,
        /// The kind of a Roc calendar
        Roc = 17,
    }

    #[diplomat::opaque]
    #[diplomat::transparent_convert]
    #[diplomat::rust_link(icu::calendar::AnyCalendar, Enum)]
    pub struct Calendar(pub Arc<icu_calendar::AnyCalendar>);

    impl Calendar {
        /// Creates a new [`Calendar`] from the specified date and time, using compiled data.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::try_new, FnInEnum)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "for_locale")]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_for_locale(locale: &Locale) -> Result<Box<Calendar>, DataError> {
            let prefs = (&locale.0).into();
            Ok(Box::new(Calendar(Arc::new(
                icu_calendar::AnyCalendar::try_new(prefs)?,
            ))))
        }

        /// Creates a new [`Calendar`] from the specified date and time, using compiled data.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::new_for_kind, FnInEnum)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "for_kind")]
        #[cfg(feature = "compiled_data")]
        pub fn create_for_kind(kind: AnyCalendarKind) -> Result<Box<Calendar>, DataError> {
            Ok(Box::new(Calendar(Arc::new(
                icu_calendar::AnyCalendar::new_for_kind(kind.into()),
            ))))
        }

        /// Creates a new [`Calendar`] from the specified date and time, using a particular data source.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::try_new, FnInEnum)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "for_locale_with_provider")]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "buffer_provider")]
        pub fn create_for_locale_with_provider(
            provider: &DataProvider,
            locale: &Locale,
        ) -> Result<Box<Calendar>, DataError> {
            let prefs = (&locale.0).into();

            Ok(Box::new(Calendar(Arc::new(
                icu_calendar::AnyCalendar::try_new_with_buffer_provider(provider.get()?, prefs)?,
            ))))
        }

        /// Creates a new [`Calendar`] from the specified date and time, using a particular data source.
        #[diplomat::rust_link(icu::calendar::AnyCalendar::new_for_kind, FnInEnum)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "for_kind_with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_for_kind_with_provider(
            provider: &DataProvider,
            kind: AnyCalendarKind,
        ) -> Result<Box<Calendar>, DataError> {
            Ok(Box::new(Calendar(Arc::new(
                icu_calendar::AnyCalendar::try_new_for_kind_with_buffer_provider(
                    provider.get()?,
                    kind.into(),
                )?,
            ))))
        }

        /// Returns the kind of this calendar
        #[diplomat::rust_link(icu::calendar::AnyCalendar::kind, FnInEnum)]
        #[diplomat::attr(auto, getter)]
        pub fn kind(&self) -> AnyCalendarKind {
            self.0.kind().into()
        }
    }
}
