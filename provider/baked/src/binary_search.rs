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
pub fn bake(
    marker_bake: &TokenStream,
    mut ids_to_idents: Vec<(DataIdentifierCow, proc_macro2::Ident)>,
    idents_to_bakes: Vec<(proc_macro2::Ident, TokenStream)>,
) -> TokenStream {
    let max_attributes_len = ids_to_idents
        .iter()
        .map(|(id, _)| id.marker_attributes.len())
        .max();
    let max_locale_len = ids_to_idents
        .iter()
        .map(|(id, _)| id.locale.to_string().len())
        .max();

    ids_to_idents.sort_by_cached_key(|(id, _)| {
        (
            id.marker_attributes.as_str().to_owned(),
            id.locale.to_string(),
        )
    });

    let idents_to_bakes = idents_to_bakes.into_iter().map(|(ident, bake)| {
        quote! {
            const #ident: &S = &#bake;
        }
    });

    let (ty, reqs_to_idents) = if ids_to_idents
        .iter()
        .all(|(id, _)| id.marker_attributes.is_empty())
    {
        // Only DataLocales
        (
            quote! { icu_provider_baked::binary_search::Locale<#max_locale_len> },
            ids_to_idents
                .iter()
                .map(|(id, i)| {
                    let locale_str = id.locale.to_string();
                    quote!((tinystr!(#max_locale_len, #locale_str), #i))
                })
                .collect::<Vec<_>>(),
        )
    } else if ids_to_idents.iter().all(|(id, _)| id.locale.is_empty()) {
        // Only marker attributes
        (
            quote! { icu_provider_baked::binary_search::Attributes<#max_attributes_len> },
            ids_to_idents
                .iter()
                .map(|(id, i)| {
                    let attribute_str = id.marker_attributes.as_str();
                    quote!((tinystr!(#max_attributes_len, #attribute_str), #i))
                })
                .collect(),
        )
    } else {
        (
            quote! { icu_provider_baked::binary_search::AttributesAndLocale<#max_attributes_len, #max_locale_len> },
            ids_to_idents
                .iter()
                .map(|(id, i)| {
                    let attribute_str = id.marker_attributes.to_string();
                    let locale_str = id.locale.to_string();
                    quote!(((tinystr!(#max_attributes_len, #attribute_str), tinystr!(#max_locale_len, #locale_str)), #i))
                })
                .collect(),
        )
    };

    quote! {
        icu_provider_baked::binary_search::Data<#ty, #marker_bake> = {
            type S = <#marker_bake as icu_provider::DynamicDataMarker>::Yokeable;
            #(#idents_to_bakes)*
            use icu_provider_baked::binary_search::tinystr::tinystr;
            icu_provider_baked::binary_search::Data(&[#(#reqs_to_idents,)*])
        }
    }
}

pub struct Data<K: BinarySearchKey, M: DataMarker>(pub &'static [(K::Type, &'static M::Yokeable)]);

impl<K: BinarySearchKey, M: DataMarker> super::DataStore<M> for Data<K, M> {
    fn get(&self, id: DataIdentifierBorrowed) -> Option<&'static M::Yokeable> {
        self.0
            .binary_search_by(|&(k, _)| K::cmp(k, id))
            .map(|i| unsafe { self.0.get_unchecked(i) }.1)
            .ok()
    }

    type IterReturn = core::iter::Map<
        core::slice::Iter<'static, (K::Type, &'static M::Yokeable)>,
        fn(&'static (K::Type, &'static M::Yokeable)) -> DataIdentifierCow<'static>,
    >;
    fn iter(&self) -> Self::IterReturn {
        self.0.iter().map(|&(k, _)| K::to_id(k))
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
