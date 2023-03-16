// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A serde serialization strategy that uses `PartialEq` to reduce serialized size.
//!
//! This create can be used with Serde derive like this:
//!
//! ```rust
//! # #[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
//! # struct Bar(String);
//!
//! #[derive(serde::Deserialize, serde::Serialize)]
//! pub struct Foo {
//!     #[serde(with = "deduplicating_array")]
//!     data: [Bar; 12],
//!     // ...
//! }
//! ```
//!
//! `Bar`s that are equal to a `Bar`s that appears earlier in the array will not be serialized
//! (instead, the index of the first occurence is serialized). Deserialization clones the first
//! `Bar` into all the indices where it occurs (hence `Bar` has to implement `Clone`).
//!
//! Human readable serialization represents skipped values as singleton arrays containing the
//! target index, e.g. the Rust array `["Foo", "Bar", "Foo"]` will serialize to JSON `["Foo", "Bar", [0]]`.
//!
//! This implies that singleton integer arrays cannot be used as array elements (they do work in Bincode,
//! but there's really not much point in using them).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(test), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]

extern crate alloc;

use alloc::fmt::{Error, Formatter};
use alloc::format;
use serde::de::{Deserialize, Deserializer, EnumAccess, Visitor};
use serde::ser::{Serialize, Serializer};

pub fn serialize<S, T, const N: usize>(array: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + PartialEq,
{
    use serde::ser::SerializeTuple;

    let human = serializer.is_human_readable();

    let mut seq = serializer.serialize_tuple(N)?;

    for i in 0..N {
        #[allow(clippy::indexing_slicing)] // i, j in 0..N
        match array.iter().take(i).position(|item| item == &array[i]) {
            None if human => seq.serialize_element(&HumanSer::Value(&array[i]))?,
            None => seq.serialize_element(&MachineSer::Value(&array[i]))?,
            Some(j) if human => seq.serialize_element(&HumanSer::<T>::Fallback([j]))?,
            Some(j) => seq.serialize_element(&MachineSer::<T>::Fallback(j))?,
        }
    }
    seq.end()
}

pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Clone,
    [HumanDe<T>; N]: Deserialize<'de>,
    [MachineDe<T>; N]: Deserialize<'de>,
{
    use core::mem::MaybeUninit;
    use serde::de::Error;

    let mut array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    if deserializer.is_human_readable() {
        for (i, r) in <[HumanDe<T>; N]>::deserialize(deserializer)?
            .into_iter()
            .enumerate()
        {
            match r {
                HumanDe::Value(v) => {
                    #[allow(clippy::indexing_slicing)] // i in 0..N by enumerate
                    array[i].write(v);
                }
                HumanDe::Fallback([j]) => unsafe {
                    // Fallbacks should always be to a previous value,
                    // which makes the assume_init_ref safe
                    if j >= i {
                        return Err(D::Error::custom(format!(
                            "Illegal forward fallback {i}->{j}",
                        )));
                    }
                    #[allow(clippy::indexing_slicing)] // j < i in 0..N by enumerate
                    array[i].write(array[j].assume_init_ref().clone());
                },
            }
        }
    } else {
        for (i, r) in
            IntoIterator::into_iter(<[MachineDe<T>; N]>::deserialize(deserializer)?).enumerate()
        {
            match r {
                MachineDe::Value(v) => {
                    #[allow(clippy::indexing_slicing)] // i in 0..N by enumerate
                    array[i].write(v);
                }
                MachineDe::Fallback(j) => unsafe {
                    // Fallbacks should always be to a previous value,
                    // which makes the assume_init_ref safe
                    if j >= i {
                        return Err(D::Error::custom(format!(
                            "Illegal forward fallback {i}->{j}",
                        )));
                    }
                    #[allow(clippy::indexing_slicing)] // j < i in 0..N by enumerate
                    array[i].write(array[j].assume_init_ref().clone());
                },
            }
        }
    }

    // https://github.com/rust-lang/rust/issues/61956
    Ok(unsafe { core::ptr::read(array.as_ptr() as *const [T; N]) })
}

#[derive(serde::Serialize)]
#[serde(untagged)]
enum HumanSer<'a, T> {
    Value(&'a T),
    Fallback([usize; 1]),
}

enum MachineSer<'a, T> {
    Value(&'a T),
    Fallback(usize),
}

// These enums are public because they are exposed in the trait bounds of
// deserialize. Serde only implements Deserialize<'de> on arrays up to
// size 32, so [Dedupe<T>;N]: Deserialize<'de> cannot be inferred from
// Dedupe<T>: Deserialize<'de> in the general case. See
// https://github.com/serde-rs/serde/issues/1937.
//
// These are not considered part of the stable API.

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
#[doc(hidden)]
#[allow(clippy::exhaustive_enums)] // internal type
pub enum HumanDe<T> {
    Value(T),
    Fallback([usize; 1]),
}

#[doc(hidden)]
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)] // internal type
pub enum MachineDe<T> {
    Value(T),
    Fallback(usize),
}

