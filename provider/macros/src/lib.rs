extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::ItemStruct;

#[proc_macro_attribute]
/// The `#[data_struct]` attribute should be applied to all types intended
/// for use in a `DataStruct`. It does the following things:
///
/// - `Apply #[derive(Yokeable, ZeroCopyFrom)]`. The `ZeroCopyFrom` derive can
///    be customized with `#[yoke(cloning_zcf)]` as needed
/// - Create a `FooMarker` struct for the type
/// - Implement `icu_provider::DataMarker` for `FooMarker`
pub fn data_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);

    TokenStream::from(data_struct_impl(item))
}

fn data_struct_impl(item: ItemStruct) -> TokenStream2 {
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

    if let Some(lt) = lifetimes.get(0) {
        quote!(
            #[doc = #docs]
            pub struct #marker;

            impl<#lt> icu_provider::DataMarker<#lt> for #marker {
                type Yokeable = #name<'static>;
                type Cart = #name<#lt>;
            }

            #[derive(Yokeable, ZeroCopyFrom)]
            #item
        )
    } else {
        quote!(
            #[doc = #docs]
            pub struct #marker;

            impl<'s> icu_provider::DataMarker<'s> for #marker {
                type Yokeable = #name;
                type Cart = #name;
            }

            #[derive(Yokeable, ZeroCopyFrom)]
            #item
        )
    }
}
