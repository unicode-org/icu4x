// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A formatter specifically for the time zone.

use crate::{
    fields::{FieldLength, TimeZone},
    input::ExtractedInput,
    provider,
};
use core::fmt;
use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormatter;
use icu_timezone::{provider::MetazoneId, TimeZoneBcp47Id, UtcOffset, ZoneVariant};
use writeable::Writeable;

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

    pub(crate) fn units(self) -> impl Iterator<Item = TimeZoneFormatterUnit> {
        match self {
            // `z..zzzz`
            ResolvedNeoTimeZoneSkeleton::SpecificShort
            | ResolvedNeoTimeZoneSkeleton::SpecificLong => [
                Some(TimeZoneFormatterUnit::SpecificNonLocation(
                    self.to_field().length,
                )),
                Some(TimeZoneFormatterUnit::LocalizedOffset(
                    self.to_field().length,
                )),
                None,
            ],
            // 'v', 'vvvv'
            ResolvedNeoTimeZoneSkeleton::GenericShort
            | ResolvedNeoTimeZoneSkeleton::GenericLong => [
                Some(TimeZoneFormatterUnit::GenericNonLocation(
                    self.to_field().length,
                )),
                Some(TimeZoneFormatterUnit::GenericLocation),
                None,
            ],
            // 'VVVV'
            ResolvedNeoTimeZoneSkeleton::Location => {
                [Some(TimeZoneFormatterUnit::GenericLocation), None, None]
            }
            // `O`, `OOOO`, `ZZZZ`
            ResolvedNeoTimeZoneSkeleton::OffsetShort | ResolvedNeoTimeZoneSkeleton::OffsetLong => [
                Some(TimeZoneFormatterUnit::LocalizedOffset(
                    self.to_field().length,
                )),
                None,
                None,
            ],
            // 'V'
            ResolvedNeoTimeZoneSkeleton::Bcp47Id => {
                [Some(TimeZoneFormatterUnit::Bcp47Id), None, None]
            }
            // 'X'
            ResolvedNeoTimeZoneSkeleton::IsoX => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcBasic,
                    minutes: IsoMinutes::Optional,
                    seconds: IsoSeconds::Never,
                })),
                None,
                None,
            ],
            // 'XX'
            ResolvedNeoTimeZoneSkeleton::IsoXX => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcBasic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                })),
                None,
                None,
            ],
            // 'XXX'
            ResolvedNeoTimeZoneSkeleton::IsoXXX => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcExtended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                })),
                None,
                None,
            ],
            // 'XXXX'
            ResolvedNeoTimeZoneSkeleton::IsoXXXX => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcBasic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                })),
                None,
                None,
            ],
            // 'XXXXX', 'ZZZZZ'
            ResolvedNeoTimeZoneSkeleton::IsoXXXXX => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::UtcExtended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                })),
                None,
                None,
            ],
            // 'x'
            ResolvedNeoTimeZoneSkeleton::Isox => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes: IsoMinutes::Optional,
                    seconds: IsoSeconds::Never,
                })),
                None,
                None,
            ],
            // 'xx'
            ResolvedNeoTimeZoneSkeleton::Isoxx => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                })),
                None,
                None,
            ],
            // 'xxx'
            ResolvedNeoTimeZoneSkeleton::Isoxxx => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Extended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Never,
                })),
                None,
                None,
            ],
            // 'xxxx', 'Z', 'ZZ', 'ZZZ'
            ResolvedNeoTimeZoneSkeleton::Isoxxxx => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Basic,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                })),
                None,
                None,
            ],
            // 'xxxxx', 'ZZZZZ'
            ResolvedNeoTimeZoneSkeleton::Isoxxxxx => [
                Some(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format: IsoFormat::Extended,
                    minutes: IsoMinutes::Required,
                    seconds: IsoSeconds::Optional,
                })),
                None,
                None,
            ],
        }
        .into_iter()
        .flatten()
    }
}

