// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
    )
)]
#![warn(missing_docs)]

//! Creates private functions that cast a `repr(transparent)` struct from its
//! inner type to its outer type.
//!
//! The generated functions are:
//!
//! - `fn tcast_ref(&Inner) -> &Outer`
//! - If `tcast(alloc)` or `tcast(std)` is specified:
//!     - `fn tcast_box(Box<Inner>) -> Box<Outer>`
//!
//! The generated functions are always private. This is to ensure external
//! users cannot violate any invariants that the outer type imposes.
//!
//! You should add your own public functions that call the generated private
//! functions after checking for invariants.
//!
//! # As a Dev-Dependency
//!
//! The primary purpose of this derive is to check for invariants and reduce
//! the amount of unsafe code in your crate.
//!
//! You can avoid depending on this crate at runtime (and the transitive
//! dependency on `syn`) by adding this crate as a dev-dependency and adding
//! a function with the same signature gated on `#[cfg(not(test))]`:
//!
//! ```
//! #[cfg_attr(test, derive(TransparentCast))]
//! #[repr(transparent)]
//! pub struct Wrap<T>(T);
//!
//! impl<T> Wrap<T> {
//!     #[cfg(not(test))]
//!     fn tcast_ref(inner: &T) -> &Self {
//!         // Safety: the tcast crate guarantees that this is safe
//!         unsafe { core::mem::transmute(inner) }
//!     }
//! }
//! ```
//!
//! # Examples
//!
//! A struct with invariants:
//!
//! ```
//! mod even {
//!     use tcast::TransparentCast;
//!
//!     #[derive(TransparentCast, Debug, PartialEq)]
//!     #[repr(transparent)]
//!     pub struct Even(u32);
//!
//!     impl Even {
//!         pub fn from_ref(input: &u32) -> Option<&Self> {
//!             if input % 2 == 0 {
//!                 Some(Self::tcast_ref(input))
//!             } else {
//!                 None
//!             }
//!         }
//!     }
//! }
//!
//! assert!(even::Even::from_ref(&32).is_some());
//! assert!(even::Even::from_ref(&33).is_none());
//! ```
//!
//! A more complex struct:
//!
//! ```
//! use core::marker::PhantomData;
//! use tcast::TransparentCast;
//!
//! #[derive(TransparentCast, Debug, PartialEq)]
//! #[tcast(std)]
//! #[repr(transparent)]
//! struct WithMarker<'a, T> {
//!     value: &'a str,
//!     _marker: PhantomData<T>,
//! }
//!
//! #[derive(Debug, PartialEq)]
//! struct MarkerType;
//!
//! assert_eq!(
//!     WithMarker::<MarkerType>::tcast_box(Box::new("hello")),
//!     Box::new(WithMarker::<MarkerType> {
//!         value: "hello",
//!         _marker: PhantomData,
//!     })
//! );
//! ```
//!
//! A dynamically-sized type (DST):
//!
//! ```
//! use core::marker::PhantomData;
//! use tcast::TransparentCast;
//!
//! #[derive(TransparentCast)]
//! #[tcast(std)]
//! #[repr(transparent)]
//! struct DynamicallySized {
//!     value: str,
//! }
//!
//! assert_eq!(
//!     DynamicallySized::tcast_ref(&"hello").value,
//!     *"hello"
//! );
//! ```
//!
//! # Incorrect Usage
//!
//! The struct must be repr(transparent):
//!
//! ```compile_fail
//! use tcast::TransparentCast;
//!
//! #[derive(TransparentCast)]
//! struct NotTransparent(u8);
//! ```
//!
//! The struct must have at least one field:
//!
//! ```compile_fail
//! use tcast::TransparentCast;
//!
//! #[derive(TransparentCast)]
//! #[repr(transparent)]
//! struct UnitStruct;
//! ```
//!
//! The struct must not have multiple non-zero-sized fields:
//!
//! ```compile_fail
//! #[repr(transparent)]
//! struct MultipleNonZeroFields(u8, u8, u8);
//! ```
//!
//! The struct can be annotated with tcast(alloc) or tcast(std) but not both:
//!
//! ```compile_fail
//! use tcast::TransparentCast;
//!
//! #[derive(TransparentCast)]
//! #[tcast(std)]
//! #[tcast(alloc)]
//! #[repr(transparent)]
//! struct TooManyAttributes(u8);
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DeriveInput, FieldsNamed, FieldsUnnamed,
    Meta, Token,
};

/// Derive macro that adds private transparent cast functions to a type.
///
/// See the crate-level docs for more details.
#[proc_macro_derive(TransparentCast, attributes(tcast))]
pub fn tcast_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(zf_derive_impl(&input))
}

