// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{ule::VarULE, VarZeroSlice, VarZeroVec};
use databake::*;

use super::Index32;

impl<T: VarULE + ?Sized> Bake for VarZeroVec<'_, T> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        // Safe because self.as_bytes is a safe input
        quote! { unsafe { ::zerovec::VarZeroVec::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}

impl<T: VarULE + ?Sized> Bake for VarZeroVec<'_, T, Index32> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        // Safe because self.as_bytes is a safe input
        quote! { unsafe { ::zerovec::VarZeroVec::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}

impl<T: VarULE + ?Sized> Bake for &VarZeroSlice<T> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        // Safe because self.as_bytes is a safe input
        quote! { unsafe { ::zerovec::VarZeroSlice::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}

#[test]
fn test_baked_vec() {
    test_bake!(
        VarZeroVec<str>,
        const: unsafe {
            crate::VarZeroVec::from_bytes_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8,
            ])
        },
        zerovec
    );
}

#[test]
fn test_baked_slice() {
    test_bake!(
        &VarZeroSlice<str>,
        const: unsafe {
            crate::VarZeroSlice::from_bytes_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8,
            ])
        },
        zerovec
    );
}
