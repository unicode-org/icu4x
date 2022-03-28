// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic
    )
)]

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::AttributeArgs;
use syn::ItemStruct;
use syn::Meta;
use syn::NestedMeta;

#[cfg(test)]
mod tests;

#[proc_macro_attribute]
/// The `#[data_struct]` attribute should be applied to all types intended
/// for use in a `DataStruct`. It does the following things:
///
/// - `Apply #[derive(Yokeable, ZeroFrom)]`. The `ZeroFrom` derive can
///    be customized with `#[zerofrom(clone)]` on non-ZeroFrom fields.
///
/// In addition, the attribute can be used to implement `DataMarker` and/or `ResourceMarker`
/// by adding symbols with optional key strings:
///
/// ```
/// use icu_provider::prelude::*;
/// use std::borrow::Cow;
///
/// #[icu_provider::data_struct(
///     FooV1Marker,
///     BarV1Marker = "demo/bar@1",
///     BazV1Marker = "demo/baz@1"
/// )]
/// pub struct FooV1<'data> {
///     message: Cow<'data, str>,
/// };
///
/// // Note: FooV1Marker implements `DataMarker` but not `ResourceMarker`.
/// // The other two implement `ResourceMarker`.
///
/// assert_eq!(BarV1Marker::KEY.get_path(), "demo/bar@1");
/// assert_eq!(BazV1Marker::KEY.get_path(), "demo/baz@1");
/// ```
pub fn data_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemStruct);

    TokenStream::from(data_struct_impl(attr, item))
}

fn data_struct_impl(attr: AttributeArgs, item: ItemStruct) -> TokenStream2 {
    if item.generics.type_params().count() > 0 {
        return syn::Error::new(
            item.generics.span(),
            "#[data_struct] does not support type parameters",
        )
        .to_compile_error();
    }
    let lifetimes = item.generics.lifetimes().collect::<Vec<_>>();

    let name = &item.ident;

    let name_with_lt = if lifetimes.get(0).is_some() {
        quote!(#name<'static>)
    } else {
        quote!(#name)
    };

    if lifetimes.len() > 1 {
        return syn::Error::new(
            item.generics.span(),
            "#[data_struct] does not support more than one lifetime parameter",
        )
        .to_compile_error();
    }

    let mut result = TokenStream2::new();

    for single_attr in attr.into_iter() {
        match single_attr {
            NestedMeta::Meta(Meta::NameValue(name_value)) => {
                let marker_name = &name_value.path;
                let key_lit = &name_value.lit;
                let key_str = match key_lit {
                    syn::Lit::Str(lit_str) => lit_str.value(),
                    #[allow(clippy::panic)]
                    // TODO(#1668) Clippy exceptions need docs or fixing.
                    _ => panic!("Key must be a string"),
                };
                let docs = format!("Marker type for [`{}`]: \"{}\"", name, key_str);
                result.extend(quote!(
                    #[doc = #docs]
                    pub struct #marker_name;
                    impl icu_provider::DataMarker for #marker_name {
                        type Yokeable = #name_with_lt;
                    }
                    impl icu_provider::ResourceMarker for #marker_name {
                        const KEY: icu_provider::ResourceKey = icu_provider::resource_key!(#key_lit);
                    }
                ));
            }
            NestedMeta::Meta(Meta::Path(marker_name)) => {
                let docs = format!("Marker type for [`{}`]", name);
                result.extend(quote!(
                    #[doc = #docs]
                    pub struct #marker_name;
                    impl icu_provider::DataMarker for #marker_name {
                        type Yokeable = #name_with_lt;
                    }
                ));
            }
            #[allow(clippy::panic)] // TODO(#1668) Clippy exceptions need docs or fixing.
            _ => {
                panic!("Invalid attribute to #[data_struct]")
            }
        }
    }

    result.extend(quote!(
        #[derive(yoke::Yokeable, zerofrom::ZeroFrom)]
        #item
    ));

    result
}
