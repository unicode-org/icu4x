// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LiteMap;
use databake::*;

/// Bakes a LiteMap into Rust code for fast runtime construction from data. Use this impl during
/// code generation or in a `build.rs` script.
///
/// For the most efficient bake, bake the [`LiteMap`] with a slice store.
///
/// # Examples
///
/// ```
/// use databake::*;
/// use litemap::LiteMap;
///
/// let mut litemap = LiteMap::new_vec();
/// litemap.insert(1usize, "one");
/// litemap.insert(2usize, "one");
/// litemap.insert(10usize, "ten");
///
/// let litemap_slice = litemap.as_sliced();
///
/// assert_eq!(
///     litemap_slice.bake(&Default::default()).to_string(),
///     r#"litemap :: LiteMap :: from_sorted_store_unchecked (& [(1usize , "one") , (2usize , "one") , (10usize , "ten")])"#,
/// );
/// ```
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
        LiteMap<usize, &str, &[(usize, &str)]>,
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
