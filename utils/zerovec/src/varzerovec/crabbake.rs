// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{ule::VarULE, VarZeroSlice, VarZeroVec};
use crabbake::*;

impl<T: VarULE + ?Sized> Bakeable for VarZeroVec<'_, T> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        // Safe because self.as_bytes is a safe input
        quote! { unsafe { ::zerovec::VarZeroVec::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}

impl<T: VarULE + ?Sized> Bakeable for &VarZeroSlice<T> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let bytes = self.as_bytes();
        // Safe because self.as_bytes is a safe input
        quote! { unsafe { ::zerovec::VarZeroSlice::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}
