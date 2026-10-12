// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
// #![cfg_attr(
//     not(test),
//     deny(
//         clippy::indexing_slicing,
//         clippy::unwrap_used,
//         clippy::expect_used,
//         clippy::panic,
//     )
// )]
#![warn(missing_docs)]

//! Custom derives for `Bake`

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Data, DeriveInput, Ident, Path, PathSegment, Token, Visibility,
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
    // Baked data is generated in one configuration but compiled in another (e.g. data is
    // generated with `serde` enabled, but the baked data is consumed without it). If a variant
    // only exists in some configurations, the baked code might refer to a non-existent variant.
    if let Data::Enum(e) = &input.data {
        if let Some(attr) = e
            .variants
            .iter()
            .flat_map(|v| &v.attrs)
            .find(|a| a.path().is_ident("cfg"))
        {
            return syn::Error::new_spanned(
                attr,
                "deriving Bake is not supported on enums with #[cfg]-gated variants, \
                as baked data might be compiled in a different configuration. \
                Implement Bake manually instead.",
            )
            .to_compile_error();
        }
    }

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

    let bake_body = structure.each_variant(|vi| {
        let recursive_calls = vi.bindings().iter().map(|b| {
            let ident = b.binding.clone();
            quote! { let #ident =  #ident.bake(env); }
        });

        let constructor = vi.construct(|f, i| {
            assert!(
                f.vis == syn::parse_str::<Visibility>("pub").unwrap()
                    || matches!(input.data, Data::Enum(_)),
                "deriving Bake requires public fields"
            );
            let ident = &vi.bindings()[i].binding;
            quote! { # #ident }
        });

        quote! {
            #(#recursive_calls)*
            databake::quote! { #path::#constructor }
        }
    });

    let borrows_size_body = structure.each_variant(|vi| {
        let recursive_calls = vi.bindings().iter().map(|b| {
            let ident = b.binding.clone();
            quote! { #ident.borrows_size() }
        });

        quote! {
            0 #(+ #recursive_calls)*
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
                    #bake_body
                    &_ => unreachable!() // ZST references aren't uninhabited
                }
            }
        }
        gen impl databake::BakeSize for @Self {
            fn borrows_size(&self) -> usize {
                match self {
                    #borrows_size_body
                    &_ => unreachable!() // ZST references aren't uninhabited
                }
            }
        }
    })
}

#[test]
fn test_reject_cfg_variants() {
    let output = bake_derive_impl(&syn::parse_quote! {
        #[databake(path = test)]
        enum Foo {
            A,
            #[cfg(feature = "serde")]
            B,
        }
    })
    .to_string();
    assert!(output.contains("compile_error"), "{output}");

    // Not rejected without cfg
    let output = bake_derive_impl(&syn::parse_quote! {
        #[databake(path = test)]
        enum Foo {
            A,
            B,
        }
    })
    .to_string();
    assert!(!output.contains("compile_error"), "{output}");
}
