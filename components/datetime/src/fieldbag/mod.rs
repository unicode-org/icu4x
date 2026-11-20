// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for expressing field-by-field models for datetime formats.

mod conversion;
pub mod field;
mod skeleton;

use core::{fmt, str::FromStr};

pub use skeleton::DateTimeFieldBagParseError;

use crate::fieldsets;
use field::*;
use writeable::Writeable;

/// A bag of individual datetime formatting fields.
///
/// This struct is designed to mirror the ECMA-402 `Intl.DateTimeFormat` options bag, with minor
/// extensions to cover additional features implemented by ICU4X.
///
/// This struct is convertible back and forth to a UTS #35 skeleton string via [`Writeable`]
/// (and [`Display`](fmt::Display)) and [`FromStr`] (or [`Self::try_from_skeleton`]), which serves
/// as its lossless string interchange format (except that standalone `hour_kind` without `hour`
/// is omitted, as UTS #35 encodes clock kind on the hour symbol). However, it only implements the
/// subset of skeleton string functionality required by ECMA-402 and ICU4X.
///
/// An all-`None` [`DateTimeFieldBag::default()`] is an incomplete intermediate state for building
/// a request; if converted directly to a field set without setting any fields, it defaults to
/// formatting the year, month, and day (`YMD`).
///
/// # Examples
///
/// Format a year and era based on a skeleton string:
///
/// ```
/// use icu::datetime::DateTimeFormatter;
/// use icu::datetime::fieldbag::DateTimeFieldBag;
/// use icu::locale::locale;
/// use writeable::assert_writeable_eq;
///
/// let formatter = DateTimeFormatter::try_new(
///     locale!("uk").into(),
///     DateTimeFieldBag::from_skeleton("Gy").to_composite_field_set(),
/// )
/// .unwrap();
/// let input = icu::time::ZonedDateTime::try_strict_from_str(
///     "2026-09-23T12:03-0700[America/Los_Angeles]",
///     icu::calendar::Iso,
///     icu::time::zone::IanaParserBorrowed::new(),
/// )
/// .unwrap();
/// assert_writeable_eq!(formatter.format(&input), "2026 н. е.");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct DateTimeFieldBag {
    /// The representation of the era.
    ///
    /// If `None`, do not show the era, unless required by other fields.
    pub era: Option<Era>,

    /// The representation of the year.
    ///
    /// If `None`, do not show the year, unless required by other fields.
    pub year: Option<Year>,

    /// The representation of the month.
    ///
    /// If `None`, do not show the month, unless required by other fields.
    pub month: Option<Month>,

    /// The representation of the day.
    ///
    /// If `None`, do not show the day, unless required by other fields.
    pub day: Option<Day>,

    /// The representation of the weekday.
    ///
    /// If `None`, do not show the weekday.
    pub weekday: Option<Weekday>,

    /// The formatting style used for day periods like "in the morning", "am", "noon", "n" etc.
    ///
    /// If `None`, use AM/PM.
    ///
    /// Ignored unless a 12-hour clock is used.
    ///
    /// Note: Many locales use the same string irrespective of the width specified.
    pub day_period: Option<DayPeriod>,

    /// Choice between 12-hour and 24-hour time.
    ///
    /// If `None`, use the locale preference.
    ///
    /// Ignored unless `hour` is set.
    ///
    /// This corresponds to the `hour12` field in ECMA-402.
    pub hour_kind: Option<HourKind>,

    /// The representation of the hour.
    ///
    /// If `None`, do not show the hour, unless required by other fields.
    pub hour: Option<Hour>,

    /// The representation of the minute.
    ///
    /// If `None`, do not show the minute, unless required by other fields.
    pub minute: Option<Minute>,

    /// The representation of the second.
    ///
    /// If `None`, do not show the second, unless required by other fields.
    pub second: Option<Second>,

    /// The number of digits used to represent fractions of a second.
    /// Any additional digits are truncated.
    ///
    /// If `None`, do not show fractional seconds.
    pub fractional_second_digits: Option<FractionalSecondDigits>,

    /// The localized representation of the time zone name.
    ///
    /// If `None`, do not show the time zone.
    pub time_zone_name: Option<TimeZoneName>,
}

impl DateTimeFieldBag {
    /// Creates a [`DateTimeFieldBag`] from a skeleton string.
    ///
    /// This function rejects strings with duplicate fields, unknown fields, non-field
    /// characters, and other conditions in the [`DateTimeFieldBagParseError`]. For a
    /// more lenient parse function, use [`DateTimeFieldBag::from_skeleton`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use icu::datetime::fieldbag::field::Year;
    ///
    /// let bag = DateTimeFieldBag::try_from_skeleton("GGGGy").unwrap();
    ///
    /// let mut expected = DateTimeFieldBag::default();
    /// expected.era = Some(Era::Long);
    /// expected.year = Some(Year::Numeric);
    /// assert_eq!(bag, expected);
    ///
    /// let err_bag = DateTimeFieldBag::try_from_skeleton("...GGGGy...");
    /// assert!(err_bag.is_err());
    /// ```
    pub fn try_from_skeleton(skeleton: &str) -> Result<Self, DateTimeFieldBagParseError> {
        let parse_result = skeleton::uts35_to_fieldbag(skeleton);
        if let Some(err) = parse_result.1 {
            Err(err)
        } else {
            Ok(parse_result.0)
        }
    }

