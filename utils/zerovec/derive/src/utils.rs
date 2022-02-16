// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use quote::quote;

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parenthesized, parse2, Attribute, Error, Field, Fields, Ident, Index, Result, Token};

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

fn suffixed_ident(name: &str, suffix: usize, s: Span) -> Ident {
    Ident::new(&format!("{name}_{suffix}"), s)
}

/// Given an iterator over ULE struct fields, returns code that calculates field sizes and generates a line
/// of code per field based on the per_field_code function (whose parameters are the field, the identifier of the const
/// for the previous offset, the identifier for the const for the next offset, and the field index)
pub(crate) fn generate_per_field_offsets<'a>(
    iter: impl Iterator<Item = &'a Field>,
    // (field, prev_offset_ident, size_ident, index)
    mut per_field_code: impl FnMut(&Field, &Ident, &Ident, usize) -> TokenStream2, // (code, remaining_offset)
) -> (TokenStream2, syn::Ident) {
    let mut prev_offset_ident = Ident::new("ZERO", Span::call_site());
    let mut code = quote!(const ZERO: usize = 0);

    for (i, field) in iter.enumerate() {
        let ty = &field.ty;
        let new_offset_ident = suffixed_ident("OFFSET", i, field.span());
        let size_ident = suffixed_ident("SIZE", i, field.span());
        let pf_code = per_field_code(field, &prev_offset_ident, &size_ident, i);
        code = quote! {
            #code;
            const #size_ident: usize = ::core::mem::size_of::<#ty>();
            const #new_offset_ident: usize = #prev_offset_ident + #size_ident;
            #pf_code;
        };

        prev_offset_ident = new_offset_ident;
    }

    (code, prev_offset_ident)
}

pub fn field_accessor(f: &Field, index: usize) -> TokenStream2 {
    if let Some(ref i) = f.ident {
        quote!(#i)
    } else {
        let index = Index::from(index);
        quote!(#index)
    }
}

pub fn field_setter(f: &Field) -> TokenStream2 {
    if let Some(ref i) = f.ident {
        quote!(#i: )
    } else {
        quote!()
    }
}

/// Extracts a single `zerovec::name` attribute
pub fn extract_zerovec_attribute_named<'a>(
    attrs: &mut Vec<Attribute>,
    name: &str,
) -> Option<Attribute> {
    let mut ret = None;
    attrs.retain(|a| {
        // skip the "zerovec" part
        let second_segment = a.path.segments.iter().skip(1).next();

        if let Some(second) = second_segment {
            if second.ident == name {
                if ret.is_none() {
                    ret = Some(a.clone());
                    return false;
                }
            }
        }

        true
    });
    ret
}

/// Removes all attributes with `zerovec` in the name and places them in a separate vector
pub fn extract_zerovec_attributes(attrs: &mut Vec<Attribute>) -> Vec<Attribute> {
    let mut ret = vec![];
    attrs.retain(|a| {
        if a.path.segments.len() == 2 && a.path.segments[0].ident == "zerovec" {
            ret.push(a.clone());
            return false;
        }
        return true;
    });
    ret
}

pub fn check_attr_empty(attr: &Option<Attribute>, name: &str) -> Result<()> {
    if let Some(ref attr) = *attr {
        if !attr.tokens.is_empty() {
            return Err(Error::new(
                attr.span(),
                format!("#[zerovec::{name}] does not support arguments"),
            ));
        }
    }
    Ok(())
}

/// Removes all known zerovec:: attributes from attrs and validates them
/// Returns (skip_kv, skip_ord)
pub fn extract_attributes_common(attrs: &mut Vec<Attribute>, name: &str) -> Result<(bool, bool)> {
    let mut zerovec_attrs = extract_zerovec_attributes(attrs);

    let skip_kv = extract_zerovec_attribute_named(&mut zerovec_attrs, "skip_kv");
    let skip_ord = extract_zerovec_attribute_named(&mut zerovec_attrs, "skip_ord");

    if let Some(attr) = zerovec_attrs.get(0) {
        return Err(Error::new(
            attr.span(),
            format!("Found unknown or duplicate attribute for #[{name}]"),
        ));
    }

    check_attr_empty(&skip_kv, "skip_kv")?;
    check_attr_empty(&skip_ord, "skip_ord")?;

    let skip_kv = skip_kv.is_some();
    let skip_ord = skip_ord.is_some();

    Ok((skip_kv, skip_ord))
}
