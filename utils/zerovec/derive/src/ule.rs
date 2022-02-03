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
