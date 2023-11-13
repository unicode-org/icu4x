// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LiteMap;
use databake::*;

impl<K, V, S> Bake for LiteMap<K, V, S>
where
    S: Bake,
{
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("litemap");
        let store = self.values.bake(env);
        quote! { litemap::LiteMap::from_sorted_store_unchecked(#store) }
    }
}

#[test]
fn test_baked_map() {
    // Const construction:
    test_bake!(
        LiteMap<usize, &'static str, &'static [(usize, &'static str)]>,
        const: crate::LiteMap::from_sorted_store_unchecked(
                &[
                    (1usize, "one"),
                    (2usize, "two"),
                    (10usize, "ten")
                ]
            ),
        litemap
    );
    // Non-const construction:
    test_bake!(
        LiteMap<usize, String, Vec<(usize, String)>>,
        crate::LiteMap::from_sorted_store_unchecked(
                alloc::vec![
                    (1usize, "one".to_owned()),
                    (2usize, "two".to_owned()),
                    (10usize, "ten".to_owned()),
                ]
            ),
        litemap
    );
}
