// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cal::abstract_gregorian::{impl_with_abstract_gregorian, GregorianYears};
use crate::cal::iso::IsoEra;
use crate::calendar_arithmetic::ArithmeticDate;
use crate::error::DateError;
use crate::provider::{
    CalendarJapaneseExtendedV1, CalendarJapaneseModernV1, EraStartDate, JapaneseEras,
};
use crate::{types, AsCalendar, Date};
use icu_provider::prelude::*;
use tinystr::{tinystr, TinyStr16};

/// The [Japanese Calendar] (with modern eras only)
///
/// The [Japanese calendar] is a solar calendar used in Japan, with twelve months.
/// The months and days are identical to that of the Gregorian calendar, however the years are counted
/// differently using the Japanese era system.
///
/// This calendar only contains eras after Meiji, for all historical eras, check out [`JapaneseExtended`].
///
/// This type can be used with [`Date`] to represent dates in this calendar.
///
/// [Japanese calendar]: https://en.wikipedia.org/wiki/Japanese_calendar
///
/// # Era codes
///
/// This calendar currently supports seven era codes. It supports the five post-Meiji eras
/// (`meiji`, `taisho`, `showa`, `heisei`, `reiwa`), as well as using the Gregorian
/// `bce` (alias `bc`), and `ce` (alias `ad`) for dates before the Meiji era.
///
/// Future eras will also be added to this type when they are decided.
///
/// These eras are loaded from data, requiring a data provider capable of providing [`CalendarJapaneseModernV1`]
/// data.
///
/// # Month codes
///
/// This calendar supports 12 solar month codes (`M01` - `M12`)
#[derive(Clone, Debug, Default)]
pub struct Japanese {
    eras: DataPayload<CalendarJapaneseModernV1>,
}

/// The [Japanese Calendar] (with historical eras)
///
/// The [Japanese calendar] is a solar calendar used in Japan, with twelve months.
/// The months and days are identical to that of the Gregorian calendar, however the years are counted
/// differently using the Japanese era system.
///
/// This type can be used with [`Date`] to represent dates in this calendar.
///
/// [Japanese calendar]: https://en.wikipedia.org/wiki/Japanese_calendar
///
/// # Era codes
///
/// This calendar supports a large number of era codes. It supports the five post-Meiji eras
/// (`meiji`, `taisho`, `showa`, `heisei`, `reiwa`). Pre-Meiji eras are represented
/// with their names converted to lowercase ascii and followed by their start year. E.g. the *Ten'ō*
/// era (781 - 782 CE) has the code `teno-781`. The  Gregorian `bce` (alias `bc`), and `ce` (alias `ad`)
/// are used for dates before the first known era era.
///
///
/// These eras are loaded from data, requiring a data provider capable of providing [`CalendarJapaneseExtendedV1`]
/// data.
///
/// # Month codes
///
/// This calendar supports 12 solar month codes (`M01` - `M12`)
#[derive(Clone, Debug, Default)]
pub struct JapaneseExtended(Japanese);

impl Japanese {
    /// Creates a new [`Japanese`] using only modern eras (post-meiji) from compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self {
            eras: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_JAPANESE_MODERN_V1,
            ),
        }
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
    ]);

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<D: DataProvider<CalendarJapaneseModernV1> + ?Sized>(
        provider: &D,
    ) -> Result<Self, DataError> {
        Ok(Self {
            eras: provider.load(Default::default())?.payload,
        })
    }

    pub(crate) const DEBUG_NAME: &'static str = "Japanese";
}

impl JapaneseExtended {
    /// Creates a new [`Japanese`] from using all eras (including pre-meiji) from compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self(Japanese {
            eras: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_CALENDAR_JAPANESE_EXTENDED_V1,
            ),
        })
    }

    icu_provider::gen_buffer_data_constructors!(() -> error: DataError,
        functions: [
            new: skip,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
    ]);

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<D: DataProvider<CalendarJapaneseExtendedV1> + ?Sized>(
        provider: &D,
    ) -> Result<Self, DataError> {
        Ok(Self(Japanese {
            eras: provider.load(Default::default())?.payload.cast(),
        }))
    }

    pub(crate) const DEBUG_NAME: &'static str = "Japanese (historical era data)";
}