    /// Creates a [`DateTimeFieldBag`] from a skeleton string.
    ///
    /// This function gracefully ignores duplicate fields, unknown fields, non-field
    /// characters, and other conditions in the [`DateTimeFieldBagParseError`]. For a
    /// more strict parse function, use [`DateTimeFieldBag::try_from_skeleton`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::Era;
    /// use icu::datetime::fieldbag::field::Year;
    ///
    /// let bag = DateTimeFieldBag::from_skeleton("GGGGy");
    ///
    /// let mut expected = DateTimeFieldBag::default();
    /// expected.era = Some(Era::Long);
    /// expected.year = Some(Year::Numeric);
    /// assert_eq!(bag, expected);
    ///
    /// let lenient_bag = DateTimeFieldBag::from_skeleton("...GGGGy...");
    /// assert_eq!(lenient_bag, expected);
    /// ```
    pub fn from_skeleton(skeleton: &str) -> Self {
        // drop the error
        skeleton::uts35_to_fieldbag(skeleton).0
    }

    /// Converts this [`DateTimeFieldBag`] into a [`CompositeFieldSet`].
    ///
    /// This conversion is infallible and best-effort:
    /// - Missing intermediate fields are filled in when required by semantic skeletons
    ///   (for example, year + day becomes `YMD`, and standalone seconds promote to
    ///   [`TimePrecision::Second`](crate::options::TimePrecision::Second)).
    /// - Per-field text widths (`Short`, `Long`, `Narrow`) collapse into a single
    ///   [`Length`](crate::options::Length), and two-digit numeric `month`, `day`, or `hour`
    ///   widths map to [`Alignment::Column`](crate::options::Alignment::Column).
    /// - Standalone `day_period` without `hour` promotes to hour precision, while
    ///   standalone `hour_kind` without `hour` is ignored.
    /// - An all-`None` empty bag defaults to `YMD`.
    /// - `hour_kind` is not carried on [`CompositeFieldSet`] (hour cycle is configured via
    ///   [`DateTimeFormatterPreferences`](crate::DateTimeFormatterPreferences)).
    ///
    /// For smaller data size and narrower input types, use [`DateTimeFieldBag::to_field_set_builder`].
    ///
    /// # Examples
    ///
    /// End-to-end from a [`DateTimeFieldBag`] to a formatted datetime string:
    ///
    /// ```
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::{Era, Month, Year};
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::options;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // 1. Make the DateTimeFieldBag
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.era = Some(Era::Short);
    /// bag.year = Some(Year::Numeric);
    /// bag.month = Some(Month::Short);
    /// assert_writeable_eq!(bag, "GyMMM");
    ///
    /// // 2. Make the field set
    /// let fieldset = bag.to_composite_field_set();
    /// assert_eq!(
    ///     fieldset,
    ///     fieldsets::enums::CompositeFieldSet::CalendarPeriod(
    ///         fieldsets::enums::CalendarPeriodFieldSet::YM(
    ///             fieldsets::YM::medium()
    ///                 .with_year_style(options::YearStyle::WithEra)
    ///         )
    ///     )
    /// );
    ///
    /// // 3. Make the formatter
    /// let formatter =
    ///     DateTimeFormatter::try_new(locale!("fr").into(), fieldset).unwrap();
    /// let input = icu::time::ZonedDateTime::try_strict_from_str(
    ///     "2026-09-23T12:03-0700[America/Los_Angeles]",
    ///     icu::calendar::Iso,
    ///     icu::time::zone::IanaParserBorrowed::new(),
    /// )
    /// .unwrap();
    /// assert_writeable_eq!(formatter.format(&input), "sept. 2026 ap. J.-C.");
    /// ```
    ///
    /// [`CompositeFieldSet`]: fieldsets::enums::CompositeFieldSet
    pub fn to_composite_field_set(self) -> fieldsets::enums::CompositeFieldSet {
        conversion::fieldbag_to_fieldset(&self)
            .build_composite()
            .unwrap_or_else(|err| {
                debug_assert!(
                    false,
                    "field bag should be convertible to CompositeFieldSet: {self:?} {err:?}"
                );
                // this is an error case. GIGO with YMD format.
                fieldsets::enums::CompositeFieldSet::Date(fieldsets::enums::DateFieldSet::YMD(
                    fieldsets::YMD::medium(),
                ))
            })
    }

