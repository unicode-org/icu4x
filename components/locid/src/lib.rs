// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_locid` is one of the [`ICU4X`] components.
//!
//! This API provides necessary functionality for parsing, manipulating, and serializing Unicode Language
//! and Locale Identifiers.
//!
//! The crate provides algorithms for parsing a string into a well-formed language or locale identifier
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
//! use icu::locid::Locale;
//! use icu::locid::subtags::{Language, Region};
//!
//! let mut loc: Locale = "en-US".parse()
//!     .expect("Parsing failed.");
//!
//! let lang: Language = "en".parse()
//!     .expect("Parsing failed.");
//! let region: Region = "US".parse()
//!     .expect("Parsing failed.");
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
//! For more details, see [`Locale`] and [`LanguageIdentifier`].
//!
//! [`UTS #35: Unicode LDML 3. Unicode Language and Locale Identifiers`]: https://unicode.org/reports/tr35/tr35.html#Unicode_Language_and_Locale_Identifiers
//! [`ICU4X`]: ../icu/index.html
//! [`Unicode Extensions`]: extensions

#[macro_use]
mod helpers;

pub mod extensions;
mod langid;
mod locale;
mod parser;
#[cfg(feature = "serde")]
mod serde;
pub mod subtags;

pub use langid::LanguageIdentifier;
pub use locale::Locale;
pub use parser::errors::ParserError;
