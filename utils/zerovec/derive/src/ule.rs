// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error, Ident};

fn suffixed_ident(name: &str, suffix: usize, s: Span) -> Ident {
    Ident::new(&format!("{name}_{suffix}"), s)
}

pub fn derive_impl(input: &DeriveInput) -> TokenStream2 {
    if !crate::utils::has_valid_repr(&input.attrs) {
        return Error::new(
            input.span(),
            "derive(ULE) must be applied to a #[repr(C)] or #[repr(transparent)] type",
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

    let mut prev_offset_ident = Ident::new("ZERO", Span::call_site());
    let mut validators = quote!(const ZERO: usize = 0);

    for (i, field) in struc.fields.iter().enumerate() {
        let ty = &field.ty;
        let new_offset_ident = suffixed_ident("OFFSET", i, field.span());
        let size_ident = suffixed_ident("SIZE", i, field.span());
        validators = quote! {
            #validators;
            const #size_ident: usize = ::core::mem::size_of::<#ty>();
            const #new_offset_ident: usize = #prev_offset_ident + #size_ident;
            <#ty as zerovec::ule::ULE>::validate_byte_slice(&bytes[#new_offset_ident .. #new_offset_ident + #size_ident])?;
        };

        prev_offset_ident = new_offset_ident;
    }

    let name = &input.ident;

    // Safety (based on the safety checklist on the ULE trait):
    //  1. #name does not include any uninitialized or padding bytes.
    //     (achieved by enforcing #[repr(transparent)] or #[repr(packed)] on a struct of only ULE types)
    //  2. CharULE is aligned to 1 byte.
    //     (achieved by enforcing #[repr(transparent)] or #[repr(packed)] on a struct of only ULE types)
    //  3. The impl of validate_byte_slice() returns an error if any byte is not valid.
    //  4. The impl of validate_byte_slice() returns an error if there are extra bytes.
    //  5. The other ULE methods use the default impl.
    //  6. [This impl does not enforce the equality constraint, it is up to the user to do so, ideally via a custom derive]
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
                }
                Ok(())
            }
        }
    }
}
