// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
//! Parsers for extended date time string and ISO Duration parsing.
//!
//! The [Internet Extended Date/Time Fmt (IXDTF)][ixdtf-draft] as laid out by Serialising
//! Extended Data About Times and Events' (sedate) builds on RFC3339's time stamp specification and
//! ISO8601 to provide an optional extension syntax for date time strings.
//!
//! ## Date Time Extended Examples
//!
//! - `2024-03-02T08:48:00-05:00[America/New_York]`
//! - `2024-03-02T08:48:00-05:00[-05:00]`
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601]`
//!
//! ## Example Usage
//!
//! ```
//! use ixdtf::parser::{IxdtfParser, records::{IsoParseRecord, DateRecord, TimeRecord, TimeZone}};
//!
//! let ixdtf_str = "2024-03-02T08:48:00-05:00[America/New_York]";
//!
//! let result = IxdtfParser::new(ixdtf_str).parse_date_time().unwrap();
//!
//! let date = result.date;
//! let time = result.time.unwrap();
//! let tz = result.tz.unwrap();
//!
//! assert_eq!(date.year, 2024);
//! assert_eq!(date.month, 3);
//! assert_eq!(date.day, 2);
//! assert_eq!(time.hour, 8);
//! assert_eq!(time.minute, 48);
//! assert_eq!(tz.name, Some("America/New_York".to_owned()));
//!
//! ```
//!
//! ## Date Time Strings
//!
//! These extended suffixes are optional, so the `IXDTFParser` will also still parse any valid
//! date time strings.
//!
//! Example Valid Date Time Strings:
//!
//! - `2024-03-02`
//! - `+002024-03-02`
//! - `20240302`
//! - `+0020240302`
//! - `2024-03-02T08:48:00`
//! - `2024-03-02T08:48:00`
//!
//! ## IXDTF Extensions: A Deeper Look
//!
//! The suffix extensions come in two primary kinds: a time zone annotation and a key-value
//! annotation. The suffixes may also be flagged as critical with a '!' as a leading flag
//! character.
//!
//! ### Time Zone Annotations
//!
//! Time zone annotations can be either a valud time zone IANA name or a number
//! offset as shown previously.
//!
//! #### Valid Time Zone Annotations
//!
//! - `2024-03-02T08:48:00-5:00[America/New_York]`
//! - `2024-03-02T08:48:00-5:00[-05:00]`
//!
//! ### Key-Value Annotations
//!
//! Key-value pair annotations are any keys that are delimited by a '='. Key-value
//! pairs are can include any information. Keys can be permanent registered, provisional
//! registered, or unknown; however, only permanent keys are acted on. Any unknown key
//! may be provided, but will be ignored unless flagged critical. If an unknown key, is
//! flagged as critical, an error will be thrown.
//!
//! If duplicate keys will are provided the first will key will be returned, unless one
//! of the duplicate annotations is marked as critical, in which case an error will be
//! thrown.
//!
//! #### Permanent Registered Keys:
//!
//! - `u-ca`
//!
//! #### Valid Key-Value Annotations
//!
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601]`
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601][u-ca=japanese]`
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601][!answer-to-universe=fortytwo]`
//!
//! #### Invalid Key-Value Annotations
//!
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601]`
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601][!u-ca=japanese]`
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601][!answer-to-universe=fortytwo]`
//!
//!
//! [ixdtf-draft]: https://datatracker.ietf.org/doc/draft-ietf-sedate-datetime-extended/

#![no_std]
// TODO(#2127): Fix this lint.
#![allow(clippy::needless_return)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]

mod error;
pub mod parser;

extern crate alloc;

pub use error::ParserError;

pub type ParserResult<T> = Result<T, ParserError>;
