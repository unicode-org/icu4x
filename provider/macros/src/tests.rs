// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::data_struct_impl;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::ItemStruct;
use syn::NestedMeta;

fn check(attr: Vec<TokenStream2>, item: TokenStream2, expected: TokenStream2) {
    let actual = data_struct_impl(
        attr.into_iter()
            .map(syn::parse2)
            .collect::<syn::parse::Result<Vec<NestedMeta>>>()
            .unwrap(),
        syn::parse2(item).unwrap(),
    );
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn test_basic() {
    // #[data_struct]
    check(
        vec![],
        quote!(
            pub struct FooV1;
        ),
        quote!(
            #[doc="Marker type for [`FooV1`]"]
            pub struct FooV1Marker;
            impl icu_provider::DataMarker for FooV1Marker {
                type Yokeable = FooV1;
            }
            #[derive(Yokeable, ZeroCopyFrom)]
            pub struct FooV1;
        ),
    );
}

#[test]
fn test_resource_marker() {
    // #[data_struct("demo/foo@1")]
    check(
        vec![quote!("demo/foo@1")],
        quote!(
            pub struct FooV1;
        ),
        quote!(
            #[doc="Marker type for [`FooV1`]"]
            pub struct FooV1Marker;
            impl icu_provider::DataMarker for FooV1Marker {
                type Yokeable = FooV1;
            }
            #[derive(Yokeable, ZeroCopyFrom)]
            pub struct FooV1;
            impl icu_provider::ResourceMarker for FooV1Marker {
                const KEY: icu_provider::ResourceKey = icu_provider::resource_key!("demo/foo@1");
            }
        ),
    );
}
