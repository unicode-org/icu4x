// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_properties` is a utility crate of the [`ICU4X`] project.
//!
//! This component provides definitions of [Unicode Properties] and APIs for
//! retrieving property data in an appropriate data structure.
//!
//! APIs that return a [`UnicodeSet`] exist for binary properties and certain enumerated
//! properties. See the [`sets`] module for more details.
//!
//! APIs that return a [`CodePointTrie`] exist for certain enumerated properties. See the
//! [`maps`] module for more details.
//!
//! # Examples
//!
//! ## Property data as `UnicodeSet`s
//!
//! ```
//! use icu::properties::sets;
//! use icu_uniset::UnicodeSet;
//!
//! let provider = icu_testdata::get_provider();
//!
//! let emoji_payload =
//!     sets::get_emoji(&provider)
//!         .expect("The data should be valid");
//! let emoji_data_struct = emoji_payload.get();
//! let emoji = &emoji_data_struct.inv_list;
//! assert!(!emoji.contains_u32('木' as u32));  // U+6728
//! assert!(emoji.contains_u32('🎃' as u32));  // U+1F383 JACK-O-LANTERN
//! ```
//!
//! ## Property data as `CodePointTrie`s
//!
//! ```
//! use icu::properties::{maps, Script};
//! use icu_codepointtrie::codepointtrie::CodePointTrie;
//!
//! let provider = icu_testdata::get_provider();
//!
//! let script_payload =
//!     maps::get_script(&provider)
//!         .expect("The data should be valid");
//! let script_data_struct = script_payload.get();
//! let script = &script_data_struct.codepoint_trie;
//! assert_eq!(script.get('木' as u32), Script::Han);  // U+6728
//! assert_eq!(script.get('🎃' as u32), Script::Common);  // U+1F383 JACK-O-LANTERN
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
//! [`UnicodeSet`]: icu_uniset::UnicodeSet
//! [`sets`]: crate::sets

#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod error;
pub mod maps;
mod props;
pub mod provider;
pub mod sets;
mod trievalue;
mod ule;

pub use props::*;
