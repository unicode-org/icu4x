// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{FlexZeroSlice, FlexZeroVec};
use crabbake::*;

impl Bakeable for FlexZeroVec<'_> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        quote! { unsafe { ::zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[#(#bytes),*]).as_flexzerovec() } }
    }
}

impl Bakeable for &FlexZeroSlice {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        quote! { unsafe { ::zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[#(#bytes),*]) } }
    }
}

#[test]
fn test_baked_vec() {
    test_bake!(
        unsafe {
            crate::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8
            ])
            .as_flexzerovec()
        },
        zerovec
    );
}

#[test]
fn test_baked_slice() {
    test_bake!(
        unsafe {
            crate::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8
            ])
        },
        zerovec
    );
}
