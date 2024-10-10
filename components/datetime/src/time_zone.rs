// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A formatter specifically for the time zone.

use crate::{
    fields::{FieldLength, TimeZone},
    provider, DateTimeWriteError,
};
use core::fmt;
use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormatter;
use icu_timezone::{MetazoneId, ZoneVariant};
use icu_timezone::{TimeZoneBcp47Id, UtcOffset};
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
}

/// A container contains all data payloads for time zone formatting (borrowed version).
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct TimeZoneDataPayloadsBorrowed<'a> {
    /// The data that contains meta information about how to display content.
    pub(crate) essentials: Option<&'a provider::time_zones::TimeZoneEssentialsV1<'a>>,
    /// The locations for time zones.
    pub(crate) locations: Option<&'a provider::time_zones::LocationsV1<'a>>,
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
pub(crate) enum IsoFormat {
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

/// Whether the minutes field should be optional or required.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Minutes {
    /// Minutes are always displayed.
    Required,

    /// Minutes are displayed only if they are non-zero.
    Optional,
}

/// Whether the seconds field should be optional or excluded.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Seconds {
    /// Seconds are displayed only if they are non-zero.
    Optional,

    /// Seconds are not displayed.
    Never,
}

// PT/Pacific Time
pub(crate) struct GenericNonLocationFormat;

