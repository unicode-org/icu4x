// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::AnyCalendar;
use crate::AnyCalendarKind;
use crate::Date;
use crate::Iso;
use icu_provider::prelude::*;

/// A date in the ISO calendar tagged with an optional [`AnyCalendarKind`].
///
/// This is a useful intermediate type when the calendar is known but you wish
/// to defer data loading and conversion into the calendar.
///
/// This is the return type from parsing an IXDTF string.
///
/// # Examples
///
/// ```
/// use icu::calendar::AnyCalendarKind;
/// use icu::calendar::Date;
/// use icu::calendar::TaggedDate;
/// use icu::locale::locale;
///
/// let preferred_calendar = AnyCalendarKind::get_for_locale(&locale!("und-u-ca-chinese"));
///
/// // Create a TaggedDate without loading data
/// let date = TaggedDate::from_iso_date_and_kind(
///		Date::try_new_iso(2024, 10, 28).unwrap(),
///		preferred_calendar,
/// );
///
/// // Later, convert it to a Date<AnyCalendar>
/// let any_date = date.to_any_date();
/// assert_eq!(any_date.year().era_year_or_extended(), 4661);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TaggedDate {
    iso: Date<Iso>,
    kind: Option<AnyCalendarKind>,
}

impl TaggedDate {
    /// Creates a new [`TaggedDate`].
    pub fn from_iso_date_and_kind(iso: Date<Iso>, kind: Option<AnyCalendarKind>) -> Self {
        Self { iso, kind }
    }

    /// Converts this date to an ISO date, dropping the tag, if any.
    pub fn to_iso_date(self) -> Date<Iso> {
        self.iso
    }

    /// Converts this date to a calendared date with compiled data.
    #[cfg(feature = "compiled_data")]
    pub fn to_any_date(self) -> Date<AnyCalendar> {
        if let Some(kind) = self.kind {
            let calendar = AnyCalendar::new(kind);
            Date::new_from_iso(self.iso, calendar)
        } else {
            self.iso.to_any()
        }
    }

    /// Converts this date to a calendared date, loading data from a buffer provider.
    #[cfg(feature = "serde")]
    pub fn to_any_date_with_buffer_provider<P>(
        self,
        provider: &P,
    ) -> Result<Date<AnyCalendar>, DataError>
    where
        P: BufferProvider + ?Sized,
    {
        if let Some(kind) = self.kind {
            let calendar = AnyCalendar::try_new_with_buffer_provider(provider, kind)?;
            Ok(Date::new_from_iso(self.iso, calendar))
        } else {
            Ok(self.iso.to_any())
        }
    }

    /// Converts this date to a calendared date, loading data from a data provider.
    pub fn to_any_date_unstable<P>(self, provider: &P) -> Result<Date<AnyCalendar>, DataError>
    where
        P: DataProvider<crate::provider::JapaneseErasV1Marker>
            + DataProvider<crate::provider::JapaneseExtendedErasV1Marker>
            + DataProvider<crate::provider::ChineseCacheV1Marker>
            + DataProvider<crate::provider::DangiCacheV1Marker>
            + DataProvider<crate::provider::IslamicObservationalCacheV1Marker>
            + DataProvider<crate::provider::IslamicUmmAlQuraCacheV1Marker>
            + ?Sized,
    {
        if let Some(kind) = self.kind {
            let calendar = AnyCalendar::try_new_unstable(provider, kind)?;
            Ok(Date::new_from_iso(self.iso, calendar))
        } else {
            Ok(self.iso.to_any())
        }
    }
}
