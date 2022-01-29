// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::data_struct_impl;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::ItemStruct;
use syn::parse::Parse;

#[test]
fn test_basic() {
    // #[data_struct]
    let attr = vec![];
    let item = syn::parse2(quote!(
        pub struct FooV1;
    )).unwrap();
    let expected = quote!(
        #[doc="Marker type for [`FooV1`]"]
        pub struct FooV1Marker;
        impl icu_provider::DataMarker for FooV1Marker {
            type Yokeable = FooV1;
        }
        #[derive(Yokeable, ZeroCopyFrom)]
        pub struct FooV1;
    );
    let actual = data_struct_impl(
        attr,
        item
    );
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn test_resource_marker() {
    // #[data_struct("demo/foo@1")]
    let attr = vec![
        syn::parse2(quote!("demo/foo@1")).unwrap()
    ];
    let item = syn::parse2(quote!(
        pub struct FooV1;
    )).unwrap();
    let expected = quote!(
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
    );
    let actual = data_struct_impl(
        attr,
        item
    );
    assert_eq!(expected.to_string(), actual.to_string());
}
