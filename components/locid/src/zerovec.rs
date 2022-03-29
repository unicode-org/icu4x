// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Documentation on zero-copy deserialization of locale types.
//!
//! [`Locale`] and [`LanguageIdentifier`] are highly structured types that cannot be directly
//! stored in a zero-copy data structure, such as those provided by the [`zerovec`] crate.
//! This page explains how to indirectly store these types in a [`zerovec`].
//!
//! There are two main use cases, which have different solutions:
//!
//! 1. **Lookup:** You need to locate a locale in a zero-copy vector, such as when querying a map.
//! 2. **Obtain:** You have a locale stored in a zero-copy vector, and you need to obtain a proper
//!    [`Locale`] or [`LanguageIdentifier`] for use elsewhere in your program.
//!
//! # Lookup
//!
//! To perform lookup, store the BCP-47 byte array for the locale, and then use
//! [`Locale::cmp_bytes()`] to perform an efficient, zero-allocation lookup.
//!
//! ```
//! use std::str::FromStr;
//! use icu_locid::Locale;
//! use zerovec::ZeroMap;
//!
//! // ZeroMap from locales to integers
//! let data: &[(&[u8], u32)] = &[
//!     (b"de-DE-u-hc-h12", 5),
//!     (b"en-US-u-ca-buddhist", 10),
//!     (b"my-MM", 15),
//!     (b"sr-Cyrl-ME", 20),
//!     (b"zh-TW", 25),
//! ];
//! let zm: ZeroMap<[u8], u32> = data.iter().copied().collect();
//!
//! // Get the value associated with a locale
//! let loc: Locale = "en-US-u-ca-buddhist".parse().unwrap();
//! let value = zm.get_copied_by(|bytes| loc.cmp_bytes(bytes).reverse());
//! assert_eq!(value, Some(10));
//! ```
//!
//! # Obtain
//!
//! Obtaining a [`Locale`] or [`LanguageIdentifier`] is not generally a zero-copy operation, since
//! both of these types may require memory allocation. If possible, architect your code such that
//! you do not need to obtain a structured type.
//!
//! If need the structured type, such as if you need to manipulate it in some way, there are two
//! options: storing subtags, and storing a string for parsing.
//!
//! ## Storing Subtags
//!
//! If the data being stored only contains a limited number of subtags, you can store them as a
//! tuple, and then construct the [`LanguageIdentifier`] externally.
//!
//! ```
//! use icu_locid::LanguageIdentifier;
//! use icu_locid::subtags::{Language, Script, Region};
//! use icu_locid::{language, script, region, langid};
//! use zerovec::ZeroMap;
//!
//! // ZeroMap from integer to LSR (language-script-region)
//! let data: &[(u32, (Language, Option<Script>, Option<Region>))] = &[
//!     (5, (language!("de"), None, Some(region!("DE")))),
//!     (10, (language!("en"), None, Some(region!("US")))),
//!     (15, (language!("my"), None, Some(region!("MM")))),
//!     (20, (language!("sr"), Some(script!("Cyrl")), Some(region!("ME")))),
//!     (25, (language!("zh"), None, Some(region!("TW")))),
//! ];
//! let zm: ZeroMap<u32, (Language, Option<Script>, Option<Region>)> =
//!     data.iter().copied().collect();
//!
//! // Construct a LanguageIdentifier from a tuple entry
//! let value = zm.get_copied(&25).expect("element is present");
//! let mut lid = LanguageIdentifier::default();
//! lid.language = value.0;
//! lid.script = value.1;
//! lid.region = value.2;
//!
//! assert_eq!(lid, langid!("zh-TW"));
//! ```
//!
//! ## Storing Strings
//!
//! If it is necessary to store and obtain an arbitrary locale, it is currently recommended to
//! store a BCP-47 string and parse it when needed.
//!
//! Since the string is stored in an unparsed state, it is not safe to `unwrap` the result from
//! `Locale::from_bytes()`. See [icu4x#831](https://github.com/unicode-org/icu4x/issues/831)
//! for a discussion on potential data models that could ensure that the locale is valid during
//! deserialization.
//!
//! ```
//! use icu_locid::Locale;
//! use icu_locid::langid;
//! use zerovec::ZeroMap;
//!
//! // ZeroMap from integer to locale string
//! let data: &[(u32, &[u8])] = &[
//!     (5, b"de-DE-u-hc-h12"),
//!     (10, b"en-US-u-ca-buddhist"),
//!     (15, b"my-MM"),
//!     (20, b"sr-Cyrl-ME"),
//!     (25, b"zh-TW"),
//! ];
//! let zm: ZeroMap<u32, [u8]> = data.iter().copied().collect();
//!
//! // Construct a Locale by parsing the string.
//! let value = zm.get(&25).expect("element is present");
//! let loc = Locale::from_bytes(value);
//!
//! assert_eq!(loc, Ok(langid!("zh-TW").into()));
//! ```
//!
//! [`Locale`]: crate::Locale
//! [`Locale::cmp_bytes()`]: crate::Locale::cmp_bytes()
//! [`LanguageIdentifier`]: crate::LanguageIdentifier
