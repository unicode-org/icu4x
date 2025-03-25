// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as slices, looked up with binary search
//!
//! TODO(#6164): This code is stale; update it before use.

use icu_provider::prelude::*;

#[allow(dead_code)]
pub(crate) fn bake(
    marker_bake: &databake::TokenStream,
    bakes_to_ids: Vec<(
        databake::TokenStream,
        std::collections::BTreeSet<DataIdentifierCow>,
    )>,
) -> (databake::TokenStream, usize) {
    use databake::*;
    use proc_macro2::{Ident, Span};

    let mut idents_to_bakes = Vec::new();

    let ids_to_idents = bakes_to_ids
        .iter()
        .flat_map(|(bake, ids)| {
            let min_id = ids.first().unwrap();

            let ident = Ident::new(
                &format!("_{}_{}", min_id.marker_attributes.as_str(), min_id.locale)
                    .chars()
                    .map(|ch| {
                        if ch == '-' {
                            '_'
                        } else {
                            ch.to_ascii_uppercase()
                        }
                    })
                    .collect::<String>(),
                Span::call_site(),
            );

            idents_to_bakes.push((ident.clone(), bake));
            ids.iter().map(move |id| (id.clone(), ident.clone()))
        })
        .collect::<Vec<_>>();

    let mut size = 0;

    // Data.0 is a fat pointer
    size += core::mem::size_of::<&[()]>();

    // The idents are references
    size += ids_to_idents.len() * core::mem::size_of::<&()>();

    let (ty, id_bakes_to_idents) = if ids_to_idents
        .iter()
        .all(|(id, _)| id.marker_attributes.is_empty())
    {
        // Only DataLocales
        size += ids_to_idents.len() * core::mem::size_of::<&str>();
        (
            quote! { icu_provider_baked::binary_search::Locale },
            ids_to_idents
                .iter()
                .map(|(id, ident)| {
                    let k = id.locale.to_string();
                    quote!((#k, #ident))
                })
                .collect::<Vec<_>>(),
        )
    } else if ids_to_idents.iter().all(|(id, _)| id.locale.is_default()) {
        // Only marker attributes
        size += ids_to_idents.len() * core::mem::size_of::<&str>();
        (
            quote! { icu_provider_baked::binary_search::Attributes },
            ids_to_idents
                .iter()
                .map(|(id, ident)| {
                    let k = id.marker_attributes.as_str();
                    quote!((#k, #ident))
                })
                .collect(),
        )
    } else {
        size += ids_to_idents.len() * 2 * core::mem::size_of::<&str>();
        (
            quote! { icu_provider_baked::binary_search::AttributesAndLocale },
            ids_to_idents
                .iter()
                .map(|(id, ident)| {
                    let k0 = id.marker_attributes.as_str();
                    let k1 = id.locale.to_string();
                    quote!(((#k0, #k1), #ident))
                })
                .collect(),
        )
    };

    let idents_to_bakes = idents_to_bakes.into_iter().map(|(ident, bake)| {
        quote! {
            const #ident: &S = &#bake;
        }
    });

    (
        quote! {
            icu_provider_baked::binary_search::Data<#ty, #marker_bake> = {
                type S = <#marker_bake as icu_provider::DynamicDataMarker>::DataStruct;
                #(#idents_to_bakes)*
                icu_provider_baked::binary_search::Data(&[#(#id_bakes_to_idents,)*])
            }
        },
        size,
    )
}
