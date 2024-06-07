// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::time_zone::Iso8601Format;

mod private {
    pub trait Sealed {}
}

pub enum NeoOffsetStyle {
    /// "GMT-8"
    /// `O`
    GmtShort,
    /// "GMT-08:00"
    /// `OOOO`, `ZZZZ`
    GmtLong,
    /// All `x` and `X` formats
    Iso8601(Iso8601Format),
}

pub enum NeoZoneStyle {
    Offset(NeoOffsetStyle),
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

#[non_exhaustive]
pub struct NeoZoneConfig {
    first: Option<NeoZoneStyle>,
    second: Option<NeoZoneStyle>,
    offset: NeoOffsetStyle,
}

pub trait NeoZone: private::Sealed {
    const ZONE_CONFIG: NeoZoneConfig;
}

pub enum GmtShort {}

impl private::Sealed for GmtShort {}

impl NeoZone for GmtShort {
    const ZONE_CONFIG: NeoZoneConfig = NeoZoneConfig {
        first: None,
        second: None,
        offset: NeoOffsetStyle::GmtShort,
    };
}

pub enum GenericShortOrGmt {}

impl private::Sealed for GenericShortOrGmt {}

impl NeoZone for GenericShortOrGmt {
    const ZONE_CONFIG: NeoZoneConfig = NeoZoneConfig {
        first: Some(NeoZoneStyle::GenericShort),
        second: None,
        offset: NeoOffsetStyle::GmtShort,
    };
}

pub enum GenericLocationOrGmt {}

impl private::Sealed for GenericLocationOrGmt {}

impl NeoZone for GenericLocationOrGmt {
    const ZONE_CONFIG: NeoZoneConfig = NeoZoneConfig {
        first: Some(NeoZoneStyle::GenericLocation),
        second: None,
        offset: NeoOffsetStyle::GmtShort,
    };
}

pub enum GenericShortOrLocation {}

impl private::Sealed for GenericShortOrLocation {}

impl NeoZone for GenericShortOrLocation {
    const ZONE_CONFIG: NeoZoneConfig = NeoZoneConfig {
        first: Some(NeoZoneStyle::GenericShort),
        second: Some(NeoZoneStyle::GenericLocation),
        offset: NeoOffsetStyle::GmtShort,
    };
}

