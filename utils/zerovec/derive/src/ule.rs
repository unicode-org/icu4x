// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::utils;
use syn::spanned::Spanned;
use syn::{
    parse_quote, AttributeArgs, Data, DataEnum, DataStruct, DeriveInput, Error, Expr, Field,
    Fields, Ident, Lit,
};

use std::collections::HashSet;

pub fn derive_impl(input: &DeriveInput) -> TokenStream2 {
    if !utils::has_valid_repr(&input.attrs, |r| r == "packed" || r == "transparent") {
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
                #[allow(clippy::modulo_one)]
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

pub fn make_ule_impl(attr: AttributeArgs, mut input: DeriveInput) -> TokenStream2 {
    if input.generics.type_params().next().is_some()
        || input.generics.lifetimes().next().is_some()
        || input.generics.const_params().next().is_some()
    {
        return Error::new(
            input.generics.span(),
            "#[make_ule] must be applied to a struct without any generics",
        )
        .to_compile_error();
    }

    if attr.len() != 1 {
        return Error::new(
            input.span(),
            "#[make_ule] takes one argument for the name of the ULE type it produces",
        )
        .to_compile_error();
    }
    let arg = &attr[0];
    let ule_name: Ident = parse_quote!(#arg);

    let (skip_kv, skip_ord, serde) =
        match utils::extract_attributes_common(&mut input.attrs, "make_ule") {
            Ok(val) => val,
            Err(e) => return e.to_compile_error(),
        };

    if serde {
        return Error::new(
            input.span(),
            "#[make_ule] does not support #[zerovec::serde]",
        )
        .to_compile_error();
    }

    let name = &input.ident;

    let ule_stuff = match input.data {
        Data::Struct(ref s) => make_ule_struct_impl(name, &ule_name, &input, s, skip_ord),
        Data::Enum(ref e) => make_ule_enum_impl(name, &ule_name, &input, e, skip_ord),
        _ => {
            return Error::new(input.span(), "#[make_ule] must be applied to a struct")
                .to_compile_error();
        }
    };

    let zmkv = if skip_kv {
        quote!()
    } else {
        quote!(
            impl<'a> zerovec::maps::ZeroMapKV<'a> for #name {
                type Container = zerovec::ZeroVec<'a, #name>;
                type GetType = #ule_name;
                type OwnedType = #name;
            }
        )
    };

    quote!(
        #input

        #ule_stuff

        #zmkv
    )
}

fn make_ule_enum_impl(
    name: &Ident,
    ule_name: &Ident,
    input: &DeriveInput,
    enu: &DataEnum,
    skip_ord: bool,
) -> TokenStream2 {
    // We could support more int reprs in the future if needed
    if !utils::has_valid_repr(&input.attrs, |r| r == "u8") {
        return Error::new(
            input.span(),
            "#[make_ule] can only be applied to #[repr(u8)] enums",
        )
        .to_compile_error();
    }

    // the next discriminant expected
    let mut next = 0;
    // Discriminants that have not been found in series (we might find them later)
    let mut not_found = HashSet::new();

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
                if n >= next {
                    for missing in next..n {
                        not_found.insert(missing);
                    }
                    next = n + 1;
                }

                not_found.remove(&n);

                // We require explicit discriminants so that it is clear that reordering
                // fields would be a breaking change. Furthermore, using explicit discriminants helps ensure that
                // platform-specific C ABI choices do not matter.
                // We could potentially add in explicit discriminants on the user's behalf in the future, or support
                // more complicated sets of explicit discriminant values.
                if n != i as u64 {}
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

    let not_found = not_found.iter().collect::<Vec<_>>();

    if !not_found.is_empty() {
        return Error::new(input.span(), &format!("#[make_ule] must be applied to enums with discriminants \
                                                  filling the range from 0 to a maximum; could not find {:?}", not_found))
            .to_compile_error();
    }

    let max = next as u8;

    let maybe_ord_derives = if skip_ord {
        quote!()
    } else {
        quote!(#[derive(Ord, PartialOrd)])
    };

    let vis = &input.vis;

    // Safety (based on the safety checklist on the ULE trait):
    //  1. ULE type does not include any uninitialized or padding bytes.
    //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant
    //  2. ULE type is aligned to 1 byte.
    //     (achieved by `#[repr(transparent)]` on a type that satisfies this invariant)
    //  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
    //     (Guarantees that the byte is in range of the corresponding enum.)
    //  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
    //     (This does not happen since we are backed by 1 byte.)
    //  5. The other ULE methods use the default impl.
    //  6. ULE type byte equality is semantic equality
    quote!(
        #[repr(transparent)]
        #[derive(Copy, Clone, PartialEq, Eq)]
        #maybe_ord_derives
        #vis struct #ule_name(u8);

        unsafe impl zerovec::ule::ULE for #ule_name {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {
                for byte in bytes {
                    if *byte >= #max {
                        return Err(zerovec::ZeroVecError::parse::<Self>())
                    }
                }
                Ok(())
            }
        }

        impl zerovec::ule::AsULE for #name {
            type ULE = #ule_name;

            fn to_unaligned(self) -> Self::ULE {
                // safety: the enum is repr(u8) and can be cast to a u8
                unsafe {
                    ::core::mem::transmute(self)
                }
            }

            fn from_unaligned(other: Self::ULE) -> Self {
                // safety: the enum is repr(u8) and can be cast from a u8,
                // and `#ule_name` guarantees a valid value for this enum.
                unsafe {
                    ::core::mem::transmute(other)
                }
            }
        }

        impl #name {
            /// Attempt to construct the value from its corresponding integer,
            /// returning None if not possible
            pub(crate) fn new_from_u8(value: u8) -> Option<Self> {
                if value <= #max {
                    unsafe {
                        Some(::core::mem::transmute(value))
                    }
                } else {
                    None
                }
            }
        }
    )
}