impl GenericNonLocationFormat {
    /// Writes the time zone in short generic non-location format as defined by the UTS-35 spec.
    /// e.g. PT
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    pub(crate) fn format_short<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        self.format_internal(
            sink,
            time_zone_id,
            metazone_id,
            data_payloads,
            data_payloads.mz_generic_short,
        )
    }

    /// Writes the time zone in long generic non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    pub(crate) fn format_long<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        self.format_internal(
            sink,
            time_zone_id,
            metazone_id,
            data_payloads,
            data_payloads.mz_generic_long,
        )
    }

    fn format_internal<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        names: Option<&provider::time_zones::MetazoneGenericNamesV1>,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(names) = names.as_ref() else {
            let _ = GenericLocationFormat.format(sink, time_zone_id, data_payloads)?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };

        let Some(name) = metazone_id.and_then(|mz| {
            names
                .overrides
                .get(&time_zone_id)
                .or_else(|| names.defaults.get(&mz))
        }) else {
            // UTS-35 fallback, not an error
            return GenericLocationFormat.format(sink, time_zone_id, data_payloads);
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// PDT/Pacific Daylight Time
pub(crate) struct SpecificNonLocationFormat;

impl SpecificNonLocationFormat {
    /// Writes the time zone in short specific non-location format as defined by the UTS-35 spec.
    /// e.g. PDT
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn format_short<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        offset: UtcOffset,
        zone_variant: ZoneVariant,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        self.format_internal(
            sink,
            time_zone_id,
            metazone_id,
            offset,
            zone_variant,
            data_payloads,
            fdf,
            data_payloads.mz_specific_short,
            Minutes::Optional,
        )
    }

    /// Writes the time zone in long specific non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Daylight Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn format_long<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        offset: UtcOffset,
        zone_variant: ZoneVariant,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        self.format_internal(
            sink,
            time_zone_id,
            metazone_id,
            offset,
            zone_variant,
            data_payloads,
            fdf,
            data_payloads.mz_specific_long,
            Minutes::Required,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn format_internal<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        offset: UtcOffset,
        zone_variant: ZoneVariant,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
        names: Option<&provider::time_zones::MetazoneSpecificNamesV1>,
        minutes: Minutes,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(names) = names.as_ref() else {
            let _ = LocalizedOffsetFormat.format(
                sink,
                offset,
                data_payloads,
                fdf,
                minutes,
                Seconds::Optional,
            )?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };

        let Some(name) = metazone_id.and_then(|mz| {
            names
                .overrides
                .get_2d(&time_zone_id, &zone_variant)
                .or_else(|| names.defaults.get_2d(&mz, &zone_variant))
        }) else {
            // UTS-35 fallback, not an error
            return LocalizedOffsetFormat.format(
                sink,
                offset,
                data_payloads,
                fdf,
                minutes,
                Seconds::Optional,
            );
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

/// UTC+7/UTC+07:00/UTC+07:23:08
pub(crate) struct LocalizedOffsetFormat;

impl LocalizedOffsetFormat {
    /// Writes the time zone in localized offset format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// offset formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// <https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing>
    pub(crate) fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
        minutes: Minutes,
        seconds: Seconds,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            sink.with_part(Part::ERROR, |s| {
                Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes,
                    seconds,
                }
                .format(s, offset)
            })?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };
        let Some(fdf) = fdf else {
            // TODO: use Latin FDF instead
            sink.with_part(Part::ERROR, |s| {
                Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes,
                    seconds,
                }
                .format(s, offset)
            })?;
            return Ok(Err(DateTimeWriteError::MissingFixedDecimalFormatter));
        };
        Ok(if offset.is_zero() {
            sink.write_str(&essentials.offset_zero_format)?;
            Ok(())
        } else {
            struct FormattedOffset<'a> {
                offset: UtcOffset,
                sign: char,
                separator: char,
                fdf: &'a FixedDecimalFormatter,
                minutes: Minutes,
                seconds: Seconds,
            }

            impl Writeable for FormattedOffset<'_> {
                fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
                    &self,
                    sink: &mut S,
                ) -> fmt::Result {
                    let hours = (self.offset.offset_seconds() / 3600).unsigned_abs();
                    let minutes = (self.offset.offset_seconds() / 60 % 60).unsigned_abs();
                    let seconds = (self.offset.offset_seconds() % 60).unsigned_abs();

                    sink.write_char(self.sign)?;

                    self.fdf
                        .format(&FixedDecimal::from(hours).padded_start(
                            if self.minutes == Minutes::Required {
                                2
                            } else {
                                0
                            },
                        ))
                        .write_to(sink)?;

                    if minutes != 0 || self.minutes == Minutes::Required {
                        sink.write_char(self.separator)?;
                        self.fdf
                            .format(&FixedDecimal::from(minutes).padded_start(2))
                            .write_to(sink)?;
                    }

                    if seconds != 0 && self.seconds != Seconds::Never {
                        sink.write_char(self.separator)?;
                        self.fdf
                            .format(&FixedDecimal::from(seconds).padded_start(2))
                            .write_to(sink)?;
                    }

                    Ok(())
                }
            }

            // TODO: store these in data struct
            let sign = if offset.is_positive() {
                essentials.hour_format.0.chars().next().unwrap_or('+')
            } else {
                essentials.hour_format.1.chars().next().unwrap_or('-')
            };
            let separator = essentials
                .hour_format
                .0
                .split_once(['h', 'H'])
                .unwrap_or_default()
                .1
                .chars()
                .next()
                .unwrap_or(':');

            essentials
                .offset_format
                .interpolate([FormattedOffset {
                    offset,
                    sign,
                    separator,
                    fdf,
                    minutes,
                    seconds,
                }])
                .write_to(sink)?;

            Ok(())
        })
    }
}

// Los Angeles Time
pub(crate) struct GenericLocationFormat;

impl GenericLocationFormat {
    /// Writes the time zone in generic location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    pub(crate) fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            // `unk` for `und`
            sink.write_str("Unknown")?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };
        let Some(locations) = data_payloads.locations else {
            // `unk` for `und`
            sink.write_str("Unknown")?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };

        if let Some(location) = locations.zones.get(&time_zone_id) {
            essentials
                .region_format
                .interpolate([location])
                .write_to(sink)?;
        } else {
            sink.write_str(&locations.unknown)?;
        }

        Ok(Ok(()))
    }
}

// Los Angeles Daylight Time
#[allow(dead_code)]
pub(crate) struct SpecificLocationFormat;

