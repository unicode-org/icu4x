// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::subtags::{Language, Region, Script, Variant};
use crabbake::*;

impl Bakeable for Script {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::script!(#string) }
    }
}
impl Bakeable for Language {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::language!(#string) }
    }
}
impl Bakeable for Region {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::region!(#string) }
    }
}
impl Bakeable for Variant {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::variant!(#string) }
    }
}
