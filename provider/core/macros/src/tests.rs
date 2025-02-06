// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::data_struct_impl;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn check(attr: TokenStream2, item: TokenStream2, expected: TokenStream2) {
    let actual = data_struct_impl(syn::parse2(attr).unwrap(), syn::parse2(item).unwrap());
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
#[rustfmt::skip] // inserts a comma
fn test_basic() {
    // #[data_struct]
    check(
        quote!(),
        quote!(
            pub struct Foo;
        ),
        quote!(
            #[derive(icu_provider::prelude::yoke::Yokeable, icu_provider::prelude::zerofrom::ZeroFrom)]
            pub struct Foo;
        ),
    );
}

#[test]
fn test_dyn_data_marker() {
    // #[data_struct(FooV1)]
    check(
        quote!(FooV1),
        quote!(
            pub struct Foo;
        ),
        quote!(
            #[doc = "Marker type for [`Foo`]"]
            pub struct FooV1;
            impl icu_provider::DynamicDataMarker for FooV1 {
                type DataStruct = Foo;
            }
            #[derive(icu_provider::prelude::yoke::Yokeable, icu_provider::prelude::zerofrom::ZeroFrom)]
            pub struct Foo;
        ),
    );
}

#[test]
fn test_data_marker() {
    // #[data_struct(BarV1 = "demo/bar@1")]
    check(
        quote!(BarV1 = "demo/bar@1"),
        quote!(
            pub struct Foo;
        ),
        quote!(
            #[doc = "Marker type for [`Foo`]: \"demo/bar@1\"\n\n- Fallback priority: language (default)"]
            pub struct BarV1;
            impl icu_provider::DynamicDataMarker for BarV1 {
                type DataStruct = Foo;
            }
            impl icu_provider::DataMarker for BarV1 {
                const INFO: icu_provider::DataMarkerInfo = {
                    let mut info = icu_provider::DataMarkerInfo::from_id(icu_provider::marker::data_marker_id!(BarV1));
                    info.is_singleton = false;
                    info.fallback_config.priority = icu_provider::fallback::LocaleFallbackPriority::default();
                    info.has_checksum = false;
                    info
                };
            }
            #[derive(icu_provider::prelude::yoke::Yokeable, icu_provider::prelude::zerofrom::ZeroFrom)]
            pub struct Foo;
        ),
    );
}

#[test]
fn test_multi_named_keyed_data_marker() {
    // #[data_struct(FooV1, BarV1 = "demo/bar@1", BazV1 = "demo/baz@1")]
    check(
        quote![FooV1, BarV1 = "demo/bar@1", BazV1 = "demo/baz@1",],
        quote!(
            pub struct Foo<'data>;
        ),
        quote!(
            #[doc = "Marker type for [`Foo`]"]
            pub struct FooV1;
            impl icu_provider::DynamicDataMarker for FooV1 {
                type DataStruct = Foo<'static>;
            }
            #[doc = "Marker type for [`Foo`]: \"demo/bar@1\"\n\n- Fallback priority: language (default)"]
            pub struct BarV1;
            impl icu_provider::DynamicDataMarker for BarV1 {
                type DataStruct = Foo<'static>;
            }
            impl icu_provider::DataMarker for BarV1 {
                const INFO: icu_provider::DataMarkerInfo = {
                    let mut info = icu_provider::DataMarkerInfo::from_id(icu_provider::marker::data_marker_id!(BarV1));
                    info.is_singleton = false;
                    info.fallback_config.priority = icu_provider::fallback::LocaleFallbackPriority::default();
                    info.has_checksum = false;
                    info
                };
            }
            #[doc = "Marker type for [`Foo`]: \"demo/baz@1\"\n\n- Fallback priority: language (default)"]
            pub struct BazV1;
            impl icu_provider::DynamicDataMarker for BazV1 {
                type DataStruct = Foo<'static>;
            }
            impl icu_provider::DataMarker for BazV1 {
                const INFO: icu_provider::DataMarkerInfo =  {
                    let mut info = icu_provider::DataMarkerInfo::from_id(icu_provider::marker::data_marker_id!(BazV1));
                    info.is_singleton = false;
                    info.fallback_config.priority = icu_provider::fallback::LocaleFallbackPriority::default();
                    info.has_checksum = false;
                    info
                };
            }
            #[derive(icu_provider::prelude::yoke::Yokeable, icu_provider::prelude::zerofrom::ZeroFrom)]
            pub struct Foo<'data>;
        ),
    );
}

#[test]
fn test_attributes() {
    // #[data_struct(FooV1, marker(BarV1, "demo/bar@1", fallback_by = "region", extension_kw = "ca"))]
    check(
        quote![
            FooV1,
            marker(
                BarV1,
                "demo/bar@1",
                fallback_by = "region",
                singleton,
                has_checksum
            ),
        ],
        quote!(
            pub struct Foo<'data>;
        ),
        quote!(
            #[doc = "Marker type for [`Foo`]"]
            pub struct FooV1;
            impl icu_provider::DynamicDataMarker for FooV1 {
                type DataStruct = Foo<'static>;
            }
            #[doc = "Marker type for [`Foo`]: \"demo/bar@1\"\n\n- Fallback priority: region"]
            pub struct BarV1;
            impl icu_provider::DynamicDataMarker for BarV1 {
                type DataStruct = Foo<'static>;
            }
            impl icu_provider::DataMarker for BarV1 {
                const INFO: icu_provider::DataMarkerInfo = {
                    let mut info = icu_provider::DataMarkerInfo::from_id(icu_provider::marker::data_marker_id!(BarV1));
                    info.is_singleton = true;
                    info.fallback_config.priority = icu_provider::fallback::LocaleFallbackPriority::Region;
                    info.has_checksum = true;
                    info
                };
            }
            #[derive(icu_provider::prelude::yoke::Yokeable, icu_provider::prelude::zerofrom::ZeroFrom)]
            pub struct Foo<'data>;
        ),
    );
}
