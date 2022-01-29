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

    if lifetimes.len() > 1 {
        return syn::Error::new(
            item.generics.span(),
            "#[data_struct] does not support more than one lifetime parameter",
        )
        .to_compile_error();
    }

    let docs = format!("Marker type for [`{}`]", name);

    let mut result = if lifetimes.get(0).is_some() {
        quote!(
            #[doc = #docs]
            pub struct #marker;

            impl icu_provider::DataMarker for #marker {
                type Yokeable = #name<'static>;
            }

            #[derive(Yokeable, ZeroCopyFrom)]
            #item
        )
    } else {
        quote!(
            #[doc = #docs]
            pub struct #marker;

            impl icu_provider::DataMarker for #marker {
                type Yokeable = #name;
            }

            #[derive(Yokeable, ZeroCopyFrom)]
            #item
        )
    };

    if let Some(attr0) = attr.get(0) {
        if let NestedMeta::Lit(key_str) = attr0 {
            result.extend(quote!(
                impl icu_provider::ResourceMarker for #marker {
                    const KEY: icu_provider::ResourceKey = icu_provider::resource_key!(#key_str);
                }
            ));
        }
    }

    result
}
