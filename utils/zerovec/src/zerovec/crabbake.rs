// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::ZeroVec;
use crate::{ule::AsULE, ZeroSlice};
use crabbake::*;

impl<T> Bakeable for ZeroVec<'_, T>
where
    T: AsULE + ?Sized,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("core");
        env.insert("zerovec");
        let bytes = self.as_bytes();
        quote! { unsafe { ::zerovec::ZeroVec::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}

impl<T> Bakeable for &ZeroSlice<T>
where
    T: AsULE + ?Sized,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("core");
        env.insert("zerovec");
        let bytes = self.as_bytes();
        quote! { unsafe { ::zerovec::ZeroSlice::from_bytes_unchecked(&[#(#bytes),*]) } }
    }
}
