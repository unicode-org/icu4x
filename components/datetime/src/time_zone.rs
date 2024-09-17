// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A formatter specifically for the time zone.

use crate::{
    fields::{FieldLength, TimeZone},
    input::ExtractedInput,
    provider,
};
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt;
use icu_timezone::TimeZoneBcp47Id;
use icu_timezone::UtcOffset;
use tinystr::tinystr;
use writeable::{adapters::CoreWriteAsPartsWrite, Writeable};

/// All time zone styles that this crate can format
#[derive(Debug, Copy, Clone)]
pub(crate) enum ResolvedNeoTimeZoneSkeleton {
    City,
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
}

impl ResolvedNeoTimeZoneSkeleton {
    pub(crate) fn from_field(field_symbol: TimeZone, field_length: FieldLength) -> Option<Self> {
        crate::tz_registry::field_to_resolved(field_symbol, field_length)
    }
    #[cfg(feature = "experimental")]
    pub(crate) fn to_field(self) -> crate::fields::Field {
        crate::tz_registry::resolved_to_field(self)
    }
}

/// A container contains all data payloads for time zone formatting (borrowed version).
#[derive(Debug, Copy, Clone)]
pub(crate) struct TimeZoneDataPayloadsBorrowed<'a> {
    /// The data that contains meta information about how to display content.
    pub(crate) zone_formats: Option<&'a provider::time_zones::TimeZoneFormatsV1<'a>>,
    /// The exemplar cities for time zones.
    pub(crate) exemplar_cities: Option<&'a provider::time_zones::ExemplarCitiesV1<'a>>,
    /// The generic long metazone names, e.g. Pacific Time
    pub(crate) mz_generic_long: Option<&'a provider::time_zones::MetazoneGenericNamesLongV1<'a>>,
    /// The generic short metazone names, e.g. PT
    pub(crate) mz_generic_short: Option<&'a provider::time_zones::MetazoneGenericNamesShortV1<'a>>,
    /// The specific long metazone names, e.g. Pacific Daylight Time
    pub(crate) mz_specific_long: Option<&'a provider::time_zones::MetazoneSpecificNamesLongV1<'a>>,
    /// The specific short metazone names, e.g. Pacific Daylight Time
    pub(crate) mz_specific_short:
        Option<&'a provider::time_zones::MetazoneSpecificNamesShortV1<'a>>,
}

