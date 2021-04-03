// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::TimeZoneInput;
use crate::error::DateTimeFormatError as Error;
use crate::fields::{self, FieldSymbol};
use crate::invalid_pattern_symbol;
use crate::mock::timezone::{IsoFormat, IsoMinutes, IsoSeconds};
use crate::pattern::PatternItem;
use crate::TimeZoneFormat;
use std::fmt;
use writeable::Writeable;

pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormat<'l>,
    pub(crate) time_zone: &'l T,
}

impl<'l, T> Writeable for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        write_pattern(&self.time_zone_format, self.time_zone, sink).map_err(|_| std::fmt::Error)
    }

    // TODO(#489): Implement write_len
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_pattern(&self.time_zone_format, self.time_zone, f).map_err(|_| std::fmt::Error)
    }
}

pub fn write_pattern<T, W>(
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    let pattern = &time_zone_format.pattern;
    for item in pattern.items() {
        match item {
            PatternItem::Field(field) => write_field(&field, time_zone_format, time_zone, w)?,
            PatternItem::Literal(l) => w.write_str(l)?,
        }
    }
    Ok(())
}

/// Write fields according to the UTS-35 specification.
/// https://unicode.org/reports/tr35/tr35-dates.html#dfst-zone
pub(super) fn write_field<T, W>(
    field: &fields::Field,
    time_zone_format: &TimeZoneFormat,
    time_zone: &T,
    w: &mut W,
) -> Result<(), Error>
where
    T: TimeZoneInput,
    W: fmt::Write + ?Sized,
{
    if let FieldSymbol::TimeZone(zone_symbol) = field.symbol {
        let s = match zone_symbol {
            fields::TimeZone::LowerZ => match u8::from(field.length) {
                1..=3 => time_zone_format
                    .short_specific_non_location_format(time_zone)
                    .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone)),
                4 => time_zone_format
                    .long_specific_non_location_format(time_zone)
                    .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone)),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            fields::TimeZone::UpperZ => match u8::from(field.length) {
                1..=3 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::Basic,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                ),
                4 => time_zone_format.localized_gmt_format(time_zone),
                5 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::UtcExtended,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                ),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            fields::TimeZone::UpperO => match u8::from(field.length) {
                1..=4 => time_zone_format.localized_gmt_format(time_zone),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            fields::TimeZone::LowerV => match u8::from(field.length) {
                1 => time_zone_format
                    .short_generic_non_location_format(time_zone)
                    .or_else(|| time_zone_format.generic_location_format(time_zone))
                    .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone)),
                4 => time_zone_format
                    .long_generic_non_location_format(time_zone)
                    .or_else(|| time_zone_format.generic_location_format(time_zone))
                    .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone)),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            fields::TimeZone::UpperV => match u8::from(field.length) {
                1 => todo!("#606 (BCP-47 identifiers)"),
                2 => todo!("#606 (BCP-47 identifiers)"),
                3 => time_zone_format
                    .exemplar_city(time_zone)
                    .unwrap_or_else(|| time_zone_format.unknown_city()),
                4 => time_zone_format
                    .generic_location_format(time_zone)
                    .unwrap_or_else(|| time_zone_format.localized_gmt_format(time_zone)),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            fields::TimeZone::LowerX => match u8::from(field.length) {
                1 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::UtcBasic,
                    IsoMinutes::Optional,
                    IsoSeconds::None,
                ),
                2 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::UtcBasic,
                    IsoMinutes::Required,
                    IsoSeconds::None,
                ),
                3 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::UtcExtended,
                    IsoMinutes::Required,
                    IsoSeconds::None,
                ),
                4 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::UtcBasic,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                ),
                5 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::UtcExtended,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                ),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            fields::TimeZone::UpperX => match u8::from(field.length) {
                1 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::Basic,
                    IsoMinutes::Optional,
                    IsoSeconds::None,
                ),
                2 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::Basic,
                    IsoMinutes::Required,
                    IsoSeconds::None,
                ),
                3 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::Extended,
                    IsoMinutes::Required,
                    IsoSeconds::None,
                ),
                4 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::Basic,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                ),
                5 => time_zone.gmt_offset().iso_8601_format(
                    IsoFormat::Extended,
                    IsoMinutes::Required,
                    IsoSeconds::Optional,
                ),
                length => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
        };
        w.write_str(&s)?;
    }
    Ok(())
}
