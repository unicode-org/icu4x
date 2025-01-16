// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Builder APIs for [dynamic field sets](crate::fieldsets::enums).
//!
//! These APIs are designed for when the field set is not known at compile time. This could
//! happen if:
//!
//! 1. The field set is sent over the network or read from a data file
//! 2. Implementing another interface with different types
//!
//! If the field set is known at compile time, use the static fieldset APIs instead of the
//! builder exported in this module.
//!
//! All examples below will show both ways to build a field set.
//!
//! # Examples
//!
//! ```
//! use icu::datetime::fieldsets;
//! use icu::datetime::fieldsets::builder::*;
//! use icu::datetime::fieldsets::enums::*;
//! use icu::datetime::options::*;
//!
//! // Year, Month, Day
//! // Medium length
//! // Always display the era
//!
//! let static_field_set = fieldsets::YMD::medium()
//!     .with_year_style(YearStyle::WithEra);
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.date_fields = Some(DateFields::YMD);
//! builder.length = Some(Length::Medium);
//! builder.year_style = Some(YearStyle::WithEra);
//! let dynamic_field_set = builder.build_date().unwrap();
//!
//! assert_eq!(
//!     dynamic_field_set,
//!     DateFieldSet::YMD(static_field_set),
//! );
//!
//! // Standalone Month
//! // Long length
//!
//! let static_field_set = fieldsets::M::long();
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.length = Some(Length::Long);
//! builder.date_fields = Some(DateFields::M);
//! let dynamic_field_set = builder.build_calendar_period().unwrap();
//!
//! assert_eq!(
//!     dynamic_field_set,
//!     CalendarPeriodFieldSet::M(static_field_set),
//! );
//!
//! // Weekday and Time of day
//! // Medium length, implicit in the builder
//! // Display time to the minute
//!
//! let static_field_set = fieldsets::ET::medium()
//!     .with_time_precision(TimePrecision::Minute);
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.date_fields = Some(DateFields::E);
//! builder.time_precision = Some(TimePrecision::Minute);
//! let dynamic_field_set = builder.build_date_and_time().unwrap();
//!
//! assert_eq!(
//!     dynamic_field_set,
//!     DateAndTimeFieldSet::ET(static_field_set),
//! );
//!
//! // Time and Time Zone
//! // Short length
//! // Long specific non-location time zone
//! // Display time to the millisecond
//! // Render for column alignment
//!
//! let static_field_set = fieldsets::T::short()
//!     .with_time_precision(TimePrecision::FractionalSecond(FractionalSecondDigits::F3))
//!     .with_alignment(Alignment::Column)
//!     .with_zone_specific_long();
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.length = Some(Length::Short);
//! builder.time_precision = Some(TimePrecision::FractionalSecond(FractionalSecondDigits::F3));
//! builder.alignment = Some(Alignment::Column);
//! builder.zone_style = Some(ZoneStyle::Z);
//! let dynamic_field_set = builder.build_composite().unwrap();
//!
//! assert_eq!(
//!     dynamic_field_set,
//!     CompositeFieldSet::TimeZone(static_field_set.into_enums()),
//! );
//! ```

use crate::fieldsets::{self, enums::*, Combo};
use crate::options::*;

/// An enumeration over all possible date and calendar period field sets
/// without options.
///
/// This is a builder enum. See [`builder`](crate::fieldsets::builder).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateFields {
    /// The day of the month, as in
    /// “on the 1st”.
    D,
    /// The month and day of the month, as in
    /// “January 1st”.
    MD,
    /// The year, month, and day of the month, as in
    /// “January 1st, 2000”.
    YMD,
    /// The day of the month and day of the week, as in
    /// “Saturday 1st”.
    DE,
    /// The month, day of the month, and day of the week, as in
    /// “Saturday, January 1st”.
    MDE,
    /// The year, month, day of the month, and day of the week, as in
    /// “Saturday, January 1st, 2000”.
    YMDE,
    /// The day of the week alone, as in
    /// “Saturday”.
    E,
    /// A standalone month, as in
    /// “January”.
    M,
    /// A month and year, as in
    /// “January 2000”.
    YM,
    /// A year, as in
    /// “2000”.
    Y,
}

