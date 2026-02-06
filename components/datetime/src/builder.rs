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
//! A field set builder can be serialized with the right set of Cargo features.
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
//! let static_field_set =
//!     fieldsets::YMD::medium().with_year_style(YearStyle::WithEra);
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.date_fields = Some(DateFields::YMD);
//! builder.length = Some(Length::Medium);
//! builder.year_style = Some(YearStyle::WithEra);
//! let dynamic_field_set = builder.build_date().unwrap();
//!
//! assert_eq!(dynamic_field_set, DateFieldSet::YMD(static_field_set),);
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
//! let static_field_set =
//!     fieldsets::ET::medium().with_time_precision(TimePrecision::Minute);
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.date_fields = Some(DateFields::E);
//! builder.time_precision = Some(TimePrecision::Minute);
//! let dynamic_field_set = builder.build_date_and_time().unwrap();
//!
//! assert_eq!(dynamic_field_set, DateAndTimeFieldSet::ET(static_field_set),);
//!
//! // Time and Time Zone
//! // Short length
//! // Long specific non-location time zone
//! // Display time to the millisecond
//! // Render for column alignment
//!
//! let static_field_set = fieldsets::T::short()
//!     .with_time_precision(TimePrecision::Subsecond(SubsecondDigits::S3))
//!     .with_alignment(Alignment::Column)
//!     .with_zone(fieldsets::zone::SpecificLong);
//!
//! let mut builder = FieldSetBuilder::new();
//! builder.length = Some(Length::Short);
//! builder.time_precision =
//!     Some(TimePrecision::Subsecond(SubsecondDigits::S3));
//! builder.alignment = Some(Alignment::Column);
//! builder.zone_style = Some(ZoneStyle::SpecificLong);
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
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
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
    /// The day of the month and weekday, as in
    /// “Saturday 1st”.
    DE,
    /// The month, day of the month, and weekday, as in
    /// “Saturday, January 1st”.
    MDE,
    /// The year, month, day of the month, and weekday, as in
    /// “Saturday, January 1st, 2000”.
    YMDE,
    /// The weekday alone, as in
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

impl DateFields {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[
        Self::D,
        Self::MD,
        Self::YMD,
        Self::DE,
        Self::MDE,
        Self::YMDE,
        Self::E,
        Self::M,
        Self::YM,
        Self::Y,
    ];

    /// Returns whether this [`DateFields`] variant represents a [`CalendarPeriodFieldSet`].
    pub fn is_calendar_period(self) -> bool {
        match self {
            DateFields::D => false,
            DateFields::MD => false,
            DateFields::YMD => false,
            DateFields::DE => false,
            DateFields::MDE => false,
            DateFields::YMDE => false,
            DateFields::E => false,
            DateFields::M => true,
            DateFields::YM => true,
            DateFields::Y => true,
        }
    }
}

/// An enumeration over all possible time zone styles.
///
/// This is a builder enum. See [`builder`](crate::fieldsets::builder).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(
    all(feature = "serde", feature = "unstable"),
    derive(serde::Serialize, serde::Deserialize)
)]
#[non_exhaustive]
pub enum ZoneStyle {
    /// The long specific non-location format, as in
    /// “Pacific Daylight Time”.
    SpecificLong,
    /// The short specific non-location format, as in
    /// “PDT”.
    SpecificShort,
    /// The long offset format, as in
    /// “GMT−8:00”.
    LocalizedOffsetLong,
    /// The short offset format, as in
    /// “GMT−8”.
    LocalizedOffsetShort,
    /// The long generic non-location format, as in
    /// “Pacific Time”.
    GenericLong,
    /// The short generic non-location format, as in
    /// “PT”.
    GenericShort,
    /// The location format, as in
    /// “Los Angeles time”.
    Location,
    /// The exemplar city format, as in
    /// “Los Angeles”.
    ExemplarCity,
}

impl ZoneStyle {
    /// All values of this enumeration.
    pub const VALUES: &[Self] = &[
        Self::SpecificLong,
        Self::SpecificShort,
        Self::LocalizedOffsetLong,
        Self::LocalizedOffsetShort,
        Self::GenericLong,
        Self::GenericShort,
        Self::Location,
        Self::ExemplarCity,
    ];
}