impl GregorianYears for &'_ Japanese {
    fn extended_from_era_year(&self, era: Option<&str>, year: i32) -> Result<i32, DateError> {
        let era = match era {
            Some("ce" | "ad") | None => {
                return Ok(year);
            }
            Some("bce" | "bc") => {
                return Ok(1 - year);
            }
            Some(e) => e.parse().map_err(|_| DateError::UnknownEra)?,
        };
        let era_start = self.eras.get().japanese_era_start(era)?;
        Ok(era_start.year + year - 1)
    }

    fn era_year_from_extended(&self, extended_year: i32, month: u8, day: u8) -> types::EraYear {
        let (year, era) = self.eras.get().adjusted_year_for(extended_year, month, day);
        types::EraYear {
            era,
            era_index: None,
            year,
            extended_year,
            ambiguity: types::YearAmbiguity::CenturyRequired,
        }
    }

    fn debug_name(&self) -> &'static str {
        if self.eras.get().dates_to_eras.len() > 10 {
            "Japanese  (historical era data)"
        } else {
            "Japanese"
        }
    }

    fn calendar_algorithm(&self) -> Option<crate::preferences::CalendarAlgorithm> {
        if self.eras.get().dates_to_eras.len() > 10 {
            None
        } else {
            Some(crate::preferences::CalendarAlgorithm::Japanese)
        }
    }
}

impl_with_abstract_gregorian!(Japanese, JapaneseDateInner, Japanese, this, this);

impl_with_abstract_gregorian!(
    JapaneseExtended,
    JapaneseExtendedDateInner,
    Japanese,
    this,
    &this.0
);

impl Date<Japanese> {
    /// Construct a new Japanese Date.
    ///
    /// Years are specified in the era provided, and must be in range for Japanese
    /// eras (e.g. dates past April 30 Heisei 31 must be in Reiwa; "Jun 5 Heisei 31" and "Jan 1 Heisei 32"
    /// will not be adjusted to being in Reiwa 1 and 2 respectively)
    ///
    /// However, dates may always be specified in "bce" or "ce" and they will be adjusted as necessary.
    ///
    /// ```rust
    /// use icu::calendar::cal::Japanese;
    /// use icu::calendar::{Date, Ref};
    /// use tinystr::tinystr;
    ///
    /// let japanese_calendar = Japanese::new();
    /// // for easy sharing
    /// let japanese_calendar = Ref(&japanese_calendar);
    ///
    /// let era = "heisei";
    ///
    /// let date =
    ///     Date::try_new_japanese_with_calendar(era, 14, 1, 2, japanese_calendar)
    ///         .expect("Constructing a date should succeed");
    ///
    /// assert_eq!(date.era_year().era, era);
    /// assert_eq!(date.era_year().year, 14);
    /// assert_eq!(date.month().ordinal, 1);
    /// assert_eq!(date.day_of_month().0, 2);
    ///
    /// // This function will error for eras that are out of bounds:
    /// // (Heisei was 32 years long, Heisei 33 is in Reiwa)
    /// let oob_date =
    ///     Date::try_new_japanese_with_calendar(era, 33, 1, 2, japanese_calendar);
    /// assert!(oob_date.is_err());
    ///
    /// // and for unknown eras
    /// let fake_era = "neko"; // 🐱
    /// let fake_date = Date::try_new_japanese_with_calendar(
    ///     fake_era,
    ///     10,
    ///     1,
    ///     2,
    ///     japanese_calendar,
    /// );
    /// assert!(fake_date.is_err());
    /// ```
    pub fn try_new_japanese_with_calendar<A: AsCalendar<Calendar = Japanese>>(
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        japanese_calendar: A,
    ) -> Result<Date<A>, DateError> {
        let inner = japanese_calendar
            .as_calendar()
            .eras
            .get()
            .new_japanese_date_inner(Some(era), year, month, day)?;
        Ok(Date::from_raw(inner, japanese_calendar))
    }
}

