// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::ZeroMap2d;
use crate::map::ZeroMapKV;
use crabbake::*;

impl<'a, K0, K1, V> Bakeable for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a>,
    K1: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
    K0::Container: Bakeable,
    K1::Container: Bakeable,
    V::Container: Bakeable,
    K0: ?Sized,
    K1: ?Sized,
    V: ?Sized,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys0 = self.keys0.bake(env);
        let joiner = self.joiner.bake(env);
        let keys1 = self.keys1.bake(env);
        let values = self.values.bake(env);
        quote! { ::zerovec::ZeroMap2d {
            keys0: #keys0,
            joiner: #joiner,
            keys1: #keys1,
            values: #values
        } }
    }
}
