// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "experimental")]
use crate::neo_skeleton::FractionalSecondDigits;
use crate::options::preferences::HourCycle;

pub mod datetime;
#[cfg(feature = "experimental")]
pub mod neo;
pub mod time_zone;
pub mod zoned_datetime;

/// Internal non-pattern runtime options passed to the formatter
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormattingOptions {
    pub(crate) hour_cycle: Option<HourCycle>,
    #[cfg(feature = "experimental")]
    pub(crate) fractional_second_digits: Option<FractionalSecondDigits>,
    #[cfg(not(feature = "experimental"))]
    pub(crate) fractional_second_digits: Option<u8>,
}
