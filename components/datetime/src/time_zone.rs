// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A formatter specifically for the time zone.

use crate::{
    fields::{FieldLength, TimeZone},
    input::ExtractedInput,
    provider, DateTimeWriteError,
};
use core::fmt::{self, Write};
use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormatter;
use icu_timezone::UtcOffset;
use icu_timezone::ZoneVariant;
use writeable::{Part, Writeable};

/// All time zone styles that this crate can format
#[derive(Debug, Copy, Clone)]
pub(crate) enum ResolvedNeoTimeZoneSkeleton {
    Location,
    GenericShort,
    GenericLong,
    SpecificShort,
    SpecificLong,
    OffsetShort,
    OffsetLong,
    Bcp47Id,
    // UTS 35 defines 10 variants of ISO-8601-style time zone formats.
    // They don't have their own names, so they are identified here by
    // their datetime pattern strings.
    Isox,
    Isoxx,
    Isoxxx,
    Isoxxxx,
    Isoxxxxx,
    IsoX,
    IsoXX,
    IsoXXX,
    IsoXXXX,
    IsoXXXXX,
    // TODO:
    // `VV` "America/Los_Angeles"
    // `VVV` "Los Angeles"
}

impl ResolvedNeoTimeZoneSkeleton {
    pub(crate) fn from_field(field_symbol: TimeZone, field_length: FieldLength) -> Option<Self> {
        crate::tz_registry::field_to_resolved(field_symbol, field_length)
    }
    pub(crate) fn to_field(self) -> crate::fields::Field {
        crate::tz_registry::resolved_to_field(self)
    }

