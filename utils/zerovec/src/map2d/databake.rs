// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{maps::ZeroMap2dBorrowed, maps::ZeroMapKV, ZeroMap2d};
use databake::*;

impl<'a, K0, K1, V> Bake for ZeroMap2d<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    K0::Container: Bake,
    K1::Container: Bake,
    V::Container: Bake,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys0 = self.keys0.bake(env);
        let joiner = self.joiner.bake(env);
        let keys1 = self.keys1.bake(env);
        let values = self.values.bake(env);
        quote! { unsafe { #[allow(unused_unsafe)] ::zerovec::ZeroMap2d::from_parts_unchecked(#keys0, #joiner, #keys1, #values) } }
    }
}

impl<'a, K0, K1, V> Bake for ZeroMap2dBorrowed<'a, K0, K1, V>
where
    K0: ZeroMapKV<'a> + ?Sized,
    K1: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    &'a K0::Slice: Bake,
    &'a K1::Slice: Bake,
    &'a V::Slice: Bake,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys0 = self.keys0.bake(env);
        let joiner = self.joiner.bake(env);
        let keys1 = self.keys1.bake(env);
        let values = self.values.bake(env);
        quote! { unsafe { #[allow(unused_unsafe)] ::zerovec::maps::ZeroMap2dBorrowed::from_parts_unchecked(#keys0, #joiner, #keys1, #values) } }
    }
}