/// A container contains all data payloads for time zone formatting (borrowed version).
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct TimeZoneDataPayloadsBorrowed<'a> {
    /// The data that contains meta information about how to display content.
    pub(crate) essentials: Option<&'a provider::time_zones::TimeZoneEssentialsV1<'a>>,
    /// The location names, e.g. Germany Time
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

impl ExtractedInput {
    fn zone_variant_or_guess(
        &self,
        time_zone_id: TimeZoneBcp47Id,
        offset_period: &icu_timezone::provider::ZoneOffsetPeriodV1,
    ) -> ZoneVariant {
        if let Some(zv) = self.zone_variant {
            return zv;
        }

        let Some(offset) = self.offset else {
            return ZoneVariant::standard();
        };

        let dst_offset = {
            match offset_period.0.get0(&time_zone_id) {
                Some(cursor) => {
                    let offsets = if let Some((date, time)) = self.local_time {
                        let mut offsets = None;
                        let minutes_since_local_unix_epoch =
                            icu_calendar::DateTime { date, time }.minutes_since_local_unix_epoch();
                        for (minutes, id) in cursor.iter1_copied().rev() {
                            if minutes_since_local_unix_epoch
                                <= <i32 as zerovec::ule::AsULE>::from_unaligned(*minutes)
                            {
                                offsets = Some(id);
                            } else {
                                break;
                            }
                        }
                        offsets.unwrap_or_default() // shouldn't happen
                    } else {
                        cursor
                            .iter1()
                            .map(|(_, v)| zerovec::ule::AsULE::from_unaligned(*v))
                            .next_back()
                            .unwrap_or_default() // shouldn't happen
                    };
                    (offsets.1 != 0)
                        .then_some(UtcOffset::from_eighths_of_hour(offsets.0 + offsets.1))
                }
                None => None,
            }
        };

        if Some(offset) == dst_offset {
            ZoneVariant::daylight()
        } else {
            ZoneVariant::standard()
        }
    }

    fn metazone(
        &self,
        time_zone_id: TimeZoneBcp47Id,
        metazone_period: &icu_timezone::provider::MetazonePeriodV1,
    ) -> Option<MetazoneId> {
        match metazone_period.0.get0(&time_zone_id) {
            Some(cursor) => {
                if let Some((date, time)) = self.local_time {
                    let mut metazone_id = None;
                    let minutes_since_local_unix_epoch =
                        icu_calendar::DateTime { date, time }.minutes_since_local_unix_epoch();
                    for (minutes, id) in cursor.iter1() {
                        if minutes_since_local_unix_epoch
                            >= <i32 as zerovec::ule::AsULE>::from_unaligned(*minutes)
                        {
                            metazone_id = id.get()
                        } else {
                            break;
                        }
                    }
                    metazone_id
                } else {
                    cursor.iter1().next_back().and_then(|(_, m)| m.get())
                }
            }
            None => None,
        }
    }
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

// An enum for time zone format unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum TimeZoneFormatterUnit {
    GenericNonLocation(FieldLength),
    SpecificNonLocation(FieldLength),
    GenericLocation,
    #[allow(dead_code)]
    SpecificLocation,
    #[allow(dead_code)]
    GenericPartialLocation(FieldLength),
    LocalizedOffset(FieldLength),
    Iso8601(Iso8601Format),
    Bcp47Id,
}