    pub(crate) fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let last_resort = Iso8601Format {
            format: IsoFormat::Basic,
            minutes: IsoMinutes::Required,
            seconds: IsoSeconds::Optional,
        };
        #[allow(clippy::redundant_pattern_matching)]
        match self {
            // `z..zzz`
            ResolvedNeoTimeZoneSkeleton::SpecificShort => {
                if let Err(e) =
                    SpecificNonLocationShortFormat.format(sink, input, data_payloads, fdf)?
                {
                    if let Err(oe) =
                        ShortLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                    {
                        if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset")) {
                            sink.with_part(Part::ERROR, |s| {
                                last_resort.format(s, input.offset).map(|_| ())
                            })?;
                        }
                        return Ok(Err(e.to_write_error(self.to_field())));
                    }
                }
                Ok(Ok(()))
            }
            // `zzzz`
            ResolvedNeoTimeZoneSkeleton::SpecificLong => {
                if let Err(e) =
                    SpecificNonLocationLongFormat.format(sink, input, data_payloads, fdf)?
                {
                    if let Err(oe) =
                        LongLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                    {
                        if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset")) {
                            sink.with_part(Part::ERROR, |s| {
                                last_resort.format(s, input.offset).map(|_| ())
                            })?;
                        }
                        return Ok(Err(e.to_write_error(self.to_field())));
                    }
                }
                Ok(Ok(()))
            }
            // 'v'
            ResolvedNeoTimeZoneSkeleton::GenericShort => {
                if let Err(e) =
                    GenericNonLocationShortFormat.format(sink, input, data_payloads, fdf)?
                {
                    if let Err(_) = GenericLocationFormat.format(sink, input, data_payloads, fdf)? {
                        if let Err(oe) =
                            ShortLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                        {
                            if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset"))
                            {
                                sink.with_part(Part::ERROR, |s| {
                                    last_resort.format(s, input.offset).map(|_| ())
                                })?;
                            }
                            return Ok(Err(e.to_write_error(self.to_field())));
                        }
                    }
                }
                Ok(Ok(()))
            }
            // 'vvvv'
            ResolvedNeoTimeZoneSkeleton::GenericLong => {
                if let Err(e) =
                    GenericNonLocationLongFormat.format(sink, input, data_payloads, fdf)?
                {
                    if let Err(_) = GenericLocationFormat.format(sink, input, data_payloads, fdf)? {
                        if let Err(oe) =
                            LongLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                        {
                            if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset"))
                            {
                                sink.with_part(Part::ERROR, |s| {
                                    last_resort.format(s, input.offset).map(|_| ())
                                })?;
                            }
                            return Ok(Err(e.to_write_error(self.to_field())));
                        }
                    }
                };
                Ok(Ok(()))
            }
            // 'VVVV'
            ResolvedNeoTimeZoneSkeleton::Location => {
                if let Err(e) = GenericLocationFormat.format(sink, input, data_payloads, fdf)? {
                    if let Err(oe) =
                        LongLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                    {
                        if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset")) {
                            sink.with_part(Part::ERROR, |s| {
                                last_resort.format(s, input.offset).map(|_| ())
                            })?;
                        }
                        return Ok(Err(e.to_write_error(self.to_field())));
                    }
                }
                Ok(Ok(()))
            }
            // `O`, `ZZZZ`
            ResolvedNeoTimeZoneSkeleton::OffsetShort => {
                if let Err(oe) =
                    ShortLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                {
                    if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset")) {
                        sink.with_part(Part::ERROR, |s| {
                            last_resort.format(s, input.offset).map(|_| ())
                        })?;
                    }
                    return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
                }
                Ok(Ok(()))
            }
            // `OOOO`
            ResolvedNeoTimeZoneSkeleton::OffsetLong => {
                if let Err(oe) =
                    LongLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                {
                    if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset")) {
                        sink.with_part(Part::ERROR, |s| {
                            last_resort.format(s, input.offset).map(|_| ())
                        })?;
                    }
                    return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
                }
                Ok(Ok(()))
            }
            // 'V'
            ResolvedNeoTimeZoneSkeleton::Bcp47Id => {
                if let Err(e) = Bcp47IdFormat.format(sink, input, data_payloads, fdf)? {
                    if let Err(oe) =
                        LongLocalizedOffsetFormat.format(sink, input, data_payloads, fdf)?
                    {
                        if !matches!(oe, FormatTimeZoneError::MissingInputField("zone_offset")) {
                            sink.with_part(Part::ERROR, |s| {
                                last_resort.format(s, input.offset).map(|_| ())
                            })?;
                        }
                        return Ok(Err(e.to_write_error(self.to_field())));
                    }
                }
                Ok(Ok(()))
            }
            // 'X'
            ResolvedNeoTimeZoneSkeleton::IsoX => Iso8601Format {
                format: IsoFormat::UtcBasic,
                minutes: IsoMinutes::Optional,
                seconds: IsoSeconds::Never,
            }
            .format(sink, input.offset),
            // 'XX'
            ResolvedNeoTimeZoneSkeleton::IsoXX => Iso8601Format {
                format: IsoFormat::UtcBasic,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Never,
            }
            .format(sink, input.offset),
            // 'XXX'
            ResolvedNeoTimeZoneSkeleton::IsoXXX => Iso8601Format {
                format: IsoFormat::UtcExtended,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Never,
            }
            .format(sink, input.offset),
            // 'XXXX'
            ResolvedNeoTimeZoneSkeleton::IsoXXXX => Iso8601Format {
                format: IsoFormat::UtcBasic,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Optional,
            }
            .format(sink, input.offset),
            // 'XXXXX', 'ZZZZZ'
            ResolvedNeoTimeZoneSkeleton::IsoXXXXX => Iso8601Format {
                format: IsoFormat::UtcExtended,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Optional,
            }
            .format(sink, input.offset),
            // 'x'
            ResolvedNeoTimeZoneSkeleton::Isox => Iso8601Format {
                format: IsoFormat::Basic,
                minutes: IsoMinutes::Optional,
                seconds: IsoSeconds::Never,
            }
            .format(sink, input.offset),
            // 'xx'
            ResolvedNeoTimeZoneSkeleton::Isoxx => Iso8601Format {
                format: IsoFormat::Basic,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Never,
            }
            .format(sink, input.offset),
            // 'xxx'
            ResolvedNeoTimeZoneSkeleton::Isoxxx => Iso8601Format {
                format: IsoFormat::Extended,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Never,
            }
            .format(sink, input.offset),
            // 'xxxx', 'Z', 'ZZ', 'ZZZ'
            ResolvedNeoTimeZoneSkeleton::Isoxxxx => Iso8601Format {
                format: IsoFormat::Basic,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Optional,
            }
            .format(sink, input.offset),
            // 'xxxxx', 'ZZZZZ'
            ResolvedNeoTimeZoneSkeleton::Isoxxxxx => Iso8601Format {
                format: IsoFormat::Extended,
                minutes: IsoMinutes::Required,
                seconds: IsoSeconds::Optional,
            }
            .format(sink, input.offset),
        }
    }
}

