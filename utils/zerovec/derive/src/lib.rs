// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Proc macros for generating `ULE`, `VarULE` impls and types for the `zerovec` crate

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput};

pub(crate) mod ule;
mod utils;
mod varule;

/// Custom derive for `zerovec::ULE`,
///
/// This can be attached to `Copy` structs containing only ULE types
#[proc_macro_derive(ULE)]
pub fn ule_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(ule::derive_impl(&input))
}

/// Custom derive for `zerovec::VarULE`,
///
/// This can be attached to structs containing only ULE types with one VarULE type at the end
#[proc_macro_derive(VarULE)]
pub fn varule_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(varule::derive_impl(&input))
}

/// Generate a corresponding ULE type and the relevant AsULE implementations for this type
///
/// This can be attached to structs containing only AsULE types, or C-like enums that have `#[repr(u8)]`
/// and all explicit discriminants.
///
/// The type must be `Copy`, `PartialEq`, and `Eq`.
///
/// By default this attribute will also autogenerate a `ZeroMapKV` implementation, which requires
/// `Ord` and `PartialOrd` on `Self`. You can opt out of this with `#[zerovec::skip_kv]`.
///
/// This implementation will also by default autogenerate `Ord` and `PartialOrd` on the ULE type based on
/// the implementation on `Self`. You can opt out of this with `#[zerovec::skip_ord]`
#[proc_macro_attribute]
pub fn make_ule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attr = parse_macro_input!(attr as AttributeArgs);
    TokenStream::from(ule::make_ule_impl(attr, input))
}

/// Generate a corresponding VarULE type and the relevant EncodeAsVarULE/ZeroFrom implementations for this type
///
/// This can be attached to structs containing only AsULE types with the last field being `Cow<'a, str>`,
/// `Cow<'a, [u8]>`, ZeroSlice, or VarZeroSlice.
///
/// The type must be `PartialEq` and `Eq`.
///
/// By default this attribute will also autogenerate a `ZeroMapKV` implementation, which requires
/// `Ord` and `PartialOrd` on the `VarULE` type. You can opt out of this with `#[zerovec::skip_kv]`.
///
/// This implementation will also by default autogenerate `Ord` and `PartialOrd` on the VarULE type based on
/// the implementation on `Self`. You can opt out of this with `#[zerovec::skip_ord]`
#[proc_macro_attribute]
pub fn make_varule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attr = parse_macro_input!(attr as AttributeArgs);
    TokenStream::from(varule::make_varule_impl(attr, input))
}
