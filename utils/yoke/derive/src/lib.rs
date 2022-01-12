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
/// #[yoke(prove_covariance_manually)]
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
                #[inline]
                fn transform(&self) -> &Self::Output {
                    self
                }
                #[inline]
                fn transform_owned(self) -> Self::Output {
                    self
                }
                #[inline]
                unsafe fn make(this: Self::Output) -> Self {
                    this
                }
                #[inline]
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
                    #[inline]
                    fn transform(&'a self) -> &'a Self::Output {
                        unsafe {
                            // safety: we have asserted covariance in
                            // transform_owned
                            ::core::mem::transmute(self)
                        }
                    }
                    #[inline]
                    fn transform_owned(self) -> Self::Output {
                        match self { #body }
                    }
                    #[inline]
                    unsafe fn make(this: Self::Output) -> Self {
                        use core::{mem, ptr};
                        // unfortunately Rust doesn't think `mem::transmute` is possible since it's not sure the sizes
                        // are the same
                        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
                        let ptr: *const Self = (&this as *const Self::Output).cast();
                        mem::forget(this);
                        ptr::read(ptr)
                    }
                    #[inline]
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
                #[inline]
                fn transform(&'a self) -> &'a Self::Output {
                    self
                }
                #[inline]
                fn transform_owned(self) -> Self::Output {
                    self
                }
                #[inline]
                unsafe fn make(this: Self::Output) -> Self {
                    use core::{mem, ptr};
                    // unfortunately Rust doesn't think `mem::transmute` is possible since it's not sure the sizes
                    // are the same
                    debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
                    let ptr: *const Self = (&this as *const Self::Output).cast();
                    mem::forget(this);
                    ptr::read(ptr)
                }
                #[inline]
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
/// to use `.clone()` for its implementation. The attribute can be applied to
/// fields as well.
#[proc_macro_derive(ZeroCopyFrom, attributes(yoke))]
pub fn zcf_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(zcf_derive_impl(&input))
}

fn has_cloning_zcf_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        if let Ok(i) = a.parse_args::<Ident>() {
            if i == "cloning_zcf" {
                return true;
            }
        }
        false
    })
}

fn zcf_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let tybounds = input.generics.type_params().collect::<Vec<_>>();
    let typarams = tybounds
        .iter()
        .map(|ty| ty.ident.clone())
        .collect::<Vec<_>>();
    let has_clone = has_cloning_zcf_attr(&input.attrs);
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
            impl<'zcf, #(#tybounds),*> ZeroCopyFrom<'zcf, #name<#(#typarams),*>> for #name<#(#typarams),*> where #(#bounds),* {
                fn zero_copy_from(this: &'zcf Self) -> Self {
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
                impl<'zcf> ZeroCopyFrom<'zcf, #name<'_>> for #name<'zcf> {
                    fn zero_copy_from(this: &'zcf #name<'_>) -> Self {
                        this.clone()
                    }
                }
            };
        }

        let structure = Structure::new(input);
        let generics_env = typarams.iter().cloned().collect();

        let mut zcf_bounds: Vec<WherePredicate> = vec![];
        let body = structure.each_variant(|vi| {
            let variant_cloning = has_cloning_zcf_attr(vi.ast().attrs);
            vi.construct(|f, i| {
                let binding_cloning = variant_cloning || has_cloning_zcf_attr(&f.attrs);
                let binding = format!("__binding_{}", i);
                let field = Ident::new(&binding, Span::call_site());

                if binding_cloning {
                    quote! {
                        #field.clone()
                    }
                } else {
                    let fty = replace_lifetime(&f.ty, custom_lt("'zcf"));
                    let lifetime_ty = replace_lifetime(&f.ty, custom_lt("'zcf_inner"));

                    let (has_ty, has_lt) = visitor::check_type_for_parameters(&f.ty, &generics_env);
                    if has_ty {
                        // For types without type parameters, the compiler can figure out that the field implements
                        // ZeroCopyFrom on its own. However, if there are type parameters, there may be complex preconditions
                        // to `FieldTy: ZeroCopyFrom` that need to be satisfied. We get them to be satisfied by requiring
                        // `FieldTy<'zcf>: ZeroCopyFrom<'zcf, FieldTy<'zcf_inner>>`
                        if has_lt {
                            zcf_bounds
                                .push(parse_quote!(#fty: yoke::ZeroCopyFrom<'zcf, #lifetime_ty>));
                        } else {
                            zcf_bounds.push(parse_quote!(#fty: yoke::ZeroCopyFrom<'zcf, #fty>));
                        }
                    }

                    // By doing this we essentially require ZCF to be implemented
                    // on all fields
                    quote! {
                        <#fty as yoke::ZeroCopyFrom<'zcf, #lifetime_ty>>::zero_copy_from(#field)
                    }
                }
            })
        });

        quote! {
            impl<'zcf, 'zcf_inner, #(#tybounds),*> yoke::ZeroCopyFrom<'zcf, #name<'zcf_inner, #(#typarams),*>> for #name<'zcf, #(#typarams),*>
                where
                #(#zcf_bounds,)* {
                fn zero_copy_from(this: &'zcf #name<'zcf_inner, #(#typarams),*>) -> Self {
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
    use syn::fold::Fold;
    struct ReplaceLifetime(Lifetime);

    impl Fold for ReplaceLifetime {
        fn fold_lifetime(&mut self, _: Lifetime) -> Lifetime {
            self.0.clone()
        }
    }
    ReplaceLifetime(lt).fold_type(x.clone())
}
