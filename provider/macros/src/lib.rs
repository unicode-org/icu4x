// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
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
/// - `Apply #[derive(Yokeable, ZeroCopyFrom)]`. The `ZeroCopyFrom` derive can
///    be customized with `#[yoke(cloning_zcf)]` as needed
/// - Create a `FooMarker` struct for the type
/// - Implement `icu_provider::DataMarker` for `FooMarker`
///
/// In addition, the attribute can be used to implement `ResourceMarker` by
/// adding key strings, optionally with marker symbols:
///
/// ```
/// use icu_provider::prelude::*;
/// use std::borrow::Cow;
///
/// // We also need `yoke` in scope: <https://github.com/unicode-org/icu4x/issues/1557>
/// use icu_provider::yoke;
///
/// #[icu_provider::data_struct(
///     "demo/foo@1",
///     BarV1Marker = "demo/bar@1",
///     BazV1Marker = "demo/baz@1"
/// )]
/// pub struct FooV1<'data> {
///     message: Cow<'data, str>,
/// };
///
/// assert_eq!(FooV1Marker::KEY.get_path(), "demo/foo@1");
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
    let marker = Ident::new(&format!("{}Marker", name), Span::call_site());

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

    let mut attr_it = attr.into_iter().peekable();

    let mut result = if let Some(NestedMeta::Lit(key_lit)) =
        attr_it.next_if(|v| matches!(v, NestedMeta::Lit(_)))
    {
        let key_str = match &key_lit {
            syn::Lit::Str(lit_str) => lit_str.value(),
            _ => panic!("Key must be a string"),
        };
        let docs = format!("Marker type for [`{}`]: \"{}\"", name, key_str);
        quote!(
            #[doc = #docs]
            pub struct #marker;
            impl icu_provider::DataMarker for #marker {
                type Yokeable = #name_with_lt;
            }
            impl icu_provider::ResourceMarker for #marker {
                const KEY: icu_provider::ResourceKey = icu_provider::resource_key!(#key_lit);
            }
        )
    } else {
        let docs = format!("Marker type for [`{}`]", name);
        quote!(
            #[doc = #docs]
            pub struct #marker;
            impl icu_provider::DataMarker for #marker {
                type Yokeable = #name_with_lt;
            }
        )
    };

    for single_attr in attr_it {
        match single_attr {
            NestedMeta::Meta(Meta::NameValue(name_value)) => {
                let extra_marker = &name_value.path;
                let key_lit = &name_value.lit;
                let key_str = match key_lit {
                    syn::Lit::Str(lit_str) => lit_str.value(),
                    _ => panic!("Key must be a string"),
                };
                let docs = format!("Marker type for [`{}`]: \"{}\"", name, key_str);
                result.extend(quote!(
                    #[doc = #docs]
                    pub struct #extra_marker;
                    impl icu_provider::DataMarker for #extra_marker {
                        type Yokeable = #name_with_lt;
                    }
                    impl icu_provider::ResourceMarker for #extra_marker {
                        const KEY: icu_provider::ResourceKey = icu_provider::resource_key!(#key_lit);
                    }
                ));
            }
            NestedMeta::Lit(_) => {
                panic!("Single-string key must appear first in attribute list")
            }
            NestedMeta::Meta(_) => {
                panic!("Invalid attribute to #[data_struct]")
            }
        }
    }

    result.extend(quote!(
        #[derive(yoke::Yokeable, yoke::ZeroCopyFrom)]
        #item
    ));

    result
}