impl<'a, T> Serialize for MachineSer<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        match self {
            // Serialize values as an enum variant with index 0
            MachineSer::Value(t) => serializer.serialize_newtype_variant(
                "unused-enum-name",
                0,
                "unused-enum-variant",
                t,
            ),
            // Serialize fallbacks as an (empty) variant with index = fallback + 1
            MachineSer::Fallback(fallback) => serializer.serialize_unit_variant(
                "unused-enum-name",
                (fallback + 1) as u32,
                "unused-enum-variant",
            ),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for MachineDe<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DedupeVisitor<R>(core::marker::PhantomData<R>);

        impl<'de, R: Deserialize<'de>> Visitor<'de> for DedupeVisitor<R> {
            type Value = MachineDe<R>;

            fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
                formatter.write_str("Element or fallback reference.")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                use serde::de::VariantAccess;

                let (variant, variant_access) = data.variant()?;
                Ok(match variant {
                    0 => MachineDe::Value(variant_access.newtype_variant()?),
                    n => MachineDe::Fallback(n - 1),
                })
            }
        }

        deserializer.deserialize_enum(
            "unused-enum-name",
            &[],
            DedupeVisitor(core::marker::PhantomData),
        )
    }
}

#[cfg(test)]
mod test {
    use alloc::borrow::Cow;
    use alloc::string::ToString;
    use serde::*;

    // Putting a Cow directly into the array doesn't borrow
    // for some reason, even with default array deserialization
    // (maybe https://github.com/serde-rs/serde/issues/2016).
    // This extra layer lets us test that borrowing works.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Foo<'data>(#[serde(borrow)] Cow<'data, str>);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct<'data>(#[serde(borrow, with = "super")] [Foo<'data>; 3]);

    static STRUCT: TestStruct = TestStruct([
        Foo(Cow::Borrowed("Bar")),
        Foo(Cow::Borrowed("Batz")),
        Foo(Cow::Borrowed("Bar")),
    ]);

    #[test]
    fn test_json() {
        let json = r#"["Bar","Batz",[0]]"#;

        assert_eq!(serde_json::to_string(&STRUCT).unwrap(), json);

        assert_eq!(serde_json::from_str::<TestStruct>(json).unwrap(), STRUCT);
    }

    #[test]
    fn test_postcard() {
        #[rustfmt::skip]
        let bytes = &[
            0, // [0]
                3, // "Bar"
                    66, // B
                    97, // a
                    114, // r
            0, // [1]
                4, // "Batz"
                    66, // B
                    97, // a
                    116, // t
                    122, // z
            1, // [2] => [0]
        ];

        assert_eq!(postcard::to_allocvec(&STRUCT).unwrap(), bytes);

        let de_struct = postcard::from_bytes::<TestStruct>(bytes).unwrap();

        assert_eq!(de_struct, STRUCT);
        assert!(matches!(de_struct.0[0].0, Cow::Borrowed(_)));
    }

    #[test]
    fn test_forward_fallback() {
        assert_eq!(
            serde_json::from_str::<TestStruct>(r#"[[1], "Foo", "Batz"]"#)
                .unwrap_err()
                .to_string(),
            "Illegal forward fallback 0->1"
        );
    }
}
