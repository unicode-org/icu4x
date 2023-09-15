// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This crate allows data to write itself into Rust code (bake itself in).
//!
//! Types that implement the `Bake` trait can be written into Rust expressions,
//! which allows using Rust code itself as a zero-overhead "serialization" strategy.
//!
//! # Example
//! ```
//! use databake::*;
//! # extern crate alloc;
//! use alloc::borrow::Cow;
//!
//! let data = [Some((18, Cow::Borrowed("hi")))];
//! assert_eq!(
//!     data.bake(&Default::default()).to_string(),
//!     r#"[Some ((18i32 , alloc :: borrow :: Cow :: Borrowed ("hi") ,)) ,]"#,
//! );
//! ```
//!
//! # Derive
//!
//! `Bake` can be automatically derived if the `derive` Cargo feature is enabled.
//!
//! ```
//! use databake::*;
//!
//! #[derive(Bake)]
//! #[databake(path = my_crate)]
//! struct MyStruct {
//!     number: u32,
//!     string: &'static str,
//!     slice: &'static [bool],
//! }
//!
//! #[derive(Bake)]
//! #[databake(path = my_crate)]
//! struct AnotherOne(MyStruct, char);
//! ```
//!
//! # Testing
//! The [`test_bake`] macro can be uses to assert that a particular expression is a `Bake` fixed point.
//!
//! ```no_run
//! # // https://github.com/rust-lang/rust/issues/98906
//! # use databake::*;
//! # #[derive(Bake)]
//! # #[databake(path = my_crate)]
//! # struct MyStruct {
//! #   number: u32,
//! #   string: &'static str,
//! #   slice: &'static [bool],
//! # }
//! # #[derive(Bake)]
//! # #[databake(path = my_crate)]
//! # struct AnotherOne(MyStruct, char);
//! # fn main() {
//! test_bake!(
//!     AnotherOne,
//!     const: crate::AnotherOne(
//!         crate::MyStruct {
//!           number: 17u32,
//!           string: "foo",
//!           slice: &[true, false],
//!         },
//!         'b',
//!     ),
//!     my_crate,
//! );
//! # }
//! ```

mod alloc;
mod primitives;

#[doc(no_inline)]
pub use proc_macro2::TokenStream;

#[doc(no_inline)]
pub use quote::quote;

#[cfg(feature = "derive")]
pub use databake_derive::Bake;

use std::collections::HashSet;
use std::sync::Mutex;

/// A collection of crates that are required for the evaluation of some expression.
#[derive(Default)]
pub struct CrateEnv(Mutex<HashSet<&'static str>>);

impl CrateEnv {
    /// Adds a crate to this collection. This can be called concurrently
    /// and without `mut`.
    pub fn insert(&self, krate: &'static str) {
        self.0.lock().expect("poison").insert(krate);
    }
}

impl IntoIterator for CrateEnv {
    type Item = &'static str;
    type IntoIter = <HashSet<&'static str> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_inner().expect("poison").into_iter()
    }
}

/// The `Bake` trait allows a piece of data to write itself into a Rust expression.
///
/// This can be used to generate files with hardcoded data.
pub trait Bake {
    /// Returns a [`TokenStream`] that would evaluate to `self`.
    ///
    /// Crates that are required for the evaluation of the [`TokenStream`] will be
    /// added to `ctx`.
    fn bake(&self, ctx: &CrateEnv) -> TokenStream;
}

/// This macro tests that an expression evaluates to a value that bakes to the same expression.
///
/// Its mandatory arguments are a type and an expression (of that type).
///
/// ```
/// # use databake::test_bake;
/// test_bake!(usize, 18usize);
/// ```
///
/// ## `Const`
///
/// We usually want baked output to be const constructible. To test this, add the `const:` prefix to
/// the expression:
///
/// ```
/// # use databake::test_bake;
/// test_bake!(usize, const: 18usize);
/// ```
///
/// ## Crates and imports
///
/// As most output will need to reference its crate, and its not possible to name a crate from
/// within it, a third parameter can be used to specify the crate name. The `crate` identifier
/// in the original expression will be replaced by this in the expected output.
///
/// ```no_run
/// # use databake::*;
/// # struct MyStruct(usize);
/// # impl Bake for MyStruct {
/// #   fn bake(&self, _: &CrateEnv) -> TokenStream { unimplemented!() }
/// # }
/// # // We need an explicit main to put the struct at the crate root
/// # fn main() {
/// test_bake!(
///     MyStruct,
///     crate::MyStruct(42usize), // matches `::my_crate::MyStruct(42usize)`
///     my_crate,
/// );
/// # }
/// ```
///
/// A fourth, optional, parameter is a list of crate names that are expected to be added to the
/// `CrateEnv`. The `crate`-replacement crate will always be checked.
#[macro_export]
macro_rules! test_bake {
    ($type:ty, const: $expr:expr $(, $krate:ident)? $(, [$($env_crate:ident),+])? $(,)?) => {
        const _: &$type = &$expr;
        $crate::test_bake!($type, $expr $(, $krate)? $(, [$($env_crate),+])?);
    };

    ($type:ty, $expr:expr $(, $krate:ident)? $(, [$($env_crate:ident),+])? $(,)?) => {
        let env = Default::default();
        let expr: &$type = &$expr;
        let bake = $crate::Bake::bake(expr, &env).to_string();
        let expected_bake = $crate::quote!($expr).to_string();
        $(
            let expected_bake = expected_bake.replace("crate", stringify!($krate));
        )?
        assert_eq!(bake, expected_bake);

        #[allow(unused_variables)]
        let _env = env.into_iter().collect::<std::collections::HashSet<_>>();
        $(
            assert!(_env.contains(stringify!($krate)), "Crate {:?} was not added to the CrateEnv", stringify!($krate));
        )?
        $(
            $(
                assert!(_env.contains(stringify!($env_crate)), "Crate {:?} was not added to the CrateEnv", stringify!($env_crate));
            )+
        )?
    };
}
