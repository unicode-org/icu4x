// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{runtime, PatternItem};
use zerovec::ule::AsULE;

/// Helper function which takes a runtime `Pattern` and calls
/// the callback function passing each item until the callback
/// returns a new value.
///
/// When that happens, the `Pattern` is turned into an owned one,
/// and the old item gets replaced with the new one.
///
/// The utility of this function is to allow for a single
/// item to be replaced allocating the `Pattern` only if needed.
///
/// For a variant that replaces all matching instances, see `maybe_replace`.
pub fn maybe_replace_first(
    pattern: &mut runtime::Pattern,
    f: impl Fn(&PatternItem) -> Option<PatternItem>,
) {
    let result = pattern
        .items
        .iter()
        .enumerate()
        .find_map(|(i, item)| f(&item).map(|result| (i, result)));
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    if let Some((i, result)) = result {
        pattern.items.to_mut()[i] = result.to_unaligned();
    }
}

/// Helper function which takes a runtime `Pattern` and calls
/// the callback function passing each item.
/// If the callback returns a new value, the old one gets
/// replaced with it.
///
/// The utility of this function is to allow for a pattern
/// to be altered, allocating the `Pattern` only if needed.
///
/// For a variant that replaces just the first matching instance,
/// see `maybe_replace_first`.
pub fn maybe_replace(
    pattern: &mut runtime::Pattern,
    f: impl Fn(&PatternItem) -> Option<PatternItem>,
) {
    let result = pattern
        .items
        .iter()
        .enumerate()
        .find_map(|(i, item)| f(&item).map(|result| (i, result)));
    #[allow(clippy::indexing_slicing)] // TODO(#1668) Clippy exceptions need docs or fixing.
    if let Some((i, result)) = result {
        let owned = pattern.items.to_mut();
        owned[i] = result.to_unaligned();
        owned.iter_mut().skip(i).for_each(|item| {
            if let Some(new_item) = f(&PatternItem::from_unaligned(*item)) {
                *item = new_item.to_unaligned();
            }
        });
    }
}
