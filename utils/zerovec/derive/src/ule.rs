// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::spanned::Spanned;
use syn::{
    parse_quote, AttributeArgs, Data, DataEnum, DataStruct, DeriveInput, Error, Expr, Field,
    Fields, Ident, Lit,
};

fn suffixed_ident(name: &str, suffix: usize, s: Span) -> Ident {
    Ident::new(&format!("{name}_{suffix}"), s)
}

pub fn derive_impl(input: &DeriveInput) -> TokenStream2 {
    if !crate::utils::has_valid_repr(&input.attrs, |r| r == "packed" || r == "transparent") {
        return Error::new(
            input.span(),
            "derive(ULE) must be applied to a #[repr(packed)] or #[repr(transparent)] type",
        )
        .to_compile_error();
    }
    if input.generics.type_params().next().is_some()
        || input.generics.lifetimes().next().is_some()
        || input.generics.const_params().next().is_some()
    {
        return Error::new(
            input.generics.span(),
            "derive(ULE) must be applied to a struct without any generics",
        )
        .to_compile_error();
    }
    let struc = if let Data::Struct(ref s) = input.data {
        if s.fields.iter().next().is_none() {
            return Error::new(
                input.span(),
                "derive(ULE) must be applied to a non-empty struct",
            )
            .to_compile_error();
        }
        s
    } else {
        return Error::new(input.span(), "derive(ULE) must be applied to a struct")
            .to_compile_error();
    };

    let (validators, remaining_offset) = generate_ule_validators(struc.fields.iter());

    let name = &input.ident;

    // Safety (based on the safety checklist on the ULE trait):
    //  1. #name does not include any uninitialized or padding bytes.
    //     (achieved by enforcing #[repr(transparent)] or #[repr(packed)] on a struct of only ULE types)
    //  2. #name is aligned to 1 byte.
    //     (achieved by enforcing #[repr(transparent)] or #[repr(packed)] on a struct of only ULE types)
    //  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
    //  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
    //  5. The other ULE methods use the default impl.
    //  6. [This impl does not enforce the non-safety equality constraint, it is up to the user to do so, ideally via a custom derive]
    quote! {
        unsafe impl zerovec::ule::ULE for #name {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
                const SIZE: usize = ::core::mem::size_of::<#name>();
                if bytes.len() % SIZE != 0 {
                    return Err(zerovec::ZeroVecError::length::<Self>(bytes.len()));
                }
                // Validate the bytes
                for chunk in bytes.chunks_exact(SIZE) {
                    #validators
                    debug_assert_eq!(#remaining_offset, SIZE);
                }
                Ok(())
            }
        }
    }
}