impl Date<JapaneseExtended> {
    /// Construct a new Japanese Date with all eras.
    ///
    /// Years are specified in the era provided, and must be in range for Japanese
    /// eras (e.g. dates past April 30 Heisei 31 must be in Reiwa; "Jun 5 Heisei 31" and "Jan 1 Heisei 32"
    /// will not be adjusted to being in Reiwa 1 and 2 respectively)
    ///
    /// However, dates may always be specified in "bce" or "ce" and they will be adjusted as necessary.
    ///
    /// ```rust
    /// use icu::calendar::cal::JapaneseExtended;
    /// use icu::calendar::{Date, Ref};
    /// use tinystr::tinystr;
    ///
    /// let japanext_calendar = JapaneseExtended::new();
    /// // for easy sharing
    /// let japanext_calendar = Ref(&japanext_calendar);
    ///
    /// let era = "kansei-1789";
    ///
    /// let date = Date::try_new_japanese_extended_with_calendar(
    ///     era,
    ///     7,
    ///     1,
    ///     2,
    ///     japanext_calendar,
    /// )
    /// .expect("Constructing a date should succeed");
    ///
    /// assert_eq!(date.era_year().era, era);
    /// assert_eq!(date.era_year().year, 7);
    /// assert_eq!(date.month().ordinal, 1);
    /// assert_eq!(date.day_of_month().0, 2);
    /// ```
    pub fn try_new_japanese_extended_with_calendar<A: AsCalendar<Calendar = JapaneseExtended>>(
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        japanext_calendar: A,
    ) -> Result<Date<A>, DateError> {
        let inner = japanext_calendar
            .as_calendar()
            .0
            .eras
            .get()
            .new_japanese_date_inner(Some(era), year, month, day)?;

        Ok(Date::from_raw(
            JapaneseExtendedDateInner(inner.0),
            japanext_calendar,
        ))
    }
}

const MEIJI_START: EraStartDate = EraStartDate {
    year: 1868,
    month: 10,
    day: 23,
};
const TAISHO_START: EraStartDate = EraStartDate {
    year: 1912,
    month: 7,
    day: 30,
};
const SHOWA_START: EraStartDate = EraStartDate {
    year: 1926,
    month: 12,
    day: 25,
};
const HEISEI_START: EraStartDate = EraStartDate {
    year: 1989,
    month: 1,
    day: 8,
};
const REIWA_START: EraStartDate = EraStartDate {
    year: 2019,
    month: 5,
    day: 1,
};

