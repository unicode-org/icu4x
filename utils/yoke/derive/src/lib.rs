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

//! Custom derives for `Yokeable` from the `yoke` crate.

mod lifetimes;
mod visitor;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, WherePredicate};
use synstructure::Structure;

use self::lifetimes::{custom_lt, replace_lifetime, static_lt};
use self::visitor::check_type_for_parameters;

/// Custom derive for `yoke::Yokeable`.
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
/// The derive uses a `'a` lifetime for `Yokeable<'a>`, so if one of your type's
/// fields or bounds contains a `for<'a>` binder, the derive will fail to compile;
/// it is your responsibility to choose appropriate lifetime names for `for<..>`
/// binders. (However, the type's lifetime parameter, if any, does not need to be
/// named `'a`; the `'a` lifetime will be substituted into the type as necessary.)
///
/// Beyond these cases, if the derive fails to compile due to lifetime issues, it likely
/// means that the lifetime is not covariant and `Yokeable` is not safe to implement.
#[proc_macro_derive(Yokeable, attributes(yoke))]
pub fn yokeable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(yokeable_derive_impl(&input))
}

fn yokeable_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let tybounds = input
        .generics
        .type_params()
        .map(|ty| {
            // Strip out param defaults, we don't need them in the impl
            let mut ty = ty.clone();
            ty.eq_token = None;
            ty.default = None;
            ty
        })
        .collect::<Vec<_>>();
    let typarams = tybounds
        .iter()
        .map(|ty| ty.ident.clone())
        .collect::<Vec<_>>();
    let wherebounds = input
        .generics
        .where_clause
        .iter()
        .flat_map(|wc| wc.predicates.iter())
        .filter(|p| matches!(p, WherePredicate::Type(_)))
        .collect::<Vec<_>>();
    // We require all type parameters be 'static, otherwise
    // the Yokeable impl becomes really unweildy to generate safely
    let static_bounds: Vec<WherePredicate> = typarams
        .iter()
        .map(|ty| parse_quote!(#ty: 'static))
        .collect();
    let mut lts = input.generics.lifetimes();

    let Some(lt_param) = lts.next() else {
        // There are 0 lifetime parameters, since `input.generics.lifetimes()` is empty.
        let name = &input.ident;
        return quote! {
            // This is safe because there are no lifetime parameters.
            unsafe impl<'a, #(#tybounds),*> yoke::Yokeable<'a> for #name<#(#typarams),*>
            where
                #(#static_bounds,)*
                #(#wherebounds,)*
                Self: Sized
            {
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
        };
    };

    if lts.next().is_some() {
        // We already extracted one lifetime into `source_lt`, so this means there are
        // multiple lifetimes.
        return syn::Error::new(
            input.generics.span(),
            "derive(Yokeable) cannot have multiple lifetime parameters",
        )
        .to_compile_error();
    }

    let name = &input.ident;
    let manual_covariance = input.attrs.iter().any(|a| {
        if a.path().is_ident("yoke") {
            if let Ok(i) = a.parse_args::<Ident>() {
                if i == "prove_covariance_manually" {
                    return true;
                }
            }
        }
        false
    });

    if !manual_covariance {
        // This is safe because as long as `transform()` compiles,
        // we can be sure that `'a` is a covariant lifetime on `Self`.
        //
        // In particular, the operand of `&raw const` is not a location where implicit
        // type coercion can occur, so the type of `&raw const self` is `*const &'a Self`.
        // The RHS of a `let` with an explicit type annotation allows type coercion, so
        // `transform` checks that `*const &'a Self` can coerce to `*const &'a Self::Output`.
        // Most of the possible type coercions
        // (listed at https://doc.rust-lang.org/reference/type-coercions.html)
        // do not apply, other than subtyping coercions and transitive coercions (which do
        // not add anything beyond subtyping coercions). In particular, there's nothing
        // like a `DerefRaw` on `*const T`, and `&T` does not implement `Unsize`, so
        // there cannot be an unsizing coercion from `*const &'a Self` to
        // `*const &'a Self::Output`. Therefore, `transform` compiles if and only if
        // a subtyping coercion is possible; this requires that `Self` must be a subtype
        // of `Self::Output`, just as `&'static T` is a subtype of `&'a T` (for `T: 'static`).
        // This ensures covariance.
        //
        // This will not work for structs involving ZeroMap since
        // the compiler does not know that ZeroMap is covariant.
        //
        // This custom derive can be improved to handle this case when necessary,
        // with `prove_covariance_manually`.
        return quote! {
            unsafe impl<'a, #(#tybounds),*> yoke::Yokeable<'a> for #name<'static, #(#typarams),*>
            where
                #(#static_bounds,)*
                #(#wherebounds,)*
                // See the comment for the `prove_covariance_manually` impl about `Self: Sized`.
            {
                type Output = #name<'a, #(#typarams),*>;
                #[inline]
                fn transform(&'a self) -> &'a Self::Output {
                    if false {
                        let _: *const &'a Self::Output = &raw const self;
                    }
                    self
                }
                #[inline]
                fn transform_owned(self) -> Self::Output {
                    self
                }
                #[inline]
                unsafe fn make(this: Self::Output) -> Self {
                    // Unfortunately, Rust doesn't think `mem::transmute` is possible,
                    // since it's not sure the sizes are the same.
                    ::core::debug_assert_eq!(
                        ::core::mem::size_of::<Self::Output>(),
                        ::core::mem::size_of::<Self>(),
                    );
                    let this = ::core::mem::ManuallyDrop::new(this);
                    // `Deref` coercion and then `&` to `*const` coercion.
                    let ptr: *const Self = (&this as &Self::Output as *const Self::Output).cast();

                    unsafe { ::core::ptr::read(ptr) }
                }
                #[inline]
                fn transform_mut<F>(&'a mut self, f: F)
                where
                    F: 'static + for<'b> FnOnce(&'b mut Self::Output) {
                    unsafe { f(::core::mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
                }
            }
        };
    }

    let mut structure = Structure::new(input);
    structure.bind_with(|_| synstructure::BindStyle::Move);
    let generics_env = typarams.iter().cloned().collect();
    let static_bounds: Vec<WherePredicate> = typarams
        .iter()
        .map(|ty| parse_quote!(#ty: 'static))
        .collect();
    let mut yoke_bounds: Vec<WherePredicate> = vec![];
    let lt_param = &lt_param.lifetime.ident;

    for variant_info in structure.variants() {
        for field in variant_info.ast().fields.iter() {
            let (has_ty, has_lt) = check_type_for_parameters(lt_param, &field.ty, &generics_env);

            if has_ty {
                // For field types without type or lifetime parameters, we don't require `Yokeable`.
                // For field types with lifetime parameters but no type parameters, we require `Yokeable` and
                // the compiler can figure out that the field implements `Yokeable` on its own (else, emit an
                // error, since `Yokeable` would *never* be implemented on this type).
                // However, if there are type parameters, there may be complex preconditions to
                // `FieldTy: Yokeable` that need to be satisfied. We get them to be
                // satisfied by requiring `FieldTy<'static>: Yokeable<'_, Output = FieldTy<'a>>`
                // or `FieldTy: Yokeable<'_, Output = FieldTy>`.
                // Note: the bound in the case where the field has no lifetime parameter feels trivial and
                // perhaps should be loosened, but loosening it is a possibly-breaking change.
                // Note: if `field.ty` involves a non-pure macro type, each time it's evaluated, it could be a
                // different type. The `yoke_bounds` are relied on to let the impl compile, *not* for soundness.
                // Our `borrowed_body` does not blindly assume that the fields' types implement `Yokeable`,
                // regardless of these bounds.
                let fty_static = replace_lifetime(lt_param, &field.ty, static_lt());

                if has_lt {
                    let fty_a = replace_lifetime(lt_param, &field.ty, custom_lt("'a"));
                    yoke_bounds
                        .push(parse_quote!(#fty_static: yoke::Yokeable<'a, Output = #fty_a>));
                } else {
                    yoke_bounds
                        .push(parse_quote!(#fty_static: yoke::Yokeable<'a, Output = #fty_static>));
                }
            }
        }
    }

    // `static_checks` and `output_checks` ensure that every field of the type we're deriving `Yokeable`
    // on has `FieldTy` types which are sound to treat covariantly in the `'a` parameter. That is,
    // if the checks successfully pass typeck, the below `Yokeable` impl is sound.
    // Checks on `Self` (noting that `Self: 'static` is required)
    let static_checks = structure.each(|binding| {
        let field = binding.ast();
        let field_binding = &binding.binding;

        let (has_ty, has_lt) = check_type_for_parameters(lt_param, &field.ty, &generics_env);

        if has_ty || has_lt {
            // We only evaluate this type a single time below (either here or in `output_checks`).
            // (We also evaluate it twice above in places that are not load-bearing for soundness).
            // Moreover, even if the macro type gives some strange output, the code would simply
            // fail to compile; that is, nowhere is the visible type of a field load-bearing for
            // soundness. Even if there's a horribly pathological macro type, nothing can go wrong.
            let fty_static = replace_lifetime(lt_param, &field.ty, static_lt());

            // This confirms that this `FieldTy` is a subtype of something which implements
            // `Yokeable<'a>`, and since only `'static` types can be subtypes of a `'static` type
            // (and all `Yokeable` implementors are `'static`), we have that either:
            // - `FieldTy` is some `'static` type which does NOT implement `Yokeable`, but via function
            //   pointer subtyping or something similar, is a subtype of something implementing `Yokeable`, or
            // - `FieldTy` is some type which does itself implement `Yokeable`.
            // In either of those cases, it is sound to treat `FieldTy` as covariant in the `'a`
            // parameter.
            //
            // Now, to justify that `FieldTy` (the field's actual type, not just `field.ty` which may have
            // a non-pure macro type) is a subtype of something which implements `Yokeable<'a>`:
            //
            // `field_binding` has type `&'a FieldTy` (since it's a field of `&'a Self` matched
            // as `self`). The operand of `&raw const` is not a location where implicit type coercion
            // can occur. Therefore, `&raw const #field_binding` is guaranteed to be type
            // `*const &'a FieldTy`. The argument to `__yoke_derive_require_yokeable` does allow
            // type coercion. Looking at <https://doc.rust-lang.org/reference/type-coercions.html>,
            // there are only three types of coercions that could plausibly apply:
            // - subtyping coercions,
            // - transitive coercions, and
            // - unsizing coercions.
            // (If some sort of `DerefRaw` trait gets added for `*const`, there could plausibly
            // be problems with that. But there's no reason to think that such a trait will be
            // added since it'd mess with `unsafe` code.)
            //
            // Since `&'a _` does not implement `Unsize`, we have that `*const &'a _` does not
            // allow an unsizing coercion to occur. Therefore, there are only subtyping coercions,
            // and transitive coercions add nothing on beyond subtyping coercions.
            // Therefore, if this compiles, `*const &'a FieldTy` must be a subtype of `*const &'a T`
            // where `T = #fty_static` is the generic parameter of `__yoke_derive_require_yokeable`.
            // Looking at the signature of that function generated below, we have that
            // `T: Yokeable<'a>` (if it compiles). Note that if `#fty_static` is incorrect, even if
            // there is some other `T` which would work, this will just fail to compile. Since
            // `*const _` and `&'a _` are covariant over their type parameters, we have that
            // `FieldTy` must be a subtype of `T` in order for a subtyping coercion from
            // `*const &'a FieldTy` to `*const &'a T` to occur.
            //
            // Therefore, `FieldTy` must be a subtype of something which implements `Yokeable<'a>`
            // in order for this to compile. (Though that is not a sufficient condition, if
            // there's some weird macro type.)
            quote! {
                __yoke_derive_require_yokeable::<'a, #fty_static>(&raw const #field_binding);
            }
        } else {
            // Handled below
            quote! {}
        }
    });
    let output_checks = structure.each(|binding| {
        let field = binding.ast();
        let field_binding = &binding.binding;

        let (has_ty, has_lt) = check_type_for_parameters(lt_param, &field.ty, &generics_env);

        if has_ty || has_lt {
            // Handled above.
            quote! {}
        } else {
            // See `static_checks`. We treat this very carefully in order to avoid prevent
            // non-pure macro types from causing unsoundness.
            let fty_a = replace_lifetime(lt_param, &field.ty, custom_lt("'a"));

            // No nested lifetimes, so there should be nothing to be done. However,
            // in case a macro type does something strange and accesses the available
            // `'a` lifetime, we still need to check that the field's actual type is
            // `'static` regardless of `'a` (which we can check by ensuring that it must be a subtype
            // of a `'static` type).
            // See reasoning in the `if` branch for why this works. The difference is that
            // `FieldTy` is guaranteed to be a subtype of `T = #fty_a` where `T: 'static`
            // (if this compiles). Since the field's type is a subtype of something which is `'static`,
            // it must itself be `'static`, and therefore did not manage to use `'a` via a macro.
            quote! {
                __yoke_derive_require_static::<'a, #fty_a>(&raw const #field_binding);
            }
        }
    });
    quote! {
        // SAFETY: we assert covariance in `borrowed_checks`
        unsafe impl<'a, #(#tybounds),*> yoke::Yokeable<'a> for #name<'static, #(#typarams),*>
        where
            #(#static_bounds,)*
            #(#wherebounds,)*
            #(#yoke_bounds,)*
            // Adding `Self: Sized` here doesn't work. `for<'b> #name<'b, #(#typarams),*>: Sized`
            // could work, though. (But we'd need another reserved lifetime, so probably `'_yoke_a`
            // or something, not `'b`.) Since these trait bounds are very finicky, it's best to just
            // not try unless necessary.
        {
            type Output = #name<'a, #(#typarams),*>;
            #[inline]
            fn transform(&'a self) -> &'a Self::Output {
                // These are just type asserts, we don't need to run them
                if false {
                    // This could, hypothetically, conflict with the name of one of the `FieldTy`s
                    // we read (and cause a compilation error). However, such a conflict cannot
                    // cause unsoundness, since this function is in scope no matter what.
                    // (The problem is that attempting to refer to a type named
                    // `__yoke_derive_require_yokeable` would instead refer to this function item
                    // and therefore fail.)
                    #[allow(dead_code)]
                    fn __yoke_derive_require_yokeable<'a: 'a, T: yoke::Yokeable<'a>>(_t: *const &'a T) {}

                    match self {
                        #static_checks
                    }
                }
                let output: &'a Self::Output = unsafe { ::core::mem::transmute(self) };
                if false {
                    // Same deal as above.
                    #[allow(dead_code)]
                    fn __yoke_derive_require_static<'a: 'a, T: 'static>(_t: *const &'a T) {}

                    match output {
                        #output_checks
                    }
                }
                output
            }
            #[inline]
            fn transform_owned(self) -> Self::Output {
                // Unfortunately, Rust doesn't think `mem::transmute` is possible,
                // since it's not sure the sizes are the same.
                ::core::debug_assert_eq!(
                    ::core::mem::size_of::<Self>(),
                    ::core::mem::size_of::<Self::Output>(),
                );
                let this = ::core::mem::ManuallyDrop::new(self);
                // `Deref` coercion and then `&` to `*const` coercion.
                let ptr: *const Self::Output = (&this as &Self as *const Self).cast();

                unsafe { ::core::ptr::read(ptr) }
            }
            #[inline]
            unsafe fn make(this: Self::Output) -> Self {
                // Unfortunately, Rust doesn't think `mem::transmute` is possible,
                // since it's not sure the sizes are the same.
                ::core::debug_assert_eq!(
                    ::core::mem::size_of::<Self::Output>(),
                    ::core::mem::size_of::<Self>(),
                );
                let this = ::core::mem::ManuallyDrop::new(this);
                // `Deref` coercion and then `&` to `*const` coercion.
                let ptr: *const Self = (&this as &Self::Output as *const Self::Output).cast();

                unsafe { ::core::ptr::read(ptr) }
            }
            #[inline]
            fn transform_mut<F>(&'a mut self, f: F)
            where
                F: 'static + for<'b> FnOnce(&'b mut Self::Output) {
                unsafe { f(::core::mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
            }
        }
    }
}