/// A container contains all data payloads for time zone formatting (borrowed version).
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct TimeZoneDataPayloadsBorrowed<'a> {
    /// The data that contains meta information about how to display content.
    pub(crate) essentials: Option<&'a provider::time_zones::TimeZoneEssentialsV1<'a>>,
    /// The exemplar cities for time zones.
    pub(crate) exemplar_cities: Option<&'a provider::time_zones::ExemplarCitiesV1<'a>>,
    /// The generic long metazone names, e.g. Pacific Time
    pub(crate) mz_generic_long: Option<&'a provider::time_zones::MetazoneGenericNamesV1<'a>>,
    /// The generic short metazone names, e.g. PT
    pub(crate) mz_generic_short: Option<&'a provider::time_zones::MetazoneGenericNamesV1<'a>>,
    /// The specific long metazone names, e.g. Pacific Daylight Time
    pub(crate) mz_specific_long: Option<&'a provider::time_zones::MetazoneSpecificNamesV1<'a>>,
    /// The specific short metazone names, e.g. Pacific Daylight Time
    pub(crate) mz_specific_short: Option<&'a provider::time_zones::MetazoneSpecificNamesV1<'a>>,
}

/// Determines which ISO-8601 format should be used to format the timezone offset.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum IsoFormat {
    /// ISO-8601 Basic Format.
    /// Formats zero-offset numerically.
    /// e.g. +0500, +0000
    Basic,

    /// ISO-8601 Extended Format.
    /// Formats zero-offset numerically.
    /// e.g. +05:00, +00:00
    Extended,

    /// ISO-8601 Basic Format.
    /// Formats zero-offset with the ISO-8601 UTC indicator: "Z"
    /// e.g. +0500, Z
    UtcBasic,

    /// ISO-8601 Extended Format.
    /// Formats zero-offset with the ISO-8601 UTC indicator: "Z"
    /// e.g. +05:00, Z
    UtcExtended,
}

/// Whether the minutes field should be optional or required in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum IsoMinutes {
    /// Minutes are always displayed.
    Required,

    /// Minutes are displayed only if they are non-zero.
    Optional,
}

/// Whether the seconds field should be optional or excluded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum IsoSeconds {
    /// Seconds are displayed only if they are non-zero.
    Optional,

    /// Seconds are not displayed.
    Never,
}

/// An enum for time zone fallback formats.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
#[derive(Default)]
pub enum FallbackFormat {
    /// The localized offset format for time zone format fallback.
    ///
    /// See [UTS 35 on Dates](https://unicode.org/reports/tr35/tr35-dates.html#71-time-zone-format-terminology) for more information.
    #[default]
    LocalizedOffset,
}

/// A bag of options to define how time zone will be formatted.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct TimeZoneFormatterOptions {
    /// The time zone format fallback option.
    ///
    /// See [UTS 35 on Dates](https://unicode.org/reports/tr35/tr35-dates.html#71-time-zone-format-terminology) for more information.
    pub fallback_format: FallbackFormat,
}

