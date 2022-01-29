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
    let input = quote!(
        pub struct FooV1;
    );
    let expected = quote!(
        #[doc="Marker type for [`FooV1`]"]
        pub struct FooV1Marker;
        impl icu_provider::DataMarker for FooV1Marker {
            type Yokeable = FooV1;
        }
        #[derive(Yokeable, ZeroCopyFrom)]
        pub struct FooV1;
    );
    let item = syn::parse2(input).unwrap();
    let actual = data_struct_impl(item);
    assert_eq!(expected.to_string(), actual.to_string());
}
