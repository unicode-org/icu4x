// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::utils;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    parse_quote, AttributeArgs, Data, DeriveInput, Error, Field, Fields, GenericArgument, Ident,
    Lifetime, PathArguments, Type,
};

pub fn derive_impl(input: &DeriveInput) -> TokenStream2 {
    if !utils::has_valid_repr(&input.attrs, |r| r == "packed" || r == "transparent") {
        return Error::new(
            input.span(),
            "derive(VarULE) must be applied to a #[repr(packed)] or #[repr(transparent)] type",
        )
        .to_compile_error();
    }
    if input.generics.type_params().next().is_some()
        || input.generics.lifetimes().next().is_some()
        || input.generics.const_params().next().is_some()
    {
        return Error::new(
            input.generics.span(),
            "derive(VarULE) must be applied to a struct without any generics",
        )
        .to_compile_error();
    }
    let struc = if let Data::Struct(ref s) = input.data {
        if s.fields.iter().next().is_none() {
            return Error::new(
                input.span(),
                "derive(VarULE) must be applied to a non-empty struct",
            )
            .to_compile_error();
        }
        s
    } else {
        return Error::new(input.span(), "derive(VarULE) must be applied to a struct")
            .to_compile_error();
    };

    let n_fields = struc.fields.len();

    let sizes = struc.fields.iter().take(n_fields - 1).map(|f| {
        let ty = &f.ty;
        quote!(::core::mem::size_of::<#ty>())
    });

    let (validators, remaining_offset) = if n_fields > 1 {
        // generate ULE validators
        crate::ule::generate_ule_validators(struc.fields.iter().take(n_fields - 1))
    } else {
        // no ULE subfields
        (
            quote!(const ZERO: usize = 0),
            Ident::new("ZERO", Span::call_site()),
        )
    };

    let unsized_field = &struc
        .fields
        .iter()
        .next_back()
        .expect("Already verified that struct is not empty")
        .ty;

    let name = &input.ident;
    let ule_size = Ident::new(
        &format!("__IMPL_VarULE_FOR_{name}_ULE_SIZE"),
        Span::call_site(),
    );

    // Safety (based on the safety checklist on the ULE trait):
    //  1. #name does not include any uninitialized or padding bytes
    //     (achieved by enforcing #[repr(transparent)] or #[repr(packed)] on a struct of only ULE types)
    //  2. #name is aligned to 1 byte.
    //     (achieved by enforcing #[repr(transparent)] or #[repr(packed)] on a struct of only ULE types)
    //  3. The impl of `validate_byte_slice()` returns an error if any byte is not valid.
    //  4. The impl of `validate_byte_slice()` returns an error if the slice cannot be used in its entirety
    //  5. The impl of `from_byte_slice_unchecked()` returns a reference to the same data.
    //  6. The other VarULE methods use the default impl
    //  7. [This impl does not enforce the non-safety equality constraint, it is up to the user to do so, ideally via a custom derive]
    quote! {
        // The size of the ULE section of this type
        const #ule_size: usize = 0 #(+ #sizes)*;
        unsafe impl zerovec::ule::VarULE for #name {
            #[inline]
            fn validate_byte_slice(bytes: &[u8]) -> Result<(), zerovec::ZeroVecError> {

                if bytes.len() < #ule_size {
                    return Err(zerovec::ZeroVecError::parse::<Self>());
                }
                #validators
                debug_assert_eq!(#remaining_offset, #ule_size);
                <#unsized_field as zerovec::ule::VarULE>::validate_byte_slice(&bytes[#remaining_offset..])?;
                Ok(())
            }
            #[inline]
            unsafe fn from_byte_slice_unchecked(bytes: &[u8]) -> &Self {
                // just the unsized part
                let unsized_bytes = &bytes[#ule_size..];
                let unsized_ref = <#unsized_field as zerovec::ule::VarULE>::from_byte_slice_unchecked(unsized_bytes);
                // We should use the pointer metadata APIs here when they are stable: https://github.com/rust-lang/rust/issues/81513
                // For now we rely on all DST metadata being a usize to extract it via a fake slice pointer
                let (_ptr, metadata): (usize, usize) = ::core::mem::transmute(unsized_ref);
                let entire_struct_as_slice: *const [u8] = ::core::slice::from_raw_parts(bytes.as_ptr(), metadata);
                &*(entire_struct_as_slice as *const Self)
            }
        }
    }
}

pub fn make_varule_impl(attr: AttributeArgs, input: DeriveInput) -> TokenStream2 {
    if input.generics.type_params().next().is_some()
        || input.generics.const_params().next().is_some()
        || input.generics.lifetimes().count() > 1
    {
        return Error::new(
            input.generics.span(),
            "#[make_varule] must be applied to a struct without any type or const parameters and at most one lifetime",
        )
        .to_compile_error();
    }

    let lt = input.generics.lifetimes().next();

    if let Some(lt) = lt {
        if lt.colon_token.is_some() || !lt.bounds.is_empty() {
            return Error::new(
                input.generics.span(),
                "#[make_varule] must be applied to a struct without lifetime bounds",
            )
            .to_compile_error();
        }
    }

    let lt = lt.map(|l| &l.lifetime);

    if attr.len() != 1 {
        return Error::new(
            input.span(),
            "#[make_ule] takes one argument for the name of the ULE type it produces",
        )
        .to_compile_error();
    }
    let arg = &attr[0];
    let ule_name: Ident = parse_quote!(#arg);

    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref s) => &s.fields,
        _ => {
            return Error::new(input.span(), "#[make_varule] must be applied to a struct")
                .to_compile_error();
        }
    };

    let last_field = fields.iter().next_back();
    let last_field = if let Some(ref last) = last_field {
        last
    } else {
        return Error::new(
            input.span(),
            "#[make_varule] must be applied to a struct with at least one field",
        )
        .to_compile_error();
    };

    let last_field_info = match LastField::new(&last_field.ty) {
        Ok(o) => o,
        Err(e) => return Error::new(last_field.span(), e).to_compile_error(),
    };

    let mut field_inits = crate::ule::make_ule_fields(fields.iter().take(fields.len() - 1));
    let last_field_ule = last_field_info.varule_ty();
    if let Some(ref lf_ident) = last_field.ident {
        field_inits.push(quote!(#lf_ident: #last_field_ule))
    } else {
        field_inits.push(last_field_ule)
    };

    let semi = utils::semi_for(fields);
    let repr_attr = utils::repr_for(fields);
    let field_inits = utils::wrap_field_inits(&field_inits, fields);

    let varule_struct: DeriveInput = parse_quote!(
        #[repr(#repr_attr)]
        #[derive(PartialEq)]
        struct #ule_name #field_inits #semi
    );

    let derived = derive_impl(&varule_struct);

    let maybe_lt_bound = lt.as_ref().map(|lt| quote!(<#lt>));

    let encode_impl = make_encode_impl(
        fields,
        &varule_struct.data,
        last_field,
        &last_field_info,
        name,
        &ule_name,
        &maybe_lt_bound,
    );

    let zf_impl = make_zf_impl(
        fields,
        last_field,
        &last_field_info,
        name,
        &ule_name,
        lt,
        input.span(),
    );

    quote!(
        #input

        #varule_struct

        #encode_impl

        #zf_impl

        #derived
    )
}

fn make_zf_impl(
    fields: &Fields,
    last_field: &Field,
    last_field_info: &LastField,
    name: &Ident,
    ule_name: &Ident,
    maybe_lt: Option<&Lifetime>,
    span: Span,
) -> TokenStream2 {
    if !last_field_info.has_zf() {
        return quote!();
    }

    let lt = if let Some(ref lt) = maybe_lt {
        lt
    } else {
        return Error::new(
            span,
            "Can only generate ZeroFrom impls for types with lifetimes",
        )
        .to_compile_error();
    };

    let mut field_inits = fields
        .iter()
        .take(fields.len() - 1)
        .enumerate()
        .map(|(i, f)| {
            let ty = &f.ty;
            let accessor = utils::field_accessor(f, i);
            let setter = utils::field_setter(f);
            quote!(#setter <#ty as zerovec::ule::AsULE>::from_unaligned(other.#accessor))
        })
        .collect::<Vec<_>>();

    let last_field_ty = &last_field.ty;
    let last_field_ule_ty = last_field_info.varule_ty();
    let accessor = utils::field_accessor(last_field, fields.len() - 1);
    let setter = utils::field_setter(last_field);

    let zerofrom_trait = quote!(zerovec::__zerovec_internal_reexport::ZeroFrom);

    field_inits.push(quote!(#setter <#last_field_ty as #zerofrom_trait <#lt, #last_field_ule_ty>>::zero_from(&other.#accessor) ));

    let field_inits = utils::wrap_field_inits(&field_inits, fields);

    quote!(
        impl <#lt> #zerofrom_trait <#lt, #ule_name> for #name <#lt> {
            fn zero_from(other: &#lt #ule_name) -> Self {
                Self #field_inits
            }
        }
    )
}

fn make_encode_impl(
    fields: &Fields,
    ule_struct_data: &Data,
    last_field: &Field,
    last_field_info: &LastField,
    name: &Ident,
    ule_name: &Ident,
    maybe_lt_bound: &Option<TokenStream2>,
) -> TokenStream2 {
    let ule_fields = if let Data::Struct(ref s) = *ule_struct_data {
        &s.fields
    } else {
        unreachable!("make_encode_impl() should only be passed a struct that was generated by make_varule_impl()")
    };

    let mut lengths = vec![];

    for field in fields.iter().take(fields.len() - 1) {
        let ty = &field.ty;
        lengths.push(quote!(::core::mem::size_of::<<#ty as zerovec::ule::AsULE>::ULE>()));
    }

    let last_field_name = utils::field_accessor(last_field, fields.len() - 1);

    let ule_iter = ule_fields.iter().take(ule_fields.len() - 1);
    let (encoders, remaining_offset) =
        utils::generate_per_field_offsets(ule_iter, |field, prev_offset_ident, size_ident, i| {
            let ty = &field.ty;
            let accessor = utils::field_accessor(field, i);
            quote!(
                let out = &mut dst[#prev_offset_ident .. #prev_offset_ident + #size_ident];
                let unaligned = zerovec::ule::AsULE::as_unaligned(self.#accessor);
                let unaligned_slice = &[unaligned];
                let src = <#ty as zerovec::ule::ULE>::as_byte_slice(unaligned_slice);
                out.copy_from_slice(src);
            )
        });

    let last_bytes = last_field_info.encode_func(quote!(self.#last_field_name));
    quote!(
        unsafe impl #maybe_lt_bound zerovec::ule::custom::EncodeAsVarULE<#ule_name> for #name #maybe_lt_bound {
            fn encode_var_ule_as_slices<R>(&self, cb: impl FnOnce(&[&[u8]]) -> R) -> R {
                unreachable!("other two methods implemented")
            }

            fn encode_var_ule_len(&self) -> usize {
                #(#lengths +)* #last_bytes.len()
            }
            fn encode_var_ule_write(&self, mut dst: &mut [u8]) {
                debug_assert_eq!(self.encode_var_ule_len(), dst.len());
                #encoders
                let last_bytes = #last_bytes;
                dst[#remaining_offset..].copy_from_slice(last_bytes);
            }
        }
    )
}

/// Represents a VarULE-compatible type that would typically
/// be found behind a `Cow<'a, _>` in the last field, and is represented
/// roughly the same in owned and borrowed versions
#[derive(Clone, Eq, PartialEq)]
enum OwnULETy<'a> {
    /// [T] where T: AsULE<ULE = Self>
    Slice(&'a Type),
    /// str
    Str,
}

/// Represents the type of the last field of the struct
#[derive(Clone, Eq, PartialEq)]
enum LastField<'a> {
    Cow(OwnULETy<'a>),
    ZeroVec(&'a Type),
    VarZeroVec(&'a Type),

    // Generally you should be using the above ones for maximum zero-copy, but these will still work
    Growable(OwnULETy<'a>),
    Boxed(OwnULETy<'a>),
    Ref(OwnULETy<'a>),
}

impl<'a> LastField<'a> {
    /// Construct a LastField for the type of a LastField if possible
    fn new(ty: &'a Type) -> Result<LastField<'a>, String> {
        static PATH_TYPE_IDENTITY_ERROR: &'static str =
            "Can only automatically detect corresponding VarULE types for path types \
            that are Cow, ZeroVec, VarZeroVec, Box, String, or Vec";
        static PATH_TYPE_GENERICS_ERROR: &'static str =
            "Can only automatically detect corresponding VarULE types for path \
            types with at most one lifetime and at most one generic parameter";
        match *ty {
            Type::Reference(ref tyref) => OwnULETy::new(&tyref.elem, "reference").map(LastField::Ref),
            Type::Path(ref typath) => {
                if typath.path.segments.len() != 1 {
                    return Err("Can only automatically detect corresponding VarULE types for \
                                path types with a single path segment".into());
                }
                let segment = typath.path.segments.first().unwrap();
                match segment.arguments {
                    PathArguments::None => {
                        if segment.ident == "String" {
                            Ok(LastField::Growable(OwnULETy::Str))
                        } else {
                            Err(PATH_TYPE_IDENTITY_ERROR.into())
                        }
                    }
                    PathArguments::AngleBracketed(ref params) => {
                        // At most one lifetime and exactly one generic parameter
                        let mut lifetime = None;
                        let mut generic = None;
                        for param in &params.args {
                            match param {
                                GenericArgument::Lifetime(ref lt) if lifetime.is_none() => {
                                    lifetime = Some(lt)
                                }
                                GenericArgument::Type(ref ty) if generic.is_none() => {
                                    generic = Some(ty)
                                }
                                _ => return Err(PATH_TYPE_GENERICS_ERROR.into()),
                            }
                        }

                        // Must be exactly one generic parameter
                        // (we've handled the zero generics case already)
                        let generic = if let Some(g) = generic {
                            g
                        } else {
                            return Err(PATH_TYPE_GENERICS_ERROR.into());
                        };

                        let ident = segment.ident.to_string();

                        if lifetime.is_some() {
                            match &*ident {
                                "ZeroVec" => Ok(LastField::ZeroVec(generic)),
                                "VarZeroVec" => Ok(LastField::VarZeroVec(generic)),
                                "Cow" => OwnULETy::new(generic, "Cow").map(LastField::Cow),
                                _ => Err(PATH_TYPE_IDENTITY_ERROR.into()),
                            }
                        } else {
                            match &*ident {
                                "Vec" => Ok(LastField::Growable(OwnULETy::Slice(generic))),
                                "Box" => OwnULETy::new(generic, "Box").map(LastField::Boxed),
                                _ => Err(PATH_TYPE_IDENTITY_ERROR.into()),
                            }
                        }
                    }
                    _ => Err("Can only automatically detect corresponding VarULE types for path types \
                              with none or angle bracketed generics".into()),
                }
            }
            _ => Err("Can only automatically detect corresponding VarULE types for path and reference types".into()),
        }
    }
    /// Get the tokens for the corresponding VarULE type
    fn varule_ty(&self) -> TokenStream2 {
        match *self {
            Self::Ref(ref inner)
            | Self::Cow(ref inner)
            | Self::Boxed(ref inner)
            | Self::Growable(ref inner) => {
                let inner_ule = inner.varule_ty();
                quote!(#inner_ule)
            }
            Self::ZeroVec(ref inner) => quote!(zerovec::ZeroSlice<#inner>),
            Self::VarZeroVec(ref inner) => quote!(zerovec::VarZeroSlice<#inner>),
        }
    }

    // Takes expr `value` and encodes it into a byte slice
    fn encode_func(&self, value: TokenStream2) -> TokenStream2 {
        match *self {
            Self::Ref(ref inner)
            | Self::Cow(ref inner)
            | Self::Growable(ref inner)
            | Self::Boxed(ref inner) => inner.encode_func(value),

            Self::ZeroVec(_) => quote!(#value.as_bytes()),
            Self::VarZeroVec(_) => quote!(#value.as_bytes()),
        }
    }

    fn has_zf(&self) -> bool {
        matches!(
            *self,
            Self::Ref(_) | Self::Cow(_) | Self::ZeroVec(_) | Self::VarZeroVec(_)
        )
    }
}

impl<'a> OwnULETy<'a> {
    fn new(ty: &'a Type, context: &str) -> Result<Self, String> {
        match *ty {
            Type::Slice(ref slice) => Ok(OwnULETy::Slice(&slice.elem)),
            Type::Path(ref typath) => {
                if typath.path.is_ident("str") {
                    Ok(OwnULETy::Str)
                } else {
                    Err(format!("Cannot automatically detect corresponding VarULE type for non-str path type inside a {context}"))
                }
            }
            _ => Err(format!("Cannot automatically detect corresponding VarULE type for non-slice/path type inside a {context}")),
        }
    }

    /// Get the tokens for the corresponding VarULE type
    fn varule_ty(&self) -> TokenStream2 {
        match *self {
            OwnULETy::Slice(s) => quote!([#s]),
            OwnULETy::Str => quote!(str),
        }
    }
    // Takes expr `value` and encodes it into a byte slice
    fn encode_func(&self, value: TokenStream2) -> TokenStream2 {
        match *self {
            OwnULETy::Slice(s) => {
                quote!(<[#s] as zerovec::ule::VarULE>::as_byte_slice(&*#value))
            }
            OwnULETy::Str => {
                quote!(#value.as_bytes())
            }
        }
    }
}