impl From<FallbackFormat> for TimeZoneFormatterOptions {
    fn from(fallback_format: FallbackFormat) -> Self {
        Self { fallback_format }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum FormatTimeZoneError {
    MissingZoneSymbols,
    MissingFixedDecimalFormatter,
    NameNotFound,
    MissingInputField(&'static str),
}

impl FormatTimeZoneError {
    fn to_write_error(self, field: crate::fields::Field) -> DateTimeWriteError {
        match self {
            Self::MissingFixedDecimalFormatter => DateTimeWriteError::MissingFixedDecimalFormatter,
            Self::MissingInputField(f) => DateTimeWriteError::MissingInputField(f),
            Self::MissingZoneSymbols => DateTimeWriteError::MissingZoneSymbols,
            Self::NameNotFound => DateTimeWriteError::MissingNames(field),
        }
    }
}

// Pacific Time
struct GenericNonLocationLongFormat;

impl GenericNonLocationLongFormat {
    /// Writes the time zone in long generic non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(metazone_id) = input.metazone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("metazone")));
        };
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(names) = data_payloads.mz_generic_long.as_ref() else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(name) = names
            .overrides
            .get(&time_zone_id)
            .or_else(|| names.defaults.get(&metazone_id))
        else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// PT
struct GenericNonLocationShortFormat;

impl GenericNonLocationShortFormat {
    /// Writes the time zone in short generic non-location format as defined by the UTS-35 spec.
    /// e.g. PT
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(metazone_id) = input.metazone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("metazone")));
        };
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(names) = data_payloads.mz_generic_short.as_ref() else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(name) = names
            .overrides
            .get(&time_zone_id)
            .or_else(|| names.defaults.get(&metazone_id))
        else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// PDT
struct SpecificNonLocationShortFormat;

impl SpecificNonLocationShortFormat {
    /// Writes the time zone in short specific non-location format as defined by the UTS-35 spec.
    /// e.g. PDT
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(zone_variant) = input.zone_variant else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        let Some(metazone_id) = input.metazone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("metazone")));
        };
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(names) = data_payloads.mz_specific_short.as_ref() else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(name) = names
            .overrides
            .get_2d(&time_zone_id, &zone_variant)
            .or_else(|| names.defaults.get_2d(&metazone_id, &zone_variant))
        else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// Pacific Standard Time
struct SpecificNonLocationLongFormat;

impl SpecificNonLocationLongFormat {
    /// Writes the time zone in long specific non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Daylight Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(zone_variant) = input.zone_variant else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        let Some(metazone_id) = input.metazone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("metazone")));
        };
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(names) = data_payloads.mz_specific_long.as_ref() else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(name) = names
            .overrides
            .get_2d(&time_zone_id, &zone_variant)
            .or_else(|| names.defaults.get_2d(&metazone_id, &zone_variant))
        else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

struct UnknownOffset;

impl Writeable for UnknownOffset {
    fn write_to_parts<S: writeable::PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        sink.write_char('+')?;
        sink.with_part(writeable::Part::ERROR, |sink| sink.write_str("{O}"))
    }
}

// UTC+7:00
struct LongLocalizedOffsetFormat;

impl LongLocalizedOffsetFormat {
    /// Writes the time zone in localized offset format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// offset formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// <https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(offset) = input.offset else {
            essentials
                .offset_format
                .interpolate([UnknownOffset])
                .write_to(sink)?;
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        let Some(fdf) = fdf else {
            return Ok(Err(FormatTimeZoneError::MissingFixedDecimalFormatter));
        };
        Ok(if offset.is_zero() {
            sink.write_str(&essentials.offset_zero_format)?;
            Ok(())
        } else {
            struct FormattedHour<'a> {
                format_str: &'a str,
                fdf: &'a FixedDecimalFormatter,
                offset: UtcOffset,
            }

            impl Writeable for FormattedHour<'_> {
                fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
                    &self,
                    sink: &mut S,
                ) -> fmt::Result {
                    let hours = (self.offset.offset_seconds() / 3600).unsigned_abs();
                    let minutes = (self.offset.offset_seconds() / 60 % 60).unsigned_abs();

                    for c in self.format_str.chars() {
                        match c {
                            'H' => self
                                .fdf
                                .format(&FixedDecimal::from(hours).padded_start(2))
                                .write_to(sink)?,
                            'h' => self.fdf.format(&FixedDecimal::from(hours)).write_to(sink)?,
                            'm' => self
                                .fdf
                                .format(&FixedDecimal::from(minutes).padded_start(2))
                                .write_to(sink)?,
                            c => sink.write_char(c)?,
                        }
                    }
                    Ok(())
                }
            }

            essentials
                .offset_format
                .interpolate([FormattedHour {
                    format_str: if offset.is_positive() {
                        &essentials.hour_format.0
                    } else {
                        &essentials.hour_format.1
                    },
                    fdf,
                    offset,
                }])
                .write_to(sink)?;

            Ok(())
        })
    }
}

