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
//! ## Sort levels / strengths
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
//! ## Numeric sorting on/off
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
//! 
//! # Contributor Notes
//!
//! ## Development environment (on Linux) for fuzzing and generating data
//!
//! These notes assume that ICU4X itself has been cloned to `$PROJECTS/icu4x`.
//!
//! Clone ICU4C from <https://github.com/hsivonen/icu> to `$PROJECTS/icu` and switch
//! to the branch `icu4x-collator`.
//!
//! Create a directory `$PROJECTS/localicu`
//!
//! Create a directory `$PROJECTS/icu-build` and `cd` into it.
//!
//! Run `../icu/icu4c/source/runConfigureICU --enable-debug Linux --prefix $PROJECTS/localicu --enable-static`
//!
//! Run `make`
//!
//! ### Generating data
//!
//!
//!
//! ### Testing
//!
//! `cargo test --features serde`
//!
//! ### Fuzzing
//!
//! `cargo install cargo-fuzz`
//!
//! Clone `rust_icu` from <https://github.com/google/rust_icu> to `$PROJECTS/rust_icu`.
//!
//! In `$PROJECTS/icu-build` run `make install`.
//!
//! `cd $PROJECTS/icu4x/components/collator`
//!
//! Run the fuzzer until a panic:
//!
//! `PKG_CONFIG_PATH="$PROJECTS/localicu/lib/pkgconfig" PATH="$PROJECTS/localicu/bin:$PATH" LD_LIBRARY_PATH="/$PROJECTS/localicu/lib" RUSTC_BOOTSTRAP=1 cargo +stable fuzz run compare_utf16`
//!
//! Once there is a panic, recompile with debug symbols by adding `--dev`:
//!
//! `PKG_CONFIG_PATH="$PROJECTS/localicu/lib/pkgconfig" PATH="$PROJECTS/localicu/bin:$PATH" LD_LIBRARY_PATH="$PROJECTS/localicu/lib" RUSTC_BOOTSTRAP=1 cargo +stable fuzz run --dev compare_utf16 fuzz/artifacts/compare_utf16/crash-$ARTIFACTHASH`
//!
//! Record with
//!
//! `LD_LIBRARY_PATH="$PROJECTS/localicu/lib" rr fuzz/target/x86_64-unknown-linux-gnu/debug/compare_utf16 -artifact_prefix=$PROJECTS/icu4x/components/collator/fuzz/artifacts/compare_utf16/ fuzz/artifacts/compare_utf16/crash-$ARTIFACTHASH`
//!
//! # Design notes
//!
//! * The collation element design comes from ICU4C. Some parts of the ICU4C design, notably,
//!   `Tag::BuilderDataTag`, `Tag::LeadSurrogateTag`, `Tag::LatinExpansionTag`, `Tag::U0000Tag`,
//!   and `Tag::HangulTag` are unused.
//!   - `Tag::LatinExpansionTag` might be reallocated to search expansions for archaic jamo
//!     in the future.
//!   - `Tag::HangulTag` might be reallocated to compressed hanja expansions in the future.
//!     See [issue 1315](https://github.com/unicode-org/icu4x/issues/1315).
//! * The key design difference between ICU4C and ICU4X is that ICU4C puts the canonical
//!   closure in the data (larger data) to enable lookup directly by precomposed characters
//!   while ICU4X always omits the canonical closure and always normalizes to NFD on the fly.
//! * Compared to ICU4C, normalization cannot be turned off. There also isn't a separate
//!   "Fast Latin" mode.
//! * The normalization is fused into the collation element lookup algorithm to optimize the
//!   case where an input character decomposes into two BMP characters: a base letter and a
//!   diacritic.
//!   - To optimize away a trie lookup when the combining diacritic doesn't contract,
//!     there is a linear lookup table for the combining diacritics block. Three languages
//!     tailor diacritics: Ewe, Lithuanian, and Vietnamese. Vietnamese and Ewe load an
//!     alternative table. The Lithuanian special cases are hard-coded and activatable by
//!     a metadata bit.
//! * Unfortunately, contractions that contract starters don't fit this model nicely. Therefore,
//!   there's duplicated normalization code for normalizing the lookahead for contractions.
//!   This code can, in principle, do duplicative work, but it shouldn't be excessive with
//!   real-world inputs.
//! * As a result, in terms of code provenance, the algorithms come from ICU4C, except the
//!   normalization part of the code is novel to ICU4X, and the contraction code is custom
//!   to ICU4X despite being informed by ICU4C.
//! * The way input characters are iterated over and resulting collation elements are
//!   buffered is novel to ICU4X.
//! * ICU4C can iterate backwards but ICU4X cannot. ICU4X keeps a buffer of the two most
//!   recent characters for handling prefixes. As of CLDR 40, there were only two kinds
//!   of prefixes: a single starter and a starter followed by a kana voicing mark.
//! * ICU4C sorts unpaired surrogates in their lexical order. ICU4X operates on Unicode
//!   [scalar values](https://unicode.org/glossary/#unicode_scalar_value) (any Unicode
//!   code point except high-surrogate and low-surrogate code points), so unpaired
//!   surrogates sort as REPLACEMENT CHARACTERs. Therefore, all unpaired
//!   surrogates are equal with each other.
//! * Skipping over a bit-identical prefix and then going back over "backward-unsafe"
//!   characters is currently unimplemented but isn't architecturally precluded.
//! * Hangul is handled specially:
//!   - Precomposed syllables are checked for as the first step of processing an
//!     incoming character.
//!   - Individual jamo are lookup up from a linear table instead of a trie. Unlike
//!     in ICU4C, this table covers the whole Unicode block whereas in ICU4C it covers
//!     only modern jamo for use in decomposing the precomposed syllables. The point
//!     is that search collations have a lot of duplicative (across multiple search)
//!     collations data for making archaic jamo searchable by modern jamo.
//!     Unfortunately, the shareable part isn't currently actually shareable, because
//!     the tailored CE32s refer to the expansions table in each collation. To make
//!     them truly shareable, the archaic jamo expansions need to become self-contained
//!     the way Latin mini expansions in ICU4C are self-contained.
//!
//!     One possible alternative to loading a different table for "search" would be
//!     performing the mapping of archaic jamo to the modern approximations as a
//!     special preprocessing step for the incoming characters, which would allow
//!     the lookup of the resulting modern jamo from the normal root jamo table.
//!
//!     "searchjl" is even more problematic than "search", since "searchjl" uses
//!     prefixes matches with jamo, and currently Hangul is assumed not to participate
//!     in prefix or contraction matching.

mod comparison;
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
