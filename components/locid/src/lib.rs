// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing, manipulating, and serializing Unicode Language and Locale Identifiers.
//!
//! This module is published as its own crate ([`icu_locid`](https://docs.rs/icu_locid/latest/icu_locid/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! The module provides algorithms for parsing a string into a well-formed language or locale identifier
//! as defined by [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`].
//!
//! [`Locale`] is the most common structure to use for storing information about a language,
//! script, region, variants and extensions. In almost all cases, this struct should be used as the
//! base unit for all locale management operations.
//!
//! [`LanguageIdentifier`] is a strict subset of [`Locale`] which can be useful in a narrow range of
//! cases where [`Unicode Extensions`] are not relevant.
//!
//! If in doubt, use [`Locale`].
//!
//! # Examples
//!
//! ```
//! use icu::locid::subtags::{Language, Region};
//! use icu::locid::Locale;
//!
//! let mut loc: Locale = "en-US".parse().expect("Parsing failed.");
//!
//! let lang: Language = "en".parse().expect("Parsing failed.");
//! let region: Region = "US".parse().expect("Parsing failed.");
//!
//! assert_eq!(loc.id.language, lang);
//! assert_eq!(loc.id.script, None);
//! assert_eq!(loc.id.region, Some(region));
//! assert_eq!(loc.id.variants.len(), 0);
//!
//! let region: Region = "GB".parse().expect("Parsing failed.");
//! loc.id.region = Some(region);
//!
//! assert_eq!(loc.to_string(), "en-GB");
//! ```
//!
//! ## Macros
//!
//! ```rust
//! use icu::locid::{langid, subtags_language as language, subtags_region as region};
//!
//! let lid = langid!("EN_US");
//!
//! assert_eq!(lid.language, language!("en"));
//! assert_eq!(lid.region, Some(region!("US")));
//! ```

//!
//! For more details, see [`Locale`] and [`LanguageIdentifier`].
//!
//! [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
//! [`ICU4X`]: ../icu/index.html
//! [`Unicode Extensions`]: extensions

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

#[macro_use]
mod helpers;

#[cfg(feature = "databake")]
mod databake;
pub mod extensions;
mod langid;
mod locale;
mod macros;
pub mod ordering;
mod parser;
#[cfg(feature = "serde")]
mod serde;
pub mod subtags;
#[cfg(feature = "zerovec")]
pub mod zerovec;

pub use langid::LanguageIdentifier;
pub use locale::Locale;
pub use parser::errors::ParserError;
