// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_properties` is a utility crate of the [`ICU4X`] project.
//!
//! This component provides definitions of [Unicode Properties].
//!
//! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html

mod enum_props;
mod ule;

pub use enum_props::*;