/// An error that occurs when creating a [field set](crate::fieldsets) from a builder.
// Not Copy: one of the variants contains a non-Copy type
#[derive(Debug, Clone, displaydoc::Display)]
#[ignore_extra_doc_attributes] // lines after the first won't go into `impl Display`
#[non_exhaustive]
pub enum BuilderError {
    /// The builder needs [`DateFields`] in order to build the specified field set.
    ///
    /// This variant is also returned when building a composite field set if none of the
    /// possible required options were set (date fields, time precision, zone style).
    MissingDateFields,
    /// The builder needs [`TimePrecision`] in order to build the specified field set.
    MissingTimePrecision,
    /// The builder needs [`ZoneStyle`] in order to build the specified field set.
    MissingZoneStyle,
    /// The value in [`DateFields`] is not a valid for the specified field set.
    ///
    /// This can happen if, for example:
    ///
    /// - You requested [`DateFieldSet`] but the fields are for a calendar period
    /// - You requested [`CalendarPeriodFieldSet`] but the fields are for a date
    /// - You requested a field set with time but the fields are for a calendar period
    InvalidDateFields,
    /// Superfluous options were specified.
    ///
    /// For example, you cannot set a [`YearStyle`] unless the field set contains years.
    ///
    /// The options that were _not_ read are returned back to the user.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// builder.length = Some(Length::Short);
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// builder.year_style = Some(YearStyle::WithEra);
    ///
    /// let err = builder.build_composite().unwrap_err();
    ///
    /// let BuilderError::SuperfluousOptions(superfluous_options) = err else {
    ///     panic!("error type should be SuperfluousOptions");
    /// };
    ///
    /// assert!(superfluous_options.year_style.is_some());
    /// assert!(superfluous_options.time_precision.is_none());
    /// ```
    ///
    /// [`YearStyle`] requires a field set with years:
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// builder.date_fields = Some(DateFields::MD);
    /// builder.year_style = Some(YearStyle::WithEra);
    /// let err = builder.build_composite().unwrap_err();
    ///
    /// let BuilderError::SuperfluousOptions(superfluous_options) = err else {
    ///     panic!("error type should be SuperfluousOptions");
    /// };
    ///
    /// assert!(superfluous_options.year_style.is_some());
    /// ```
    SuperfluousOptions(FieldSetBuilder),
}

impl core::error::Error for BuilderError {}

/// Serde impls: We can't directly use `derive(Serialize)` and also hide null fields
/// due to <https://github.com/serde-rs/serde/issues/2191>
#[cfg(all(feature = "serde", feature = "unstable"))]
mod _serde {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct FieldSetBuilderHuman {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub length: Option<Length>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub date_fields: Option<DateFields>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_precision: Option<TimePrecision>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub zone_style: Option<ZoneStyle>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub alignment: Option<Alignment>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub year_style: Option<YearStyle>,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct FieldSetBuilderMachine {
        pub length: Option<Length>,
        pub date_fields: Option<DateFields>,
        pub time_precision: Option<TimePrecision>,
        pub zone_style: Option<ZoneStyle>,
        pub alignment: Option<Alignment>,
        pub year_style: Option<YearStyle>,
    }

    /// Serialization for [`FieldSetBuilder`].
    ///
    /// ✨ *Enabled with the `serde` and `unstable` Cargo features.*
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// builder.date_fields = Some(DateFields::YMD);
    /// builder.length = Some(Length::Medium);
    /// builder.year_style = Some(YearStyle::WithEra);
    ///
    /// let json_str = serde_json::to_string(&builder).unwrap();
    ///
    /// assert_eq!(
    ///     json_str,
    ///     r#"{"length":"medium","dateFields":"YMD","yearStyle":"withEra"}"#
    /// );
    ///
    /// let json_parsed = serde_json::from_str(&json_str).unwrap();
    ///
    /// assert_eq!(builder, json_parsed);
    /// ```
    impl Serialize for FieldSetBuilder {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let FieldSetBuilder {
                length,
                date_fields,
                time_precision,
                zone_style,
                alignment,
                year_style,
            } = *self;
            if serializer.is_human_readable() {
                FieldSetBuilderHuman {
                    length,
                    date_fields,
                    time_precision,
                    zone_style,
                    alignment,
                    year_style,
                }
                .serialize(serializer)
            } else {
                FieldSetBuilderMachine {
                    length,
                    date_fields,
                    time_precision,
                    zone_style,
                    alignment,
                    year_style,
                }
                .serialize(serializer)
            }
        }
    }

