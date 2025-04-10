// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Parsing functionality for various popular operating systems

// Re-export all alias functions
mod aliases;
pub use aliases::*;

pub mod apple;
pub mod posix;
pub mod windows;
