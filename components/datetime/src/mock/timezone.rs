// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::date::*;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct MockTimeZone {
    pub gmt_offset: GmtOffset,
    pub time_zone_id: Option<String>,
    pub metazone: Option<String>,
    pub variant: Option<String>,
    pub country_code: Option<String>,
}

impl MockTimeZone {
    pub const fn new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<String>,
        metazone: Option<String>,
        variant: Option<String>,
        country_code: Option<String>,
    ) -> Self {
        Self {
            gmt_offset,
            time_zone_id,
            metazone,
            variant,
            country_code,
        }
    }

    pub fn try_new(
        gmt_offset: GmtOffset,
        time_zone_id: Option<String>,
        metazone: Option<String>,
        variant: Option<String>,
        country_code: Option<String>,
    ) -> Result<Self, DateTimeError> {
        Ok(Self {
            gmt_offset,
            time_zone_id,
            metazone,
            variant,
            country_code,
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
            metazone: None,
            variant: None,
            country_code: None,
        })
    }
}

impl TimeZoneInput for MockTimeZone {
    fn gmt_offset(&self) -> GmtOffset {
        self.gmt_offset
    }

    fn time_zone_id(&self) -> Option<String> {
        None
    }

    fn metazone(&self) -> Option<String> {
        None
    }

    fn variant(&self) -> Option<String> {
        None
    }

    fn country_code(&self) -> Option<String> {
        None
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct GmtOffset(i32);

impl GmtOffset {
    // TODO(nordzilla) This is wrong for some locales. This needs to use the localized hour_format.
    pub fn to_short_string(&self) -> String {
        format!("{:>+.3}:{:.2}", self.0 / 3600, (self.0 % 3600 / 60).abs())
    }

    // TODO(nordzilla) This is wrong for some locales. This needs to use the localized hour_format.
    pub fn to_long_string(&self) -> String {
        format!(
            "{:>+03.3}:{:02.2}",
            self.0 / 3600,
            (self.0 % 3600 / 60).abs()
        )
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl FromStr for GmtOffset {
    type Err = DateTimeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let offset_sign;
        match input.chars().next() {
            Some('+') => offset_sign = 1,
            Some('-') => offset_sign = -1,
            Some('Z') => return Ok(GmtOffset(0)),
            Some(c) => {
                return Err(DateTimeError::UnexpectedSymbol {
                    expected: '±',
                    found: c,
                })
            }
            None => return Err(DateTimeError::MissingTimeZoneOffset),
        };

        let seconds = match input.chars().count() {
            /* ±hh */
            3 => {
                let hour: u8 = input[1..3].parse()?;
                offset_sign * (hour as i32 * 60 * 60)
            }
            /* ±hhmm */
            5 => {
                let hour: u8 = input[1..3].parse()?;
                let minute: u8 = input[3..5].parse()?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            /* ±hh:mm */
            6 => {
                let hour: u8 = input[1..3].parse()?;
                let minute: u8 = input[4..6].parse()?;
                offset_sign * (hour as i32 * 60 * 60 + minute as i32 * 60)
            }
            _ => panic!("Invalid time-zone designator"),
        };

        if seconds > 97200 || seconds < -97200 {
            Err(DateTimeError::Overflow {
                field: "GmtOffset",
                max: 97200,
            })
        } else {
            Ok(Self(seconds))
        }
    }
}