/// An enumeration over all possible time zone styles.
///
/// This is a builder enum. See [`builder`](crate::fieldsets::builder).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ZoneStyle {
    /// The long specific non-location format, as in
    /// “Pacific Daylight Time”.
    Z,
    /// The short specific non-location format, as in
    /// “PDT”.
    Zs,
    /// The long offset format, as in
    /// “GMT−8:00”.
    O,
    /// The short offset format, as in
    /// “GMT−8”.
    Os,
    /// The long generic non-location format, as in
    /// “Pacific Time”.
    V,
    /// The short generic non-location format, as in
    /// “PT”.
    Vs,
    /// The location format, as in
    /// “Los Angeles time”.
    L,
}

/// An error that occurs when creating a [field set](crate::fieldsets) from a builder.
#[derive(Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum BuilderError {
    /// The specified fields are incompatible with the desired field set
    InvalidFields,
    /// The specified options are incompatible with the specified field set
    InvalidOptions,
}

impl core::error::Error for BuilderError {}

/// A builder for [dynamic field sets](crate::fieldsets::enums).
///
/// This builder is useful if you do not know the field set at code compilation time. If you do,
/// the static field set APIs should yield smaller binary size.
///
/// For examples, see the [module docs](crate::fieldsets::builder).
// Note: could be Copy but we don't want implicit moves
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct FieldSetBuilder {
    /// The length of a formatted date/time string.
    ///
    /// If `None`, defaults to [`Length::Medium`].
    pub length: Option<Length>,
    /// The set of date fields, such as "year and month" or "weekday".
    ///
    /// If `None`, a date will not be displayed.
    pub date_fields: Option<DateFields>,
    /// The precision to display the time of day.
    ///
    /// If `None`, a time will not be displayed.
    pub time_precision: Option<TimePrecision>,
    /// The style to display the time zone.
    ///
    /// If `None`, a time zone will not be displayed.
    pub zone_style: Option<ZoneStyle>,
    /// The alignment context, such as when displaying dates in a table.
    ///
    /// This option may be specified only if the field set can honor it.
    pub alignment: Option<Alignment>,
    /// How to display the year and era.
    ///
    /// This option may be specified only if the year is included in [`Self::date_fields`].
    pub year_style: Option<YearStyle>,
}

// This is the default length when the builder is used. This could have been defined in the macro
// that generates the `take_from_builder` fns, but it would be easily lost.
pub(crate) const DEFAULT_LENGTH: Length = Length::Medium;

enum DateOrCalendarPeriodFieldSet {
    Date(DateFieldSet),
    CalendarPeriod(CalendarPeriodFieldSet),
}