impl JapaneseEras<'_> {
    /// Given an ISO date, give year and era for that date in the Japanese calendar
    ///
    /// This will also use Gregorian eras for eras that are before the earliest era
    fn adjusted_year_for(&self, year: i32, month: u8, day: u8) -> (i32, TinyStr16) {
        let date: EraStartDate = EraStartDate { year, month, day };
        let (start, era) = self.japanese_era_for(date);
        // The year in which an era starts is Year 1, and it may be short
        // The only time this function will experience dates that are *before*
        // the era start date are for the first era (Currently, taika-645
        // for japanext, meiji for japanese),
        // In such a case, we instead fall back to Gregorian era codes
        if date < start {
            if date.year <= 0 {
                (1 - date.year, tinystr!(16, "bce"))
            } else {
                (date.year, tinystr!(16, "ce"))
            }
        } else {
            (date.year - start.year + 1, era)
        }
    }

    /// Given an date, obtain the era data (not counting spliced gregorian eras)
    fn japanese_era_for(&self, date: EraStartDate) -> (EraStartDate, TinyStr16) {
        // We optimize for the five "modern" post-Meiji eras, which are stored in a smaller
        // array and also hardcoded. The hardcoded version is not used if data indicates the
        // presence of newer eras.
        if date >= MEIJI_START
            && self.dates_to_eras.last().map(|x| x.1) == Some(tinystr!(16, "reiwa"))
        {
            // Fast path in case eras have not changed since this code was written
            return if date >= REIWA_START {
                (REIWA_START, tinystr!(16, "reiwa"))
            } else if date >= HEISEI_START {
                (HEISEI_START, tinystr!(16, "heisei"))
            } else if date >= SHOWA_START {
                (SHOWA_START, tinystr!(16, "showa"))
            } else if date >= TAISHO_START {
                (TAISHO_START, tinystr!(16, "taisho"))
            } else {
                (MEIJI_START, tinystr!(16, "meiji"))
            };
        }
        let data = &self.dates_to_eras;
        match data.binary_search_by(|(d, _)| d.cmp(&date)) {
            Ok(index) => data.get(index),
            Err(index) if index == 0 => data.get(index),
            Err(index) => data.get(index - 1).or_else(|| data.iter().next_back()),
        }
        .unwrap_or((REIWA_START, tinystr!(16, "reiwa")))
    }

    /// Returns the era start data for a given era
    fn japanese_era_start(&self, era: TinyStr16) -> Result<EraStartDate, DateError> {
        // Avoid linear search by trying well known eras
        if era == tinystr!(16, "reiwa") {
            return Ok(REIWA_START);
        } else if era == tinystr!(16, "heisei") {
            return Ok(HEISEI_START);
        } else if era == tinystr!(16, "showa") {
            return Ok(SHOWA_START);
        } else if era == tinystr!(16, "taisho") {
            return Ok(TAISHO_START);
        } else if era == tinystr!(16, "meiji") {
            return Ok(MEIJI_START);
        }

        let data = &self.dates_to_eras;
        // Try to avoid linear search by binary searching for the year suffix
        if let Some(year) = era.split('-').nth(1) {
            if let Ok(ref int) = year.parse::<i32>() {
                if let Ok(index) = data.binary_search_by(|(d, _)| d.year.cmp(int)) {
                    #[expect(clippy::expect_used)] // see expect message
                    let (era_start, code) = data
                        .get(index)
                        .expect("Indexing from successful binary search must succeed");
                    // There is a slight chance we hit the case where there are two eras in the same year
                    // There are a couple of rare cases of this, but it's not worth writing a range-based binary search
                    // to catch them since this is an optimization
                    if code == era {
                        return Ok(era_start);
                    }
                }
            }
        }

        // Avoidance didn't work. Let's find the era manually, searching back from the present
        if let Some((start, _)) = data.iter().rev().find(|d| d.1 == era) {
            return Ok(start);
        }

        Err(DateError::UnknownEra)
    }

    fn new_japanese_date_inner(
        &self,
        era: Option<&str>,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<JapaneseDateInner, DateError> {
        let era = match era {
            None => {
                return Ok(JapaneseDateInner(ArithmeticDate::new_gregorian::<IsoEra>(
                    year, month, day,
                )?));
            }
            Some("ce" | "ad") => {
                return Ok(JapaneseDateInner(ArithmeticDate::new_gregorian::<IsoEra>(
                    year, month, day,
                )?));
            }
            Some("bce" | "bc") => {
                return Ok(JapaneseDateInner(ArithmeticDate::new_gregorian::<IsoEra>(
                    1 - year,
                    month,
                    day,
                )?));
            }
            Some(e) => e.parse().map_err(|_| DateError::UnknownEra)?,
        };

        let era_start = self.japanese_era_start(era)?;

        let iso = Date::try_new_iso(era_start.year + year - 1, month, day)?;
        Ok(JapaneseDateInner(iso.inner.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ref;

    fn single_test_roundtrip(calendar: Ref<Japanese>, era: &str, year: i32, month: u8, day: u8) {
        let date = Date::try_new_japanese_with_calendar(era, year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!("Failed to construct date with {era:?}, {year}, {month}, {day}: {e:?}")
            });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            date, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day}"
        );

        // Extra coverage for https://github.com/unicode-org/icu4x/issues/4968
        assert_eq!(reconstructed.era_year().era, era);
        assert_eq!(reconstructed.era_year().year, year);
    }

    fn single_test_roundtrip_ext(
        calendar: Ref<JapaneseExtended>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
    ) {
        let date = Date::try_new_japanese_extended_with_calendar(era, year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!("Failed to construct date with {era:?}, {year}, {month}, {day}: {e:?}")
            });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            date, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day}"
        )
    }

    // test that out-of-range era values roundtrip to other eras
    fn single_test_era_range_roundtrip(
        calendar: Ref<Japanese>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        era2: &str,
        year2: i32,
    ) {
        let expected = Date::try_new_japanese_with_calendar(era2, year2, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to construct expectation date with {era2:?}, {year2}, {month}, {day}: {e:?}"
                )
            });

        let date = Date::try_new_japanese_with_calendar(era, year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!("Failed to construct date with {era:?}, {year}, {month}, {day}: {e:?}")
            });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            expected, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day} == {era2:?}, {year}"
        )
    }
    fn single_test_era_range_roundtrip_ext(
        calendar: Ref<JapaneseExtended>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        era2: &str,
        year2: i32,
    ) {
        let expected = Date::try_new_japanese_extended_with_calendar(era2, year2, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to construct expectation date with {era2:?}, {year2}, {month}, {day}: {e:?}"
                )
            });

        let date = Date::try_new_japanese_extended_with_calendar(era, year, month, day, calendar)
            .unwrap_or_else(|e| {
                panic!("Failed to construct date with {era:?}, {year}, {month}, {day}: {e:?}")
            });
        let iso = date.to_iso();
        let reconstructed = Date::new_from_iso(iso, calendar);
        assert_eq!(
            expected, reconstructed,
            "Failed to roundtrip with {era:?}, {year}, {month}, {day} == {era2:?}, {year}"
        )
    }

    fn single_test_error(
        calendar: Ref<Japanese>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        error: DateError,
    ) {
        let date = Date::try_new_japanese_with_calendar(era, year, month, day, calendar);
        assert_eq!(
            date,
            Err(error),
            "Construction with {era:?}, {year}, {month}, {day} did not return {error:?}"
        )
    }

    fn single_test_error_ext(
        calendar: Ref<JapaneseExtended>,
        era: &str,
        year: i32,
        month: u8,
        day: u8,
        error: DateError,
    ) {
        let date = Date::try_new_japanese_extended_with_calendar(era, year, month, day, calendar);
        assert_eq!(
            date,
            Err(error),
            "Construction with {era:?}, {year}, {month}, {day} did not return {error:?}"
        )
    }

    #[test]
    fn test_japanese() {
        let calendar = Japanese::new();
        let calendar_ext = JapaneseExtended::new();
        let calendar = Ref(&calendar);
        let calendar_ext = Ref(&calendar_ext);

        single_test_roundtrip(calendar, "heisei", 12, 3, 1);
        single_test_roundtrip(calendar, "taisho", 3, 3, 1);
        // Heisei did not start until later in the year
        single_test_era_range_roundtrip(calendar, "heisei", 1, 1, 1, "showa", 64);

        single_test_roundtrip_ext(calendar_ext, "heisei", 12, 3, 1);
        single_test_roundtrip_ext(calendar_ext, "taisho", 3, 3, 1);
        single_test_era_range_roundtrip_ext(calendar_ext, "heisei", 1, 1, 1, "showa", 64);

        single_test_roundtrip_ext(calendar_ext, "hakuho-672", 4, 3, 1);
        single_test_error(calendar, "hakuho-672", 4, 3, 1, DateError::UnknownEra);

        // handle bce/ce
        single_test_roundtrip(calendar, "bce", 100, 3, 1);
        single_test_roundtrip(calendar, "bce", 1, 3, 1);
        single_test_roundtrip(calendar, "ce", 1, 3, 1);
        single_test_roundtrip(calendar, "ce", 100, 3, 1);
        single_test_roundtrip_ext(calendar_ext, "ce", 100, 3, 1);
        single_test_roundtrip(calendar, "ce", 1000, 3, 1);
        single_test_era_range_roundtrip(calendar, "ce", 0, 3, 1, "bce", 1);
        single_test_era_range_roundtrip(calendar, "bce", -1, 3, 1, "ce", 2);

        // handle the cases where bce/ce get adjusted to different eras
        // single_test_gregorian_roundtrip(calendar, "ce", 2021, 3, 1, "reiwa", 3);
        single_test_era_range_roundtrip_ext(calendar_ext, "ce", 1000, 3, 1, "choho-999", 2);
        single_test_era_range_roundtrip_ext(calendar_ext, "ce", 749, 5, 10, "tenpyokampo-749", 1);
        single_test_era_range_roundtrip_ext(calendar_ext, "bce", 10, 3, 1, "bce", 10);
        single_test_era_range_roundtrip_ext(calendar_ext, "ce", -1, 3, 1, "bce", 2);

        // There were multiple eras in this year
        // This one is from Apr 14 to July 2
        single_test_roundtrip_ext(calendar_ext, "tenpyokampo-749", 1, 4, 20);
        single_test_roundtrip_ext(calendar_ext, "tenpyokampo-749", 1, 4, 14);
        single_test_roundtrip_ext(calendar_ext, "tenpyokampo-749", 1, 7, 1);
        single_test_era_range_roundtrip_ext(
            calendar_ext,
            "tenpyokampo-749",
            1,
            7,
            5,
            "tenpyoshoho-749",
            1,
        );
        single_test_era_range_roundtrip_ext(
            calendar_ext,
            "tenpyokampo-749",
            1,
            4,
            13,
            "tenpyoshoho-749",
            1,
        );
    }
}
