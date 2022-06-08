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
//! use combine::{Parser, stream};
//! use std::fs::File;
//! use tzif::tzif;
//!
//! let file = File::open("path_to_file").unwrap();
//! let stream = stream::buffered::Stream::new(
//!     stream::position::Stream::new(stream::read::Stream::new(file)),
//!     0, /* lookahead */
//! );
//! let data = tzif().parse(stream).unwrap();
//! ```
//!
//! ### Parse POSIX time-zone strings
//! ```rust
//! use combine::Parser;
//! use tzif::posix_tz_string;
//!
//! let data = posix_tz_string()
//!     .parse(b"WGT3WGST,M3.5.0/-2,M10.5.0/-1".as_slice())
//!     .unwrap();
//! ```

#![warn(missing_docs)]

/// The parsed data representations.
pub mod data;

/// The parser implementations.
pub mod parse;

pub use parse::posix::posix_tz_string;
pub use parse::tzif::tzif;
