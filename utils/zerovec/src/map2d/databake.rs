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
    K0::Slice: Bake,
    K1::Slice: Bake,
    V::Slice: Bake,
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

#[test]
fn test_baked_map() {
    test_bake!(
        ZeroMap2d<str, str, str>,
        const: unsafe {
            #[allow(unused_unsafe)]
            crate::ZeroMap2d::from_parts_unchecked(
                unsafe {
                    crate::VarZeroVec::from_bytes_unchecked(&[
                        97u8, 114u8, 99u8, 97u8, 122u8, 0u8, 99u8, 117u8, 0u8, 101u8, 110u8, 0u8,
                        102u8, 102u8, 0u8, 103u8, 114u8, 99u8, 107u8, 107u8, 0u8, 107u8, 117u8,
                        0u8, 107u8, 121u8, 0u8, 108u8, 105u8, 102u8, 109u8, 97u8, 110u8, 109u8,
                        110u8, 0u8, 112u8, 97u8, 0u8, 112u8, 97u8, 108u8, 115u8, 100u8, 0u8, 116u8,
                        103u8, 0u8, 117u8, 103u8, 0u8, 117u8, 110u8, 114u8, 117u8, 122u8, 0u8,
                        121u8, 117u8, 101u8, 122u8, 104u8, 0u8,
                    ])
                },
                unsafe {
                    crate::ZeroVec::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8,
                        0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 10u8, 0u8,
                        0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                        15u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 20u8, 0u8,
                        0u8, 0u8, 21u8, 0u8, 0u8, 0u8, 22u8, 0u8, 0u8, 0u8, 23u8, 0u8, 0u8, 0u8,
                        24u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    crate::VarZeroVec::from_bytes_unchecked(&[
                        78u8, 98u8, 97u8, 116u8, 80u8, 97u8, 108u8, 109u8, 65u8, 114u8, 97u8, 98u8,
                        71u8, 108u8, 97u8, 103u8, 83u8, 104u8, 97u8, 119u8, 65u8, 100u8, 108u8,
                        109u8, 76u8, 105u8, 110u8, 98u8, 65u8, 114u8, 97u8, 98u8, 65u8, 114u8,
                        97u8, 98u8, 89u8, 101u8, 122u8, 105u8, 65u8, 114u8, 97u8, 98u8, 76u8, 97u8,
                        116u8, 110u8, 76u8, 105u8, 109u8, 98u8, 78u8, 107u8, 111u8, 111u8, 77u8,
                        111u8, 110u8, 103u8, 65u8, 114u8, 97u8, 98u8, 80u8, 104u8, 108u8, 112u8,
                        68u8, 101u8, 118u8, 97u8, 75u8, 104u8, 111u8, 106u8, 83u8, 105u8, 110u8,
                        100u8, 65u8, 114u8, 97u8, 98u8, 67u8, 121u8, 114u8, 108u8, 68u8, 101u8,
                        118u8, 97u8, 65u8, 114u8, 97u8, 98u8, 72u8, 97u8, 110u8, 115u8, 66u8,
                        111u8, 112u8, 111u8, 72u8, 97u8, 110u8, 98u8, 72u8, 97u8, 110u8, 116u8,
                    ])
                },
                unsafe {
                    crate::VarZeroVec::from_bytes_unchecked(&[
                        74u8, 79u8, 0u8, 83u8, 89u8, 0u8, 73u8, 82u8, 0u8, 66u8, 71u8, 0u8, 71u8,
                        66u8, 0u8, 71u8, 78u8, 0u8, 71u8, 82u8, 0u8, 67u8, 78u8, 0u8, 73u8, 81u8,
                        0u8, 71u8, 69u8, 0u8, 67u8, 78u8, 0u8, 84u8, 82u8, 0u8, 73u8, 78u8, 0u8,
                        71u8, 78u8, 0u8, 67u8, 78u8, 0u8, 80u8, 75u8, 0u8, 67u8, 78u8, 0u8, 73u8,
                        78u8, 0u8, 73u8, 78u8, 0u8, 73u8, 78u8, 0u8, 80u8, 75u8, 0u8, 75u8, 90u8,
                        0u8, 78u8, 80u8, 0u8, 65u8, 70u8, 0u8, 67u8, 78u8, 0u8, 84u8, 87u8, 0u8,
                        84u8, 87u8, 0u8, 84u8, 87u8, 0u8,
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
        ZeroMap2dBorrowed<str, str, str>,
        const: unsafe {
            #[allow(unused_unsafe)]
            crate::maps::ZeroMap2dBorrowed::from_parts_unchecked(
                unsafe {
                    crate::VarZeroSlice::from_bytes_unchecked(&[
                        97u8, 114u8, 99u8, 97u8, 122u8, 0u8, 99u8, 117u8, 0u8, 101u8, 110u8, 0u8,
                        102u8, 102u8, 0u8, 103u8, 114u8, 99u8, 107u8, 107u8, 0u8, 107u8, 117u8,
                        0u8, 107u8, 121u8, 0u8, 108u8, 105u8, 102u8, 109u8, 97u8, 110u8, 109u8,
                        110u8, 0u8, 112u8, 97u8, 0u8, 112u8, 97u8, 108u8, 115u8, 100u8, 0u8, 116u8,
                        103u8, 0u8, 117u8, 103u8, 0u8, 117u8, 110u8, 114u8, 117u8, 122u8, 0u8,
                        121u8, 117u8, 101u8, 122u8, 104u8, 0u8,
                    ])
                },
                unsafe {
                    crate::ZeroSlice::from_bytes_unchecked(&[
                        2u8, 0u8, 0u8, 0u8, 3u8, 0u8, 0u8, 0u8, 4u8, 0u8, 0u8, 0u8, 5u8, 0u8, 0u8,
                        0u8, 6u8, 0u8, 0u8, 0u8, 7u8, 0u8, 0u8, 0u8, 8u8, 0u8, 0u8, 0u8, 10u8, 0u8,
                        0u8, 0u8, 12u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8, 14u8, 0u8, 0u8, 0u8,
                        15u8, 0u8, 0u8, 0u8, 16u8, 0u8, 0u8, 0u8, 17u8, 0u8, 0u8, 0u8, 20u8, 0u8,
                        0u8, 0u8, 21u8, 0u8, 0u8, 0u8, 22u8, 0u8, 0u8, 0u8, 23u8, 0u8, 0u8, 0u8,
                        24u8, 0u8, 0u8, 0u8, 25u8, 0u8, 0u8, 0u8, 28u8, 0u8, 0u8, 0u8,
                    ])
                },
                unsafe {
                    crate::VarZeroSlice::from_bytes_unchecked(&[
                        78u8, 98u8, 97u8, 116u8, 80u8, 97u8, 108u8, 109u8, 65u8, 114u8, 97u8, 98u8,
                        71u8, 108u8, 97u8, 103u8, 83u8, 104u8, 97u8, 119u8, 65u8, 100u8, 108u8,
                        109u8, 76u8, 105u8, 110u8, 98u8, 65u8, 114u8, 97u8, 98u8, 65u8, 114u8,
                        97u8, 98u8, 89u8, 101u8, 122u8, 105u8, 65u8, 114u8, 97u8, 98u8, 76u8, 97u8,
                        116u8, 110u8, 76u8, 105u8, 109u8, 98u8, 78u8, 107u8, 111u8, 111u8, 77u8,
                        111u8, 110u8, 103u8, 65u8, 114u8, 97u8, 98u8, 80u8, 104u8, 108u8, 112u8,
                        68u8, 101u8, 118u8, 97u8, 75u8, 104u8, 111u8, 106u8, 83u8, 105u8, 110u8,
                        100u8, 65u8, 114u8, 97u8, 98u8, 67u8, 121u8, 114u8, 108u8, 68u8, 101u8,
                        118u8, 97u8, 65u8, 114u8, 97u8, 98u8, 72u8, 97u8, 110u8, 115u8, 66u8,
                        111u8, 112u8, 111u8, 72u8, 97u8, 110u8, 98u8, 72u8, 97u8, 110u8, 116u8,
                    ])
                },
                unsafe {
                    crate::VarZeroSlice::from_bytes_unchecked(&[
                        74u8, 79u8, 0u8, 83u8, 89u8, 0u8, 73u8, 82u8, 0u8, 66u8, 71u8, 0u8, 71u8,
                        66u8, 0u8, 71u8, 78u8, 0u8, 71u8, 82u8, 0u8, 67u8, 78u8, 0u8, 73u8, 81u8,
                        0u8, 71u8, 69u8, 0u8, 67u8, 78u8, 0u8, 84u8, 82u8, 0u8, 73u8, 78u8, 0u8,
                        71u8, 78u8, 0u8, 67u8, 78u8, 0u8, 80u8, 75u8, 0u8, 67u8, 78u8, 0u8, 73u8,
                        78u8, 0u8, 73u8, 78u8, 0u8, 73u8, 78u8, 0u8, 80u8, 75u8, 0u8, 75u8, 90u8,
                        0u8, 78u8, 80u8, 0u8, 65u8, 70u8, 0u8, 67u8, 78u8, 0u8, 84u8, 87u8, 0u8,
                        84u8, 87u8, 0u8, 84u8, 87u8, 0u8,
                    ])
                },
            )
        },
        zerovec
    );
}
