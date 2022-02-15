// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use quote::quote;

use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse2, Attribute, Fields, Ident, Result, Token};

// Check that there are repr attributes satisfying the given predicate
pub fn has_valid_repr(attrs: &[Attribute], predicate: impl Fn(&Ident) -> bool + Copy) -> bool {
    attrs
        .iter()
        .filter(|a| a.path.get_ident().map(|a| a == "repr").unwrap_or(false))
        .any(|a| {
            parse2::<ReprAttribute>(a.tokens.clone())
                .ok()
                .and_then(|s| s.reprs.iter().find(|s| predicate(s)).map(|_| ()))
                .is_some()
        })
}

struct ReprAttribute {
    reprs: Punctuated<Ident, Token![,]>,
}

impl Parse for ReprAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _paren = parenthesized!(content in input);
        Ok(ReprAttribute {
            reprs: content.parse_terminated(Ident::parse)?,
        })
    }
}

/// Given a set of entries for struct field definitions to go inside a `struct {}` definition,
/// wrap in a () or {} based on the type of field
pub fn wrap_field_inits(streams: &[TokenStream2], fields: &Fields) -> TokenStream2 {
    match *fields {
        Fields::Named(_) => quote!( { #(#streams),* } ),
        Fields::Unnamed(_) => quote!( ( #(#streams),* ) ),
        Fields::Unit => {
            unreachable!("#[make_(var)ule] should have already checked that there are fields")
        }
    }
}

/// Return a semicolon token if necessary after the struct definition
pub fn semi_for(f: &Fields) -> TokenStream2 {
    if let Fields::Unnamed(..) = *f {
        quote!(;)
    } else {
        quote!()
    }
}

/// Returns the repr attribute to be applied to the resultant ULE or VarULE type
pub fn repr_for(f: &Fields) -> TokenStream2 {
    if f.len() == 1 {
        quote!(transparent)
    } else {
        quote!(packed)
    }
}