    /// Deserialization for [`FieldSetBuilder`].
    ///
    /// ✨ *Enabled with the `serde` and `unstable` Cargo features.*
    ///
    /// For an example, see the `Serialize` impl.
    impl<'de> Deserialize<'de> for FieldSetBuilder {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            // Note: the Deserialize impls are the same. We could even derive this
            // directly on FieldSetBuilder.
            let FieldSetBuilderHuman {
                length,
                date_fields,
                time_precision,
                zone_style,
                alignment,
                year_style,
            } = FieldSetBuilderHuman::deserialize(deserializer)?;
            Ok(FieldSetBuilder {
                length,
                date_fields,
                time_precision,
                zone_style,
                alignment,
                year_style,
            })
        }
    }
}

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
            None => return Err(BuilderError::MissingDateFields),
        };
        Ok(field_set)
    }

    /// Builds a [`DateFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_date(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(
    ///     builder.clone().build_date(),
    ///     Err(BuilderError::InvalidDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(builder.clone().build_date(), Ok(_)));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_date(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_date(mut self) -> Result<DateFieldSet, BuilderError> {
        let date_field_set = match self.build_date_or_calendar_period_without_checking_options()? {
            DateOrCalendarPeriodFieldSet::Date(fs) => fs,
            DateOrCalendarPeriodFieldSet::CalendarPeriod(_) => {
                return Err(BuilderError::InvalidDateFields)
            }
        };
        self.check_options_consumed()?;
        Ok(date_field_set)
    }

    /// Builds a [`CalendarPeriodFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_calendar_period(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(
    ///     builder.clone().build_calendar_period(),
    ///     Err(BuilderError::InvalidDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(builder.clone().build_calendar_period(), Ok(_)));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_calendar_period(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_calendar_period(mut self) -> Result<CalendarPeriodFieldSet, BuilderError> {
        let calendar_period_field_set = match self
            .build_date_or_calendar_period_without_checking_options()?
        {
            DateOrCalendarPeriodFieldSet::Date(_) => return Err(BuilderError::InvalidDateFields),
            DateOrCalendarPeriodFieldSet::CalendarPeriod(fs) => fs,
        };
        self.check_options_consumed()?;
        Ok(calendar_period_field_set)
    }

    /// Builds a [`TimeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_time(),
    ///     Err(BuilderError::MissingTimePrecision)
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(builder.clone().build_time(), Ok(_)));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(
    ///     builder.clone().build_time(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_time(mut self) -> Result<TimeFieldSet, BuilderError> {
        if self.time_precision.is_none() {
            return Err(BuilderError::MissingTimePrecision);
        }
        let time_field_set = TimeFieldSet::T(fieldsets::T::take_from_builder(&mut self));
        self.check_options_consumed()?;
        Ok(time_field_set)
    }

    fn build_zone_without_checking_options(&mut self) -> Result<ZoneFieldSet, BuilderError> {
        let zone_field_set = match self.zone_style.take() {
            Some(ZoneStyle::SpecificShort) => {
                ZoneFieldSet::SpecificShort(fieldsets::zone::SpecificShort)
            }
            Some(ZoneStyle::SpecificLong) => {
                ZoneFieldSet::SpecificLong(fieldsets::zone::SpecificLong)
            }
            Some(ZoneStyle::LocalizedOffsetLong) => {
                ZoneFieldSet::LocalizedOffsetLong(fieldsets::zone::LocalizedOffsetLong)
            }
            Some(ZoneStyle::LocalizedOffsetShort) => {
                ZoneFieldSet::LocalizedOffsetShort(fieldsets::zone::LocalizedOffsetShort)
            }
            Some(ZoneStyle::GenericLong) => ZoneFieldSet::GenericLong(fieldsets::zone::GenericLong),
            Some(ZoneStyle::GenericShort) => {
                ZoneFieldSet::GenericShort(fieldsets::zone::GenericShort)
            }
            Some(ZoneStyle::Location) => ZoneFieldSet::Location(fieldsets::zone::Location),
            Some(ZoneStyle::ExemplarCity) => {
                ZoneFieldSet::ExemplarCity(fieldsets::zone::ExemplarCity)
            }
            None => return Err(BuilderError::MissingZoneStyle),
        };
        Ok(zone_field_set)
    }

    /// Builds a [`ZoneFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_zone(),
    ///     Err(BuilderError::MissingZoneStyle)
    /// ));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(builder.clone().build_zone(), Ok(_)));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_zone(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_zone(mut self) -> Result<ZoneFieldSet, BuilderError> {
        let zone_field_set = self.build_zone_without_checking_options()?;
        self.check_options_consumed()?;
        Ok(zone_field_set)
    }

    /// Builds a [`DateAndTimeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_date_and_time(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(
    ///     builder.clone().build_date_and_time(),
    ///     Err(BuilderError::MissingTimePrecision)
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_date_and_time(),
    ///     Err(BuilderError::InvalidDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(builder.clone().build_date_and_time(), Ok(_)));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(
    ///     builder.clone().build_date_and_time(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_date_and_time(mut self) -> Result<DateAndTimeFieldSet, BuilderError> {
        let Some(date_fields) = self.date_fields.take() else {
            return Err(BuilderError::MissingDateFields);
        };
        if self.time_precision.is_none() {
            return Err(BuilderError::MissingTimePrecision);
        }
        let date_and_time_field_set = match date_fields {
            DateFields::D => DateAndTimeFieldSet::DT(fieldsets::DT::take_from_builder(&mut self)),
            DateFields::MD => {
                DateAndTimeFieldSet::MDT(fieldsets::MDT::take_from_builder(&mut self))
            }
            DateFields::YMD => {
                DateAndTimeFieldSet::YMDT(fieldsets::YMDT::take_from_builder(&mut self))
            }
            DateFields::DE => {
                DateAndTimeFieldSet::DET(fieldsets::DET::take_from_builder(&mut self))
            }
            DateFields::MDE => {
                DateAndTimeFieldSet::MDET(fieldsets::MDET::take_from_builder(&mut self))
            }
            DateFields::YMDE => {
                DateAndTimeFieldSet::YMDET(fieldsets::YMDET::take_from_builder(&mut self))
            }
            DateFields::E => DateAndTimeFieldSet::ET(fieldsets::ET::take_from_builder(&mut self)),
            DateFields::M | DateFields::YM | DateFields::Y => {
                return Err(BuilderError::InvalidDateFields)
            }
        };
        self.check_options_consumed()?;
        Ok(date_and_time_field_set)
    }

    /// Builds a [`CompositeDateTimeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_composite_datetime(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(
    ///     builder.clone().build_composite_datetime(),
    ///     Ok(CompositeDateTimeFieldSet::CalendarPeriod(_))
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_composite_datetime(),
    ///     Err(BuilderError::InvalidDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(
    ///     builder.clone().build_composite_datetime(),
    ///     Ok(CompositeDateTimeFieldSet::DateTime(_))
    /// ));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(
    ///     builder.clone().build_composite_datetime(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
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
            (false, false) => Err(BuilderError::MissingDateFields),
        }
    }

    /// Builds a [`Combo`] for a zoned date.
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date(),
    ///     Err(BuilderError::MissingZoneStyle)
    /// ));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date(),
    ///     Err(BuilderError::InvalidDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(builder.clone().build_zoned_date(), Ok(_)));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_zoned_date(mut self) -> Result<ZonedDateFieldSet, BuilderError> {
        let zone_field_set = self.build_zone_without_checking_options()?;
        let date_field_set = self.build_date()?;
        Ok(date_field_set.with_zone(zone_field_set))
    }

    /// Builds a [`Combo`] for a zoned time.
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_zoned_time(),
    ///     Err(BuilderError::MissingZoneStyle)
    /// ));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_time(),
    ///     Err(BuilderError::MissingTimePrecision)
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(builder.clone().build_zoned_time(), Ok(_)));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_time(),
    ///     Err(BuilderError::SuperfluousOptions(_))
    /// ));
    /// ```
    pub fn build_zoned_time(mut self) -> Result<ZonedTimeFieldSet, BuilderError> {
        let zone_field_set = self.build_zone_without_checking_options()?;
        let time_field_set = self.build_time()?;
        Ok(time_field_set.with_zone(zone_field_set))
    }

    /// Builds a [`Combo`] for a zoned date and time.
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date_and_time(),
    ///     Err(BuilderError::MissingZoneStyle)
    /// ));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date_and_time(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date_and_time(),
    ///     Err(BuilderError::MissingTimePrecision)
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_zoned_date_and_time(),
    ///     Err(BuilderError::InvalidDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(builder.clone().build_zoned_date_and_time(), Ok(_)));
    /// ```
    pub fn build_zoned_date_and_time(mut self) -> Result<ZonedDateAndTimeFieldSet, BuilderError> {
        let zone_field_set = self.build_zone_without_checking_options()?;
        let datetime_field_set = self.build_date_and_time()?;
        Ok(datetime_field_set.with_zone(zone_field_set))
    }

    /// Builds a [`CompositeFieldSet`].
    ///
    /// An error will occur if incompatible fields or options were set in the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::fieldsets;
    /// use icu::datetime::fieldsets::builder::*;
    /// use icu::datetime::fieldsets::enums::*;
    /// use icu::datetime::options::*;
    ///
    /// let mut builder = FieldSetBuilder::new();
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Err(BuilderError::MissingDateFields)
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YM);
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::CalendarPeriod(_))
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::Date(_))
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::DateTime(_))
    /// ));
    ///
    /// builder.date_fields = None;
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::Time(_))
    /// ));
    ///
    /// builder.zone_style = Some(ZoneStyle::SpecificLong);
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::TimeZone(_))
    /// ));
    ///
    /// builder.time_precision = None;
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::Zone(_))
    /// ));
    ///
    /// builder.date_fields = Some(DateFields::YMD);
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::DateZone(_))
    /// ));
    ///
    /// builder.time_precision = Some(TimePrecision::Minute);
    /// assert!(matches!(
    ///     builder.clone().build_composite(),
    ///     Ok(CompositeFieldSet::DateTimeZone(_))
    /// ));
    /// ```
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
            (false, false, false) => Err(BuilderError::MissingDateFields),
        }
    }

    fn check_options_consumed(self) -> Result<(), BuilderError> {
        if self != Self::default() {
            Err(BuilderError::SuperfluousOptions(self))
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
        ZoneStyle::SpecificLong,
        ZoneStyle::SpecificShort,
        ZoneStyle::LocalizedOffsetLong,
        ZoneStyle::LocalizedOffsetShort,
        ZoneStyle::GenericLong,
        ZoneStyle::GenericShort,
        ZoneStyle::Location,
        ZoneStyle::ExemplarCity,
    ];

    #[cfg(all(feature = "serde", feature = "unstable"))]
    fn check_serde(value: &FieldSetBuilder) {
        let json_str = serde_json::to_string(value).unwrap();
        let json_parsed: FieldSetBuilder = serde_json::from_str(&json_str).unwrap();
        assert_eq!(value, &json_parsed);
        let bincode_bytes = bincode::serialize(value).unwrap();
        let bincode_parsed: FieldSetBuilder = bincode::deserialize(&bincode_bytes).unwrap();
        assert_eq!(value, &bincode_parsed);
    }

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
            builder.clone().build_composite().unwrap();
            #[cfg(all(feature = "serde", feature = "unstable"))]
            check_serde(&builder);
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
            builder.clone().build_composite().unwrap();
            #[cfg(all(feature = "serde", feature = "unstable"))]
            check_serde(&builder);
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
        builder.clone().build_composite().unwrap();
        #[cfg(all(feature = "serde", feature = "unstable"))]
        check_serde(&builder);
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
            builder.clone().build_composite().unwrap();
            #[cfg(all(feature = "serde", feature = "unstable"))]
            check_serde(&builder);
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
            builder.clone().build_composite().unwrap();
            #[cfg(all(feature = "serde", feature = "unstable"))]
            check_serde(&builder);
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
            builder.clone().build_composite().unwrap_err();
            #[cfg(all(feature = "serde", feature = "unstable"))]
            check_serde(&builder);
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
                builder.clone().build_composite().unwrap();
                #[cfg(all(feature = "serde", feature = "unstable"))]
                check_serde(&builder);
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
                builder.clone().build_composite().unwrap_err();
                #[cfg(all(feature = "serde", feature = "unstable"))]
                check_serde(&builder);
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
            builder.clone().build_composite().unwrap();
            #[cfg(all(feature = "serde", feature = "unstable"))]
            check_serde(&builder);
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
                builder.clone().build_composite().unwrap();
                #[cfg(all(feature = "serde", feature = "unstable"))]
                check_serde(&builder);
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
                builder.clone().build_composite().unwrap_err();
                #[cfg(all(feature = "serde", feature = "unstable"))]
                check_serde(&builder);
            }
        }
    }
}
