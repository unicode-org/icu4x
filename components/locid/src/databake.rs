// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::subtags::{Language, Region, Script, Variant};
use databake::*;

impl Bake for Script {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::subtags_script!(#string) }
    }
}

#[test]
fn bake_script() {
    test_bake!(Script, const: crate::subtags_script!("Hant"), icu_locid);
}

impl Bake for Language {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::subtags_language!(#string) }
    }
}

#[test]
fn bake_language() {
    test_bake!(Language, const: crate::subtags_language!("de"), icu_locid);
}

impl Bake for Region {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::subtags_region!(#string) }
    }
}

#[test]
fn bake_region() {
    test_bake!(Region, const: crate::subtags_region!("FR"), icu_locid);
}

impl Bake for Variant {
    fn bake(&self, env: &CrateEnv) -> TokenStream {
        env.insert("icu_locid");
        let string = self.as_str();
        quote! { ::icu_locid::subtags_variant!(#string) }
    }
}

#[test]
fn bake_variant() {
    test_bake!(Variant, const: crate::subtags_variant!("posix"), icu_locid);
}
