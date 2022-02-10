// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse2, Attribute, Ident, Result, Token};

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