fn get_expr_int(e: &Expr) -> Option<u64> {
    if let Ok(Lit::Int(ref i)) = syn::parse2(quote!(#e)) {
        return i.base10_parse().ok();
    }

    None
}

fn make_ule_struct_impl(
    name: &Ident,
    ule_name: &Ident,
    input: &DeriveInput,
    struc: &DataStruct,
    skip_ord: bool,
) -> TokenStream2 {
    if struc.fields.iter().next().is_none() {
        return Error::new(
            input.span(),
            "#[make_ule] must be applied to a non-empty struct",
        )
        .to_compile_error();
    }
    let field_inits = make_ule_fields(struc.fields.iter());
    let field_inits = utils::wrap_field_inits(&field_inits, &struc.fields);

    let semi = utils::semi_for(&struc.fields);
    let repr_attr = utils::repr_for(&struc.fields);
    let vis = &input.vis;

    let ule_struct: DeriveInput = parse_quote!(
        #[repr(#repr_attr)]
        #[derive(Copy, Clone, PartialEq, Eq)]
        #vis struct #ule_name #field_inits #semi
    );
    let derived = derive_impl(&ule_struct);

    let mut as_ule_conversions = vec![];
    let mut from_ule_conversions = vec![];

    for (i, field) in struc.fields.iter().enumerate() {
        let ty = &field.ty;
        let i = syn::Index::from(i);
        if let Some(ref ident) = field.ident {
            as_ule_conversions
                .push(quote!(#ident: <#ty as zerovec::ule::AsULE>::to_unaligned(self.#ident)));
            from_ule_conversions.push(
                quote!(#ident: <#ty as zerovec::ule::AsULE>::from_unaligned(unaligned.#ident)),
            );
        } else {
            as_ule_conversions.push(quote!(<#ty as zerovec::ule::AsULE>::to_unaligned(self.#i)));
            from_ule_conversions
                .push(quote!(<#ty as zerovec::ule::AsULE>::from_unaligned(unaligned.#i)));
        };
    }

    let as_ule_conversions = utils::wrap_field_inits(&as_ule_conversions, &struc.fields);
    let from_ule_conversions = utils::wrap_field_inits(&from_ule_conversions, &struc.fields);
    let asule_impl = quote!(
        impl zerovec::ule::AsULE for #name {
            type ULE = #ule_name;
            fn to_unaligned(self) -> Self::ULE {
                #ule_name #as_ule_conversions
            }
            fn from_unaligned(unaligned: Self::ULE) -> Self {
                Self #from_ule_conversions
            }
        }
    );

    let maybe_ord_impls = if skip_ord {
        quote!()
    } else {
        quote!(
            impl core::cmp::PartialOrd for #ule_name {
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    let this = <#name as zerovec::ule::AsULE>::from_unaligned(*self);
                    let other = <#name as zerovec::ule::AsULE>::from_unaligned(*other);
                    <#name as core::cmp::PartialOrd>::partial_cmp(&this, &other)
                }
            }

            impl core::cmp::Ord for #ule_name {
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    let this = <#name as zerovec::ule::AsULE>::from_unaligned(*self);
                    let other = <#name as zerovec::ule::AsULE>::from_unaligned(*other);
                    <#name as core::cmp::Ord>::cmp(&this, &other)
                }
            }
        )
    };

    quote!(
        #asule_impl

        #ule_struct

        #derived

        #maybe_ord_impls
    )
}

/// Given an iterator over ULE struct fields, returns code validating that a slice variable `bytes` contains valid instances of those ULE types
/// in order, plus the byte offset of any remaining unvalidated bytes. ULE types should not have any remaining bytes, but VarULE types will since
/// the last field is the unsized one.
pub(crate) fn generate_ule_validators<'a>(
    iter: impl Iterator<Item = &'a Field>,
    // (validators, remaining_offset)
) -> (TokenStream2, syn::Ident) {
    utils::generate_per_field_offsets(iter, |field, prev_offset_ident, size_ident, _| {
        let ty = &field.ty;
        quote!(<#ty as zerovec::ule::ULE>::validate_byte_slice(&bytes[#prev_offset_ident .. #prev_offset_ident + #size_ident])?;)
    })
}

/// Make corresponding ULE fields for each field
pub(crate) fn make_ule_fields<'a>(iter: impl Iterator<Item = &'a Field>) -> Vec<TokenStream2> {
    iter.map(|f| {
        let ty = &f.ty;
        let ty = quote!(<#ty as zerovec::ule::AsULE>::ULE);
        if let Some(ref ident) = f.ident {
            quote!(#ident: #ty)
        } else {
            quote!(#ty)
        }
    })
    .collect::<Vec<_>>()
}
