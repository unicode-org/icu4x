// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldSymbol};
use crate::pattern::{PatternError, PatternItem};
use crate::provider::gregory::patterns::PatternPluralsFromPatternsV1Marker;
use crate::{
    date::TimeZoneInput,
    time_zone::{IsoFormat, IsoMinutes, IsoSeconds, TimeZoneFormat, TimeZoneFormatConfig},
};
use icu_provider::DataPayload;
use writeable::Writeable;

pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormat,
    pub(crate) time_zone: &'l T,
}

impl<'l, T> Writeable for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_zone(self.time_zone_format, self.time_zone, sink).map_err(|_| core::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_zone(self.time_zone_format, self.time_zone, f).map_err(|_| core::fmt::Error)
    }
}

pub(crate) fn write_zone<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    match (&time_zone_format.patterns, &time_zone_format.config) {
        (Some(patterns), None) => write_pattern(time_zone_format, time_zone, patterns, w),
        (None, Some(config)) => write_config(time_zone_format, time_zone, config, w),
        (Some(_), Some(_)) => {
            unreachable!("TimeZoneFormat should have a configuration or a pattern, but found both")
        }
        (None, None) => unreachable!(
            "TimeZoneFormat should have either a configuration or a pattern, but found neither"
        ),
    }
}

fn write_config<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    config: &TimeZoneFormatConfig,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    match config {
        TimeZoneFormatConfig::GenericNonLocationLong => {
            time_zone_format
                .long_generic_non_location_format(w, time_zone)
                .or_else(|_| time_zone_format.generic_location_format(w, time_zone))
                .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?;
        }
        TimeZoneFormatConfig::GenericNonLocationShort => {
            time_zone_format
                .short_generic_non_location_format(w, time_zone)
                .or_else(|_| time_zone_format.generic_location_format(w, time_zone))
                .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?;
        }
        TimeZoneFormatConfig::GenericLocation => {
            time_zone_format
                .generic_location_format(w, time_zone)
                .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?;
        }
        TimeZoneFormatConfig::SpecificNonLocationLong => {
            time_zone_format
                .long_specific_non_location_format(w, time_zone)
                .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?;
        }
        TimeZoneFormatConfig::SpecificNonLocationShort => {
            time_zone_format
                .short_specific_non_location_format(w, time_zone)
                .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?;
        }
        TimeZoneFormatConfig::LocalizedGMT => {
            time_zone_format.localized_gmt_format(w, time_zone)?;
        }
        TimeZoneFormatConfig::Iso8601(iso_format, iso_minutes, iso_seconds) => {
            time_zone_format.iso8601_format(
                w,
                time_zone,
                *iso_format,
                *iso_minutes,
                *iso_seconds,
            )?;
        }
    }
    Ok(())
}

fn write_pattern<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    patterns: &DataPayload<PatternPluralsFromPatternsV1Marker>,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    let pattern = patterns
        .get()
        .0
        .clone()
        .expect_pattern("Expected a single pattern");
    for item in pattern.items.iter() {
        match item {
            PatternItem::Field(field) => write_field(field, time_zone_format, time_zone, w)?,
            PatternItem::Literal(ch) => w.write_char(ch)?,
        }
    }
    Ok(())
}

/// Write fields according to the UTS-35 specification.
/// https://unicode.org/reports/tr35/tr35-dates.html#dfst-zone
pub(super) fn write_field<T, W>(
    field: fields::Field,
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    if let FieldSymbol::TimeZone(zone_symbol) = field.symbol {
        match zone_symbol {
            fields::TimeZone::LowerZ => match field.length.idx() {
                1..=3 => time_zone_format
                    .short_specific_non_location_format(w, time_zone)
                    .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?,
                4 => time_zone_format
                    .long_specific_non_location_format(w, time_zone)
                    .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
            fields::TimeZone::UpperZ => match field.length.idx() {
                1..=3 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::Basic,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                )?,
                4 => time_zone_format.localized_gmt_format(w, time_zone)?,
                5 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::UtcExtended,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                )?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
            fields::TimeZone::UpperO => match field.length.idx() {
                1..=4 => time_zone_format.localized_gmt_format(w, time_zone)?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
            fields::TimeZone::LowerV => match field.length.idx() {
                1 => time_zone_format
                    .short_generic_non_location_format(w, time_zone)
                    .or_else(|_| time_zone_format.generic_location_format(w, time_zone))
                    .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?,
                4 => time_zone_format
                    .long_generic_non_location_format(w, time_zone)
                    .or_else(|_| time_zone_format.generic_location_format(w, time_zone))
                    .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
            fields::TimeZone::UpperV => match field.length.idx() {
                1 => todo!("#606 (BCP-47 identifiers)"),
                2 => todo!("#606 (BCP-47 identifiers)"),
                3 => time_zone_format
                    .exemplar_city(w, time_zone)
                    .or_else(|_| time_zone_format.unknown_city(w))?,
                4 => time_zone_format
                    .generic_location_format(w, time_zone)
                    .or_else(|_| time_zone_format.localized_gmt_format(w, time_zone))?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
            fields::TimeZone::LowerX => match field.length.idx() {
                1 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::UtcBasic,
                    IsoMinutes::Optional,
                    IsoSeconds::Never,
                )?,
                2 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::UtcBasic,
                    IsoMinutes::Required,
                    IsoSeconds::Never,
                )?,
                3 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::UtcExtended,
                    IsoMinutes::Required,
                    IsoSeconds::Never,
                )?,
                4 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::UtcBasic,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                )?,
                5 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::UtcExtended,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                )?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
            fields::TimeZone::UpperX => match field.length.idx() {
                1 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::Basic,
                    IsoMinutes::Optional,
                    IsoSeconds::Never,
                )?,
                2 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::Basic,
                    IsoMinutes::Required,
                    IsoSeconds::Never,
                )?,
                3 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::Extended,
                    IsoMinutes::Required,
                    IsoSeconds::Never,
                )?,
                4 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::Basic,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                )?,
                5 => time_zone_format.iso8601_format(
                    w,
                    time_zone,
                    IsoFormat::Extended,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                )?,
                _ => {
                    return Err(Error::Pattern(PatternError::FieldLengthInvalid(
                        FieldSymbol::TimeZone(zone_symbol),
                    )))
                }
            },
        }
    }
    Ok(())
}