impl<'a> TimeZoneDataPayloadsBorrowed<'a> {
    pub(crate) fn empty() -> Self {
        TimeZoneDataPayloadsBorrowed {
            zone_formats: None,
            exemplar_cities: None,
            mz_generic_long: None,
            mz_generic_short: None,
            mz_specific_long: None,
            mz_specific_short: None,
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

/// Whether a field should be zero-padded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub(crate) enum ZeroPadding {
    /// Add zero-padding.
    On,

    /// Do not add zero-padding.
    Off,
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

// Pacific Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct GenericNonLocationLongFormat {}

// PT
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct GenericNonLocationShortFormat {}

// Pacific Standard Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SpecificNonLocationLongFormat {}

// PDT
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SpecificNonLocationShortFormat {}

// Los Angeles Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct GenericLocationFormat {}

// GMT-07:00
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct LocalizedOffsetFormat {}

// -07:00
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Iso8601Format {
    pub(crate) format: IsoFormat,
    pub(crate) minutes: IsoMinutes,
    pub(crate) seconds: IsoSeconds,
}

impl Iso8601Format {
    pub(crate) fn default_for_fallback() -> Self {
        Self {
            format: IsoFormat::Basic,
            minutes: IsoMinutes::Required,
            seconds: IsoSeconds::Optional,
        }
    }
}

// It is only used for pattern in special case and not public to users.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ExemplarCityFormat {}

// It is only used for pattern in special case and not public to users.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Bcp47IdFormat {}

// An enum for time zone format unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum TimeZoneFormatterUnit {
    GenericNonLocationLong(GenericNonLocationLongFormat),
    GenericNonLocationShort(GenericNonLocationShortFormat),
    SpecificNonLocationLong(SpecificNonLocationLongFormat),
    SpecificNonLocationShort(SpecificNonLocationShortFormat),
    GenericLocation(GenericLocationFormat),
    ExemplarCity(ExemplarCityFormat),
    Bcp47Id(Bcp47IdFormat),
    WithFallback(FallbackTimeZoneFormatterUnit),
}

impl Default for TimeZoneFormatterUnit {
    fn default() -> Self {
        TimeZoneFormatterUnit::WithFallback(FallbackTimeZoneFormatterUnit::LocalizedOffset(
            LocalizedOffsetFormat {},
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum FallbackTimeZoneFormatterUnit {
    LocalizedOffset(LocalizedOffsetFormat),
    Iso8601(Iso8601Format),
}

/// Load a fallback format for timezone. The fallback format will be executed if there are no
/// matching format results.
impl From<FallbackFormat> for FallbackTimeZoneFormatterUnit {
    fn from(value: FallbackFormat) -> Self {
        match value {
            FallbackFormat::LocalizedOffset => Self::LocalizedOffset(LocalizedOffsetFormat {}),
        }
    }
}

#[derive(Debug)]
pub(super) enum FormatTimeZoneError {
    MissingZoneSymbols,
    NameNotFound,
    #[allow(dead_code)] // the field is not being read but it may be useful
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
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error>;
}

pub(super) trait FormatTimeZoneWithFallback {
    fn format_offset<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error>;
}

impl FormatTimeZone for TimeZoneFormatterUnit {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match self {
            Self::GenericNonLocationLong(unit) => unit.format(sink, input, data_payloads),
            Self::GenericNonLocationShort(unit) => unit.format(sink, input, data_payloads),
            Self::SpecificNonLocationLong(unit) => unit.format(sink, input, data_payloads),
            Self::SpecificNonLocationShort(unit) => unit.format(sink, input, data_payloads),
            Self::GenericLocation(unit) => unit.format(sink, input, data_payloads),
            Self::WithFallback(unit) => unit.format(sink, input, data_payloads),
            Self::ExemplarCity(unit) => unit.format(sink, input, data_payloads),
            Self::Bcp47Id(unit) => unit.format(sink, input, data_payloads),
        }
    }
}

impl FormatTimeZone for FallbackTimeZoneFormatterUnit {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match self {
            Self::LocalizedOffset(unit) => unit.format(sink, input, data_payloads),
            Self::Iso8601(unit) => unit.format(sink, input, data_payloads),
        }
    }
}

impl FormatTimeZoneWithFallback for FallbackTimeZoneFormatterUnit {
    fn format_offset<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match self {
            Self::LocalizedOffset(unit) => unit.format_offset(sink, offset, data_payloads),
            Self::Iso8601(unit) => unit.format_offset(sink, offset, data_payloads),
        }
    }
}

impl FormatTimeZone for GenericNonLocationLongFormat {
    /// Writes the time zone in long generic non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_generic_long
            .as_ref()
            .and_then(|metazones| {
                input
                    .time_zone_id
                    .and_then(|tz| metazones.overrides.get(&tz))
            })
            .or_else(|| {
                data_payloads
                    .mz_generic_long
                    .as_ref()
                    .and_then(|metazones| {
                        input.metazone_id.and_then(|mz| metazones.defaults.get(&mz))
                    })
            });

        Ok(match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)?),
            None => Err(FormatTimeZoneError::NameNotFound),
        })
    }
}

impl FormatTimeZone for GenericNonLocationShortFormat {
    /// Writes the time zone in short generic non-location format as defined by the UTS-35 spec.
    /// e.g. PT
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_generic_short
            .as_ref()
            .and_then(|metazones| {
                input
                    .time_zone_id
                    .and_then(|tz| metazones.overrides.get(&tz))
            })
            .or_else(|| {
                data_payloads
                    .mz_generic_short
                    .as_ref()
                    .and_then(|metazones| {
                        input.metazone_id.and_then(|mz| metazones.defaults.get(&mz))
                    })
            });

        Ok(match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)?),
            None => Err(FormatTimeZoneError::NameNotFound),
        })
    }
}

