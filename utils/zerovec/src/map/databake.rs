// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{maps::ZeroMapBorrowed, maps::ZeroMapKV, ZeroMap};
use databake::*;

impl<'a, K, V> Bake for ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    K::Container: Bake,
    V::Container: Bake,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys = self.keys.bake(env);
        let values = self.values.bake(env);
        quote! { unsafe { #[allow(unused_unsafe)] ::zerovec::ZeroMap::from_parts_unchecked(#keys, #values) } }
    }
}

impl<'a, K, V> Bake for ZeroMapBorrowed<'a, K, V>
where
    K: ZeroMapKV<'a> + ?Sized,
    V: ZeroMapKV<'a> + ?Sized,
    &'a K::Slice: Bake,
    &'a V::Slice: Bake,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("zerovec");
        let keys = self.keys.bake(env);
        let values = self.values.bake(env);
        quote! { unsafe { #[allow(unused_unsafe)] ::zerovec::maps::ZeroMapBorrowed::from_parts_unchecked(#keys, #values) } }
    }
}

#[test]
fn test_baked_map() {
    test_bake!(
        ZeroMap<str, str>,
        const: unsafe {
            #[allow(unused_unsafe)]
            crate::ZeroMap::from_parts_unchecked(
                unsafe {
                    crate::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 97u8,
                        100u8, 98u8, 99u8,
                    ])
                },
                unsafe {
                    crate::VarZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 69u8, 82u8,
                        65u8, 49u8, 69u8, 82u8, 65u8, 48u8,
                    ])
                },
            )
        },
        zerovec
    );
}

#[test]
fn test_baked_borrowed_map() {
    test_bake!(
        ZeroMapBorrowed<str, str>,
        const: unsafe {
            #[allow(unused_unsafe)]
            crate::maps::ZeroMapBorrowed::from_parts_unchecked(
                unsafe {
                    crate::VarZeroSlice::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 97u8,
                        100u8, 98u8, 99u8,
                    ])
                },
                unsafe {
                    crate::VarZeroSlice::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 69u8, 82u8,
                        65u8, 49u8, 69u8, 82u8, 65u8, 48u8,
                    ])
                },
            )
        },
        zerovec
    );
}
