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
/// This can be attached to `Copy` structs containing only ULE types.
///
/// Most of the time, it is recommended one use [`#[make_ule]`](macro@make_ule) instead of defining
/// a custom ULE type.
#[proc_macro_derive(ULE)]
pub fn ule_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(ule::derive_impl(&input))
}

/// Custom derive for `zerovec::VarULE`,
///
/// This can be attached to structs containing only ULE types with one VarULE type at the end.
///
/// Most of the time, it is recommended one use [`#[make_varule]`](macro@make_varule) instead of defining
/// a custom VarULE type.
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
///
/// For enums, this implementation will generate a crate-public `fn new_from_u8(value: u8) -> Option<Self>`
/// method on the main type that allows one to construct the value from a u8. If this method is desired
/// to be more public, it should be wrapped.
///
/// # Example
///
/// ```rust
/// use zerovec::ZeroVec;
///
/// #[zerovec::make_ule(DateULE)]
/// #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
/// struct Date {
///     y: u64,
///     m: u8,
///     d: u8
/// }
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Dates<'a> {
///     #[serde(borrow)]
///     dates: ZeroVec<'a, Date>   
/// }
///
/// let dates = Dates { dates: ZeroVec::alloc_from_slice(&[Date {y: 1985, m: 9, d: 3}, Date {y: 1970, m: 2, d: 20}, Date {y: 1990, m: 6, d: 13}]) };
///
/// let bincode_bytes = bincode::serialize(&dates)
///     .expect("Serialization should be successful");
///
/// // Will deserialize without allocations
/// let deserialized: Dates = bincode::deserialize(&bincode_bytes)
///     .expect("Deserialization should be successful");
///
/// assert_eq!(deserialized.dates.get(1).unwrap().y, 1970);
/// assert_eq!(deserialized.dates.get(2).unwrap().d, 13);
///
/// ```
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
/// `EncodeAsVarULE` and `ZeroFrom` are useful for avoiding the need to deal with the VarULE type directly. In particular,
/// it is recommended to use `ZeroFrom` to convert the VarULE type back to this type in a cheap, zero-copy way (see the example
/// for more details).
///
/// Provided the type implements `serde::Serialize` and `serde::Deserialize`, this attribute can also generate
/// the relevant serialize/deserialize implementations for the `VarULE` type if you apply the `#[zerovec::serde]`
/// attribute. Those impls are required to support human-readable serialization of the VarZeroVec.
/// This needs the `serde` feature to be enabled on the `zerovec` crate to work.
///
/// By default this attribute will also autogenerate a `ZeroMapKV` implementation, which requires
/// `Ord` and `PartialOrd` on the `VarULE` type. You can opt out of this with `#[zerovec::skip_kv]`.
///
/// This implementation will also by default autogenerate `Ord` and `PartialOrd` on the VarULE type based on
/// the implementation on `Self`. You can opt out of this with `#[zerovec::skip_ord]`
///
/// Note that this implementation will autogenerate `EncodeAsVarULE` impls for _both_ `Self` and `&Self`
/// for convenience. This allows for a little more flexibility in how things are encoded.
///
/// # Example
///
/// ```rust
/// use zerovec::{ZeroVec, VarZeroVec, ZeroMap};
/// use std::borrow::Cow;
/// use zerovec::ule::encode_varule_to_box;
/// use zerofrom::ZeroFrom;
///
/// // custom fixed-size ULE type for ZeroVec
/// #[zerovec::make_ule(DateULE)]
/// #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
/// struct Date {
///     y: u64,
///     m: u8,
///     d: u8
/// }
///
/// // custom variable sized VarULE type for VarZeroVec
/// #[zerovec::make_varule(PersonULE)]
/// #[zerovec::serde]
/// #[derive(Clone, PartialEq, Eq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
/// struct Person<'a> {
///     birthday: Date,
///     favorite_character: char,
///     #[serde(borrow)]
///     name: Cow<'a, str>,
/// }
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Data<'a> {
///     // note: VarZeroVec always must reference the ULE type directly
///     #[serde(borrow)]
///     important_people: VarZeroVec<'a, PersonULE>,
/// }
///
///
/// let person1 = Person {
///     birthday: Date { y: 1990, m: 9, d: 7},
///     favorite_character: 'π',
///     name: Cow::from("Kate")
/// };
/// let person2 = Person {
///     birthday: Date { y: 1960, m: 5, d: 25},
///     favorite_character: '冇',
///     name: Cow::from("Jesse")
/// };
///
/// let important_people = VarZeroVec::from(&[person1, person2]);
/// let data = Data { important_people };
///
/// let bincode_bytes = bincode::serialize(&data)
///     .expect("Serialization should be successful");
///
/// // Will deserialize without allocations
/// let deserialized: Data = bincode::deserialize(&bincode_bytes)
///     .expect("Deserialization should be successful");
///
/// assert_eq!(&deserialized.important_people.get(1).unwrap().name, "Jesse");
/// assert_eq!(&deserialized.important_people.get(0).unwrap().name, "Kate");
///
/// // Since VarZeroVec produces PersonULE types, it's convenient to use ZeroFrom
/// // to recoup Person values in a zero-copy way
/// let person_converted: Person = ZeroFrom::zero_from(deserialized.important_people.get(1).unwrap());
/// assert_eq!(person_converted.name, "Jesse");
/// assert_eq!(person_converted.birthday.y, 1960);
/// ```
#[proc_macro_attribute]
pub fn make_varule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let attr = parse_macro_input!(attr as AttributeArgs);
    TokenStream::from(varule::make_varule_impl(attr, input))
}
