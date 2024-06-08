// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for neo time zone formatting.

use crate::fields;
#[cfg(feature = "experimental")]
use crate::neo_marker::private;
use crate::time_zone::Iso8601TimeZoneStyle;

/// Field for a custom time zone.
///
/// `vv` is currently unassigned.
pub(crate) const CUSTOM_TIME_ZONE_FIELD: fields::Field = fields::Field {
    symbol: fields::FieldSymbol::TimeZone(fields::TimeZone::LowerV),
    length: fields::FieldLength::TwoDigit,
};

/// Time zone offset formatting style.
#[derive(Debug, Copy, Clone)]
pub enum NeoOffsetStyle {
    /// "GMT-8"
    /// `O`
    GmtShort,
    /// "GMT-08:00"
    /// `OOOO`, `ZZZZ`
    GmtLong,
    /// All `x` and `X` formats
    Iso8601(Iso8601TimeZoneStyle),
}

/// Time zone formatting style, not including offsets.
#[derive(Debug, Copy, Clone)]
pub enum NeoZoneStyle {
    /// "PDT"
    /// `z..zzz`
    SpecificShort,
    /// "Pacific Daylight Time"
    /// `zzzz`
    SpecificLong,
    /// "PT"
    /// `v`
    GenericShort,
    /// "Pacific Time"
    /// `vvvv`
    GenericLong,
    /// "uslax"
    /// `V`
    IdentifierShort,
    /// "America/Los_Angeles"
    /// `VV`
    IdentifierLong,
    /// "Los Angeles"
    /// `VVV`
    City,
    /// "Los Angeles Time"
    /// `VVVV`
    GenericLocation,
    /// "Pacific Time (Los Angeles)"
    /// not available via field?
    GenericPartialLocation,
}

/// A specification for the desired display of a time zone.
///
/// Unlike date and time, we support only a single time zone component, but the
/// specific component can change.
///
/// Since the length of a time zone can vary independent of the date and time,
/// the time zone lengths are directly encoded into this enum.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct NeoZoneComponents {
    /// The primary time zone style.
    pub first: Option<NeoZoneStyle>,
    /// The fallback time zone style if the primary style is unavailable.
    pub second: Option<NeoZoneStyle>,
    /// The offset style if the time zone cannot be formatted.
    pub offset: NeoOffsetStyle,
}

impl NeoZoneComponents {
    pub(crate) fn from_field(
        field: fields::TimeZone,
        length: fields::FieldLength,
    ) -> Option<Self> {
        match (field, length) {
            // TODO: This should actually be GenericShortOrLocation according to spec
            (fields::TimeZone::LowerV, fields::FieldLength::One) => Some(GenericShortOrGmt::ZONE_CONFIG),
            _ => todo!()
        }
    }

    pub(crate) fn to_field(self) -> fields::Field {
        // TODO: Return real fields from here, too
        CUSTOM_TIME_ZONE_FIELD
    }

    pub(crate) fn default_for_fallback() -> Self {
        Self {
            first: None,
            second: None,
            offset: NeoOffsetStyle::Iso8601(Iso8601TimeZoneStyle::default_for_fallback()),
        }
    }
}

/// A type that has a time zone configuration associated with it.
// TODO: Make this require sealed
pub trait HasZoneComponents {
    /// The configuration for formatting this time zone.
    const ZONE_CONFIG: NeoZoneComponents;
}

/// "GMT-8"
#[derive(Debug)]
pub enum GmtShort {}

#[cfg(feature = "experimental")]
impl private::Sealed for GmtShort {}

impl HasZoneComponents for GmtShort {
    const ZONE_CONFIG: NeoZoneComponents = NeoZoneComponents {
        first: None,
        second: None,
        offset: NeoOffsetStyle::GmtShort,
    };
}

/// "PT" or "GMT-8"
#[derive(Debug)]
pub enum GenericShortOrGmt {}

#[cfg(feature = "experimental")]
impl private::Sealed for GenericShortOrGmt {}

impl HasZoneComponents for GenericShortOrGmt {
    const ZONE_CONFIG: NeoZoneComponents = NeoZoneComponents {
        first: Some(NeoZoneStyle::GenericShort),
        second: None,
        offset: NeoOffsetStyle::GmtShort,
    };
}

/// "Los Angeles Time" or "GMT-8"
#[derive(Debug)]
pub enum GenericLocationOrGmt {}

#[cfg(feature = "experimental")]
impl private::Sealed for GenericLocationOrGmt {}

impl HasZoneComponents for GenericLocationOrGmt {
    const ZONE_CONFIG: NeoZoneComponents = NeoZoneComponents {
        first: Some(NeoZoneStyle::GenericLocation),
        second: None,
        offset: NeoOffsetStyle::GmtShort,
    };
}

/// "PT" or "Los Angeles Time" or "GMT-8"
#[derive(Debug)]
pub enum GenericShortOrLocation {}

#[cfg(feature = "experimental")]
impl private::Sealed for GenericShortOrLocation {}

impl HasZoneComponents for GenericShortOrLocation {
    const ZONE_CONFIG: NeoZoneComponents = NeoZoneComponents {
        first: Some(NeoZoneStyle::GenericShort),
        second: Some(NeoZoneStyle::GenericLocation),
        offset: NeoOffsetStyle::GmtShort,
    };
}
