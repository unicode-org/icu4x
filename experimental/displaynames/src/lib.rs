// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Display names for regions
//!
//! This module is published as its own crate ([`icu_displaynames`](https://docs.rs/icu_displaynames/latest/icu_displaynames/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! </div>
//!
//! [`ICU4X`]: ../icu/index.html

// TODO: expand documentation

#![warn(missing_docs)]

pub mod displaynames;
pub mod options;
pub mod provider;

pub use displaynames::LanguageDisplayNames;
pub use displaynames::RegionDisplayNames;
pub use options::DisplayNamesOptions;
pub use options::Fallback;
pub use options::LanguageDisplay;
pub use options::Style;
