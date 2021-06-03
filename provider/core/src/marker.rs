// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Marker types and traits for DataProvider.

use yoke::Yokeable;

/// Trait marker for data structs. All types delivered by the data provider must be associated with
/// something implementing this trait.
pub trait DataMarker<'s> {
    type Yokeable: for<'a> Yokeable<'a>;
    type Cart: 's + ?Sized;
}
