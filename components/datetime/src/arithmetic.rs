// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Assorted functions to help with date calculations.

use crate::pattern::{Pattern, TimeGranularity};

/// Returns [`true`] if the most granular time being displayed will align with
/// the top of the hour, otherwise returns [`false`].
/// e.g. `12:00:00` is at the top of the hour for any display granularity.
/// e.g. `12:00:05` is only at the top of the hour if the seconds are not displayed.
pub fn is_top_of_hour(pattern: &Pattern, minute: u8, second: u8) -> bool {
    match pattern.most_granular_time() {
        None | Some(TimeGranularity::Hours) => true,
        Some(TimeGranularity::Minutes) => minute == 0,
        Some(TimeGranularity::Seconds) => minute + second == 0,
    }
}
