// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_properties` is a utility crate of the [`ICU4X`] project.
//!
//! This component provides definitions of [Unicode Properties] and APIs for
//! retrieving property data in an appropriate data structure.
//!
//! Currently, only binary property APIs are supported, with APIs that return
//! a [`UnicodeSet`]. See the [`props`] module for more details.
//!
//! [`ICU4X`]: ../icu/index.html
//! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
//! [`UnicodeSet`]: icu_uniset::UnicodeSet
//! [`props`]: crate::props

pub mod enum_props;
#[allow(unused)]
pub mod props;
pub mod provider;
mod ule;

pub use enum_props::*;
