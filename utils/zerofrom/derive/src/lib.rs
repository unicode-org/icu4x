// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom derives for `ZeroCopyFrom` from the `zerofrom` crate.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, Lifetime, Type, WherePredicate};
use synstructure::Structure;

mod visitor;

/// Custom derive for `zerofrom::ZeroCopyFrom`,
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
            impl<'zcf, #(#tybounds),*> zerofrom::ZeroCopyFrom<'zcf, #name<#(#typarams),*>> for #name<#(#typarams),*> where #(#bounds),* {
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
                impl<'zcf> zerofrom::ZeroCopyFrom<'zcf, #name<'_>> for #name<'zcf> {
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
                                .push(parse_quote!(#fty: zerofrom::ZeroCopyFrom<'zcf, #lifetime_ty>));
                        } else {
                            zcf_bounds.push(parse_quote!(#fty: zerofrom::ZeroCopyFrom<'zcf, #fty>));
                        }
                    }

                    // By doing this we essentially require ZCF to be implemented
                    // on all fields
                    quote! {
                        <#fty as zerofrom::ZeroCopyFrom<'zcf, #lifetime_ty>>::zero_copy_from(#field)
                    }
                }
            })
        });

        quote! {
            impl<'zcf, 'zcf_inner, #(#tybounds),*> zerofrom::ZeroCopyFrom<'zcf, #name<'zcf_inner, #(#typarams),*>> for #name<'zcf, #(#typarams),*>
                where
                #(#zcf_bounds,)* {
                fn zero_copy_from(this: &'zcf #name<'zcf_inner, #(#typarams),*>) -> Self {
                    match *this { #body }
                }
            }
        }
    }
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
