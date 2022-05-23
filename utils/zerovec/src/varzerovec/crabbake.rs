// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::VarZeroVec;
use crate::ule::*;
use crabbake::*;

impl<T: VarULE + ?Sized> Bakeable for VarZeroVec<'_, T> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        // Safe because VarZeroSlice has the same layout as [u8]
        quote! { unsafe {
            static BYTES: &[u8] = &[#(#bytes),*];
            ::zerovec::VarZeroVec::Borrowed(core::mem::transmute(BYTES))
        }}
    }
}
