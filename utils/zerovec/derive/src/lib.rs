// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Proc macros for generating `ULE`, `VarULE` impls and types for the `zerovec` crate

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod ule;
mod utils;

/// Custom derive for `zerovec::ULE`,
///
/// This can be attached to structs containing only ULE types
#[proc_macro_derive(ULE)]
pub fn ule_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(ule::derive_impl(&input))
}
