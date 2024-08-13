// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Helper structs to generate a different output than given input.
//!
//! This allows e.g. to store a `String` but output a `&'static str`.

use crate::{Bake, BakeSize, CrateEnv};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use proc_macro2::TokenStream;
use quote::quote;

/// Let [`Bake`] output a `&'static str` for anything that can be `AsRef<str>`.
///
/// It can be helpful if the type needs to be constructed (e.g. [`String`], [`Cow<str>`](std::borrow::Cow)) but should result in a `&'static str`.
///
/// This requires that the crate using the generated data needs a different struct.
///
/// ```
/// use databake::{Bake, converter::AsStaticStr};
///
/// #[derive(Bake)]
/// #[databake(path = my_crate)]
/// struct Data {
///     number: usize,
///     string: AsStaticStr<String>, // can be written as StringAsStaticStr
/// }
///
/// let data = Data {
///     number: 6,
///     string: 6.to_string().into()
/// };
///
/// assert_eq!(
///     data.bake(&Default::default()).to_string(),
///     r#"my_crate :: Data { number : 6usize , string : "6" , }"#
/// );
///
/// mod my_crate {
///     struct Data {
///         number: usize,
///         string: &'static str,
///     }
/// }
/// ```
#[derive(Default)]
#[repr(transparent)]
pub struct AsStaticStr<T>(pub T)
where
    T: AsRef<str>;

impl<T> AsStaticStr<T>
where
    T: AsRef<str>,
{
    #[inline]
    pub fn into(self) -> T {
        self.0
    }
}

impl<T> Bake for AsStaticStr<T>
where
    T: AsRef<str>,
{
    fn bake(&self, _ctx: &CrateEnv) -> TokenStream {
        let value = &self.0.as_ref();
        quote!(#value)
    }
}

impl<T> BakeSize for AsStaticStr<T>
where
    T: AsRef<str>,
{
    fn borrows_size(&self) -> usize {
        self.0.as_ref().len()
    }
}

impl<T> Deref for AsStaticStr<T>
where
    T: AsRef<str>,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for AsStaticStr<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for AsStaticStr<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[test]
fn as_static_str() {
    use std::borrow::Cow;

    assert_eq!(
        AsStaticStr(Cow::<str>::Owned("hi".to_string()))
            .bake(&Default::default())
            .to_string(),
        r#""hi""#
    );
}

/// Alias for [`AsStaticStr<String>`].
pub type StringAsStaticStr = AsStaticStr<String>;

/// Let [`Bake`] output a `&'static [T]` for anything that can be iterated over.
///
/// It can be helpful if the type needs to be constructed (e.g. [`Vec<T>`]) but should result in a `&'static [T]`.
///
/// This requires that the crate using the generated data needs a different struct.
///
/// ```
/// use databake::{converter::IteratorAsRefSlice, Bake};
///
/// #[derive(Bake, Default)]
/// #[databake(path = my_crate)]
/// struct Data {
///     numbers: IteratorAsRefSlice<Vec<usize>, usize>, // can be written as `VecAsRefSlice<usize>`
/// }
///
/// let mut data = Data::default();
/// data.numbers.push(6);
///
/// assert_eq!(
///     data.bake(&Default::default()).to_string(),
///     r#"my_crate :: Data { numbers : & [6usize ,] , }"#
/// );
///
/// mod my_crate {
///     struct Data {
///         numbers: &'static [usize],
///     }
/// }
/// ```
#[derive(Default)]
#[repr(transparent)]
pub struct IteratorAsRefSlice<B, T>(pub B, pub PhantomData<T>)
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake;

impl<B, T> IteratorAsRefSlice<B, T>
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake,
{
    #[inline]
    pub fn into(self) -> B {
        self.0
    }
}

impl<B, T> Bake for IteratorAsRefSlice<B, T>
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        let mut inner = TokenStream::new();
        for e in self.0.into_iter() {
            let e = e.bake(ctx);
            inner.extend(quote! {#e,});
        }

        quote! {&[#inner]}
    }
}

impl<B, T> BakeSize for IteratorAsRefSlice<B, T>
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: BakeSize,
{
    fn borrows_size(&self) -> usize {
        self.0.into_iter().map(|x| x.borrows_size()).sum()
    }
}

impl<B, T> Deref for IteratorAsRefSlice<B, T>
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake,
{
    type Target = B;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B, T> DerefMut for IteratorAsRefSlice<B, T>
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<B, T> From<B> for IteratorAsRefSlice<B, T>
where
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake,
{
    #[inline]
    fn from(value: B) -> Self {
        Self(value, PhantomData)
    }
}

impl<B, T> FromIterator<T> for IteratorAsRefSlice<B, T>
where
    B: FromIterator<T>,
    for<'a> &'a B: IntoIterator<Item = &'a T>,
    T: Bake,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect(), PhantomData)
    }
}

#[test]
fn as_ref_slice() {
    use std::collections::HashSet;

    let mut set = IteratorAsRefSlice::<HashSet<usize>, usize>::default();
    set.insert(42);
    assert_eq!(
        set.bake(&Default::default()).to_string(),
        r#"& [42usize ,]"#
    )
}

/// Alias for [`IteratorAsRefSlice<Vec<T>, T>`].
pub type VecAsRefSlice<T> = IteratorAsRefSlice<Vec<T>, T>;

#[test]
fn vec_as_ref_slice() {
    assert_eq!(
        VecAsRefSlice::from(vec![6u8])
            .bake(&Default::default())
            .to_string(),
        r#"& [6u8 ,]"#
    )
}
