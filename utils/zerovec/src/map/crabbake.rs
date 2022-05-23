// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{ZeroMap, ZeroMapKV};
use crabbake::*;

impl<'a, K, V> Bakeable for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized + Ord,
    V: ZeroMapKV<'a> + ?Sized,
    K::Container: Bakeable,
    V::Container: Bakeable,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys = self.keys.bake(env);
        let values = self.values.bake(env);
        quote! { ::zerovec::ZeroMap { keys: #keys, values: #values } }
    }
}
