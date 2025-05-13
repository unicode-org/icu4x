// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module is for reserved units with their fixed IDs.
//!
//! The purpose of this module is to ensure that certain units have fixed IDs that do not change.
//! For example, the ID for "meter" is 0, and this ID should remain constant and unchangeable.
//! This stability is crucial for consistent unit creation and parsing across different
//! parts of the ICU4X library and its consumers.

/// A constant array of reserved unit identifiers with fixed IDs.
/// Each unit's position in the array represents its unique ID.
/// For instance, "meter" is assigned ID 0, and "gram" is assigned ID 1.
pub const RESERVED_UNIT_IDS: &[&str] = &[
    "meter", // ID 0
    "gram",  // ID 1
];
