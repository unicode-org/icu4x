// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::any::TypeId;

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
        } else {
            unreachable!("sealed")
        };
        quote! {
            // Safety: the store comes from a valid Pattern
            unsafe { icu_pattern::Pattern::<#b, _>::from_store_unchecked(#store) }
        }
    }
}

#[test]
fn test_baked() {
    use alloc::borrow::Cow;
    use ::databake::test_bake;
    test_bake!(
        Pattern<SinglePlaceholder, Cow<str>>,
        const: unsafe { crate::Pattern::<SinglePlaceholder, Cow<str>>::from_store_unchecked(Cow::Borrowed("")) },
        icu_provider
    );
}
