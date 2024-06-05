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
        .map(|((l, a), i)| {
            (
                DataRequest {
                    locale: &l,
                    marker_attributes: &a,
                    ..Default::default()
                }
                .legacy_encode(),
                quote!(#i),
            )
        })
        .collect::<Vec<_>>();

    data.sort_by(|(a, _), (b, _)| a.cmp(b));

    let n = data.len();
    let data = data.iter().map(|(r, i)| quote!((#r, &#i)));

    quote! {
        static DATA: [(&str, & #struct_type); #n] = [#(#data,)*];
        fn lookup(req: icu_provider::DataRequest) -> Option<&'static #struct_type> {
            DATA.binary_search_by(|(k, _)| req.legacy_cmp(k.as_bytes()).reverse())
                .map(|i| (*unsafe { DATA.get_unchecked(i) }).1)
                .ok()
        }
    }
}
