// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::TinyAsciiStr;
use crabbake::*;

impl<const N: usize> Bakeable for TinyAsciiStr<N> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("tinystr");
        let string = self.as_str();
        quote! {
            ::tinystr::tinystr!(#N, #string)
        }
    }
}