impl FieldSetBuilder {
    /// Creates a new, empty [`FieldSetBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    fn build_date_or_calendar_period_without_checking_options(
        &mut self,
    ) -> Result<DateOrCalendarPeriodFieldSet, BuilderError> {
        use DateOrCalendarPeriodFieldSet::*;
        let field_set = match self.date_fields.take() {
            Some(DateFields::D) => Date(DateFieldSet::D(fieldsets::D::take_from_builder(self))),
            Some(DateFields::MD) => Date(DateFieldSet::MD(fieldsets::MD::take_from_builder(self))),
            Some(DateFields::YMD) => {
                Date(DateFieldSet::YMD(fieldsets::YMD::take_from_builder(self)))
            }
            Some(DateFields::DE) => Date(DateFieldSet::DE(fieldsets::DE::take_from_builder(self))),
            Some(DateFields::MDE) => {
                Date(DateFieldSet::MDE(fieldsets::MDE::take_from_builder(self)))
            }
            Some(DateFields::YMDE) => {
                Date(DateFieldSet::YMDE(fieldsets::YMDE::take_from_builder(self)))
            }
            Some(DateFields::E) => Date(DateFieldSet::E(fieldsets::E::take_from_builder(self))),
            Some(DateFields::M) => CalendarPeriod(CalendarPeriodFieldSet::M(
                fieldsets::M::take_from_builder(self),
            )),
            Some(DateFields::YM) => CalendarPeriod(CalendarPeriodFieldSet::YM(
                fieldsets::YM::take_from_builder(self),
            )),
            Some(DateFields::Y) => CalendarPeriod(CalendarPeriodFieldSet::Y(
                fieldsets::Y::take_from_builder(self),
            )),
            Option::None => return Err(BuilderError::InvalidFields),
        };
        Ok(field_set)
    }

    /// Builds a [`DateFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_date(mut self) -> Result<DateFieldSet, BuilderError> {
        let date_field_set = match self.build_date_or_calendar_period_without_checking_options()? {
            DateOrCalendarPeriodFieldSet::Date(fs) => fs,
            DateOrCalendarPeriodFieldSet::CalendarPeriod(_) => {
                return Err(BuilderError::InvalidFields)
            }
        };
        self.check_options_consumed()?;
        Ok(date_field_set)
    }

    /// Builds a [`CalendarPeriodFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_calendar_period(mut self) -> Result<CalendarPeriodFieldSet, BuilderError> {
        let calendar_period_field_set =
            match self.build_date_or_calendar_period_without_checking_options()? {
                DateOrCalendarPeriodFieldSet::Date(_) => return Err(BuilderError::InvalidFields),
                DateOrCalendarPeriodFieldSet::CalendarPeriod(fs) => fs,
            };
        self.check_options_consumed()?;
        Ok(calendar_period_field_set)
    }

    /// Builds a [`TimeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_time(mut self) -> Result<TimeFieldSet, BuilderError> {
        let time_field_set = TimeFieldSet::T(fieldsets::T::take_from_builder(&mut self));
        self.check_options_consumed()?;
        Ok(time_field_set)
    }

    fn build_zone_without_checking_options(&mut self) -> Result<ZoneFieldSet, BuilderError> {
        let zone_field_set = match self.zone_style.take() {
            Some(ZoneStyle::Z) => ZoneFieldSet::Z(fieldsets::Z::take_from_builder(self)),
            Some(ZoneStyle::Zs) => ZoneFieldSet::Zs(fieldsets::Zs::take_from_builder(self)),
            Some(ZoneStyle::O) => ZoneFieldSet::O(fieldsets::O::take_from_builder(self)),
            Some(ZoneStyle::Os) => ZoneFieldSet::Os(fieldsets::Os::take_from_builder(self)),
            Some(ZoneStyle::V) => ZoneFieldSet::V(fieldsets::V::take_from_builder(self)),
            Some(ZoneStyle::Vs) => ZoneFieldSet::Vs(fieldsets::Vs::take_from_builder(self)),
            Some(ZoneStyle::L) => ZoneFieldSet::L(fieldsets::L::take_from_builder(self)),
            Option::None => return Err(BuilderError::InvalidFields),
        };
        Ok(zone_field_set)
    }

    /// Builds a [`ZoneFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_zone(mut self) -> Result<ZoneFieldSet, BuilderError> {
        let zone_field_set = self.build_zone_without_checking_options()?;
        self.check_options_consumed()?;
        Ok(zone_field_set)
    }

    /// Builds a [`DateAndTimeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_date_and_time(mut self) -> Result<DateAndTimeFieldSet, BuilderError> {
        if self.time_precision.is_none() {
            return Err(BuilderError::InvalidOptions);
        }
        let date_and_time_field_set = match self.date_fields.take() {
            Some(DateFields::D) => {
                DateAndTimeFieldSet::DT(fieldsets::DT::take_from_builder(&mut self))
            }
            Some(DateFields::MD) => {
                DateAndTimeFieldSet::MDT(fieldsets::MDT::take_from_builder(&mut self))
            }
            Some(DateFields::YMD) => {
                DateAndTimeFieldSet::YMDT(fieldsets::YMDT::take_from_builder(&mut self))
            }
            Some(DateFields::DE) => {
                DateAndTimeFieldSet::DET(fieldsets::DET::take_from_builder(&mut self))
            }
            Some(DateFields::MDE) => {
                DateAndTimeFieldSet::MDET(fieldsets::MDET::take_from_builder(&mut self))
            }
            Some(DateFields::YMDE) => {
                DateAndTimeFieldSet::YMDET(fieldsets::YMDET::take_from_builder(&mut self))
            }
            Some(DateFields::E) => {
                DateAndTimeFieldSet::ET(fieldsets::ET::take_from_builder(&mut self))
            }
            Some(DateFields::M) | Some(DateFields::YM) | Some(DateFields::Y) | Option::None => {
                return Err(BuilderError::InvalidFields)
            }
        };
        self.check_options_consumed()?;
        Ok(date_and_time_field_set)
    }

    /// Builds a [`CompositeDateTimeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_composite_datetime(mut self) -> Result<CompositeDateTimeFieldSet, BuilderError> {
        // Check for the presence of date and time, then delegate to the correct impl.
        match (self.date_fields.is_some(), self.time_precision.is_some()) {
            (true, false) => {
                let field_set = match self
                    .build_date_or_calendar_period_without_checking_options()?
                {
                    DateOrCalendarPeriodFieldSet::Date(fs) => CompositeDateTimeFieldSet::Date(fs),
                    DateOrCalendarPeriodFieldSet::CalendarPeriod(fs) => {
                        CompositeDateTimeFieldSet::CalendarPeriod(fs)
                    }
                };
                self.check_options_consumed()?;
                Ok(field_set)
            }
            (false, true) => self.build_time().map(CompositeDateTimeFieldSet::Time),
            (true, true) => self
                .build_date_and_time()
                .map(CompositeDateTimeFieldSet::DateTime),
            (false, false) => Err(BuilderError::InvalidFields),
        }
    }

    /// Builds a [`CompositeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    pub fn build_composite(mut self) -> Result<CompositeFieldSet, BuilderError> {
        // Check for the presence of date, time, and zone, then delegate to the correct impl.
        match (
            self.date_fields.is_some(),
            self.time_precision.is_some(),
            self.zone_style.is_some(),
        ) {
            (true, false, false) => {
                let field_set =
                    match self.build_date_or_calendar_period_without_checking_options()? {
                        DateOrCalendarPeriodFieldSet::Date(fs) => CompositeFieldSet::Date(fs),
                        DateOrCalendarPeriodFieldSet::CalendarPeriod(fs) => {
                            CompositeFieldSet::CalendarPeriod(fs)
                        }
                    };
                self.check_options_consumed()?;
                Ok(field_set)
            }
            (false, true, false) => self.build_time().map(CompositeFieldSet::Time),
            (true, true, false) => self.build_date_and_time().map(CompositeFieldSet::DateTime),
            (false, false, true) => self.build_zone().map(CompositeFieldSet::Zone),
            (true, false, true) => {
                let zone_field_set = self.build_zone_without_checking_options()?;
                let date_field_set = self.build_date()?;
                Ok(CompositeFieldSet::DateZone(Combo::new(
                    date_field_set,
                    zone_field_set,
                )))
            }
            (false, true, true) => {
                let zone_field_set = self.build_zone_without_checking_options()?;
                let time_field_set = self.build_time()?;
                Ok(CompositeFieldSet::TimeZone(Combo::new(
                    time_field_set,
                    zone_field_set,
                )))
            }
            (true, true, true) => {
                let zone_field_set = self.build_zone_without_checking_options()?;
                let date_and_time_field_set = self.build_date_and_time()?;
                Ok(CompositeFieldSet::DateTimeZone(Combo::new(
                    date_and_time_field_set,
                    zone_field_set,
                )))
            }
            (false, false, false) => Err(BuilderError::InvalidFields),
        }
    }

    fn check_options_consumed(self) -> Result<(), BuilderError> {
        if self != Self::default() {
            Err(BuilderError::InvalidOptions)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static DATE_FIELD_SETS: &[DateFields] = &[
        DateFields::D,
        DateFields::MD,
        DateFields::YMD,
        DateFields::DE,
        DateFields::MDE,
        DateFields::YMDE,
        DateFields::E,
    ];

    static CALENDAR_PERIOD_FIELD_SETS: &[DateFields] =
        &[DateFields::M, DateFields::YM, DateFields::Y];

    static ZONE_STYLES: &[ZoneStyle] = &[
        ZoneStyle::Z,
        ZoneStyle::Zs,
        ZoneStyle::O,
        ZoneStyle::Os,
        ZoneStyle::V,
        ZoneStyle::Vs,
        ZoneStyle::L,
    ];

    #[test]
    fn test_date_field_sets() {
        for date_fields in DATE_FIELD_SETS.iter() {
            let mut builder = FieldSetBuilder::new();
            builder.date_fields = Some(*date_fields);
            builder.clone().build_date().unwrap();
            builder.clone().build_calendar_period().unwrap_err();
            builder.clone().build_time().unwrap_err();
            builder.clone().build_zone().unwrap_err();
            builder.clone().build_date_and_time().unwrap_err();
            builder.clone().build_composite_datetime().unwrap();
            builder.build_composite().unwrap();
        }
    }

    #[test]
    fn test_calendar_period_field_sets() {
        for date_fields in CALENDAR_PERIOD_FIELD_SETS.iter() {
            let mut builder = FieldSetBuilder::new();
            builder.date_fields = Some(*date_fields);
            builder.clone().build_date().unwrap_err();
            builder.clone().build_calendar_period().unwrap();
            builder.clone().build_time().unwrap_err();
            builder.clone().build_zone().unwrap_err();
            builder.clone().build_date_and_time().unwrap_err();
            builder.clone().build_composite_datetime().unwrap();
            builder.build_composite().unwrap();
        }
    }

    #[test]
    fn test_time_field_sets() {
        let mut builder = FieldSetBuilder::new();
        builder.time_precision = Some(TimePrecision::Minute);
        builder.clone().build_date().unwrap_err();
        builder.clone().build_calendar_period().unwrap_err();
        builder.clone().build_time().unwrap();
        builder.clone().build_zone().unwrap_err();
        builder.clone().build_date_and_time().unwrap_err();
        builder.clone().build_composite_datetime().unwrap();
        builder.build_composite().unwrap();
    }

    #[test]
    fn test_zone_field_sets() {
        for zone_style in ZONE_STYLES.iter() {
            let mut builder = FieldSetBuilder::new();
            builder.zone_style = Some(*zone_style);
            builder.clone().build_date().unwrap_err();
            builder.clone().build_calendar_period().unwrap_err();
            builder.clone().build_time().unwrap_err();
            builder.clone().build_zone().unwrap();
            builder.clone().build_date_and_time().unwrap_err();
            builder.clone().build_composite_datetime().unwrap_err();
            builder.build_composite().unwrap();
        }
    }

    #[test]
    fn test_date_time_field_sets() {
        for date_fields in DATE_FIELD_SETS.iter() {
            let mut builder = FieldSetBuilder::new();
            builder.date_fields = Some(*date_fields);
            builder.time_precision = Some(TimePrecision::Minute);
            builder.clone().build_date().unwrap_err();
            builder.clone().build_calendar_period().unwrap_err();
            builder.clone().build_time().unwrap_err();
            builder.clone().build_zone().unwrap_err();
            builder.clone().build_date_and_time().unwrap();
            builder.clone().build_composite_datetime().unwrap();
            builder.build_composite().unwrap();
        }
    }

    #[test]
    fn test_calendar_period_time_field_sets() {
        // Should always fail
        for date_fields in CALENDAR_PERIOD_FIELD_SETS.iter() {
            let mut builder = FieldSetBuilder::new();
            builder.date_fields = Some(*date_fields);
            builder.time_precision = Some(TimePrecision::Minute);
            builder.clone().build_date().unwrap_err();
            builder.clone().build_calendar_period().unwrap_err();
            builder.clone().build_time().unwrap_err();
            builder.clone().build_zone().unwrap_err();
            builder.clone().build_date_and_time().unwrap_err();
            builder.clone().build_composite_datetime().unwrap_err();
            builder.build_composite().unwrap_err();
        }
    }

    #[test]
    fn test_date_zone_field_sets() {
        for date_fields in DATE_FIELD_SETS.iter() {
            for zone_style in ZONE_STYLES.iter() {
                let mut builder = FieldSetBuilder::new();
                builder.date_fields = Some(*date_fields);
                builder.zone_style = Some(*zone_style);
                builder.clone().build_date().unwrap_err();
                builder.clone().build_calendar_period().unwrap_err();
                builder.clone().build_time().unwrap_err();
                builder.clone().build_zone().unwrap_err();
                builder.clone().build_date_and_time().unwrap_err();
                builder.clone().build_composite_datetime().unwrap_err();
                builder.build_composite().unwrap();
            }
        }
    }

    #[test]
    fn test_calendar_period_zone_field_sets() {
        // Should always fail
        for date_fields in CALENDAR_PERIOD_FIELD_SETS.iter() {
            for zone_style in ZONE_STYLES.iter() {
                let mut builder = FieldSetBuilder::new();
                builder.date_fields = Some(*date_fields);
                builder.zone_style = Some(*zone_style);
                builder.clone().build_date().unwrap_err();
                builder.clone().build_calendar_period().unwrap_err();
                builder.clone().build_time().unwrap_err();
                builder.clone().build_zone().unwrap_err();
                builder.clone().build_date_and_time().unwrap_err();
                builder.clone().build_composite_datetime().unwrap_err();
                builder.build_composite().unwrap_err();
            }
        }
    }

    #[test]
    fn test_time_zone_field_sets() {
        for zone_style in ZONE_STYLES.iter() {
            let mut builder = FieldSetBuilder::new();
            builder.time_precision = Some(TimePrecision::Minute);
            builder.zone_style = Some(*zone_style);
            builder.clone().build_date().unwrap_err();
            builder.clone().build_calendar_period().unwrap_err();
            builder.clone().build_time().unwrap_err();
            builder.clone().build_zone().unwrap_err();
            builder.clone().build_date_and_time().unwrap_err();
            builder.clone().build_composite_datetime().unwrap_err();
            builder.build_composite().unwrap();
        }
    }

    #[test]
    fn test_date_time_zone_field_sets() {
        for date_fields in DATE_FIELD_SETS.iter() {
            for zone_style in ZONE_STYLES.iter() {
                let mut builder = FieldSetBuilder::new();
                builder.date_fields = Some(*date_fields);
                builder.time_precision = Some(TimePrecision::Minute);
                builder.zone_style = Some(*zone_style);
                builder.clone().build_date().unwrap_err();
                builder.clone().build_calendar_period().unwrap_err();
                builder.clone().build_time().unwrap_err();
                builder.clone().build_zone().unwrap_err();
                builder.clone().build_date_and_time().unwrap_err();
                builder.clone().build_composite_datetime().unwrap_err();
                builder.build_composite().unwrap();
            }
        }
    }

    #[test]
    fn test_calendar_period_time_zone_field_sets() {
        // Should always fail
        for date_fields in CALENDAR_PERIOD_FIELD_SETS.iter() {
            for zone_style in ZONE_STYLES.iter() {
                let mut builder = FieldSetBuilder::new();
                builder.date_fields = Some(*date_fields);
                builder.time_precision = Some(TimePrecision::Minute);
                builder.zone_style = Some(*zone_style);
                builder.clone().build_date().unwrap_err();
                builder.clone().build_calendar_period().unwrap_err();
                builder.clone().build_time().unwrap_err();
                builder.clone().build_zone().unwrap_err();
                builder.clone().build_date_and_time().unwrap_err();
                builder.clone().build_composite_datetime().unwrap_err();
                builder.build_composite().unwrap_err();
            }
        }
    }
}
