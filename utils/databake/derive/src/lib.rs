// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom derives for `Bake`

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

/// This custom derive auto-implements the `Bake` trait on any given type that has public
/// fields that also implement `Bake`.
///
/// For a type `Person` defined in the module `module` of crate `bar`, this derive
/// can be used as follows:
///
/// ```rust
/// use databake::Bake;
///
/// #[derive(Bake)]
/// #[databake(path = bar::module)]
/// pub struct Person<'a> {
///     pub name: &'a str,
///     pub age: u32,
/// }
/// ```
#[proc_macro_derive(Bake, attributes(databake))]
pub fn bake_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(bake_derive_impl(&input))
}

fn bake_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let mut structure = Structure::new(input);

    struct PathAttr(Punctuated<PathSegment, Token![::]>);

    impl Parse for PathAttr {
        fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
            let i: Ident = input.parse()?;
            if i != "path" {
                return Err(input.error(format!("expected token \"path\", found {i:?}")));
            }
            input.parse::<Token![=]>()?;
            Ok(Self(input.parse::<Path>()?.segments))
        }
    }

    let path = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("databake"))
        .expect("missing databake(path = ...) attribute")
        .parse_args::<PathAttr>()
        .unwrap()
        .0;

    let body = structure.each_variant(|vi| {
        let recursive_bakes = vi.bindings().iter().map(|b| {
            let ident = b.binding.clone();
            quote! { let #ident =  #ident.bake(env); }
        });

        let constructor = vi.construct(|_, i| {
            let ident = &vi.bindings()[i].binding;
            quote! { # #ident }
        });

        quote! {
            #(#recursive_bakes)*
            databake::quote! { #path::#constructor }
        }
    });

    structure.add_bounds(AddBounds::Fields);

    let crate_name = path.iter().next().unwrap();
    let crate_name = quote!(#crate_name).to_string();

    structure.gen_impl(quote! {
        gen impl databake::Bake for @Self {
            fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                env.insert(#crate_name);
                match self {
                    #body
                }
            }
        }
    })
}
