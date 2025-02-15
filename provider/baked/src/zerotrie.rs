// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as as [`ZeroTrieSimpleAscii`]

// This is a valid separator as `DataLocale` will never produce it.
const ID_SEPARATOR: u8 = 0x1E;

use icu_provider::prelude::*;
pub use icu_provider::DynamicDataMarker;
pub use zerotrie::ZeroTrieSimpleAscii;
use zerovec::VarZeroVec;

#[cfg(feature = "export")]
pub(crate) enum BakedValue<'a> {
    VarULE(&'a [u8]),
    Struct(databake::TokenStream),
}

#[cfg(feature = "export")]
pub(crate) fn bake(
    marker_bake: &databake::TokenStream,
    bakes_to_ids: &[(BakedValue, &std::collections::BTreeSet<DataIdentifierCow>)],
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
        const TRIE: icu_provider_baked::zerotrie::ZeroTrieSimpleAscii<&'static [u8]> = icu_provider_baked:: #baked_trie;
    };

    let baked_values = if matches!(bakes_to_ids.first(), Some((BakedValue::Struct(_), _))) {
        let bakes = bakes_to_ids.iter().map(|(bake, _)| match bake {
            BakedValue::Struct(tokens) => tokens,
            BakedValue::VarULE(_) => {
                unreachable!(
                    "All instances should equivalently return Some or None in MaybeAsVarULE"
                )
            }
        });
        quote! {
            const VALUES: &'static [<#marker_bake as icu_provider_baked::zerotrie::DynamicDataMarker>::DataStruct] = &[#(#bakes,)*];
        }
    } else {
        let byteses = bakes_to_ids
            .iter()
            .map(|(bake, _)| match bake {
                BakedValue::VarULE(bytes) => *bytes,
                BakedValue::Struct(_) => {
                    unreachable!(
                        "All instances should equivalently return Some or None in MaybeAsVarULE"
                    )
                }
            })
            .collect::<Vec<&[u8]>>();
        let vzv = VarZeroVec::<[u8]>::from(&byteses);
        let vzv_bytes = vzv.as_bytes().bake(ctx);
        quote! {
            const VALUES_VZV: &'static zerovec::VarZeroSlice<<<#marker_bake as icu_provider_baked::zerotrie::DynamicDataMarker>::DataStruct as icu_provider::marker::MaybeAsVarULE>::VarULE> = unsafe { zerovec::VarZeroSlice::from_bytes_unchecked(#vzv_bytes) };
        }
    };

    (
        quote! {
            // Safety invariant upheld: see above
            icu_provider_baked::zerotrie::Data<#marker_bake> = {
                #baked_trie
                #baked_values
                unsafe {
                    icu_provider_baked::zerotrie::Data::from_trie_and_values_unchecked(TRIE, VALUES)
                }
            }

        },
        core::mem::size_of::<Data<icu_provider::hello_world::HelloWorldV1>>()
            + trie.as_borrowed_slice().borrows_size(),
    )
}

pub struct Data<M: DataMarker> {
    // Unsafe invariant: actual values contained MUST be valid indices into `values`
    trie: ZeroTrieSimpleAscii<&'static [u8]>,
    values: &'static [M::DataStruct],
}

impl<M: DataMarker> Data<M> {
    /// Construct from a trie and values
    ///
    /// # Safety
    /// The actual values contained in the trie must be valid indices into `values`
    pub const unsafe fn from_trie_and_values_unchecked(
        trie: ZeroTrieSimpleAscii<&'static [u8]>,
        values: &'static [M::DataStruct],
    ) -> Self {
        Self { trie, values }
    }
}

impl<M: DataMarker> super::DataStore<M> for Data<M> {
    fn get(
        &self,
        id: DataIdentifierBorrowed,
        attributes_prefix_match: bool,
    ) -> Option<&'static <M>::DataStruct> {
        use writeable::Writeable;
        let mut cursor = self.trie.cursor();
        let _is_ascii = id.locale.write_to(&mut cursor);
        if !id.marker_attributes.is_empty() {
            cursor.step(ID_SEPARATOR);
            id.marker_attributes.write_to(&mut cursor).ok()?;
            loop {
                if let Some(v) = cursor.take_value() {
                    break Some(v);
                }
                if !attributes_prefix_match || cursor.probe(0).is_none() {
                    break None;
                }
            }
        } else {
            cursor.take_value()
        }
        // Safety: Allowed since `i` came from the trie and the field safety invariant
        .map(|i| unsafe { self.values.get_unchecked(i) })
    }

    #[cfg(feature = "alloc")]
    type IterReturn = core::iter::FilterMap<
        zerotrie::ZeroTrieStringIterator<'static>,
        fn((alloc::string::String, usize)) -> Option<DataIdentifierCow<'static>>,
    >;
    #[cfg(feature = "alloc")]
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
