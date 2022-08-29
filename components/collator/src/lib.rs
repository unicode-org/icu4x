// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Various collation-related algorithms and constants in this file are
// adapted from ICU4C and, therefore, are subject to the ICU license as
// described in LICENSE.

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

//! Comparing strings according to language-dependent conventions.
//!
//! This module is published as its own crate ([`icu_collator`](https://docs.rs/icu_collator/latest/icu_collator/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! `Collator` is the main structure of the component. It accepts a set of arguments
//! which allow it to collect necessary data from the data provider, and once
//! instantiated, can be used to compare strings.
//!
//! Refer to the ICU User Guide sections for Collation that give an
//! [introduction](https://unicode-org.github.io/icu/userguide/collation/) and explain
//! [basic concepts](https://unicode-org.github.io/icu/userguide/collation/concepts.html).
//!
//! # Examples
//!
//! As its most basic purpose, `Collator` offers locale-aware ordering:
//!
//! ```
//! use core::cmp::Ordering;
//! use icu_collator::*;
//! use icu_locid::{Locale, langid};
//!
//! let data_provider = icu_testdata::get_provider();
//!
//! let locale_es: Locale = langid!("es").into();
//! let mut options = CollatorOptions::new();
//! options.set_strength(Some(Strength::Primary));
//! let collator_es: Collator =
//!     Collator::try_new_unstable(&data_provider, &locale_es.into(), options).unwrap();
//!
//! assert_eq!(collator_es.compare("manna", "mañana"), Ordering::Less);
//!
//! let locale_en: Locale = langid!("en").into();
//! let mut options = CollatorOptions::new();
//! options.set_strength(Some(Strength::Primary));
//! let collator_en: Collator =
//!     Collator::try_new_unstable(&data_provider, &locale_en.into(), options).unwrap();
//!
//! assert_eq!(collator_en.compare("manna", "mañana"), Ordering::Greater);
//!
//! ```
//!
//! ## Examples of `CollatorOptions`
//!
//! The [`CollatorOptions`] struct configures specific custom behavior for the `Collator`.  See docs
//! for [`CollatorOptions`] for more details.  Some basic descriptions and examples are below.
//!
//! ## Strength
//!
//! The degree of sensitivity in how to determine that strings are distinct.
//!
//! ```
//! use core::cmp::Ordering;
//! use icu_collator::*;
//!
//! let data_provider = icu_testdata::get_provider();
//!  
//! // Primary Level
//!
//! let mut options_l1 = CollatorOptions::new();
//! options_l1.set_strength(Some(Strength::Primary));
//! let collator_l1: Collator =
//!     Collator::try_new_unstable(&data_provider, &Default::default(), options_l1).unwrap();
//!
//! assert_eq!(collator_l1.compare("a", "b"), Ordering::Less);  // primary
//! assert_eq!(collator_l1.compare("as", "às"), Ordering::Equal);  // secondary
//! assert_eq!(collator_l1.compare("às", "at"), Ordering::Less);
//! assert_eq!(collator_l1.compare("ao", "Ao"), Ordering::Equal);  // tertiary
//! assert_eq!(collator_l1.compare("Ao", "aò"), Ordering::Equal);
//! assert_eq!(collator_l1.compare("A", "Ⓐ"), Ordering::Equal);
//!
//! // Secondary Level
//!
//! let mut options_l2 = CollatorOptions::new();
//! options_l2.set_strength(Some(Strength::Secondary));
//! let collator_l2: Collator =
//!     Collator::try_new_unstable(&data_provider, &Default::default(), options_l2).unwrap();
//!
//! assert_eq!(collator_l2.compare("a", "b"), Ordering::Less);  // primary
//! assert_eq!(collator_l2.compare("as", "às"), Ordering::Less);  // secondary
//! assert_eq!(collator_l2.compare("às", "at"), Ordering::Less);
//! assert_eq!(collator_l2.compare("ao", "Ao"), Ordering::Equal);  // tertiary
//! assert_eq!(collator_l2.compare("Ao", "aò"), Ordering::Less);
//! assert_eq!(collator_l2.compare("A", "Ⓐ"), Ordering::Equal);
//!
//! // Tertiary Level
//!
//! let mut options_l3 = CollatorOptions::new();
//! options_l3.set_strength(Some(Strength::Tertiary));
//! let collator_l3: Collator =
//!     Collator::try_new_unstable(&data_provider, &Default::default(), options_l3).unwrap();
//!
//! assert_eq!(collator_l3.compare("a", "b"), Ordering::Less);  // primary
//! assert_eq!(collator_l3.compare("as", "às"), Ordering::Less);  // secondary
//! assert_eq!(collator_l3.compare("às", "at"), Ordering::Less);
//! assert_eq!(collator_l3.compare("ao", "Ao"), Ordering::Less);  // tertiary
//! assert_eq!(collator_l3.compare("Ao", "aò"), Ordering::Less);
//! assert_eq!(collator_l3.compare("A", "Ⓐ"), Ordering::Less);
//! ```
//!
//! ## Alternate Handling
//!
//! Allows alternate handling for certain customized collation orderings, including the option to
//! ignore the special handling for the strings of such customizations.
//!
//! ## Case Level
//!
//! Whether to distinguish case in sorting, even for sorting levels higher than tertiary,
//! without having to use tertiary level just to enable case level differences.
//!
//! ```
//! use core::cmp::Ordering;
//! use icu_collator::*;
//!
//! // Primary
//!
//! let data_provider = icu_testdata::get_provider();
//! let mut options = CollatorOptions::new();
//! options.set_strength(Some(Strength::Primary));
//! options.set_case_level(Some(false));
//! let primary =
//!   Collator::try_new_with_buffer_provider(&data_provider,
//!                     &Default::default(),
//!                     options).unwrap();
//!
//! assert_eq!(primary.compare("DEAL", "ⓓⓔⓐⓛ"), Ordering::Equal);
//! assert_eq!(primary.compare("ⓓⓔⓐⓛ", "déjavu"), Ordering::Less);
//! assert_eq!(primary.compare("déjavu", "dent"), Ordering::Less);
//! assert_eq!(primary.compare("dent", "develop"), Ordering::Less);
//!
//! // Primary with case level on
//!
//! options.set_strength(Some(Strength::Primary));
//! options.set_case_level(Some(true));
//! let primary_and_case =
//!   Collator::try_new_with_buffer_provider(&data_provider,
//!                     &Default::default(),
//!                     options).unwrap();
//!
//! assert_eq!(primary_and_case.compare("DEAL", "ⓓⓔⓐⓛ"), Ordering::Equal);
//! assert_eq!(primary_and_case.compare("ⓓⓔⓐⓛ", "déjavu"), Ordering::Less);
//! assert_eq!(primary_and_case.compare("déjavu", "dent"), Ordering::Less);
//! assert_eq!(primary_and_case.compare("dent", "develop"), Ordering::Less);
//!
//! // Tertiary
//!
//! options.set_strength(Some(Strength::Tertiary));
//! options.set_case_level(Some(false));
//! let tertiary =
//!   Collator::try_new_with_buffer_provider(&data_provider,
//!                     &Default::default(),
//!                     options).unwrap();
//!
//! assert_eq!(tertiary.compare("ⓓⓔⓐⓛ", "DEAL"), Ordering::Less);
//! assert_eq!(tertiary.compare("DEAL", "déjavu"), Ordering::Less);
//! assert_eq!(tertiary.compare("déjavu", "dent"), Ordering::Less);
//! assert_eq!(tertiary.compare("dent", "develop"), Ordering::Less);
//!
//!
//! ```
//!
//! ## Case First
//!
//! Whether to swap the ordering of uppercase and lowercase.
//!
//! ## Backward second level
//!
//! Compare the second level in backward order.
//!
//! ## Numeric
//!
//! When set to `true` (on), any sequence of decimal
//! digits is sorted at a primary level accoding to the
//! numeric value.
//!
//! ```
//! use core::cmp::Ordering;
//! use icu_collator::*;
//!
//! let data_provider = icu_testdata::get_provider();
//!
//! // Numerical sorting off
//!
//! let mut options_num_off = CollatorOptions::new();
//! options_num_off.set_numeric(Some(false));
//! let collator_num_off: Collator =
//!     Collator::try_new_unstable(&data_provider, &Default::default(), options_num_off).unwrap();
//! assert_eq!(collator_num_off.compare("a10b", "a2b"), Ordering::Less);
//!
//! // Numerical sorting on
//!
//! let mut options_num_on = CollatorOptions::new();
//! options_num_on.set_numeric(Some(true));
//! let collator_num_on: Collator =
//!     Collator::try_new_unstable(&data_provider, &Default::default(), options_num_on).unwrap();
//! assert_eq!(collator_num_on.compare("a10b", "a2b"), Ordering::Greater);
//! ```

mod comparison;
pub mod docs;
mod elements;
pub mod error;
mod options;
pub mod provider;

extern crate alloc;

pub use comparison::Collator;
pub use options::AlternateHandling;
pub use options::CaseFirst;
pub use options::CollatorOptions;
pub use options::MaxVariable;
pub use options::Strength;

#[cfg(all(test, feature = "serde"))]
mod tests;
