// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom derives for `Bakeable`

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    DeriveInput, Ident, Path, PathSegment, Token,
};
use synstructure::{AddBounds, Structure};

#[proc_macro_derive(Bakeable, attributes(crabbake))]
pub fn bakeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(bakeable_derive_impl(&input))
}

fn bakeable_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let mut structure = Structure::new(input);

    struct PathAttr(Punctuated<PathSegment, Token![::]>);

    impl Parse for PathAttr {
        fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
            let i: Ident = input.parse()?;
            if i != "path" {
                return Err(input.error(format!("expected token \"path\", found {:?}", i)));
            }
            input.parse::<Token![=]>()?;
            Ok(Self(input.parse::<Path>()?.segments))
        }
    }

    let path = input
        .attrs
        .iter()
        .find(|a| a.path.is_ident("crabbake"))
        .expect("missing crabbake(path = ...) attribute")
        .parse_args::<PathAttr>()
        .unwrap()
        .0;

    let body = structure.each_variant(|vi| {
        let recursive_bakes = vi.bindings().iter().map(|b| {
            let ident = b.binding.clone();
            quote! { let #ident =  #ident.bake(ctx); }
        });

        let constructor = vi.construct(|_, i| {
            let ident = &vi.bindings()[i].binding;
            quote! { # #ident }
        });

        quote! {
            #(#recursive_bakes)*
            crabbake::quote! { ::#path::#constructor }
        }
    });

    structure.add_bounds(AddBounds::Fields);

    let crate_name = path.iter().next().unwrap();
    let crate_name = quote!(#crate_name).to_string();

    structure.gen_impl(quote! {
        gen impl crabbake::Bakeable for @Self {
            fn bake(&self, ctx: &crabbake::CrateEnv) -> crabbake::TokenStream {
                ctx.insert(#crate_name);
                match self {
                    #body
                }
            }
        }
    })
}
