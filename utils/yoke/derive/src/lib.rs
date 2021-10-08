// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom derives for `Yokeable` and `ZeroCopyFrom` from the `yoke` crate.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, Lifetime, Type, WherePredicate};
use synstructure::Structure;

mod visitor;

/// Custom derive for `yoke::Yokeable`,
///
/// If your struct contains `zerovec::ZeroMap`, then the compiler will not
/// be able to guarantee the lifetime covariance due to the generic types on
/// the `ZeroMap` itself. You must add the following attribute in order for
/// the custom derive to work with `ZeroMap`.
///
/// ```rust,ignore
/// #[derive(Yokeable)]
/// #[yoke(manually_prove_covariance)]
/// ```
///
/// Beyond this case, if the derive fails to compile due to lifetime issues, it
/// means that the lifetime is not covariant and `Yokeable` is not safe to implement.
#[proc_macro_derive(Yokeable, attributes(yoke))]
pub fn yokeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(yokeable_derive_impl(&input))
}

fn yokeable_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let tybounds = input.generics.type_params().collect::<Vec<_>>();
    let typarams = tybounds
        .iter()
        .map(|ty| ty.ident.clone())
        .collect::<Vec<_>>();
    // We require all type parameters be 'static, otherwise
    // the Yokeable impl becomes really unweildy to generate safely
    let static_bounds: Vec<WherePredicate> = typarams
        .iter()
        .map(|ty| parse_quote!(#ty: 'static))
        .collect();
    let lts = input.generics.lifetimes().count();
    if lts == 0 {
        let name = &input.ident;
        quote! {
            // This is safe because there are no lifetime parameters.
            unsafe impl<'a, #(#tybounds),*> yoke::Yokeable<'a> for #name<#(#typarams),*> where #(#static_bounds),* {
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
            // This is safe because there are no lifetime parameters.
            unsafe impl<'a, #(#tybounds),*> yoke::IsCovariant<'a> for #name<#(#typarams),*> where #(#static_bounds),* {}
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
        let manual_covariance = input.attrs.iter().any(|a| {
            if let Ok(i) = a.parse_args::<Ident>() {
                if i == "prove_covariance_manually" {
                    return true;
                }
            }
            false
        });
        if manual_covariance {
            let mut structure = Structure::new(input);
            let generics_env = typarams.iter().cloned().collect();
            let static_bounds: Vec<WherePredicate> = typarams
                .iter()
                .map(|ty| parse_quote!(#ty: 'static))
                .collect();
            let mut yoke_bounds: Vec<WherePredicate> = vec![];
            structure.bind_with(|_| synstructure::BindStyle::Move);
            let body = structure.each_variant(|vi| {
                vi.construct(|f, i| {
                    let binding = format!("__binding_{}", i);
                    let field = Ident::new(&binding, Span::call_site());
                    let fty = replace_lifetime(&f.ty, static_lt());

                    let (has_ty, has_lt) = visitor::check_type_for_parameters(&f.ty, &generics_env);
                    if has_ty {
                        // For types without type parameters, the compiler can figure out that the field implements
                        // Yokeable on its own. However, if there are type parameters, there may be complex preconditions
                        // to `FieldTy: Yokeable` that need to be satisfied. We get them to be satisfied by requiring
                        // `FieldTy<'static>: Yokeable<FieldTy<'a>>`
                        if has_lt {
                            let a_ty = replace_lifetime(&f.ty, custom_lt("'a"));
                            yoke_bounds
                                .push(parse_quote!(#fty: yoke::Yokeable<'a, Output = #a_ty>));
                        } else {
                            yoke_bounds.push(parse_quote!(#fty: yoke::Yokeable<'a, Output = #fty>));
                        }
                    }
                    // By calling transform_owned on all fields, we manually prove
                    // that the lifetimes are covariant, since this requirement
                    // must already be true for the type that implements transform_owned().
                    quote! {
                        <#fty as yoke::Yokeable<'a>>::transform_owned(#field)
                    }
                })
            });
            return quote! {
                unsafe impl<'a, #(#tybounds),*> yoke::Yokeable<'a> for #name<'static, #(#typarams),*>
                    where #(#static_bounds,)*
                    #(#yoke_bounds,)* {
                    type Output = #name<'a, #(#typarams),*>;
                    fn transform(&'a self) -> &'a Self::Output {
                        unsafe {
                            // safety: we have asserted covariance in
                            // transform_owned
                            ::core::mem::transmute(self)
                        }
                    }
                    fn transform_owned(self) -> Self::Output {
                        match self { #body }
                    }
                    unsafe fn make(this: Self::Output) -> Self {
                        use core::{mem, ptr};
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
                        unsafe { f(core::mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
                    }
                }
            };
        }
        quote! {
            // This is safe because as long as `transform()` compiles,
            // we can be sure that `'a` is a covariant lifetime on `Self`
            //
            // This will not work for structs involving ZeroMap since
            // the compiler does not know that ZeroMap is covariant.
            //
            // This custom derive can be improved to handle this case when
            // necessary
            unsafe impl<'a, #(#tybounds),*> yoke::Yokeable<'a> for #name<'static, #(#typarams),*> where #(#static_bounds),* {
                type Output = #name<'a, #(#typarams),*>;
                fn transform(&'a self) -> &'a Self::Output {
                    self
                }
                fn transform_owned(self) -> Self::Output {
                    self
                }
                unsafe fn make(this: Self::Output) -> Self {
                    use core::{mem, ptr};
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
                    unsafe { f(core::mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
                }
            }
            // This is safe because it is in the same block as the above impl, which only compiles
            // if 'a is a covariant lifetime.
            unsafe impl<'a, #(#tybounds),*> yoke::IsCovariant<'a> for #name<'a, #(#typarams),*> where #(#static_bounds),* {}
        }
    }
}

/// Custom derive for `yoke::ZeroCopyFrom`,
///
/// This implements `ZeroCopyFrom<Ty> for Ty` for types
/// without a lifetime parameter, and `ZeroCopyFrom<Ty<'data>> for Ty<'static>`
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
    let tybounds = input.generics.type_params().collect::<Vec<_>>();
    let typarams = tybounds
        .iter()
        .map(|ty| ty.ident.clone())
        .collect::<Vec<_>>();
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
        let (clone, clone_trait) = if has_clone {
            (quote!(this.clone()), quote!(Clone))
        } else {
            (quote!(*this), quote!(Copy))
        };
        let bounds: Vec<WherePredicate> = typarams
            .iter()
            .map(|ty| parse_quote!(#ty: #clone_trait + 'static))
            .collect();
        quote! {
            impl<#(#tybounds),*> ZeroCopyFrom<#name<#(#typarams),*>> for #name<#(#typarams),*> where #(#bounds),* {
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
                impl<'data> ZeroCopyFrom<#name<'data>> for #name<'static> {
                    fn zero_copy_from<'b>(this: &'b #name<'data>) -> #name<'b> {
                        this.clone()
                    }
                }
            };
        }

        let structure = Structure::new(input);
        let generics_env = typarams.iter().cloned().collect();
        let static_bounds: Vec<WherePredicate> = typarams
            .iter()
            .map(|ty| parse_quote!(#ty: 'static))
            .collect();

        let mut zcf_bounds: Vec<WherePredicate> = vec![];
        let body = structure.each_variant(|vi| {
            vi.construct(|f, i| {
                let binding = format!("__binding_{}", i);
                let field = Ident::new(&binding, Span::call_site());
                let fty = replace_lifetime(&f.ty, static_lt());
                let lifetime_ty = replace_lifetime(&f.ty, custom_lt("'data"));

                let (has_ty, has_lt) = visitor::check_type_for_parameters(&f.ty, &generics_env);
                if has_ty {
                    // For types without type parameters, the compiler can figure out that the field implements
                    // ZeroCopyFrom on its own. However, if there are type parameters, there may be complex preconditions
                    // to `FieldTy: ZeroCopyFrom` that need to be satisfied. We get them to be satisfied by requiring
                    // `FieldTy<'static>: ZeroCopyFrom<FieldTy<'data>>` as well as
                    // `for<'data_hrtb> FieldTy<'static>: Yokeable<'data_hrtb, Output = FieldTy<'data_hrtb>>`
                    if has_lt {
                        let hrtb_ty = replace_lifetime(&f.ty, custom_lt("'data_hrtb"));
                        zcf_bounds.push(parse_quote!(#fty: yoke::ZeroCopyFrom<#lifetime_ty>));
                        zcf_bounds.push(parse_quote!(for<'data_hrtb> #fty: yoke::Yokeable<'data_hrtb, Output = #hrtb_ty>));
                    } else {
                        zcf_bounds.push(parse_quote!(#fty: yoke::ZeroCopyFrom<#fty>));
                        zcf_bounds.push(parse_quote!(for<'data_hrtb> #fty: yoke::Yokeable<'data_hrtb, Output = #fty>));
                    }
                }

                // By doing this we essentially require ZCF to be implemented
                // on all fields
                quote! {
                    <#fty as yoke::ZeroCopyFrom<#lifetime_ty>>::zero_copy_from(#field)
                }
            })
        });

        quote! {
            impl<'data, #(#tybounds),*> yoke::ZeroCopyFrom<#name<'data, #(#typarams),*>> for #name<'static, #(#typarams),*>
                where #(#static_bounds,)*
                #(#zcf_bounds,)* {
                fn zero_copy_from<'b>(this: &'b #name<'data, #(#typarams),*>) -> #name<'b, #(#typarams),*> {
                    match *this { #body }
                }
            }
        }
    }
}

fn static_lt() -> Lifetime {
    Lifetime::new("'static", Span::call_site())
}

fn custom_lt(s: &str) -> Lifetime {
    Lifetime::new(s, Span::call_site())
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
