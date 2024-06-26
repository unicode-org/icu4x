// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::any::TypeId;

use crate::DoublePlaceholder;
use crate::SinglePlaceholder;

use super::*;
use ::databake::{quote, Bake, CrateEnv, TokenStream};

impl<B, Store> Bake for Pattern<B, Store>
where
    B: 'static,
    Store: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("icu_pattern");
        let store = self.store.bake(ctx);
        let b = if TypeId::of::<B>() == TypeId::of::<SinglePlaceholder>() {
            quote!(icu_pattern::SinglePlaceholder)
        } else if TypeId::of::<B>() == TypeId::of::<DoublePlaceholder>() {
            quote!(icu_pattern::DoublePlaceholder)
        } else {
            unreachable!("all impls of sealed trait PatternBackend should be covered")
        };
        quote! {
            icu_pattern::Pattern::<#b, _>::from_store_unchecked(#store)
        }
    }
}

#[test]
fn test_baked() {
    use ::databake::test_bake;
    use alloc::borrow::Cow;
    test_bake!(
        Pattern<SinglePlaceholder, Cow<str>>,
        const,
        crate::Pattern::<crate::SinglePlaceholder, _>::from_store_unchecked(alloc::borrow::Cow::Borrowed("")),
        icu_pattern
    );
}