// UTC+7
struct ShortLocalizedOffsetFormat;

impl ShortLocalizedOffsetFormat {
    /// Writes the time zone in localized offset format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// offset formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// <https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(offset) = input.offset else {
            essentials
                .offset_format
                .interpolate([UnknownOffset])
                .write_to(sink)?;
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        let Some(fdf) = fdf else {
            return Ok(Err(FormatTimeZoneError::MissingFixedDecimalFormatter));
        };
        Ok(if offset.is_zero() {
            sink.write_str(&essentials.offset_zero_format)?;
            Ok(())
        } else {
            struct FormattedHour<'a> {
                format_str: &'a str,
                fdf: &'a FixedDecimalFormatter,
                offset: UtcOffset,
            }

            impl Writeable for FormattedHour<'_> {
                fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
                    &self,
                    sink: &mut S,
                ) -> fmt::Result {
                    let hours = (self.offset.offset_seconds() / 3600).unsigned_abs();
                    let minutes = (self.offset.offset_seconds() / 60 % 60).unsigned_abs();

                    let mut between_hours_and_minutes = false;
                    for c in self.format_str.chars() {
                        match c {
                            'h' | 'H' => {
                                self.fdf.format(&hours.into()).write_to(sink)?;
                                between_hours_and_minutes = true;
                            }
                            'm' => {
                                if minutes != 0 {
                                    self.fdf.format(&minutes.into()).write_to(sink)?;
                                }
                                between_hours_and_minutes = false;
                            }
                            _c if between_hours_and_minutes && minutes == 0 => {}
                            c => sink.write_char(c)?,
                        }
                    }
                    Ok(())
                }
            }

            essentials
                .offset_format
                .interpolate([FormattedHour {
                    format_str: if offset.is_positive() {
                        &essentials.hour_format.0
                    } else {
                        &essentials.hour_format.1
                    },
                    fdf,
                    offset,
                }])
                .write_to(sink)?;

            Ok(())
        })
    }
}

// Los Angeles Time
struct GenericLocationFormat;

impl GenericLocationFormat {
    /// Writes the time zone in generic location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(exemplar_cities) = data_payloads.exemplar_cities else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(location) = exemplar_cities.0.get(&time_zone_id) else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        essentials
            .region_format
            .interpolate([location])
            .write_to(sink)?;

        Ok(Ok(()))
    }
}

// Los Angeles Daylight Time
#[allow(dead_code)]
struct SpecificLocationFormat;

impl SpecificLocationFormat {
    /// Writes the time zone in a specific location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(dead_code)]
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(zone_variant) = input.zone_variant else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(exemplar_cities) = data_payloads.exemplar_cities else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(location) = exemplar_cities.0.get(&time_zone_id) else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        Ok(if zone_variant == ZoneVariant::daylight() {
            essentials
                .region_format_dt
                .interpolate([location])
                .write_to(sink)?;

            Ok(())
        } else if zone_variant == ZoneVariant::standard() {
            essentials
                .region_format_st
                .interpolate([location])
                .write_to(sink)?;

            Ok(())
        } else {
            sink.with_part(writeable::Part::ERROR, |sink| {
                essentials
                    .region_format
                    .interpolate([location])
                    .write_to(sink)
            })?;

            Err(FormatTimeZoneError::MissingInputField("zone_offset"))
        })
    }
}

// Pacific Time (Los Angeles)
#[allow(dead_code)]
struct GenericPartialLocationLongFormat;

