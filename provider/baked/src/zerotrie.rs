// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as as [`ZeroTrieSimpleAscii`]

// This is a valid separator as `DataLocale` will never produce it.
const ID_SEPARATOR: u8 = 0x1E;

#[cfg(feature = "export")]
use databake::*;
use icu_provider::prelude::*;
pub use zerotrie::ZeroTrieSimpleAscii;

#[cfg(feature = "export")]
pub(crate) fn bake(
    marker_bake: &TokenStream,
    ids_to_idents: Vec<(DataIdentifierCow, proc_macro2::Ident)>,
    idents_to_bakes: Vec<(proc_macro2::Ident, TokenStream)>,
) -> (TokenStream, usize) {
    let bakes = idents_to_bakes.iter().map(|(_, bake)| bake);

    let bake_indices = idents_to_bakes
        .iter()
        .enumerate()
        .map(|(i, (ident, _))| (ident, i))
        .collect::<std::collections::HashMap<&proc_macro2::Ident, usize>>();

    let trie = ZeroTrieSimpleAscii::from_iter(ids_to_idents.iter().map(|(id, ident)| {
        let mut encoded = id.locale.to_string().into_bytes();
        if !id.marker_attributes.is_empty() {
            encoded.push(ID_SEPARATOR);
            encoded.extend_from_slice(id.marker_attributes.as_bytes());
        }
        (encoded, bake_indices[ident])
    }));

    let baked_trie = trie.as_borrowed_slice().bake(&Default::default());

    (
        quote! {
            icu_provider_baked::zerotrie::Data<#marker_bake> = icu_provider_baked::zerotrie::Data {
                trie: icu_provider_baked:: #baked_trie,
                values: &[#(#bakes,)*],
            }
        },
        core::mem::size_of::<Data<icu_provider::hello_world::HelloWorldV1Marker>>()
            + trie.as_borrowed_slice().borrows_size(),
    )
}

pub struct Data<M: DataMarker> {
    pub trie: ZeroTrieSimpleAscii<&'static [u8]>,
    pub values: &'static [M::Yokeable],
}

impl<M: DataMarker> super::DataStore<M> for Data<M> {
    fn get(&self, id: DataIdentifierBorrowed) -> Option<&'static <M>::Yokeable> {
        use writeable::Writeable;
        let mut cursor = self.trie.cursor();
        let _is_ascii = id.locale.write_to(&mut cursor);
        if !id.marker_attributes.is_empty() {
            cursor.step(ID_SEPARATOR);
            id.marker_attributes.write_to(&mut cursor).ok()?;
        }
        cursor
            .take_value()
            .map(|i| unsafe { self.values.get_unchecked(i) })
    }

    type IterReturn = core::iter::FilterMap<
        core::iter::Map<
            zerotrie::ZeroTrieIterator<'static>,
            fn((alloc::vec::Vec<u8>, usize)) -> (alloc::string::String, usize),
        >,
        fn((alloc::string::String, usize)) -> Option<DataIdentifierCow<'static>>,
    >;
    fn iter(&'static self) -> Self::IterReturn {
        #![allow(unused_imports)]
        use alloc::borrow::ToOwned;
        self.trie.iter().filter_map(move |(s, _)| {
            if let Some((locale, attrs)) = s.split_once(ID_SEPARATOR as char) {
                Some(DataIdentifierCow::from_owned(
                    DataMarkerAttributes::try_from_str(attrs).ok()?.to_owned(),
                    locale.parse().ok()?,
                ))
            } else {
                s.parse().ok().map(DataIdentifierCow::from_locale)
            }
        })
    }
}
