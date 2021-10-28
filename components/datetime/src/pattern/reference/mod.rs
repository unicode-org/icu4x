// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Reference `Pattern` implementation intended to follow Unicode
//! specification, and become publicly available for tooling to use
//! for parsing/inspecting/modifying and serialization.
//!
//! The runtime `Pattern` uses parsing/serialization from this module.
mod display;
mod generic;
mod parser;
pub(crate) mod pattern;

pub use generic::GenericPattern;
pub use parser::Parser;
pub use pattern::Pattern;
