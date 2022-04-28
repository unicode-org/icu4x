// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Yoke;
use crate::Yokeable;
use crabbake::{Bakeable, CrateEnv, TokenStream};

impl<Y, C> Bakeable for Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    for<'a> <Y as Yokeable<'a>>::Output: Bakeable,
{
    fn bake<'a>(&'a self, ctx: &CrateEnv) -> TokenStream {
        self.get().bake(ctx)
    }
}
