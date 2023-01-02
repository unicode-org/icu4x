// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A parser for [Time Zone Information Format (`TZif`)](https://tools.ietf.org/id/draft-murchison-tzdist-tzif-00.html) files.
//!
//! Also includes a parser for [POSIX time-zone strings](https://www.gnu.org/software/libc/manual/html_node/TZ-Variable.html),
//! which is used by the TZif parser, but also available separately.
//!
//! Resources to generate `TZif` files are provided by the [IANA database](https://www.iana.org/time-zones).
//! `TZif` files are also included in some operating systems.
//!
//! # Examples
//!
//! ### Parse TZif Files
//! ```no_run
//! let data = tzif::parse_tzif_file("path_to_file").unwrap();
//! ```
//!
//! ### Parse POSIX time-zone strings
//! ```rust
//! let data =
//!     tzif::parse_posix_tz_string(b"WGT3WGST,M3.5.0/-2,M10.5.0/-1").unwrap();
//! ```

#![warn(missing_docs)]

use combine::Parser;
use data::{posix::PosixTzString, tzif::TzifData};
use error::Error;

/// The parsed data representations.
pub mod data;

/// The parser implementations.
pub mod parse;

/// Error types an implementations.
pub mod error;

/// Parses a `TZif` file
pub fn parse_tzif(bytes: &[u8]) -> Result<TzifData, Error> {
    Ok(parse::tzif::tzif().parse(bytes)?.0)
}

/// Parses a POSIX time-zone string from the given bytes.
pub fn parse_posix_tz_string(bytes: &[u8]) -> Result<PosixTzString, Error> {
    Ok(parse::posix::posix_tz_string().parse(bytes)?.0)
}
