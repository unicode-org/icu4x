// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as slices, looked up with binary search

#[cfg(feature = "export")]
use databake::*;
use icu_provider::prelude::*;

#[cfg(feature = "export")]
pub fn bake(
    marker_bake: &TokenStream,
    reqs_to_idents: Vec<((DataLocale, DataMarkerAttributes), proc_macro2::Ident)>,
    idents_to_bakes: Vec<(proc_macro2::Ident, TokenStream)>,
) -> TokenStream {
    let mut reqs_to_idents = reqs_to_idents
        .into_iter()
        .map(|((l, a), i)| ((a.to_string(), l.to_string()), quote!(#i)))
        .collect::<Vec<_>>();

    reqs_to_idents.sort_by(|(a, _), (b, _)| a.cmp(b));

    let idents_to_bakes = idents_to_bakes.into_iter().map(|(ident, bake)| {
        quote! {
            const #ident: &S = &#bake;
        }
    });

    let (ty, reqs_to_idents) = if reqs_to_idents.iter().all(|((a, _), _)| a.is_empty()) {
        // Only DataLocales
        (
            quote! { icu_provider_baked::binary_search::Locale },
            reqs_to_idents
                .iter()
                .map(|((_, l), i)| quote!((#l, #i)))
                .collect::<Vec<_>>(),
        )
    } else if reqs_to_idents.iter().all(|((_, l), _)| *l == "und") {
        // Only marker attributes
        (
            quote! { icu_provider_baked::binary_search::Attributes },
            reqs_to_idents
                .iter()
                .map(|((a, _), i)| quote!((#a, #i)))
                .collect(),
        )
    } else {
        (
            quote! { icu_provider_baked::binary_search::AttributesAndLocale },
            reqs_to_idents
                .iter()
                .map(|((a, l), i)| quote!(((#a, #l), #i)))
                .collect(),
        )
    };

    quote! {
        icu_provider_baked::binary_search::Data<#ty, #marker_bake> = {
            type S = <#marker_bake as icu_provider::DynamicDataMarker>::Yokeable;
            #(#idents_to_bakes)*
            icu_provider_baked::binary_search::Data(&[#(#reqs_to_idents,)*])
        }
    }
}

pub struct Data<K: BinarySearchKey, M: DataMarker>(pub &'static [(K::Type, &'static M::Yokeable)]);

impl<K: BinarySearchKey, M: DataMarker> super::DataStore<M> for Data<K, M> {
    fn get(&self, req: DataRequest) -> Option<&'static M::Yokeable> {
        self.0
            .binary_search_by(|&(k, _)| K::cmp(k, req))
            .map(|i| unsafe { self.0.get_unchecked(i) }.1)
            .ok()
    }

    type IterReturn = core::iter::Map<
        core::slice::Iter<'static, (K::Type, &'static M::Yokeable)>,
        fn(&'static (K::Type, &'static M::Yokeable)) -> (DataLocale, DataMarkerAttributes),
    >;
    fn iter(&self) -> Self::IterReturn {
        self.0.iter().map(|&(k, _)| K::to_req(k))
    }
}

pub trait BinarySearchKey: 'static {
    type Type: Ord + Copy + 'static;

    fn cmp(k: Self::Type, req: DataRequest) -> core::cmp::Ordering;
    fn to_req(k: Self::Type) -> (DataLocale, DataMarkerAttributes);
}

pub struct Locale;

impl BinarySearchKey for Locale {
    type Type = &'static str;

    fn cmp(locale: Self::Type, req: DataRequest) -> core::cmp::Ordering {
        req.locale.strict_cmp(locale.as_bytes()).reverse()
    }

    fn to_req(locale: Self::Type) -> (DataLocale, DataMarkerAttributes) {
        (locale.parse().unwrap(), Default::default())
    }
}

pub struct Attributes;

impl BinarySearchKey for Attributes {
    type Type = &'static str;

    fn cmp(attributes: Self::Type, req: DataRequest) -> core::cmp::Ordering {
        attributes.cmp(&**req.marker_attributes)
    }

    fn to_req(attributes: Self::Type) -> (DataLocale, DataMarkerAttributes) {
        (Default::default(), attributes.parse().unwrap())
    }
}

pub struct AttributesAndLocale;

impl BinarySearchKey for AttributesAndLocale {
    type Type = (&'static str, &'static str);

    fn cmp((attributes, locale): Self::Type, req: DataRequest) -> core::cmp::Ordering {
        attributes
            .cmp(&**req.marker_attributes)
            .then_with(|| req.locale.strict_cmp(locale.as_bytes()).reverse())
    }

    fn to_req((attributes, locale): Self::Type) -> (DataLocale, DataMarkerAttributes) {
        (locale.parse().unwrap(), attributes.parse().unwrap())
    }
}
