// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::*;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct MockTimeZone {
    pub gmt_offset: GmtOffset,
    pub time_zone_id: Option<String>,
    pub metazone_id: Option<String>,
    pub time_variant: Option<String>,
}

impl MockTimeZone {
    pub const fn new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<String>,
        metazone_id: Option<String>,
        time_variant: Option<String>,
    ) -> Self {
        Self {
            gmt_offset,
            time_zone_id,
            metazone_id,
            time_variant,
        }
    }

    pub fn try_new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<String>,
        metazone_id: Option<String>,
        time_variant: Option<String>,
    ) -> Result<Self, DateTimeError> {
        Ok(Self {
            gmt_offset,
            time_zone_id,
            metazone_id,
            time_variant,
        })
    }
}
impl FromStr for MockTimeZone {
    type Err = DateTimeError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let gmt_offset = GmtOffset::from_str(input)?;
        Ok(Self {
            gmt_offset,
            time_zone_id: None,
            metazone_id: None,
            time_variant: None,
        })
    }
}

impl TimeZoneInput for MockTimeZone {
    fn gmt_offset(&self) -> GmtOffset {
        self.gmt_offset
    }

    fn time_zone_id(&self) -> Option<&str> {
        self.time_zone_id.as_ref().map(AsRef::as_ref)
    }

    fn metazone_id(&self) -> Option<&str> {
        self.metazone_id.as_ref().map(AsRef::as_ref)
    }

    fn time_variant(&self) -> Option<&str> {
        self.time_variant.as_ref().map(AsRef::as_ref)
    }
}
