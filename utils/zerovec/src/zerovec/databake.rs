// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::ZeroVec;
use crate::{ule::AsULE, ZeroSlice};
use databake::*;

impl<T> Bake for ZeroVec<'_, T>
where
    T: AsULE + ?Sized,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = databake::Bake::bake(&self.as_bytes(), env);
        quote! { unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(#bytes) } }
    }
}

impl<T> Bake for &ZeroSlice<T>
where
    T: AsULE + ?Sized,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = databake::Bake::bake(&self.as_bytes(), env);
        quote! { unsafe { ::zerovec::ZeroSlice::from_bytes_unchecked(#bytes) } }
    }
}

#[test]
fn test_baked_vec() {
    test_bake!(
        ZeroVec<u32>,
        const: unsafe {
            crate::ZeroVec::from_bytes_unchecked(&[2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8])
        },
        zerovec
    );
}

#[test]
fn test_baked_slice() {
    test_bake!(
        &ZeroSlice<u32>,
        const: unsafe {
            crate::ZeroSlice::from_bytes_unchecked(&[2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8])
        },
        zerovec
    );
}