#[derive(Debug)]
pub(super) enum FormatTimeZoneError {
    MissingZoneSymbols,
    MissingFixedDecimalFormatter,
    Fallback,
    MissingInputField(&'static str),
}

pub(super) trait FormatTimeZone {
    /// Tries to write the timezone to the sink. If a DateTimeError is returned, the sink
    /// has not been touched, so another format can be attempted.
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error>;
}

impl FormatTimeZone for TimeZoneFormatterUnit {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
        fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match *self {
            Self::GenericNonLocation(length) => {
                GenericNonLocationFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::SpecificNonLocation(length) => {
                SpecificNonLocationFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::GenericLocation => GenericLocationFormat.format(sink, input, data_payloads, fdf),
            Self::SpecificLocation => {
                SpecificLocationFormat.format(sink, input, data_payloads, fdf)
            }
            Self::GenericPartialLocation(length) => {
                GenericPartialLocationFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::LocalizedOffset(length) => {
                LocalizedOffsetFormat(length).format(sink, input, data_payloads, fdf)
            }
            Self::Iso8601(iso) => iso.format(sink, input, data_payloads, fdf),
            Self::Bcp47Id => Bcp47IdFormat.format(sink, input, data_payloads, fdf),
        }
    }
}

// PT / Pacific Time
struct GenericNonLocationFormat(FieldLength);

impl FormatTimeZone for GenericNonLocationFormat {
    /// Writes the time zone in generic non-location format as defined by the UTS-35 spec.
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
        let Some(names) = (match self.0 {
            FieldLength::Wide => data_payloads.mz_generic_long.as_ref(),
            _ => data_payloads.mz_generic_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let metazone_id = input.metazone(
            time_zone_id,
            icu_timezone::provider::Baked::SINGLETON_METAZONE_PERIOD_V1_MARKER,
        );

        let Some(name) = metazone_id.and_then(|mz| {
            names
                .overrides
                .get(&time_zone_id)
                .or_else(|| names.defaults.get(&mz))
        }) else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// PDT / Pacific Daylight Time
struct SpecificNonLocationFormat(FieldLength);

impl FormatTimeZone for SpecificNonLocationFormat {
    /// Writes the time zone in short specific non-location format as defined by the UTS-35 spec.
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
        let zone_variant = input.zone_variant_or_guess(
            time_zone_id,
            icu_timezone::provider::Baked::SINGLETON_ZONE_OFFSET_PERIOD_V1_MARKER,
        );

        let Some(names) = (match self.0 {
            FieldLength::Wide => data_payloads.mz_specific_long.as_ref(),
            _ => data_payloads.mz_specific_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let metazone_id = input.metazone(
            time_zone_id,
            icu_timezone::provider::Baked::SINGLETON_METAZONE_PERIOD_V1_MARKER,
        );

        let Some(name) = metazone_id.and_then(|mz| {
            names
                .overrides
                .get_2d(&time_zone_id, &zone_variant)
                .or_else(|| names.defaults.get_2d(&mz, &zone_variant))
        }) else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        sink.write_str(name)?;

        Ok(Ok(()))
    }
}

// UTC+7:00
struct LocalizedOffsetFormat(FieldLength);

impl FormatTimeZone for LocalizedOffsetFormat {
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
        let Some(offset) = input.offset else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        let Some(essentials) = data_payloads.essentials else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(fdf) = fdf else {
            return Ok(Err(FormatTimeZoneError::MissingFixedDecimalFormatter));
        };
        Ok(if offset.is_zero() {
            sink.write_str(&essentials.offset_zero)?;
            Ok(())
        } else {
            struct FormattedOffset<'a> {
                offset: UtcOffset,
                separator: &'a str,
                fdf: &'a FixedDecimalFormatter,
                length: FieldLength,
            }

            impl Writeable for FormattedOffset<'_> {
                fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
                    &self,
                    sink: &mut S,
                ) -> fmt::Result {
                    self.fdf
                        .format(
                            &FixedDecimal::from(self.offset.hours_part())
                                .with_sign_display(fixed_decimal::SignDisplay::Always)
                                .padded_start(if self.length == FieldLength::Wide {
                                    2
                                } else {
                                    0
                                }),
                        )
                        .write_to(sink)?;

                    if self.length == FieldLength::Wide
                        || self.offset.minutes_part() != 0
                        || self.offset.seconds_part() != 0
                    {
                        sink.write_str(self.separator)?;
                        self.fdf
                            .format(&FixedDecimal::from(self.offset.minutes_part()).padded_start(2))
                            .write_to(sink)?;
                    }

                    if self.offset.seconds_part() != 0 {
                        sink.write_str(self.separator)?;
                        self.fdf
                            .format(&FixedDecimal::from(self.offset.seconds_part()).padded_start(2))
                            .write_to(sink)?;
                    }

                    Ok(())
                }
            }

            essentials
                .offset_pattern
                .interpolate([FormattedOffset {
                    offset,
                    separator: &essentials.offset_separator,
                    fdf,
                    length: self.0,
                }])
                .write_to(sink)?;

            Ok(())
        })
    }
}

// Los Angeles Time
struct GenericLocationFormat;

impl FormatTimeZone for GenericLocationFormat {
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

        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        if let Some(location) = locations.locations.get(&time_zone_id) {
            locations
                .pattern_generic
                .interpolate([location])
                .write_to(sink)?;
        } else {
            sink.write_str(&locations.unknown)?;
        }

        Ok(Ok(()))
    }
}

// Los Angeles Daylight Time
struct SpecificLocationFormat;

impl FormatTimeZone for SpecificLocationFormat {
    /// Writes the time zone in a specific location format as defined by the UTS-35 spec.
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
        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let zone_variant = input.zone_variant_or_guess(
            time_zone_id,
            icu_timezone::provider::Baked::SINGLETON_ZONE_OFFSET_PERIOD_V1_MARKER,
        );