impl SpecificLocationFormat {
    /// Writes the time zone in a specific location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(dead_code)]
    pub(crate) fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
        zone_variant: ZoneVariant,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            // `unk` for `und`
            sink.write_str("Unknown")?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };
        let Some(locations) = data_payloads.locations else {
            // `unk` for `und`
            sink.write_str("Unknown")?;
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };

        if let Some(location) = locations.zones.get(&time_zone_id) {
            if zone_variant == ZoneVariant::daylight() {
                &essentials.region_format_dt
            } else {
                &essentials.region_format_st
            }
            .interpolate([location])
            .write_to(sink)?;
        } else {
            sink.write_str(&locations.unknown)?;
        }

        Ok(Ok(()))
    }
}

// Pacific Time (Los Angeles)/PT (Los Angeles)
#[allow(dead_code)]
pub(crate) struct GenericPartialLocation;

impl GenericPartialLocation {
    /// Writes the time zone in a long generic partial location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time (Los Angeles)
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(dead_code)]
    pub(crate) fn format_long<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,

        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        self.format_internal(
            sink,
            time_zone_id,
            metazone_id,
            data_payloads,
            data_payloads.mz_generic_long,
        )
    }

    /// Writes the time zone in a short generic partial location format as defined by the UTS-35 spec.
    /// e.g. PT (Los Angeles)
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    #[allow(dead_code)]
    pub(crate) fn format_short<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,

        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        self.format_internal(
            sink,
            time_zone_id,
            metazone_id,
            data_payloads,
            data_payloads.mz_generic_short,
        )
    }

    pub(crate) fn format_internal<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,

        time_zone_id: TimeZoneBcp47Id,
        metazone_id: Option<MetazoneId>,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        names: Option<&provider::time_zones::MetazoneGenericNamesV1>,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };
        let Some(locations) = data_payloads.locations else {
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };
        let Some(non_locations) = names else {
            return Ok(Err(DateTimeWriteError::MissingZoneSymbols));
        };

        let Some(location) = locations.zones.get(&time_zone_id) else {
            // UTS-35 fallback, not an error
            return GenericLocationFormat.format(sink, time_zone_id, data_payloads);
        };
        let Some(non_location) = metazone_id.and_then(|mz| {
            non_locations
                .overrides
                .get(&time_zone_id)
                .or_else(|| non_locations.defaults.get(&mz))
        }) else {
            // UTS-35 fallback, not an error
            return GenericLocationFormat.format(sink, time_zone_id, data_payloads);
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
    pub(crate) minutes: Minutes,
    pub(crate) seconds: Seconds,
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
    /// [`Minutes`] can be required or optional.
    /// [`Seconds`] can be optional or never.
    pub(crate) fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
    ) -> Result<(), fmt::Error> {
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
            return sink.write_char('Z');
        }

        let extended_format = matches!(self.format, IsoFormat::Extended | IsoFormat::UtcExtended);

        sink.write_char(if offset.is_positive() { '+' } else { '-' })?;

        format_time_segment(sink, (offset.offset_seconds() / 3600).unsigned_abs())?;

        if self.minutes == Minutes::Required || offset.has_minutes() {
            if extended_format {
                sink.write_char(':')?;
            }
            format_time_segment(sink, (offset.offset_seconds() % 3600 / 60).unsigned_abs())?;
        }

        if self.seconds == Seconds::Optional && offset.has_seconds() {
            if extended_format {
                sink.write_char(':')?;
            }
            format_time_segment(sink, (offset.offset_seconds() % 3600 % 60).unsigned_abs())?;
        }

        Ok(())
    }
}

// It is only used for pattern in special case and not public to users.
pub(crate) struct Bcp47IdFormat;

impl Bcp47IdFormat {
    pub(crate) fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        time_zone_id: TimeZoneBcp47Id,
    ) -> Result<Result<(), DateTimeWriteError>, fmt::Error> {
        sink.write_str(&time_zone_id)?;

        Ok(Ok(()))
    }
}
