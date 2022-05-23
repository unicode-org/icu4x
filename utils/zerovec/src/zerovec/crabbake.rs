// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::ZeroVec;
use crate::ule::AsULE;
use crabbake::*;

impl<T> Bakeable for ZeroVec<'_, T>
where
    T: AsULE + ?Sized,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("core");
        env.insert("zerovec");
        let bytes = self.as_bytes();
        let size: usize = core::mem::size_of::<T::ULE>();
        // Safe because self.as_bytes() and self.as_ule_slice() are the same slice
        // with different length metadata.
        quote! { unsafe {
            static DATA: &[u8] = &[#(#bytes),*];
            let (data, mut metadata): (usize, usize) = core::mem::transmute(DATA);
            metadata /= #size;
            zerovec::ZeroVec::Borrowed(core::mem::transmute((data, metadata)))
        }}
    }
}