        if let Some(location) = locations.locations.get(&time_zone_id) {
            if zone_variant == ZoneVariant::daylight() {
                &locations.pattern_daylight
            } else if zone_variant == ZoneVariant::standard() {
                &locations.pattern_standard
            } else {
                &locations.pattern_generic
            }
            .interpolate([location])
            .write_to(sink)?;
        } else {
            sink.write_str(&locations.unknown)?;
        }

        Ok(Ok(()))
    }
}

// Pacific Time (Los Angeles) / PT (Los Angeles)
struct GenericPartialLocationFormat(FieldLength);

impl FormatTimeZone for GenericPartialLocationFormat {
    /// Writes the time zone in a long generic partial location format as defined by the UTS-35 spec.
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

        let Some(locations) = data_payloads.locations else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        let Some(non_locations) = (match self.0 {
            FieldLength::Wide => data_payloads.mz_generic_long.as_ref(),
            _ => data_payloads.mz_generic_short.as_ref(),
        }) else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };

        let metazone_id = input.metazone(
            time_zone_id,
            icu_timezone::provider::Baked::SINGLETON_METAZONE_PERIOD_V1_MARKER,
        );

        let Some(location) = locations.locations.get(&time_zone_id) else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };
        let Some(non_location) = metazone_id.and_then(|mz| {
            non_locations
                .overrides
                .get(&time_zone_id)
                .or_else(|| non_locations.defaults.get(&mz))
        }) else {
            return Ok(Err(FormatTimeZoneError::Fallback));
        };

        locations
            .pattern_partial_location
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

impl FormatTimeZone for Iso8601Format {
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
        input: &ExtractedInput,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(offset) = input.offset else {
            return Ok(Err(FormatTimeZoneError::MissingInputField("zone_offset")));
        };
        self.format_infallible(sink, offset).map(|()| Ok(()))
    }
}

impl Iso8601Format {
    pub(crate) fn format_infallible<W: writeable::PartsWrite + ?Sized>(
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

        format_time_segment(sink, offset.hours_part().unsigned_abs())?;

        if self.minutes == IsoMinutes::Required
            || (self.minutes == IsoMinutes::Optional && offset.minutes_part() != 0)
        {
            if extended_format {
                sink.write_char(':')?;
            }
            format_time_segment(sink, offset.minutes_part())?;
        }

        if self.seconds == IsoSeconds::Optional && offset.seconds_part() != 0 {
            if extended_format {
                sink.write_char(':')?;
            }
            format_time_segment(sink, offset.seconds_part())?;
        }

        Ok(())
    }
}

// It is only used for pattern in special case and not public to users.
struct Bcp47IdFormat;

impl FormatTimeZone for Bcp47IdFormat {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
        _fdf: Option<&FixedDecimalFormatter>,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let time_zone_id = input
            .time_zone_id
            .unwrap_or(TimeZoneBcp47Id(tinystr::tinystr!(8, "unk")));

        sink.write_str(&time_zone_id)?;

        Ok(Ok(()))
    }
}
