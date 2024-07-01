// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as slices, looked up with binary search

pub extern crate tinystr;

use alloc::borrow::ToOwned;
#[cfg(feature = "export")]
use databake::*;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;

#[cfg(feature = "export")]
pub(crate) fn bake(
    marker_bake: &TokenStream,
    mut ids_to_idents: Vec<(DataIdentifierCow, proc_macro2::Ident)>,
    idents_to_bakes: Vec<(proc_macro2::Ident, TokenStream)>,
) -> (TokenStream, usize) {
    let mut size = 0;

    // Data.0 is a fat pointer
    size += core::mem::size_of::<Data<Locale<1>, icu_provider::hello_world::HelloWorldV1Marker>>();

    let max_attributes_len = ids_to_idents
        .iter()
        .map(|(id, _)| id.marker_attributes.len())
        .max()
        .unwrap();
    let max_locale_len = ids_to_idents
        .iter()
        .map(|(id, _)| id.locale.to_string().len())
        .max()
        .unwrap();

    // The idents are references
    size += ids_to_idents.len() * core::mem::size_of::<&()>();

    ids_to_idents.sort_by_cached_key(|(id, _)| {
        (
            id.marker_attributes.as_str().to_string(),
            id.locale.to_string(),
        )
    });

    let (ty, keys) = if ids_to_idents
        .iter()
        .all(|(id, _)| id.marker_attributes.is_empty())
    {
        // Only DataLocales
        size += ids_to_idents.len() * max_locale_len;
        (
            quote! { icu_provider_baked::binary_search::Locale<#max_locale_len> },
            ids_to_idents
                .iter()
                .map(|(id, _)| {
                    let k = id.locale.to_string();
                    quote!(tinystr!(#max_locale_len, #k))
                })
                .collect::<Vec<_>>(),
        )
    } else if ids_to_idents.iter().all(|(id, _)| id.locale.is_empty()) {
        // Only marker attributes
        size += ids_to_idents.len() * max_attributes_len;
        (
            quote! { icu_provider_baked::binary_search::Attributes<#max_attributes_len> },
            ids_to_idents
                .iter()
                .map(|(id, _)| {
                    let k = id.marker_attributes.as_str();
                    quote!(tinystr!(#max_attributes_len, #k))
                })
                .collect(),
        )
    } else {
        size += ids_to_idents.len() * (max_attributes_len + max_locale_len);
        (
            quote! { icu_provider_baked::binary_search::AttributesAndLocale<#max_attributes_len, #max_locale_len> },
            ids_to_idents
                .iter()
                .map(|(id, _)| {
                    let k0 = id.marker_attributes.as_str();
                    let k1 = id.locale.to_string();
                    quote!((
                        tinystr!(#max_attributes_len, #k0),
                        tinystr!(#max_locale_len, #k1)
                    ))
                })
                .collect(),
        )
    };

    let values = ids_to_idents.iter().map(|(_, ident)| ident);

    let idents_to_bakes = idents_to_bakes.into_iter().map(|(ident, bake)| {
        quote! {
            const #ident: &S = &#bake;
        }
    });

    (
        quote! {
            icu_provider_baked::binary_search::Data<#ty, #marker_bake> = icu_provider_baked::binary_search::Data(
                {
                    use icu_provider_baked::binary_search::tinystr::tinystr;
                    &[#(#keys,)*]
                },
                {
                    type S = <#marker_bake as icu_provider::DynamicDataMarker>::Yokeable;
                    #(#idents_to_bakes)*
                    &[#(#values,)*]
                },
            )
        },
        size,
    )
}

pub struct Data<K: BinarySearchKey, M: DataMarker>(
    pub &'static [K::Type],
    pub &'static [&'static M::Yokeable],
);

impl<K: BinarySearchKey, M: DataMarker> super::DataStore<M> for Data<K, M> {
    fn get(&self, id: DataIdentifierBorrowed) -> Option<&'static M::Yokeable> {
        self.0
            .binary_search_by(|&k| K::cmp(k, id))
            .map(|i| *unsafe { self.1.get_unchecked(i) })
            .ok()
    }

    type IterReturn = core::iter::Map<
        core::slice::Iter<'static, K::Type>,
        fn(&'static K::Type) -> DataIdentifierCow<'static>,
    >;
    fn iter(&self) -> Self::IterReturn {
        self.0.iter().map(|&k| K::to_id(k))
    }
}

pub trait BinarySearchKey: 'static {
    type Type: Ord + Copy + 'static;

    fn cmp(k: Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering;
    fn to_id(k: Self::Type) -> DataIdentifierCow<'static>;
}

pub struct Locale<const N: usize>;

impl<const N: usize> BinarySearchKey for Locale<N> {
    type Type = TinyAsciiStr<N>;

    fn cmp(locale: Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering {
        id.locale.strict_cmp(locale.as_bytes()).reverse()
    }

    fn to_id(locale: Self::Type) -> DataIdentifierCow<'static> {
        DataIdentifierCow::from_locale(locale.parse().unwrap())
    }
}

pub struct Attributes<const N: usize>;

impl<const N: usize> BinarySearchKey for Attributes<N> {
    type Type = TinyAsciiStr<N>;

    fn cmp(attributes: Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering {
        attributes.as_str().cmp(id.marker_attributes)
    }

    fn to_id(attributes: Self::Type) -> DataIdentifierCow<'static> {
        DataIdentifierCow::from_marker_attributes_owned(
            DataMarkerAttributes::from_str_or_panic(attributes.as_str()).to_owned(),
        )
    }
}

pub struct AttributesAndLocale<const N: usize, const M: usize>;

impl<const N: usize, const M: usize> BinarySearchKey for AttributesAndLocale<N, M> {
    type Type = (TinyAsciiStr<N>, TinyAsciiStr<M>);

    fn cmp((attributes, locale): Self::Type, id: DataIdentifierBorrowed) -> core::cmp::Ordering {
        attributes
            .as_str()
            .cmp(id.marker_attributes)
            .then_with(|| id.locale.strict_cmp(locale.as_bytes()).reverse())
    }

    fn to_id((attributes, locale): Self::Type) -> DataIdentifierCow<'static> {
        DataIdentifierCow::from_owned(
            DataMarkerAttributes::from_str_or_panic(attributes.as_str()).to_owned(),
            locale.parse().unwrap(),
        )
    }
}
