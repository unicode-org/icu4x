// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fieldsets::{self, enums::*};
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

#[derive(Debug, displaydoc::Display)]
#[non_exhaustive]
pub enum BuilderError {
    /// The specified fields are incompatible with the desired field set
    InvalidFields,
    /// The specified options are incompatible with the specified field set
    InvalidOptions,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct FieldSetBuilder {
    pub length: Option<Length>,
    pub date_fields: Option<DateFields>,
    pub time_precision: Option<TimePrecision>,
    pub zone_style: Option<ZoneStyle>,
    pub alignment: Option<Alignment>,
    pub year_style: Option<YearStyle>,
}

const DEFAULT_LENGTH: Length = Length::Medium;

impl FieldSetBuilder {
    pub fn build_date(mut self) -> Result<DateFieldSet, BuilderError> {
        let date_field_set = match self.date_fields.take() {
            Some(DateFields::D) => {
                DateFieldSet::D(fieldsets::D::take_from_builder(&mut self, DEFAULT_LENGTH))
            }
            Some(DateFields::MD) => {
                DateFieldSet::MD(fieldsets::MD::take_from_builder(&mut self, DEFAULT_LENGTH))
            }
            Some(DateFields::YMD) => {
                DateFieldSet::YMD(fieldsets::YMD::take_from_builder(&mut self, DEFAULT_LENGTH))
            }
            Some(DateFields::DE) => {
                DateFieldSet::DE(fieldsets::DE::take_from_builder(&mut self, DEFAULT_LENGTH))
            }
            Some(DateFields::MDE) => {
                DateFieldSet::MDE(fieldsets::MDE::take_from_builder(&mut self, DEFAULT_LENGTH))
            }
            Some(DateFields::YMDE) => DateFieldSet::YMDE(fieldsets::YMDE::take_from_builder(
                &mut self,
                DEFAULT_LENGTH,
            )),
            Some(DateFields::E) => {
                DateFieldSet::E(fieldsets::E::take_from_builder(&mut self, DEFAULT_LENGTH))
            }
            Some(DateFields::M) | Some(DateFields::YM) | Some(DateFields::Y) | None => {
                return Err(BuilderError::InvalidFields)
            }
        };
        self.check_options_consumed()?;
        Ok(date_field_set)
    }

    pub fn build_time(mut self) -> Result<TimeFieldSet, BuilderError> {
        let time_field_set =
            TimeFieldSet::T(fieldsets::T::take_from_builder(&mut self, DEFAULT_LENGTH));
        self.check_options_consumed()?;
        Ok(time_field_set)
    }

    pub fn build_datetime(mut self) -> Result<CompositeDateTimeFieldSet, BuilderError> {
        let datetime_field_set = match (self.date_fields.take(), self.time_precision.is_some()) {
            (Some(DateFields::D), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::D(
                fieldsets::D::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::MD), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::MD(
                fieldsets::MD::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::YMD), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::YMD(
                fieldsets::YMD::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::DE), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::DE(
                fieldsets::DE::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::MDE), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::MDE(
                fieldsets::MDE::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::YMDE), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::YMDE(
                fieldsets::YMDE::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::E), false) => CompositeDateTimeFieldSet::Date(DateFieldSet::E(
                fieldsets::E::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::M), false) => {
                CompositeDateTimeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::M(
                    fieldsets::M::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::YM), false) => {
                CompositeDateTimeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::YM(
                    fieldsets::YM::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::Y), false) => {
                CompositeDateTimeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::Y(
                    fieldsets::Y::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::D), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::DT(
                    fieldsets::DT::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::MD), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::MDT(
                    fieldsets::MDT::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::YMD), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::YMDT(
                    fieldsets::YMDT::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::DE), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::DET(
                    fieldsets::DET::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::MDE), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::MDET(
                    fieldsets::MDET::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::YMDE), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::YMDET(
                    fieldsets::YMDET::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (Some(DateFields::E), true) => {
                CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(
                    fieldsets::ET::take_from_builder(&mut self, DEFAULT_LENGTH),
                ))
            }
            (None, true) => CompositeDateTimeFieldSet::Time(TimeFieldSet::T(
                fieldsets::T::take_from_builder(&mut self, DEFAULT_LENGTH),
            )),
            (Some(DateFields::M), true)
            | (Some(DateFields::YM), true)
            | (Some(DateFields::Y), true)
            | (None, false) => {
                return Err(BuilderError::InvalidFields);
            }
        };
        self.check_options_consumed()?;
        Ok(datetime_field_set)
    }

    pub fn build(self) -> Result<CompositeFieldSet, BuilderError> {
        todo!()
    }

    fn check_options_consumed(self) -> Result<(), BuilderError> {
        if self != Self::default() {
            Err(BuilderError::InvalidOptions)
        } else {
            Ok(())
        }
    }
}
