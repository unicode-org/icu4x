// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;
use alloc::borrow::{Cow, ToOwned};

#[doc(hidden)]
pub use proc_macro2::TokenStream;
#[doc(hidden)]
pub use quote::quote;

#[cfg(feature = "derive")]
pub use crabbake_derive::Bakeable;

use std::collections::HashSet;
use std::sync::Mutex;

#[derive(Default)]
pub struct CrateEnv(Mutex<HashSet<&'static str>>);

impl CrateEnv {
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

pub trait Bakeable {
    fn bake(&self, ctx: &CrateEnv) -> TokenStream;
}

macro_rules! literal {
    ($($type:ty),*) => {
        $(
            impl Bakeable for $type {
                fn bake(&self, _: &CrateEnv) -> TokenStream {
                    quote! {
                        #self
                    }
                }
            }
        )*
    }
}

literal!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, &str);

impl<'a, T> Bakeable for &'a [T]
where
    T: Bakeable,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            &[#(#data),*]
        }
    }
}

impl<'a, T, const N: usize> Bakeable for [T; N]
where
    T: Bakeable,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            [#(#data),*]
        }
    }
}

impl<T> Bakeable for Option<T>
where
    T: Bakeable,
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
        impl<$($ty),*> Bakeable for ($($ty),*) where $($ty: Bakeable),* {
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

impl<T: ?Sized + ToOwned> Bakeable for Cow<'_, T>
where
    for<'a> &'a T: Bakeable,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("alloc");
        let t = <&T as Bakeable>::bake(&&**self, ctx);
        quote! {
            ::alloc::borrow::Cow::Borrowed(#t)
        }
    }
}

impl<'a, T> Bakeable for &'a T
where
    T: Bakeable,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let t = <T as Bakeable>::bake(*self, ctx);
        quote! {
            &#t
        }
    }
}

#[test]
fn test_primitives() {
    let val = &[Some((18, Cow::Borrowed("hi")))][..];
    let ctx = CrateEnv::default();
    assert_eq!(
        val.bake(&ctx).to_string(),
        quote! {
            &[Some((18i32, alloc::borrow::Cow::Borrowed("hi")))]
        }
        .to_string()
    );
    assert_eq!(ctx.into_iter().collect(), vec!["alloc"]);
}
