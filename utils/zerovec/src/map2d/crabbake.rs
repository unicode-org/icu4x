// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{maps::ZeroMapKV, ZeroMap2d};
use crabbake::*;

impl<'a, K0, K1, V> Bakeable for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    K0::Container: Bakeable,
    K1::Container: Bakeable,
    V::Container: Bakeable,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys0 = self.keys0.bake(env);
        let joiner = self.joiner.bake(env);
        let keys1 = self.keys1.bake(env);
        let values = self.values.bake(env);
        quote! { unsafe { ::zerovec::ZeroMap2d::from_raw_parts(#keys0, #joiner, #keys1, #values) } }
    }
}