impl GenericPartialLocationLongFormat {
    /// Writes the time zone in a long generic partial location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time (Los Angeles)
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(dead_code)]
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(metazone_id) = input.metazone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("metazone")));
        };

        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(exemplar_cities) = data_payloads.exemplar_cities else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(non_locations) = data_payloads.mz_generic_long.as_ref() else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(location) = exemplar_cities.0.get(&time_zone_id) else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };
        let Some(non_location) = non_locations
            .overrides
            .get(&time_zone_id)
            .or_else(|| non_locations.defaults.get(&metazone_id))
        else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        essentials
            .fallback_format
            .interpolate((location, non_location))
            .write_to(sink)?;

        Ok(Ok(()))
    }
}

// PT (Los Angeles)
#[allow(dead_code)]
struct GenericPartialLocationShortFormat;

impl GenericPartialLocationShortFormat {
    /// Writes the time zone in a short generic partial location format as defined by the UTS-35 spec.
    /// e.g. PT (Los Angeles)
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(dead_code)]
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };
        let Some(metazone_id) = input.metazone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("metazone")));
        };

        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(exemplar_cities) = data_payloads.exemplar_cities else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(non_locations) = data_payloads.mz_generic_short.as_ref() else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let Some(location) = exemplar_cities.0.get(&time_zone_id) else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };
        let Some(non_location) = non_locations
            .overrides
            .get(&time_zone_id)
            .or_else(|| non_locations.defaults.get(&metazone_id))
        else {
            return Ok(Err(FormatTimeZoneError::NameNotFound));
        };

        essentials
            .fallback_format
            .interpolate((location, non_location))
            .write_to(sink)?;

        Ok(Ok(()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Iso8601Format {
    pub(crate) format: IsoFormat,
    pub(crate) minutes: IsoMinutes,
    pub(crate) seconds: IsoSeconds,
}

impl Iso8601Format {
    /// Writes a [`UtcOffset`](crate::input::UtcOffset) in ISO-8601 format according to the
    /// given formatting options.
    ///
    /// [`IsoFormat`] determines whether the format should be Basic or Extended,
    /// and whether a zero-offset should be formatted numerically or with
    /// The UTC indicator: "Z"
    /// - Basic    e.g. +0800
    /// - Extended e.g. +08:00
    ///
    /// [`IsoMinutes`] can be required or optional.
    /// [`IsoSeconds`] can be optional or never.
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: Option<UtcOffset>,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(offset) = offset else {
            sink.with_part(Part::ERROR, |s| s.write_str("{X}"))?;
            return Ok(Err(DateTimeWriteError::MissingInputField("zone_offset")));
        };

        fn format_time_segment<W: writeable::PartsWrite + ?Sized>(
            sink: &mut W,
            n: u32,
        ) -> fmt::Result {
            if n < 10 {
                sink.write_char('0')?;
            }
            n.write_to(sink)
        }

        if offset.is_zero() && matches!(self.format, IsoFormat::UtcBasic | IsoFormat::UtcExtended) {
            sink.write_char('Z')?;
            return Ok(Ok(()));
        }

        let extended_format = matches!(self.format, IsoFormat::Extended | IsoFormat::UtcExtended);

        sink.write_char(if offset.is_positive() { '+' } else { '-' })?;

        format_time_segment(sink, (offset.offset_seconds() / 3600).unsigned_abs())?;

        if self.minutes == IsoMinutes::Required
            || (self.minutes == IsoMinutes::Optional && offset.has_minutes())
        {
            if extended_format {
                sink.write_char(':')?;
            }
            format_time_segment(sink, (offset.offset_seconds() % 3600 / 60).unsigned_abs())?;
        }

        if self.seconds == IsoSeconds::Optional && offset.has_seconds() {
            if extended_format {
                sink.write_char(':')?;
            }
            format_time_segment(sink, (offset.offset_seconds() % 3600 % 60).unsigned_abs())?;
        }

        Ok(Ok(()))
    }
}

// It is only used for pattern in special case and not public to users.
struct Bcp47IdFormat;

impl Bcp47IdFormat {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(time_zone_id) = input.time_zone_id else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("time_zone_id")));
        };

        sink.write_str(&time_zone_id)?;

        Ok(Ok(()))
    }
}