impl FormatTimeZone for SpecificNonLocationShortFormat {
    /// Writes the time zone in short specific non-location format as defined by the UTS-35 spec.
    /// e.g. PDT
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_specific_short
            .as_ref()
            .and_then(|metazones| {
                input.time_zone_id.and_then(|tz| {
                    input
                        .zone_variant
                        .and_then(|variant| metazones.overrides.get_2d(&tz, &variant))
                })
            })
            .or_else(|| {
                data_payloads
                    .mz_specific_short
                    .as_ref()
                    .and_then(|metazones| {
                        input.metazone_id.and_then(|mz| {
                            input
                                .zone_variant
                                .and_then(|variant| metazones.defaults.get_2d(&mz, &variant))
                        })
                    })
            });

        Ok(match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)?),
            None => Err(FormatTimeZoneError::NameNotFound),
        })
    }
}

impl FormatTimeZone for SpecificNonLocationLongFormat {
    /// Writes the time zone in long specific non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Daylight Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_specific_long
            .as_ref()
            .and_then(|metazones| {
                input.time_zone_id.and_then(|tz| {
                    input
                        .zone_variant
                        .and_then(|variant| metazones.overrides.get_2d(&tz, &variant))
                })
            })
            .or_else(|| {
                data_payloads
                    .mz_specific_long
                    .as_ref()
                    .and_then(|metazones| {
                        input.metazone_id.and_then(|mz| {
                            input
                                .zone_variant
                                .and_then(|variant| metazones.defaults.get_2d(&mz, &variant))
                        })
                    })
            });

        Ok(match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)?),
            None => Err(FormatTimeZoneError::NameNotFound),
        })
    }
}

impl FormatTimeZoneWithFallback for LocalizedOffsetFormat {
    /// Writes the time zone in localized offset format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// offset formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// <https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing>
    fn format_offset<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(zone_formats) = data_payloads.zone_formats else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        Ok(if offset.is_zero() {
            sink.write_str(&zone_formats.offset_zero_format)?;
            Ok(())
        } else {
            // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
            let mut scratch = String::new();
            sink.write_str(
                &zone_formats
                    .offset_format
                    .replace(
                        "{0}",
                        if offset.is_positive() {
                            &zone_formats.hour_format.0
                        } else {
                            &zone_formats.hour_format.1
                        },
                    )
                    // support all combos of "(HH|H):mm" by replacing longest patterns first.
                    .replace("HH", {
                        scratch.clear();
                        let _infallible = format_offset_hours(
                            &mut CoreWriteAsPartsWrite(&mut scratch),
                            offset,
                            ZeroPadding::On,
                        );
                        &scratch
                    })
                    .replace("mm", {
                        scratch.clear();
                        let _infallible =
                            format_offset_minutes(&mut CoreWriteAsPartsWrite(&mut scratch), offset);
                        &scratch
                    })
                    .replace('H', {
                        scratch.clear();
                        let _infallible = format_offset_hours(
                            &mut CoreWriteAsPartsWrite(&mut scratch),
                            offset,
                            ZeroPadding::Off,
                        );
                        &scratch
                    }),
            )?;
            Ok(())
        })
    }
}

impl FormatTimeZone for LocalizedOffsetFormat {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match input
            .offset
            .ok_or(FormatTimeZoneError::MissingInputField("offset"))
        {
            Ok(offset) => self.format_offset(sink, offset, data_payloads),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl FormatTimeZone for GenericLocationFormat {
    /// Writes the time zone in generic location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// <https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology>
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        let Some(zone_formats) = data_payloads.zone_formats else {
            return Ok(Err(FormatTimeZoneError::MissingZoneSymbols));
        };
        // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
        let formatted_time_zone: Option<alloc::string::String> = data_payloads
            .exemplar_cities
            .as_ref()
            .and_then(|cities| input.time_zone_id.and_then(|id| cities.0.get(&id)))
            .map(|location| zone_formats.region_format.replace("{0}", location));
        Ok(match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(&ftz)?),
            None => Err(FormatTimeZoneError::NameNotFound),
        })
    }
}

impl FormatTimeZoneWithFallback for Iso8601Format {
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
    fn format_offset<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        self.format_infallible(sink, offset).map(|()| Ok(()))
    }
}

