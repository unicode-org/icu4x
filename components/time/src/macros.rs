// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Macros for `icu_time`.

/// A macro allowing for compile-time construction of a [`UtcOffset`](crate::zone::UtcOffset).
///
/// The macro will perform syntax validation of the offset string.
///
/// # Examples
///
/// ```
/// use icu::time::{utc_offset, zone::UtcOffset};
///
/// const OFFSET: UtcOffset = utc_offset!("-07:00");
///
/// let offset: UtcOffset = "-07:00".parse().unwrap();
///
/// assert_eq!(OFFSET, offset);
/// ```
#[macro_export]
macro_rules! utc_offset {
    ($offset:literal) => {
        const {
            match $crate::zone::UtcOffset::try_from_str($offset) {
                Ok(offset) => offset,
                Err(_) => panic!(concat!("Invalid UTC offset string: ", $offset)),
            }
        }
    };
}
