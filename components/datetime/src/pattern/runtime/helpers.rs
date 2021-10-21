// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{runtime, PatternItem};
use zerovec::{ule::AsULE, ZeroVec};

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
    match pattern.items {
        ZeroVec::Owned(ref mut vec) => {
            for item in vec {
                let aligned = PatternItem::from_unaligned(*item);
                if let Some(new_item) = f(&aligned) {
                    *item = new_item.as_unaligned();
                    break;
                }
            }
        }
        ZeroVec::Borrowed(_) => {
            let mut result = None;
            for (idx, item) in pattern.items.iter().enumerate() {
                if let Some(new_item) = f(&item) {
                    result = Some((idx, new_item));
                    break;
                }
            }
            if let Some((idx, new_item)) = result {
                let owned = pattern.items.to_mut();
                let item = owned.get_mut(idx).unwrap();
                *item = new_item.as_unaligned();
            }
        }
        _ => unimplemented!(),
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
    match pattern.items {
        ZeroVec::Owned(ref mut vec) => {
            for item in vec {
                let aligned = PatternItem::from_unaligned(*item);
                if let Some(new_item) = f(&aligned) {
                    *item = new_item.as_unaligned();
                }
            }
        }
        ZeroVec::Borrowed(_) => {
            let mut result = None;
            for (idx, item) in pattern.items.iter().enumerate() {
                if let Some(new_item) = f(&item) {
                    result = Some((idx, new_item));
                    break;
                }
            }
            if let Some((idx, new_item)) = result {
                let owned = pattern.items.to_mut();
                let item = owned.get_mut(idx).unwrap();
                *item = new_item.as_unaligned();
                for item in owned.iter_mut().skip(idx) {
                    let aligned = PatternItem::from_unaligned(*item);
                    if let Some(new_item) = f(&aligned) {
                        *item = new_item.as_unaligned();
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
}
