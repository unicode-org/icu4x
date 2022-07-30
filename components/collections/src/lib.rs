// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Collections of API for use in ICU.
//!
//! This module is published as its own crate ([`icu_collections`](https://docs.rs/icu_collections/latest/icu_collections/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! ICU4X [`CodePointTrie`](crate::codepointtrie::CodePointTrie) is designed to provide a read-only view of CodePointTrie data that is exported
//! from ICU4C. Detailed information about the design of the data structure can be found in the documentation
//! for the [`CodePointTrie`](crate::codepointtrie::CodePointTrie) struct.
//!

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]
#![warn(missing_docs)]

#[doc(inline)]
pub use icu_codepointtrie as codepointtrie;
