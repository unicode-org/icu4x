// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom derives for `Bake`

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
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
///
/// # Custom baked type
///
/// To bake to a different type than this, use `custom_bake`
/// and implement `CustomBake`.
///
/// ```rust
/// use databake::Bake;
/// use databake::CustomBake;
///
/// #[derive(Bake)]
/// #[databake(path = bar::module)]
/// #[databake(path = custom_bake)]
/// pub struct Message<'a> {
///     pub message: &'a str,
/// }
///
/// // Bake to a string:
/// impl CustomBake for Message<'_> {
///     type BakedType<'a> = &'a str where Self: 'a;
///     fn to_custom_bake(&self) -> Self::BakedType<'_> {
///         &self.message
///     }
/// }
///
/// impl<'a> Message<'a> {
///     pub fn from_custom_bake(message: &'a str) -> Self {
///         Self { message }
///     }
/// }
/// ```
///
/// If the constructor is unsafe, use `custom_bake_unsafe`
/// and implement `CustomBakeUnsafe`.
#[proc_macro_derive(Bake, attributes(databake))]
pub fn bake_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(bake_derive_impl(&input))
}

fn bake_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let mut structure = Structure::new(input);

    enum DatabakeAttr {
        Path(Punctuated<PathSegment, Token![::]>),
        CustomBake,
        CustomBakeUnsafe,
    }

    impl Parse for DatabakeAttr {
        fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
            let i: Ident = input.parse()?;
            if i == "path" {
                input.parse::<Token![=]>()?;
                Ok(Self::Path(input.parse::<Path>()?.segments))
            } else if i == "custom_bake" {
                Ok(Self::CustomBake)
            } else if i == "custom_bake_unsafe" {
                Ok(Self::CustomBakeUnsafe)
            } else {
                Err(input.error(format!("expected token \"path\", found {i:?}")))
            }
        }
    }

    let attrs = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("databake"))
        .map(|a| a.parse_args::<DatabakeAttr>().unwrap())
        .collect::<Vec<_>>();

    let path = attrs
        .iter()
        .filter_map(|a| match a {
            DatabakeAttr::Path(path) => Some(path),
            _ => None,
        })
        .next()
        .expect("missing databake(path = ...) attribute");

    let is_custom_bake = attrs
        .iter()
        .find(|a| matches!(a, DatabakeAttr::CustomBake))
        .is_some();

    let is_custom_bake_unsafe = attrs
        .iter()
        .find(|a| matches!(a, DatabakeAttr::CustomBakeUnsafe))
        .is_some();

    let bake_body = if is_custom_bake || is_custom_bake_unsafe {
        let type_ident = &structure.ast().ident;
        let baked_ident = format_ident!("baked");
        if is_custom_bake_unsafe {
            quote! {
                x => {
                    let baked = databake::CustomBakeUnsafe::to_custom_bake(x).bake(env);
                    databake::quote! {
                        // Safety: the bake is generated from `CustomBakeUnsafe::to_custom_bake`
                        unsafe { #path::#type_ident::from_custom_bake(##baked_ident) }
                    }
                }
            }
        } else {
            quote! {
                x => {
                    let baked = databake::CustomBake::to_custom_bake(x).bake(env);
                    databake::quote! { #path::#type_ident::from_custom_bake(##baked_ident) }
                }
            }
        }
    } else {
        structure.each_variant(|vi| {
            let recursive_calls = vi.bindings().iter().map(|b| {
                let ident = b.binding.clone();
                quote! { let #ident =  #ident.bake(env); }
            });

            let constructor = vi.construct(|_, i| {
                let ident = &vi.bindings()[i].binding;
                quote! { # #ident }
            });

            quote! {
                #(#recursive_calls)*
                databake::quote! { #path::#constructor }
            }
        })
    };

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