impl Iso8601Format {
    pub(crate) fn format_infallible<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        offset: UtcOffset,
    ) -> Result<(), fmt::Error> {
        if offset.is_zero() && matches!(self.format, IsoFormat::UtcBasic | IsoFormat::UtcExtended) {
            return sink.write_char('Z');
        }

        let extended_format = matches!(self.format, IsoFormat::Extended | IsoFormat::UtcExtended);

        sink.write_char(if offset.is_positive() { '+' } else { '-' })?;

        format_offset_hours(sink, offset, ZeroPadding::On)?;

        if self.minutes == IsoMinutes::Required
            || (self.minutes == IsoMinutes::Optional && offset.has_minutes())
        {
            if extended_format {
                sink.write_char(':')?;
            }
            format_offset_minutes(sink, offset)?;
        }

        if self.seconds == IsoSeconds::Optional && offset.has_seconds() {
            if extended_format {
                sink.write_char(':')?;
            }
            format_offset_seconds(sink, offset)?;
        }

        Ok(())
    }
}

impl FormatTimeZone for Iso8601Format {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        match input
            .offset
            .ok_or(FormatTimeZoneError::MissingInputField("offset"))
        {
            Ok(offset) => self.format_offset(sink, offset, data_payloads),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl FormatTimeZone for ExemplarCityFormat {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        // Writes the exemplar city associated with this time zone.
        let formatted_exemplar_city = data_payloads
            .exemplar_cities
            .as_ref()
            .and_then(|cities| input.time_zone_id.and_then(|id| cities.0.get(&id)));

        Ok(match formatted_exemplar_city {
            Some(ftz) => Ok(sink.write_str(ftz)?),
            None => {
                // Writes the unknown city "Etc/Unknown" for the current locale.
                //
                // If there is no localized form of "Etc/Unknown" for the current locale,
                // returns the "Etc/Unknown" value of the `und` locale as a hard-coded string.
                //
                // This can be used as a fallback if [`exemplar_city()`](TimeZoneFormatter::exemplar_city())
                // is unable to produce a localized form of the time zone's exemplar city in the current locale.
                let formatted_unknown_city = data_payloads
                    .exemplar_cities
                    .as_ref()
                    .and_then(|cities| cities.0.get(&TimeZoneBcp47Id(tinystr!(8, "unk"))))
                    .unwrap_or(&Cow::Borrowed("Unknown"));
                Ok(sink.write_str(formatted_unknown_city)?)
            }
        })
    }
}

impl FormatTimeZone for Bcp47IdFormat {
    fn format<W: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut W,
        input: &ExtractedInput,
        _data_payloads: TimeZoneDataPayloadsBorrowed,
    ) -> Result<Result<(), FormatTimeZoneError>, fmt::Error> {
        Ok(match input.time_zone_id {
            Some(bcp47_id) => {
                sink.write_str(&bcp47_id)?;
                Ok(())
            }
            None => Err(FormatTimeZoneError::MissingInputField("time_zone_id")),
        })
    }
}

/// Formats a time segment with optional zero-padding.
fn format_time_segment<W: writeable::PartsWrite + ?Sized>(
    sink: &mut W,
    n: u8,
    padding: ZeroPadding,
) -> fmt::Result {
    debug_assert!((0..60).contains(&n));
    if padding == ZeroPadding::On && n < 10 {
        sink.write_char('0')?;
    }
    n.write_to(sink)
}

fn format_offset_hours<W: writeable::PartsWrite + ?Sized>(
    sink: &mut W,
    offset: UtcOffset,
    padding: ZeroPadding,
) -> fmt::Result {
    format_time_segment(
        sink,
        (offset.offset_seconds() / 3600).unsigned_abs() as u8,
        padding,
    )
}

fn format_offset_minutes<W: writeable::PartsWrite + ?Sized>(
    sink: &mut W,
    offset: UtcOffset,
) -> fmt::Result {
    format_time_segment(
        sink,
        (offset.offset_seconds() % 3600 / 60).unsigned_abs() as u8,
        ZeroPadding::On,
    )
}

fn format_offset_seconds<W: writeable::PartsWrite + ?Sized>(
    sink: &mut W,
    offset: UtcOffset,
) -> fmt::Result {
    format_time_segment(
        sink,
        (offset.offset_seconds() % 3600 % 60).unsigned_abs() as u8,
        ZeroPadding::On,
    )
}