fn wrap_field_inits(streams: &[TokenStream2], fields: &Fields) -> TokenStream2 {
    match *fields {
        Fields::Named(_) => quote!( { #(#streams),* } ),
        Fields::Unnamed(_) => quote!( ( #(#streams),* ) ),
        Fields::Unit => {
            unreachable!("#[make_ule] should have already checked that there are fields")
        }
    }
}

pub fn make_ule_impl(attr: AttributeArgs, input: DeriveInput) -> TokenStream2 {
    if input.generics.type_params().next().is_some()
        || input.generics.lifetimes().next().is_some()
        || input.generics.const_params().next().is_some()
    {
        return Error::new(
            input.generics.span(),
            "derive(ULE) must be applied to a struct without any generics",
        )
        .to_compile_error();
    }

    if !attr.is_empty() {
        return Error::new(
            input.span(),
            "#[make_ule] does not currently support any arguments",
        )
        .to_compile_error();
    }
    let name = &input.ident;
    let ule_name = Ident::new(&format!("{}ULE", name), Span::call_site());

    let ule_stuff = match input.data {
        Data::Struct(ref s) => make_ule_struct_impl(name, &ule_name, &input, s),
        Data::Enum(ref e) => make_ule_enum_impl(name, &ule_name, &input, e),
        _ => {
            return Error::new(input.span(), "#[make_ule] must be applied to a struct")
                .to_compile_error();
        }
    };

    // Todo: opt-out for ZMKV impl

    quote!(
        #input

        #ule_stuff

        impl<'a> zerovec::map::ZeroMapKV<'a> for #name {
            type Container = zerovec::ZeroVec<'a, #name>;
            type GetType = #ule_name;
            type OwnedType = #name;
        }
    )
}

fn make_ule_enum_impl(
    name: &Ident,
    ule_name: &Ident,
    input: &DeriveInput,
    enu: &DataEnum,
) -> TokenStream2 {
    // We could support more int reprs in the future if needed
    if !crate::utils::has_valid_repr(&input.attrs, |r| r == "u8") {
        return Error::new(
            input.span(),
            "#[make_ule] can only be applied to #[repr(u8)] enums",
        )
        .to_compile_error();
    }

    for (i, variant) in enu.variants.iter().enumerate() {
        if variant.fields != Fields::Unit {
            // This can be supported in the future, see zerovec/design_doc.md
            return Error::new(
                variant.span(),
                "#[make_ule] can only be applied to enums with dataless variants",
            )
            .to_compile_error();
        }

        if let Some((_, ref discr)) = variant.discriminant {
            if let Some(n) = get_expr_int(discr) {
                // We require explicit discriminants so that it is clear that reordering
                // fields would be a breaking change. Furthermore, using explicit discriminants helps ensure that
                // platform-specific C ABI choices do not matter.
                // We could potentially add in explicit discriminants on the user's behalf in the future, or support
                // more complicated sets of explicit discriminant values.
                if n != i as u64 {
                    return Error::new(discr.span(), &format!("#[make_ule] must be applied to enums with discriminants \
                                                              increasing in order from 0, expected {i}, found {n}"))
                        .to_compile_error();
                }
            } else {
                return Error::new(
                    discr.span(),
                    "#[make_ule] must be applied to enums with explicit integer discriminants",
                )
                .to_compile_error();
            }
        } else {
            return Error::new(
                variant.span(),
                "#[make_ule] must be applied to enums with explicit discriminants",
            )
            .to_compile_error();
        }
    }

    let max = enu.variants.len() as u8;

    // Safety (based on the safety checklist on the ULE trait):
    //  1. ULE type does not include any uninitialized or padding bytes.
    //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant
    //  2. ULE type is aligned to 1 byte.
    //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
    //  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
    //  4. The impl of validate_byte_slice() returns an error if there are extra bytes (this never happens)
    //  5. The other ULE methods use the default impl.
    //  6. ULE type byte equality is semantic equality
    quote!(
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
        struct #ule_name(u8);

        unsafe impl zerovec::ule::ULE for #ule_name {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
                for byte in bytes {
                    if *byte >= #max {
                        return Err(ZeroVecError::parse::<Self>())
                    }
                }
                Ok(())
            }
        }

        impl zerovec::ule::AsULE for #name {
            type ULE = #ule_name;

            fn as_unaligned(self) -> Self::ULE {
                unsafe {
                    ::core::mem::transmute(self)
                }
            }

            fn from_unaligned(other: Self::ULE) -> Self {
                unsafe {
                    ::core::mem::transmute(other)
                }
            }
        }

        impl ::core::convert::TryFrom<u8> for #name {
            type Error = ();
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                if value <= #max {
                    unsafe {
                        Ok(::core::mem::transmute(value))
                    }
                } else {
                    Err(())
                }
            }
        }

        impl From<#name> for u8 {
            fn from(other: #name) -> Self {
                other as u8
            }
        }
    )
}

fn get_expr_int(e: &Expr) -> Option<u64> {
    if let Expr::Lit(ref l) = *e {
        if let Lit::Int(ref i) = l.lit {
            return i.base10_parse().ok();
        }
    }

    None
}

