// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as slices, looked up with binary search
//!
//! TODO(#6164): This code is stale; update it before use.

use icu_provider::prelude::*;

pub struct Data<K: BinarySearchKey, M: DataMarker>(
    pub &'static [(K::Type, &'static M::DataStruct)],
);

impl<K: BinarySearchKey, M: DataMarker> super::DataStore<M> for Data<K, M> {
    fn get(
        &self,
        id: DataIdentifierBorrowed,
        attributes_prefix_match: bool,
    ) -> Option<DataPayload<M>> {
        self.0
            .binary_search_by(|&(k, _)| K::cmp(k, id))
            .or_else(|e| {
                if attributes_prefix_match && e <= self.0.len() {
                    Ok(e)
                } else {
                    Err(e)
                }
            })
            // Safety: binary_search returns in-bounds indices when returning Ok.
            // The err case in `or_else` above only returns in-bounds Ok values
            .map(|i| unsafe { self.0.get_unchecked(i) }.1)
            .map(DataPayload::from_static_ref)
            .ok()
    }

    #[cfg(feature = "alloc")]
    type IterReturn = core::iter::Map<
        core::slice::Iter<'static, (K::Type, &'static M::DataStruct)>,
        fn(&'static (K::Type, &'static M::DataStruct)) -> DataIdentifierCow<'static>,
    >;
    #[cfg(feature = "alloc")]
    fn iter(&self) -> Self::IterReturn {
        self.0.iter().map(|&(k, _)| K::to_id(k))
    }
}

pub trait BinarySearchKey: 'static {
    type Type: Ord + Copy + 'static;

    fn cmp(k: Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering;
    #[cfg(feature = "alloc")]
    fn to_id(k: Self::Type) -> DataIdentifierCow<'static>;
}

pub struct Locale;

impl BinarySearchKey for Locale {
    type Type = &'static str;

    fn cmp(locale: Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering {
        id.locale.strict_cmp(locale.as_bytes()).reverse()
    }

    #[cfg(feature = "alloc")]
    fn to_id(locale: Self::Type) -> DataIdentifierCow<'static> {
        DataIdentifierCow::from_locale(locale.parse().unwrap())
    }
}

pub struct Attributes;

impl BinarySearchKey for Attributes {
    type Type = &'static str;

    fn cmp(attributes: Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering {
        attributes.cmp(id.marker_attributes)
    }

    #[cfg(feature = "alloc")]
    fn to_id(attributes: Self::Type) -> DataIdentifierCow<'static> {
        DataIdentifierCow::from_marker_attributes(DataMarkerAttributes::from_str_or_panic(
            attributes,
        ))
    }
}

pub struct AttributesAndLocale;

impl BinarySearchKey for AttributesAndLocale {
    type Type = (&'static str, &'static str);

    fn cmp((attributes, locale): Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering {
        attributes
            .cmp(id.marker_attributes)
            .then_with(|| id.locale.strict_cmp(locale.as_bytes()).reverse())
    }

    #[cfg(feature = "alloc")]
    fn to_id((attributes, locale): Self::Type) -> DataIdentifierCow<'static> {
        DataIdentifierCow::from_borrowed_and_owned(
            DataMarkerAttributes::from_str_or_panic(attributes),
            locale.parse().unwrap(),
        )
    }
}
