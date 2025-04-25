// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as as [`ZeroTrieSimpleAscii`]

use icu_provider::baked::zerotrie::*;
use icu_provider::prelude::*;
pub use zerotrie::ZeroTrieSimpleAscii;

use icu_provider::export::ExportMarker;

pub(crate) fn bake(
    marker_bake: &databake::TokenStream,
    bakes_to_ids: &[(
        &DataPayload<ExportMarker>,
        &std::collections::BTreeSet<DataIdentifierCow>,
    )],
    ctx: &databake::CrateEnv,
) -> (databake::TokenStream, usize) {
    use databake::*;

    // Safety invariant upheld: the only values being added to the trie are `baked_index`
    // values, which come from `bakes`
    let trie = ZeroTrieSimpleAscii::from_iter(bakes_to_ids.iter().enumerate().flat_map(
        |(bake_index, (_, ids))| {
            ids.iter().map(move |id| {
                let mut encoded = id.locale.to_string().into_bytes();
                if !id.marker_attributes.is_empty() {
                    encoded.push(ID_SEPARATOR);
                    encoded.extend_from_slice(id.marker_attributes.as_bytes());
                }
                (encoded, bake_index)
            })
        },
    ));

    let baked_trie = trie.as_borrowed_slice().bake(&Default::default());
    let baked_trie = quote! {
        const TRIE: icu_provider::baked::zerotrie::ZeroTrieSimpleAscii<&'static [u8]> = icu_provider::baked:: #baked_trie;
    };

    let payloads = bakes_to_ids
        .iter()
        .map(|(payload, _)| *payload)
        .collect::<Vec<_>>();

    let maybe_vzv_tokens = DataPayload::tokenize_encoded_seq(&payloads, ctx);

    let (baked_values, value_store_ty) = if let Some(vzv_tokens) = maybe_vzv_tokens {
        (
            quote! {
                const VALUES: &'static zerovec::VarZeroSlice<<<#marker_bake as icu_provider::baked::zerotrie::DynamicDataMarker>::DataStruct as icu_provider::ule::MaybeAsVarULE>::EncodedStruct> = #vzv_tokens;
            },
            quote! {
                icu_provider::baked::zerotrie::DataForVarULEs
            },
        )
    } else {
        let bakes = payloads.iter().map(|payload| payload.tokenize(ctx));
        (
            quote! {
                const VALUES: &'static [<#marker_bake as icu_provider::baked::zerotrie::DynamicDataMarker>::DataStruct] = &[#(#bakes,)*];
            },
            quote! {
                icu_provider::baked::zerotrie::Data
            },
        )
    };

    (
        quote! {
            // Safety invariant upheld: see above
            #value_store_ty<#marker_bake> = {
                #baked_trie
                #baked_values
                unsafe {
                    #value_store_ty::from_trie_and_values_unchecked(TRIE, VALUES)
                }
            }

        },
        core::mem::size_of::<Data<icu_provider::hello_world::HelloWorldV1>>()
            + trie.as_borrowed_slice().borrows_size(),
    )
}
