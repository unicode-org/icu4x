// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{FlexZeroSlice, FlexZeroVec};
use crabbake::*;

impl Bakeable for FlexZeroVec<'_> {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("core");
        env.insert("zerovec");
        let bytes = self.as_bytes();
        quote! { unsafe { ::zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[#(#bytes),*]).as_flexzerovec() } }
    }
}

impl Bakeable for &FlexZeroSlice {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("core");
        env.insert("zerovec");
        let bytes = self.as_bytes();
        quote! { unsafe { ::zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[#(#bytes),*]) } }
    }
}

#[cfg(test)]
#[allow(non_camel_case_types)]
mod test {
    use super::{FlexZeroSlice, FlexZeroVec};
    use core::str::FromStr;
    use crabbake::{Bakeable, TokenStream};

    // [1, 22, 333, 4444];
    const BYTES: &[u8] = &[2, 0x01, 0x00, 0x16, 0x00, 0x4D, 0x01, 0x5C, 0x11];

    #[rustfmt::skip]
    mod baked {
        use crate as zerovec;
        use super::{FlexZeroSlice, FlexZeroVec};

        pub const BAKED_VEC: FlexZeroVec<'static> = unsafe {
            zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8
            ])
            .as_flexzerovec()
        };

        // README: Except for the '::zerovec' instead of 'zerovec',
        // this string should be identical to the code above.
        pub const BAKED_VEC_STR: &str = "unsafe {
            ::zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8
            ])
            .as_flexzerovec()
        }";

        pub const BAKED_SLICE: &FlexZeroSlice = unsafe {
            zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8
            ])
        };

        // README: Except for the '::zerovec' instead of 'zerovec',
        // this string should be identical to the code above.
        pub const BAKED_SLICE_STR: &str = "unsafe {
            ::zerovec::vecs::FlexZeroSlice::from_byte_slice_unchecked(&[
                2u8, 1u8, 0u8, 22u8, 0u8, 77u8, 1u8, 92u8, 17u8
            ])
        }";
    }

    #[test]
    fn test_baked_vec() {
        let reference: FlexZeroVec = FlexZeroVec::parse_byte_slice(BYTES).expect("parse");
        assert_eq!(baked::BAKED_VEC, reference);
        let ctx = crabbake::CrateEnv::default();
        let actual_tokens = reference.bake(&ctx);
        assert_eq!(
            actual_tokens.to_string(),
            TokenStream::from_str(baked::BAKED_VEC_STR)
                .unwrap()
                .to_string()
        );
    }

    #[test]
    fn test_baked_slice() {
        let reference: &FlexZeroSlice = FlexZeroSlice::parse_byte_slice(BYTES).expect("parse");
        assert_eq!(baked::BAKED_SLICE, reference);
        let ctx = crabbake::CrateEnv::default();
        let actual_tokens = reference.bake(&ctx);
        assert_eq!(
            actual_tokens.to_string(),
            TokenStream::from_str(baked::BAKED_SLICE_STR)
                .unwrap()
                .to_string()
        );
    }
}
