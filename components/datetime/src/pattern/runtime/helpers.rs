// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{runtime, PatternItem};
use zerovec::{ule::AsULE, ZeroVec};

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
