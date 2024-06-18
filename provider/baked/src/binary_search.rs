// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data stored as slices, looked up with binary search

#[cfg(feature = "export")]
use databake::*;
#[cfg(feature = "export")]
use icu_provider::prelude::*;

#[cfg(feature = "export")]
pub fn bake(
    struct_type: &TokenStream,
    data: impl IntoIterator<Item = ((DataLocale, DataMarkerAttributes), proc_macro2::Ident)>,
) -> TokenStream {
    let mut data = data
        .into_iter()
        .map(|((l, a), i)| ((a.to_string(), l.to_string()), quote!(#i)))
        .collect::<Vec<_>>();

    data.sort_by(|(a, _), (b, _)| a.cmp(b));

    let n = data.len();

    if data.iter().all(|((a, _), _)| a.is_empty()) {
        // Only DataLocales
        let data = data.iter().map(|((_, l), i)| quote!((#l, &#i)));

        quote! {
            static DATA: [(&str, & #struct_type); #n] = [#(#data,)*];
            fn lookup(req: icu_provider::DataRequest) -> Option<&'static #struct_type> {
                DATA.binary_search_by(|(l, _)| req.locale.strict_cmp(l.as_bytes()).reverse())
                    .map(|i| (*unsafe { DATA.get_unchecked(i) }).1)
                    .ok()
            }
        }
    } else if data.iter().all(|((_, l), _)| *l == "und") {
        // Only marker attributes
        let data = data.iter().map(|((a, _), i)| quote!((#a, &#i)));

        quote! {
            static DATA: [(&str, & #struct_type); #n] = [#(#data,)*];
            fn lookup(req: icu_provider::DataRequest) -> Option<&'static #struct_type> {
                DATA.binary_search_by(|(a, _)| a.cmp(&&**req.marker_attributes))
                    .map(|i| (*unsafe { DATA.get_unchecked(i) }).1)
                    .ok()
            }
        }
    } else {
        let data = data.iter().map(|((a, l), i)| quote!(((#a, #l), &#i)));

        quote! {
            static DATA: [((&str, &str), & #struct_type); #n] = [#(#data,)*];
            fn lookup(req: icu_provider::DataRequest) -> Option<&'static #struct_type> {
                DATA.binary_search_by(|((a, l), _)| a.cmp(&&**req.marker_attributes).then_with(|| req.locale.strict_cmp(l.as_bytes()).reverse()))
                    .map(|i| (*unsafe { DATA.get_unchecked(i) }).1)
                    .ok()
            }
        }
    }
}
