// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Proc macros for generating `ULE`, `VarULE` impls and types for the `zerovec` crate

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub(crate) mod ule;
mod utils;
mod varule;

/// Custom derive for `zerovec::ULE`,
///
/// This can be attached to structs containing only ULE types
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