fn make_ule_struct_impl(
    name: &Ident,
    ule_name: &Ident,
    input: &DeriveInput,
    struc: &DataStruct,
) -> TokenStream2 {
    if struc.fields.iter().next().is_none() {
        return Error::new(
            input.span(),
            "#[make_ule] must be applied to a non-empty struct",
        )
        .to_compile_error();
    }
    let ule_fields = struc
        .fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            let ty = quote!(<#ty as zerovec::ule::AsULE>::ULE);
            if let Some(ref ident) = f.ident {
                quote!(#ident: #ty)
            } else {
                quote!(#ty)
            }
        })
        .collect::<Vec<_>>();

    let repr_attr = if struc.fields.len() == 1 {
        quote!(transparent)
    } else {
        quote!(packed)
    };

    let field_inits = wrap_field_inits(&ule_fields, &struc.fields);
    let semi = if let Fields::Unnamed(..) = struc.fields {
        quote!(;)
    } else {
        quote!()
    };
    let ule_struct: DeriveInput = parse_quote!(
        #[repr(#repr_attr)]
        #[derive(Copy, Clone, PartialEq)]
        struct #ule_name #field_inits #semi

    );
    let derived = derive_impl(&ule_struct);

    let mut as_ule_conversions = vec![];
    let mut from_ule_conversions = vec![];

    for (i, field) in struc.fields.iter().enumerate() {
        let ty = &field.ty;
        let i = syn::Index::from(i);
        if let Some(ref ident) = field.ident {
            as_ule_conversions
                .push(quote!(#ident: <#ty as zerovec::ule::AsULE>::as_unaligned(self.#ident)));
            from_ule_conversions.push(
                quote!(#ident: <#ty as zerovec::ule::AsULE>::from_unaligned(unaligned.#ident)),
            );
        } else {
            as_ule_conversions.push(quote!(<#ty as zerovec::ule::AsULE>::as_unaligned(self.#i)));
            from_ule_conversions
                .push(quote!(<#ty as zerovec::ule::AsULE>::from_unaligned(unaligned.#i)));
        };
    }

    let as_ule_conversions = wrap_field_inits(&as_ule_conversions, &struc.fields);
    let from_ule_conversions = wrap_field_inits(&from_ule_conversions, &struc.fields);
    let asule_impl = quote!(
        impl zerovec::ule::AsULE for #name {
            type ULE = #ule_name;
            fn as_unaligned(self) -> Self::ULE {
                #ule_name #as_ule_conversions
            }
            fn from_unaligned(unaligned: Self::ULE) -> Self {
                Self #from_ule_conversions
            }
        }
    );
    quote!(
        #asule_impl

        #ule_struct

        #derived
    )
}

/// Given an iterator over ULE struct fields, returns code validating that a slice variable `bytes` contains valid instances of those ULE types
/// in order, plus the byte offset of any remaining unvalidated bytes. ULE types should not have any remaining bytes, but VarULE types will since
/// the last field is the unsized one.
pub(crate) fn generate_ule_validators<'a>(
    iter: impl Iterator<Item = &'a Field>,
    // (validators, remaining_offset)
) -> (TokenStream2, syn::Ident) {
    let mut prev_offset_ident = Ident::new("ZERO", Span::call_site());
    let mut validators = quote!(const ZERO: usize = 0);

    for (i, field) in iter.enumerate() {
        let ty = &field.ty;
        let new_offset_ident = suffixed_ident("OFFSET", i, field.span());
        let size_ident = suffixed_ident("SIZE", i, field.span());
        validators = quote! {
            #validators;
            const #size_ident: usize = ::core::mem::size_of::<#ty>();
            const #new_offset_ident: usize = #prev_offset_ident + #size_ident;
            <#ty as zerovec::ule::ULE>::validate_byte_slice(&bytes[#prev_offset_ident .. #prev_offset_ident + #size_ident])?;
        };

        prev_offset_ident = new_offset_ident;
    }

    (validators, prev_offset_ident)
}
