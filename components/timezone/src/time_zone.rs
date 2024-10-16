// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{TimeZoneBcp47Id, UtcOffset, ZoneVariant};
use icu_calendar::{Date, Iso, Time};

/// A utility type that can hold time zone information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // these four fields fully cover the needs of UTS 35
pub struct TimeZoneInfo {
    /// The BCP47 time-zone identifier.
    pub time_zone_id: TimeZoneBcp47Id,
    /// The UTC offset, if known.
    ///
    /// This field is not enforced to be consistent with the time zone id.
    pub offset: Option<UtcOffset>,
    /// The time variant e.g. daylight or standard, if known.
    pub zone_variant: Option<ZoneVariant>,
    /// The time at which to interpret the time zone.
    ///
    /// This can be set in order to get correct historical time zone names.
    /// If it's not set, the most recent data for the time zone will be used.
    pub local_time: Option<(Date<Iso>, Time)>,
}

impl TimeZoneInfo {
    /// Creates a time zone info with no information.
    pub const fn unknown() -> Self {
        Self {
            offset: None,
            time_zone_id: TimeZoneBcp47Id::unknown(),
            zone_variant: None,
            local_time: None,
        }
    }

    /// Creates a new [`TimeZoneInfo`] for the UTC time zone.
    pub const fn utc() -> Self {
        Self {
            offset: Some(UtcOffset::zero()),
            time_zone_id: TimeZoneBcp47Id(tinystr::tinystr!(8, "utc")),
            zone_variant: Some(ZoneVariant::standard()),
            local_time: None,
        }
    }
}
