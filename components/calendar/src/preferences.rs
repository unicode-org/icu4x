// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Locale preferences used by this crate

#[doc(inline)]
/// **This is a reexport of a type in [`icu::locale`](icu_locale_core::preferences::extensions::unicode::keywords)**.
#[doc = "\n"] // prevent autoformatting
pub use icu_locale_core::preferences::extensions::unicode::keywords::CalendarAlgorithm;
/// **This is a reexport of a type in [`icu::locale`](icu_locale_core::preferences::extensions::unicode::keywords)**.
#[doc = "\n"] // prevent autoformatting
pub use icu_locale_core::preferences::extensions::unicode::keywords::FirstDay;
#[doc(inline)]
/// **This is a reexport of a type in [`icu::locale`](icu_locale_core::preferences::extensions::unicode::keywords)**.
#[doc = "\n"] // prevent autoformatting
pub use icu_locale_core::preferences::extensions::unicode::keywords::HijriCalendarAlgorithm;

use icu_locale_core::preferences::define_preferences;

define_preferences!(
    /// The preferences for calendars formatting.
    [Copy]
    CalendarPreferences,
    {
        /// The user's preferred calendar system.
        calendar_algorithm: CalendarAlgorithm
    }
);

define_preferences!(
    /// The preferences for the week information.
    [Copy]
    WeekPreferences,
    {
        /// The first day of the week
        first_weekday: FirstDay
    }
);

impl CalendarPreferences {
    /// Selects the [`CalendarAlgorithm`] appropriate for the given [`CalendarPreferences`].
    ///
    /// # Example
    ///
    /// ```
    /// use icu::calendar::preferences::{CalendarAlgorithm, CalendarPreferences};
    /// use icu::locale::locale;
    ///
    /// assert_eq!(CalendarPreferences::from(&locale!("und")).resolved_algorithm(), CalendarAlgorithm::Gregory);
    /// assert_eq!(CalendarPreferences::from(&locale!("und-US-u-ca-hebrew")).resolved_algorithm(), CalendarAlgorithm::Hebrew);
    /// assert_eq!(CalendarPreferences::from(&locale!("und-AF")).resolved_algorithm(), CalendarAlgorithm::Persian);
    /// assert_eq!(CalendarPreferences::from(&locale!("und-US-u-rg-thxxxx")).resolved_algorithm(), CalendarAlgorithm::Buddhist);
    /// ```
    pub fn resolved_algorithm(self) -> CalendarAlgorithm {
        let region = self
            .locale_preferences
            .to_data_locale_region_priority()
            .region;
        let region = region.as_ref().map(|r| r.as_str());
        // This is tested to be consistent with CLDR in icu_provider_source::calendar::test_calendar_resolution
        match self.calendar_algorithm {
            Some(CalendarAlgorithm::Hijri(None)) => match region {
                Some("AE" | "BH" | "KW" | "QA" | "SA") => {
                    CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Umalqura))
                }
                _ => CalendarAlgorithm::Hijri(Some(HijriCalendarAlgorithm::Civil)),
            },
            Some(a) => a,
            None => match region {
                Some("TH") => CalendarAlgorithm::Buddhist,
                Some("AF" | "IR") => CalendarAlgorithm::Persian,
                _ => CalendarAlgorithm::Gregory,
            },
        }
    }
}