    /// Converts this [`DateTimeFieldBag`] into a [`FieldSetBuilder`](fieldsets::builder::FieldSetBuilder).
    ///
    /// This conversion uses the same best-effort mapping rules as
    /// [`DateTimeFieldBag::to_composite_field_set`] (including defaulting an all-`None` bag to
    /// [`DateFields::YMD`](fieldsets::builder::DateFields::YMD)) so that calling
    /// [`FieldSetBuilder::build_composite`](fieldsets::builder::FieldSetBuilder::build_composite)
    /// on the returned builder always succeeds.
    ///
    /// Use this function to build a specific field set category (such as
    /// [`build_date`](fieldsets::builder::FieldSetBuilder::build_date),
    /// [`build_calendar_period`](fieldsets::builder::FieldSetBuilder::build_calendar_period),
    /// [`build_time`](fieldsets::builder::FieldSetBuilder::build_time), or
    /// [`build_zone`](fieldsets::builder::FieldSetBuilder::build_zone)) for smaller data size
    /// and narrower input types. For a direct infallible conversion to a composite field set,
    /// use [`DateTimeFieldBag::to_composite_field_set`].
    ///
    /// # Examples
    ///
    /// End-to-end from a [`DateTimeFieldBag`] to a formatted datetime string:
    ///
    /// ```
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldbag::field::{Era, Month, Year};
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::options;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// // 1. Make the DateTimeFieldBag
    /// let mut bag = DateTimeFieldBag::default();
    /// bag.era = Some(Era::Short);
    /// bag.year = Some(Year::Numeric);
    /// bag.month = Some(Month::Short);
    /// assert_writeable_eq!(bag, "GyMMM");
    ///
    /// // 2. Make the field set
    /// let builder = bag.to_field_set_builder();
    /// let fieldset = builder.build_calendar_period().unwrap();
    /// assert_eq!(
    ///     fieldset,
    ///     fieldsets::enums::CalendarPeriodFieldSet::YM(
    ///         fieldsets::YM::medium()
    ///             .with_year_style(options::YearStyle::WithEra)
    ///     )
    /// );
    ///
    /// // 3. Make the formatter
    /// let formatter =
    ///     DateTimeFormatter::try_new(locale!("fr").into(), fieldset).unwrap();
    /// let input = icu::calendar::Date::try_new_iso(2026, 9, 23).unwrap();
    /// assert_writeable_eq!(formatter.format(&input), "sept. 2026 ap. J.-C.");
    /// ```
    pub fn to_field_set_builder(self) -> fieldsets::builder::FieldSetBuilder {
        conversion::fieldbag_to_fieldset(&self)
    }

    /// Reconstructs a representative [`DateTimeFieldBag`] from a
    /// [`FieldSetBuilder`](fieldsets::builder::FieldSetBuilder).
    ///
    /// Because [`FieldSetBuilder`](fieldsets::builder::FieldSetBuilder) stores coarse
    /// category-level options rather than individual field widths, this conversion expands
    /// [`Length`](crate::options::Length), [`Alignment`](crate::options::Alignment), and
    /// [`YearStyle`](crate::options::YearStyle) into canonical per-field widths (and leaves
    /// `hour_kind` and `day_period` unset).
    ///
    /// # Examples
    ///
    /// Get a field bag from a formatter:
    ///
    /// ```
    /// use icu::datetime::DateTimeFormatter;
    /// use icu::datetime::fieldbag::DateTimeFieldBag;
    /// use icu::datetime::fieldsets;
    /// use icu::locale::locale;
    /// use writeable::assert_writeable_eq;
    ///
    /// let formatter = DateTimeFormatter::try_new(
    ///     locale!("it").into(),
    ///     fieldsets::YM::medium(),
    /// )
    /// .unwrap();
    /// let bag = DateTimeFieldBag::from_field_set_builder(
    ///     &formatter.to_field_set_builder(),
    /// );
    ///
    /// assert_writeable_eq!(bag, "yMMM");
    /// ```
    pub fn from_field_set_builder(builder: &fieldsets::builder::FieldSetBuilder) -> Self {
        conversion::fieldset_to_fieldbag(builder)
    }
}

#[test]
fn test_fieldbag_size() {
    assert!(size_of::<DateTimeFieldBag>() <= 16);
}

impl FromStr for DateTimeFieldBag {
    type Err = DateTimeFieldBagParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_skeleton(s)
    }
}

impl Writeable for DateTimeFieldBag {
    fn write_to<W: ?Sized + fmt::Write>(&self, sink: &mut W) -> fmt::Result {
        skeleton::fieldbag_to_uts35(self, sink)
    }
}

writeable::impl_display_with_writeable!(DateTimeFieldBag);
