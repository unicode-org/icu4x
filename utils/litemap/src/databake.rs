// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::LiteMap;
use databake::*;

/// Bakes a LiteMap into Rust code for fast runtime construction from data. Use this impl during
/// code generation or in a `build.rs` script.
///
/// For the most efficient bake, bake the [`LiteMap`] with a slice store. Use functions such as
/// the following for converting an allocated [`LiteMap`] to a borrowing [`LiteMap`]:
///
/// - [`LiteMap::to_borrowed_keys()`]
/// - [`LiteMap::to_borrowed_values()`]
/// - [`LiteMap::to_borrowed_keys_values()`]
/// - [`LiteMap::as_sliced()`]
///
/// # Examples
///
/// ```
/// use databake::*;
/// use litemap::LiteMap;
///
/// // Construct the LiteMap fully owned and allocated:
/// let mut litemap_alloc = LiteMap::new_vec();
/// litemap_alloc.insert(1usize, "one".to_string());
/// litemap_alloc.insert(2usize, "two".to_string());
/// litemap_alloc.insert(10usize, "ten".to_string());
///
/// // Convert to a borrowed type for baking:
/// let litemap_str: LiteMap<usize, &str> = litemap_alloc.to_borrowed_values();
/// let litemap_slice = litemap_str.as_sliced();
///
/// // The bake will now work for const construction:
/// assert_eq!(
///     litemap_slice.bake(&Default::default()).to_string(),
///     r#"litemap :: LiteMap :: from_sorted_store_unchecked (& [(1usize , "one") , (2usize , "two") , (10usize , "ten")])"#,
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
