// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom derives for `Yokeable` and `ZeroCopyFrom` from the `yoke` crate.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Ident, Lifetime, Type};
use synstructure::Structure;

/// Custom derive for `yoke::Yokeable`,
///
/// If this fails to compile for lifetime issues, it means that
/// the lifetime is not covariant and `Yokeable` is not
///
/// Note that right now this will fail to compile on structs involving
/// `zerovec::ZeroMap`.
/// Please comment on https://github.com/unicode-org/icu4x/issues/844
/// if you need this
#[proc_macro_derive(Yokeable)]
pub fn yokeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(yokeable_derive_impl(&input))
}

fn yokeable_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let typarams = input.generics.type_params().count();
    if typarams != 0 {
        return syn::Error::new(
            input.generics.span(),
            "derive(Yokeable) does not support type parameters",
        )
        .to_compile_error();
    }
    let lts = input.generics.lifetimes().count();
    if lts == 0 {
        let name = &input.ident;
        quote! {
            unsafe impl<'a> yoke::Yokeable<'a> for #name {
                type Output = Self;
                fn transform(&self) -> &Self::Output {
                    self
                }
                fn transform_owned(self) -> Self::Output {
                    self
                }
                unsafe fn make(this: Self::Output) -> Self {
                    this
                }
                fn transform_mut<F>(&'a mut self, f: F)
                where
                    F: 'static + for<'b> FnOnce(&'b mut Self::Output) {
                    f(self)
                }
            }
        }
    } else {
        if lts != 1 {
            return syn::Error::new(
                input.generics.span(),
                "derive(Yokeable) cannot have multiple lifetime parameters",
            )
            .to_compile_error();
        }
        let name = &input.ident;
        quote! {
            // This is safe because as long as `transform()` compiles,
            // we can be sure that `'a` is a covariant lifetime on `Self`
            //
            // This will not work for structs involving ZeroMap since
            // the compiler does not know that ZeroMap is covariant.
            //
            // This custom derive can be improved to handle this case when
            // necessary
            unsafe impl<'a> yoke::Yokeable<'a> for #name<'static> {
                type Output = #name<'a>;
                fn transform(&'a self) -> &'a Self::Output {
                    self
                }
                fn transform_owned(self) -> Self::Output {
                    self
                }
                unsafe fn make(this: Self::Output) -> Self {
                    use std::{mem, ptr};
                    // unfortunately Rust doesn't think `mem::transmute` is possible since it's not sure the sizes
                    // are the same
                    debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
                    let ptr: *const Self = (&this as *const Self::Output).cast();
                    mem::forget(this);
                    ptr::read(ptr)
                }
                fn transform_mut<F>(&'a mut self, f: F)
                where
                    F: 'static + for<'b> FnOnce(&'b mut Self::Output) {
                    unsafe { f(std::mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
                }
            }
        }
    }
}

/// Custom derive for `yoke::ZeroCopyFrom`,
///
/// This implements `ZeroCopyFrom<Ty> for Ty` for types
/// without a lifetime parameter, and `ZeroCopyFrom<Ty<'s>> for Ty<'static>`
/// for types with a lifetime parameter.
///
/// Apply the `#[yoke(cloning_zcf)]` attribute if you wish for this custom derive
/// to use `.clone()` for its implementation.
#[proc_macro_derive(ZeroCopyFrom, attributes(yoke))]
pub fn zcf_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(zcf_derive_impl(&input))
}

fn zcf_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let typarams = input.generics.type_params().count();
    if typarams != 0 {
        return syn::Error::new(
            input.generics.span(),
            "derive(ZeroCopyFrom) does not support type parameters",
        )
        .to_compile_error();
    }
    let has_clone = input.attrs.iter().any(|a| {
        if let Ok(i) = a.parse_args::<Ident>() {
            if i == "cloning_zcf" {
                return true;
            }
        }
        false
    });
    let lts = input.generics.lifetimes().count();
    let name = &input.ident;
    if lts == 0 {
        let clone = if has_clone {
            quote!(this.clone())
        } else {
            quote!(*this)
        };
        quote! {
            impl ZeroCopyFrom<#name> for #name {
                fn zero_copy_from(this: &Self) -> Self {
                    #clone
                }
            }
        }
    } else {
        if lts != 1 {
            return syn::Error::new(
                input.generics.span(),
                "derive(ZeroCopyFrom) cannot have multiple lifetime parameters",
            )
            .to_compile_error();
        }
        if has_clone {
            return quote! {
                impl<'s> ZeroCopyFrom<#name<'static>> for #name<'static> {
                    fn zero_copy_from(this: &Self) -> Self {
                        this.clone()
                    }
                }
            };
        }

        let structure = Structure::new(&input);
        let body = structure.each_variant(|vi| {
            vi.construct(|f, i| {
                let binding = format!("__binding_{}", i);
                let field = Ident::new(&binding, Span::call_site());
                let fty = replace_lifetime(&f.ty, static_lt());
                quote! {
                    <#fty as yoke::ZeroCopyFrom<_>>::zero_copy_from(#field)
                }
            })
        });

        quote! {
            impl<'s> yoke::ZeroCopyFrom<#name<'s>> for #name<'static> {
                fn zero_copy_from<'b>(this: &'b #name<'s>) -> #name<'b> {
                    match *this { #body }
                }
            }
        }
    }
}

fn static_lt() -> Lifetime {
    Lifetime::new("'static", Span::call_site())
}

fn replace_lifetime(x: &Type, lt: Lifetime) -> Type {
    match *x {
        Type::Group(ref inner) => {
            let mut inner = inner.clone();
            inner.elem = Box::new(replace_lifetime(&inner.elem, lt));
            Type::Group(inner)
        }
        Type::Array(ref inner) => {
            let mut inner = inner.clone();
            inner.elem = Box::new(replace_lifetime(&inner.elem, lt));
            Type::Array(inner)
        }
        Type::Paren(ref inner) => {
            let mut inner = inner.clone();
            inner.elem = Box::new(replace_lifetime(&inner.elem, lt));
            Type::Paren(inner)
        }
        Type::Reference(ref inner) => {
            let mut inner = inner.clone();
            inner.elem = Box::new(replace_lifetime(&inner.elem, lt.clone()));
            inner.lifetime = inner.lifetime.as_ref().map(|_| lt);
            Type::Reference(inner)
        }
        Type::Path(ref path) => {
            let mut path = path.clone();
            for segment in path.path.segments.iter_mut() {
                if let syn::PathArguments::AngleBracketed(ref mut a) = segment.arguments {
                    for arg in a.args.iter_mut() {
                        match arg {
                            syn::GenericArgument::Lifetime(ref mut l) => *l = lt.clone(),
                            syn::GenericArgument::Type(ref mut t) => {
                                *t = replace_lifetime(t, lt.clone())
                            }
                            _ => (),
                        }
                    }
                }
            }
            Type::Path(path)
        }
        ref other => other.clone(),
    }
}
