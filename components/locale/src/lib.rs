//! `icu-locale` is one of the [`ICU4X`] components.
//!
//! It is a core API for parsing, manipulating, and serializing Unicode Language
//! and Locale Identifiers.
//!
//! The crate provides algorithms for parsing a string into a well-formed language identifier
//! as defined by [`UTS #35: Unicode LDML 3.1 Unicode Language Identifier`].
//!
//! # Example
//!
//! ```
//! use icu_locale::LanguageIdentifier;
//! use icu_locale::subtags::{Language, Region};
//!
//! let mut li: LanguageIdentifier = "en-US".parse()
//!     .expect("Parsing failed.");
//!
//! let lang: Language = "en".parse()
//!     .expect("Parsing failed.");
//! let region: Region = "US".parse()
//!     .expect("Parsing failed.");
//!
//! assert_eq!(li.language, lang);
//! assert_eq!(li.script, None);
//! assert_eq!(li.region, Some(region));
//! assert_eq!(li.variants.len(), 0);
//!
//! let region: Region = "GB".parse().expect("Parsing failed.");
//! li.region = Some(region);
//!
//! assert_eq!(li.to_string(), "en-GB");
//! ```
//!
//! For more details, see [`LanguageIdentifier`].
//!
//! [`UTS #35: Unicode LDML 3.1 Unicode Language Identifier`]: https://unicode.org/reports/tr35/tr35.html#Unicode_language_identifier
//! [`LanguageIdentifier`]: ./struct.LanguageIdentifier.html
//! [`ICU4X`]: https://github.com/unicode-org/icu4x
mod langid;
mod parser;
pub mod subtags;

pub use langid::LanguageIdentifier;
pub use parser::errors::ParserError;