fn zf_derive_impl(input: &DeriveInput) -> TokenStream2 {
    if !has_repr_transparent(&input.attrs) {
        return syn::Error::new_spanned(input, "TransparentCast requires #[repr(transparent)]")
            .to_compile_error();
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let transparent_ty = match transparent_over_type(input) {
        Ok(ty) => ty,
        Err(err) => return err.to_compile_error(),
    };

    let tcast_ref_impl = quote! {
        /// Casts an inner type reference to an outer type reference.
        #[allow(dead_code)]
        fn tcast_ref<'tcast>(inner: &'tcast #transparent_ty) -> &'tcast #ident #ty_generics {
            // Safety: As verified above, the input type is transparent over transparent_ty.
            // This is a private function. The caller is responsible for upholding any potential field invariants.
            unsafe { core::mem::transmute(inner) }
        }
    };

    let tcast_box_impl = match tcast_box_mode(&input.attrs) {
        Ok(Some(TcastBoxMode::Alloc)) => quote! {
            /// Casts an inner type box to an outer type box.
            #[allow(dead_code)]
            fn tcast_box(
                inner: alloc::boxed::Box<#transparent_ty>,
            ) -> alloc::boxed::Box<#ident #ty_generics> {
                // Safety: As verified above, the input type is transparent over transparent_ty.
                // This is a private function. The caller is responsible for upholding any potential field invariants.
                unsafe { core::mem::transmute(inner) }
            }
        },
        Ok(Some(TcastBoxMode::Std)) => quote! {
            /// Casts an inner type box to an outer type box.
            #[allow(dead_code)]
            fn tcast_box(
                inner: std::boxed::Box<#transparent_ty>,
            ) -> std::boxed::Box<#ident #ty_generics> {
                // Safety: As verified above, the input type is transparent over transparent_ty.
                // This is a private function. The caller is responsible for upholding any potential field invariants.
                unsafe { core::mem::transmute(inner) }
            }
        },
        Ok(None) => TokenStream2::new(),
        Err(err) => return err.to_compile_error(),
    };

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #tcast_ref_impl
            #tcast_box_impl
        }
    }
}

/// Returns whether one of the given attributes is repr(transparent).
fn has_repr_transparent(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }

        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        if let Ok(metas) = attr.parse_args_with(parser) {
            for meta in metas {
                if let Meta::Path(path) = meta {
                    if path.is_ident("transparent") {
                        return true;
                    }
                }
            }
        }
    }

    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TcastBoxMode {
    Alloc,
    Std,
}

/// Returns the requested tcast box mode, if any.
fn tcast_box_mode(attrs: &[Attribute]) -> Result<Option<TcastBoxMode>, syn::Error> {
    let mut mode = None;
    for attr in attrs {
        if !attr.path().is_ident("tcast") {
            continue;
        }

        let parser = Punctuated::<Meta, Token![,]>::parse_terminated;
        if let Ok(metas) = attr.parse_args_with(parser) {
            for meta in metas {
                if let Meta::Path(path) = meta {
                    let new_mode = if path.is_ident("alloc") {
                        TcastBoxMode::Alloc
                    } else if path.is_ident("std") {
                        TcastBoxMode::Std
                    } else {
                        continue;
                    };
                    if mode.is_some() {
                        return Err(syn::Error::new_spanned(
                            attr,
                            "TransparentCast cannot combine tcast(alloc) and tcast(std)",
                        ));
                    }
                    mode = Some(new_mode);
                }
            }
        }
    }

    Ok(mode)
}

/// Returns the inner type of a transparent struct.
fn transparent_over_type(input: &DeriveInput) -> Result<&syn::Type, syn::Error> {
    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(FieldsNamed { named: fields, .. })
            | syn::Fields::Unnamed(FieldsUnnamed {
                unnamed: fields, ..
            }) => {
                let mut non_zero_fields = fields
                    .into_iter()
                    .filter(|field| !is_zero_sized_field(&field.ty));

                match (non_zero_fields.next(), non_zero_fields.next()) {
                    (Some(first), None) => Ok(&first.ty),
                    _ => Err(syn::Error::new_spanned(
                        &data.fields,
                        "TransparentCast requires exactly one non-zero-sized field",
                    )),
                }
            }
            syn::Fields::Unit => Err(syn::Error::new_spanned(
                &data.fields,
                "TransparentCast requires at least one non-zero-sized field",
            )),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => Err(syn::Error::new_spanned(
            input,
            "TransparentCast only supports structs",
        )),
    }
}

/// Returns whether the given field is zero-sized.
///
/// Currently this just checks if the field is `PhantomData`.
/// This should be improved in the future.
fn is_zero_sized_field(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(ident) = type_path.path.get_ident() {
            return ident == "PhantomData";
        }

        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "PhantomData" {
                return true;
            }
        }
    }

    false
}
