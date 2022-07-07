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
//!     r#"[Some ((18i32 , :: alloc :: borrow :: Cow :: Borrowed ("hi")))]"#,
//! );
//! ```

extern crate alloc;
use alloc::borrow::{Cow, ToOwned};

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
    /// Returns a [`TokenStream`] that would evalutate to `self`.
    ///
    /// Crates that are required for the evaluation of the [`TokenStream`] will be
    /// added to `ctx`.
    fn bake(&self, ctx: &CrateEnv) -> TokenStream;
}

macro_rules! literal {
    ($($type:ty),*) => {
        $(
            impl Bake for $type {
                fn bake(&self, _: &CrateEnv) -> TokenStream {
                    quote! {
                        #self
                    }
                }
            }
        )*
    }
}

literal!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str, char);

impl<'a, T> Bake for &'a [T]
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            &[#(#data),*]
        }
    }
}

impl<'a, T, const N: usize> Bake for [T; N]
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            [#(#data),*]
        }
    }
}

impl<T> Bake for Option<T>
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        match self {
            None => quote! { None },
            Some(t) => {
                let t = t.bake(ctx);
                quote! {
                    Some(#t)
                }
            }
        }
    }
}

macro_rules! tuple {
    ($($ty:ident, $ident:ident),*) => {
        impl<$($ty),*> Bake for ($($ty),*) where $($ty: Bake),* {
            fn bake(&self, ctx: &CrateEnv) -> TokenStream {
                let ($($ident),*) = self;
                $(
                    let $ident = $ident.bake(ctx);
                )*
                quote! {
                    ($(#$ident),*)
                }
            }
        }
    }
}

tuple!(A, a, B, b);
tuple!(A, a, B, b, C, c);
tuple!(A, a, B, b, C, c, D, d);
tuple!(A, a, B, b, C, c, D, d, E, e);

impl<T: ?Sized + ToOwned> Bake for Cow<'_, T>
where
    for<'a> &'a T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("alloc");
        let t = <&T as Bake>::bake(&&**self, ctx);
        quote! {
            ::alloc::borrow::Cow::Borrowed(#t)
        }
    }
}

impl<'a, T> Bake for &'a T
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let t = <T as Bake>::bake(*self, ctx);
        quote! {
            &#t
        }
    }
}

/// This macro tests that an expression evaluates to a value that bakes to the same expression.
///
///
/// ```
/// # use databake::test_bake;
/// test_bake!(18usize);
/// ```
///
///
/// As most output will need to reference its crate, and its not possible to name a crate from
/// within it, the second (optional) parameter can be used to specify the crate name. The `crate`
/// identifier in the original expression will be replaced by this in the expected output.
///
/// This test will pass if `MyStruct::bake` returns `::my_crate::MyStruct(42usize)`:
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
///     crate::MyStruct(42usize),
///     my_crate,
/// );
/// # }
/// ```
///
///
/// A third, optional, parameter is a list of crate names that are expected to be added to the
/// `CrateEnv`. The `crate`-replacement crate will always be checked.
#[macro_export]
macro_rules! test_bake {
    ($expr:expr $(, $krate:ident)? $(, [$($env_crate:ident),+])? $(,)?) => {
        let env = Default::default();
        let bake = $crate::Bake::bake(&($expr), &env).to_string();
        let expected_bake = $crate::quote!($expr).to_string();
        $(
            let expected_bake = expected_bake.replace("crate", concat!(":: ", stringify!($krate)));
        )?
        assert_eq!(bake, expected_bake);

        #[allow(unused_variable)]
        let env = env.into_iter().collect::<std::collections::HashSet<_>>();
        $(
            assert!(env.contains(stringify!($krate)), "Crate {:?} was not added to the CrateEnv", stringify!($krate));
        )?
        $(
            $(
                assert!(env.contains(stringify!($env_crate)), "Crate {:?} was not added to the CrateEnv", stringify!($env_crate));
            )+
        )?
    };
}
