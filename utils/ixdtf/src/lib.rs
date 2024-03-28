// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsers for extended date time string and Duration parsing.
//!
//! The [Internet Extended Date/Time Fmt (IXDTF)][ixdtf-draft] as laid out by Serialising
//! Extended Data About Times and Events' (sedate) builds on RFC3339's time stamp specification and
//! ISO8601 to provide an optional extension syntax for date time strings.
//!
//! # Date Time Extended Examples
//!
//! - `2024-03-02T08:48:00-05:00[America/New_York]`
//! - `2024-03-02T08:48:00-05:00[-05:00]`
//! - `2024-03-02T08:48:00-05:00[u-ca=iso8601]`
//!
//! ## Example Usage
//!
//! ```
//! use ixdtf::parsers::{IxdtfParser, records::{Sign, TimeZoneRecord, UTCOffsetRecord}};
//!
//! let ixdtf_str = "2024-03-02T08:48:00-05:00[America/New_York]";
//!
//! let result = IxdtfParser::new(ixdtf_str).parse().unwrap();
//!
//! let date = result.date.unwrap();
//! let time = result.time.unwrap();
//! let offset = result.offset.unwrap();
//! let tz_annotation = result.tz.unwrap();
//!
//! assert_eq!(date.year, 2024);
//! assert_eq!(date.month, 3);
//! assert_eq!(date.day, 2);
//! assert_eq!(time.hour, 8);
//! assert_eq!(time.minute, 48);
//! assert_eq!(offset.sign, Sign::Negative);
//! assert_eq!(offset.hour, 5);
//! assert_eq!(offset.minute, 0);
//! assert!(!tz_annotation.critical);
//! assert_eq!(tz_annotation.tz, TimeZoneRecord::Name("America/New_York"));
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
//! pairs are can include any information. Keys can be permanently registered, provisionally
//! registered, or unknown; however, only permanent keys are acted on by the parser. All
//! annotations will be parsed and returned to the user in a `Vec` of annotations.
//!
//! If duplicate registered keys are provided the first will key will be returned, unless one
//! of the duplicate annotations is marked as critical, in which case an error will be
//! thrown by the parser.
//!
//! #### Permanent Registered Keys
//!
//! - `u-ca`
//!
//! #### Valid Annotations Examples
//!
//! - (1) `2024-03-02T08:48:00-05:00[America/New_York][u-ca=iso8601]`
//! - (2) `2024-03-02T08:48:00-05:00[u-ca=iso8601][u-ca=japanese]`
//! - (3) `2024-03-02T08:48:00-05:00[u-ca=iso8601][answer-to-universe=fortytwo]`
//!
//! (1) is a basic annotation string that has a Time Zone and calendar annotation.
//!
//! (2) has a duplicate calendar, but neither calendar is flagged as critical so
//! the first calendar is returned while the second calendar is ignored.
//!
//! (3) shows an unknown key-value annotation. In this situation, the annotation
//! is not flagged as critical, so the annotation is valid.
//!
//! #### Invalid Annotations Examples
//!
//! The below invalid annotations have to primary groups: (a) invalid annotations that
//! `ixdtf` will handle and throw an error, and (b) invalid annotations that will NOT
//! be handled by `ixdtf`, but will be presented to the user to handle as they see fit.
//!
//! - (1) `2024-03-02T08:48:00-05:00[u-ca=iso8601][America/New_York]`
//! - (2) `2024-03-02T08:48:00-05:00[u-ca=iso8601][!u-ca=japanese]`
//! - (3) `2024-03-02T08:48:00-05:00[u-ca=iso8601][!answer-to-universe=fortytwo]`
//! - (4) `2024-03-02T08:48:00+01:00[America/New_York]`
//!
//! (1) belongs to group (a) and shows a Time Zone annotation that is not currently
//! in the correct order with the key value. When parsing this invalid annotation, `ixdtf`
//! will attempt to parse the Time Zone annotation as a key-value annotation.
//!
//! ```rust
//! use ixdtf::{parsers::IxdtfParser, ParserError};
//!
//! let example_one = "2024-03-02T08:48:00-05:00[u-ca=iso8601][America/New_York]";
//!
//! let mut ixdtf = IxdtfParser::new(example_one);
//!
//! let result = ixdtf.parse();
//!
//! assert_eq!(result, Err(ParserError::AnnotationKeyLeadingChar));
//! ```
//!
//! (2) belongs to group (a) and shows a duplicate registered key; however, in
//! this case, one of the registered keys is flagged as critical, which throws an error
//! as the ixdtf string must be treated as erroneous
//!
//! ```rust
//! use ixdtf::{parsers::IxdtfParser, ParserError};
//!
//! let example_one = "2024-03-02T08:48:00-05:00[u-ca=iso8601][!u-ca=japanese]";
//!
//! let result = IxdtfParser::new(example_one).parse();
//!
//! assert_eq!(result, Err(ParserError::CriticalDuplicateCalendar));
//! ```
//!
//! (3) belongs to group (b) and shows an unknown key flagged as critical. `ixdtf` is
//! agnostic regarding the ambiguity caused by the criticality of an unknown key. This will
//! be provided to the user to handle the unknown key's critical flag as they see fit.
//!
//! ```rust
//! use ixdtf::parsers::IxdtfParser;
//!
//! let example_one = "2024-03-02T08:48:00-05:00[u-ca=iso8601][!answer-to-universe=fortytwo]";
//!
//! let mut ixdtf = IxdtfParser::new(example_one);
//!
//! let result = ixdtf.parse().unwrap();
//!
//! let annotation = &result.annotations[0];
//!
//! // While an unknown annotation should not be critical, it is up to the user
//! // to act on that error.
//! assert!(annotation.critical);
//! assert_eq!(annotation.key, "answer-to-universe");
//! assert_eq!(annotation.value, "fortytwo");
//! ```
//!
//! (4) belongs to group (b) and shows an ambiguous Time Zone caused by a misalignment
//! of the offset and the Time Zone annotation. It is up to the user to handle this ambiguity
//! between the offset and annotation.
//!
//! ```rust
//! use ixdtf::parsers::{IxdtfParser, records::TimeZoneRecord};
//!
//! let example_one = "2024-03-02T08:48:00+01:00[!America/New_York]";
//!
//! let mut ixdtf = IxdtfParser::new(example_one);
//!
//! let result = ixdtf.parse().unwrap();
//!
//! let tz_annotation = result.tz.unwrap();
//! let offset = result.offset.unwrap();
//!
//! // The time zone annotation and offset conflict with each other, and must therefore be
//! // resolved by the user.
//! assert!(tz_annotation.critical);
//! assert_eq!(tz_annotation.tz, TimeZoneRecord::Name("America/New_York"));
//! assert_eq!(offset.hour, 1);
//! ```
//!
//! ## Additional DateTime grammar resources
//!
//! Additional resources for Date and Time string grammar can be found in [RFC3339][rfc3339]
//! and the [Temporal proposal][temporal-grammar].
//!
//! [ixdtf-draft]: https://datatracker.ietf.org/doc/draft-ietf-sedate-datetime-extended/
//! [rfc3339]: https://datatracker.ietf.org/doc/html/rfc3339
//! [temporal-grammar]: https://tc39.es/proposal-temporal/#sec-temporal-iso8601grammar

#![no_std]
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
pub mod parsers;

extern crate alloc;

pub use error::ParserError;

pub type ParserResult<T> = Result<T, ParserError>;
