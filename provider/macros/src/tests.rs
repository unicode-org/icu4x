// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::data_struct_impl;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, NestedMeta};

fn check(attr: Vec<TokenStream2>, item: TokenStream2, expected: TokenStream2) {
    let actual = data_struct_impl(
        attr.into_iter()
            .map(syn::parse2)
            .collect::<syn::parse::Result<Vec<NestedMeta>>>()
            .unwrap(),
        syn::parse2::<DeriveInput>(item).unwrap(),
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
            #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
            pub struct FooV1;
        ),
    );
}

#[test]
fn test_data_marker() {
    // #[data_struct(FooV1Marker)]
    check(
        vec![quote!(FooV1Marker)],
        quote!(
            pub struct FooV1;
        ),
        quote!(
            #[doc = "Marker type for [`FooV1`]"]
            pub struct FooV1Marker;
            impl icu_provider::DataMarker for FooV1Marker {
                type Yokeable = FooV1;
            }
            #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
            pub struct FooV1;
        ),
    );
}

#[test]
fn test_resource_marker() {
    // #[data_struct(BarV1Marker = "demo/bar@1")]
    check(
        vec![quote!(BarV1Marker = "demo/bar@1")],
        quote!(
            pub struct FooV1;
        ),
        quote!(
            #[doc = "Marker type for [`FooV1`]: \"demo/bar@1\""]
            pub struct BarV1Marker;
            impl icu_provider::DataMarker for BarV1Marker {
                type Yokeable = FooV1;
            }
            impl icu_provider::ResourceMarker for BarV1Marker {
                const KEY: icu_provider::ResourceKey = icu_provider::resource_key!("demo/bar@1");
            }
            #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
            pub struct FooV1;
        ),
    );
}

#[test]
fn test_multi_named_resource_marker() {
    // #[data_struct(FooV1Marker, BarV1Marker = "demo/bar@1", BazV1Marker = "demo/baz@1")]
    check(
        vec![
            quote!(FooV1Marker),
            quote!(BarV1Marker = "demo/bar@1"),
            quote!(BazV1Marker = "demo/baz@1"),
        ],
        quote!(
            pub struct FooV1<'data>;
        ),
        quote!(
            #[doc = "Marker type for [`FooV1`]"]
            pub struct FooV1Marker;
            impl icu_provider::DataMarker for FooV1Marker {
                type Yokeable = FooV1<'static>;
            }
            #[doc = "Marker type for [`FooV1`]: \"demo/bar@1\""]
            pub struct BarV1Marker;
            impl icu_provider::DataMarker for BarV1Marker {
                type Yokeable = FooV1<'static>;
            }
            impl icu_provider::ResourceMarker for BarV1Marker {
                const KEY: icu_provider::ResourceKey = icu_provider::resource_key!("demo/bar@1");
            }
            #[doc = "Marker type for [`FooV1`]: \"demo/baz@1\""]
            pub struct BazV1Marker;
            impl icu_provider::DataMarker for BazV1Marker {
                type Yokeable = FooV1<'static>;
            }
            impl icu_provider::ResourceMarker for BazV1Marker {
                const KEY: icu_provider::ResourceKey = icu_provider::resource_key!("demo/baz@1");
            }
            #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
            pub struct FooV1<'data>;
        ),
    );
}

#[test]
fn test_crabbake() {
    check(
        vec![quote!(BarV1Marker = "demo/bar@1")],
        quote!(
            #[crabbake(path = test::path)]
            pub struct FooV1;
        ),
        quote!(
            #[doc = "Marker type for [`FooV1`]: \"demo/bar@1\""]
            #[derive(Default, crabbake::Bakeable)]
            #[crabbake(path = test::path)]
            pub struct BarV1Marker;
            impl icu_provider::DataMarker for BarV1Marker {
                type Yokeable = FooV1;
            }
            impl icu_provider::ResourceMarker for BarV1Marker {
                const KEY: icu_provider::ResourceKey = icu_provider::resource_key!("demo/bar@1");
            }
            #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
            #[crabbake(path = test::path)]
            pub struct FooV1;
        ),
    );
}
