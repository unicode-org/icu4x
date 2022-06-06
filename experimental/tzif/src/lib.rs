// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A parser for [Time Zone Information Format (TZif)](https://tools.ietf.org/id/draft-murchison-tzdist-tzif-00.html) files.
//!
//! Resources to generate TZif files are provided by the [IANA database](https://www.iana.org/time-zones).
//! TZif files are also included in some operating systems.

#![warn(missing_docs)]

/// The parsed data representations.
pub mod data;

/// The parser implementations.
pub mod parse;
