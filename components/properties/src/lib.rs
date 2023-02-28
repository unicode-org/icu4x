// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Definitions of [Unicode Properties] and APIs for
//! retrieving property data in an appropriate data structure.
//!
//! This module is published as its own crate ([`icu_properties`](https://docs.rs/icu_properties/latest/icu_properties/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! APIs that return a [`CodePointSetData`] exist for binary properties and certain enumerated
//! properties. See the [`sets`] module for more details.
//!
//! APIs that return a [`CodePointMapData`] exist for certain enumerated properties. See the
//! [`maps`] module for more details.
//!
//! # Examples
//!
//! ## Property data as `CodePointSetData`s
//!
//! ```
//! use icu::properties::{maps, sets, GeneralCategory};
//!
//! // A binary property as a `CodePointSetData`
//!
//! let data = sets::load_emoji(&icu_testdata::unstable())
//!     .expect("The data should be valid");
//! let emoji = data.as_borrowed();
//!
//! assert!(emoji.contains('🎃')); // U+1F383 JACK-O-LANTERN
//! assert!(!emoji.contains('木')); // U+6728
//!
//! // An individual enumerated property value as a `CodePointSetData`
//!
//! let data = maps::load_general_category(&icu_testdata::unstable())
//!     .expect("The data should be valid");
//! let gc = data.as_borrowed();
//! let line_sep_data = gc.get_set_for_value(GeneralCategory::LineSeparator);
//! let line_sep = line_sep_data.as_borrowed();
//!
//! assert!(line_sep.contains32(0x2028));
//! assert!(!line_sep.contains32(0x2029));
//! ```
//!
//! ## Property data as `CodePointMapData`s
//!
//! ```
//! use icu::properties::{maps, Script};
//!
//! let map = maps::load_script(&icu_testdata::unstable())
//!     .expect("The data should be valid");
//! let script = map.as_borrowed();
//!
//! assert_eq!(script.get('🎃'), Script::Common); // U+1F383 JACK-O-LANTERN
//! assert_eq!(script.get('木'), Script::Han); // U+6728
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
//! [`CodePointSetData`]: crate::sets::CodePointSetData
//! [`CodePointMapData`]: crate::maps::CodePointMapData
//! [`sets`]: crate::sets

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

#[cfg(feature = "bidi")]
pub mod bidi;

mod error;
pub mod maps;

// NOTE: The Pernosco debugger has special knowledge
// of the `CanonicalCombiningClass` struct inside the `props`
// module. Please do not change the crate-module-qualified
// name of that struct without coordination.
mod props;

pub mod exemplar_chars;
pub mod provider;
#[allow(clippy::exhaustive_structs)] // TODO
pub mod script;
pub mod sets;
mod trievalue;

pub use props::{
    BidiClass, BidiPairedBracketType, CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GeneralCategoryGroup,
    GraphemeClusterBreak, LineBreak, Script, SentenceBreak, WordBreak,
};

pub use error::PropertiesError;

#[doc(inline)]
pub use PropertiesError as Error;
