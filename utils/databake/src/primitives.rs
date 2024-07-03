// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::marker::PhantomData;

use super::*;

macro_rules! literal {
    ($($type:ty,)*) => {
        $(
            impl Bake for $type {
                fn bake(&self, _: &CrateEnv) -> TokenStream {
                    quote! {
                        #self
                    }
                }
            }

            impl BakeSize for $type {
                fn borrows_size(&self) -> usize {
                    0
                }
            }
        )*
    }
}

literal!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, bool,);

#[test]
fn literal() {
    test_bake!(u16, const, 3849u16);
    assert_eq!(42u16.borrows_size(), 0);
}

impl<'a> Bake for &'a str {
    fn bake(&self, _: &CrateEnv) -> TokenStream {
        quote! {
            #self
        }
    }
}

impl<'a> BakeSize for &'a str {
    fn borrows_size(&self) -> usize {
        self.len()
    }
}

impl<'a, T> Bake for &'a T
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        if core::mem::size_of::<Self>() == core::mem::size_of::<&[u8]>()
            && core::any::type_name::<Self>() == core::any::type_name::<&[u8]>()
        {
            // Safety: `&T` and `&[u8]` have the same size. Note that `self: &&T`, so this copies
            // `&T`, not `T` itself.
            // There are no types smaller than `u8`, so there won't be an alignment or allocation-length
            // issue even if T is not actually `[u8]`.
            return byte_string(unsafe { core::mem::transmute_copy(self) });
        }
        let t = <T as Bake>::bake(*self, ctx);
        quote! {
            &#t
        }
    }
}

impl<'a, T> BakeSize for &'a T
where
    T: BakeSize,
{
    fn borrows_size(&self) -> usize {
        core::mem::size_of_val::<T>(*self) + (*self).borrows_size()
    }
}

#[test]
fn r#ref() {
    test_bake!(&f32, const, &934.34f32);
    assert_eq!(BakeSize::borrows_size(&&934.34f32), 4);
}

impl<'a, T> Bake for &'a [T]
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        if core::mem::size_of::<T>() == core::mem::size_of::<u8>()
            && core::any::type_name::<T>() == core::any::type_name::<u8>()
        {
            // Safety: self.as_ptr()'s allocation is at least self.len() bytes long,
            // initialised, and well-alligned.
            return byte_string(unsafe {
                core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len())
            });
        }
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            &[#(#data),*]
        }
    }
}

impl<'a, T> BakeSize for &'a [T]
where
    T: BakeSize,
{
    fn borrows_size(&self) -> usize {
        std::mem::size_of_val(*self) + self.iter().map(BakeSize::borrows_size).sum::<usize>()
    }
}

fn byte_string(bytes: &[u8]) -> TokenStream {
    let byte_string = proc_macro2::Literal::byte_string(bytes);
    // Before clippy 1.70 there's a bug (https://github.com/rust-lang/rust-clippy/pull/10603) where a byte
    // string like b"\0\\01" would incorrectly trigger this lint. This was due to it swallowing the slash's
    // escape slash when trying to match the first "\0" with another digit, and then seeing b"\01".
    // This workaround is conservative as it doesn't actually check for swallowing, only whether an escaped
    // slash appears before 0[0-7].
    let suppress_octal_false_positive = if bytes
        .windows(3)
        .any(|b| matches!(b, &[b'\\', b'0', b'0'..=b'7']))
    {
        quote!(#[allow(clippy::octal_escapes)])
    } else {
        quote!()
    };
    quote!(#suppress_octal_false_positive #byte_string)
}

#[test]
fn slice() {
    // Cannot use test_bake! as it's not possible to write a closed slice expression (&[1] has type &[usize; 1])
    let slice: &[bool] = &[];
    assert_eq!(Bake::bake(&slice, &Default::default()).to_string(), "& []");
    assert_eq!(BakeSize::borrows_size(&slice), 0);
    let slice: &[bool] = &[true];
    assert_eq!(
        Bake::bake(&slice, &Default::default()).to_string(),
        "& [true]",
    );
    assert_eq!(BakeSize::borrows_size(&slice), 1);
    let slice: &[bool] = &[true, false];
    assert_eq!(
        Bake::bake(&slice, &Default::default()).to_string(),
        "& [true , false]",
    );
    assert_eq!(BakeSize::borrows_size(&slice), 2);
    let slice: &[u8] = b"hello";
    assert_eq!(
        Bake::bake(&slice, &Default::default()).to_string(),
        r#"b"hello""#,
    );
    assert_eq!(BakeSize::borrows_size(&slice), 5);
}

impl<T, const N: usize> Bake for [T; N]
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        if core::mem::size_of::<T>() == core::mem::size_of::<u8>()
            && core::any::type_name::<T>() == core::any::type_name::<u8>()
        {
            // Safety: self.as_ptr()'s allocation is at least self.len() bytes long,
            // initialised, and well-alligned.
            let bytestr =
                byte_string(unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, N) });
            return quote!(*#bytestr);
        }
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            [#(#data),*]
        }
    }
}

impl<T, const N: usize> BakeSize for [T; N]
where
    T: BakeSize,
{
    fn borrows_size(&self) -> usize {
        self.iter().map(BakeSize::borrows_size).sum()
    }
}

#[test]
fn array() {
    test_bake!([bool; 0], const, []);
    test_bake!([bool; 1], const, [true]);
    test_bake!([bool; 2], const, [true, false]);
    assert_eq!(BakeSize::borrows_size(&["hello", "world"]), 10);
    test_bake!([u8; 5], const, *b"hello");
    assert_eq!(BakeSize::borrows_size(b"hello"), 0);
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

impl<T> BakeSize for Option<T>
where
    T: BakeSize,
{
    fn borrows_size(&self) -> usize {
        self.as_ref()
            .map(BakeSize::borrows_size)
            .unwrap_or_default()
    }
}

#[test]
fn option() {
    test_bake!(Option<&'static str>, const, Some("hello"));
    assert_eq!(BakeSize::borrows_size(&Some("hello")), 5);
    test_bake!(Option<&'static str>, const, None);
    assert_eq!(BakeSize::borrows_size(&None::<&'static str>), 0);
}

impl<T, E> Bake for Result<T, E>
where
    T: Bake,
    E: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        match self {
            Ok(ok) => {
                let ok = ok.bake(ctx);
                quote! { Ok(#ok) }
            }
            Err(e) => {
                let e = e.bake(ctx);
                quote! {
                    Err(#e)
                }
            }
        }
    }
}

impl<T, E> BakeSize for Result<T, E>
where
    T: BakeSize,
    E: BakeSize,
{
    fn borrows_size(&self) -> usize {
        self.as_ref()
            .map_or_else(BakeSize::borrows_size, BakeSize::borrows_size)
    }
}

#[test]
fn result() {
    test_bake!(Result<&'static str, ()>, const, Ok("hello"));
    assert_eq!(BakeSize::borrows_size(&Ok::<_, ()>("hello")), 5);
    test_bake!(Result<&'static str, ()>, const, Err(()));
    assert_eq!(BakeSize::borrows_size(&Err::<&'static str, _>("hi")), 2);
}

macro_rules! tuple {
    ($ty:ident, $ident:ident) => {
        impl<$ty> Bake for ($ty,) where $ty: Bake {
            fn bake(&self, ctx: &CrateEnv) -> TokenStream {
                let $ident = self.0.bake(ctx);
                quote! {
                    (#$ident,)
                }
            }
        }

        impl<$ty> BakeSize for ($ty,) where $ty: BakeSize {
            fn borrows_size(&self) -> usize {
                self.0.borrows_size()
            }
        }
    };
    ($($ty:ident, $ident:ident),*) => {
        impl<$($ty),*> Bake for ($($ty,)*) where $($ty: Bake),* {
            fn bake(&self, _ctx: &CrateEnv) -> TokenStream {
                let ($($ident,)*) = self;
                $(
                    let $ident = $ident.bake(_ctx);
                )*
                quote! {
                    ($(#$ident),*)
                }
            }
        }

        impl<$($ty),*> BakeSize for ($($ty,)*) where $($ty: BakeSize),* {
            fn borrows_size(&self) -> usize {
                let ($($ident,)*) = self;
                #[allow(unused_mut)]
                let mut r = 0;
                $(
                    r += BakeSize::borrows_size($ident);
                )*
                r
            }
        }
    }
}

tuple!();
tuple!(A, a);
tuple!(A, a, B, b);
tuple!(A, a, B, b, C, c);
tuple!(A, a, B, b, C, c, D, d);
tuple!(A, a, B, b, C, c, D, d, E, e);
tuple!(A, a, B, b, C, c, D, d, E, e, F, f);
tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g);
tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h);
tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h, I, i);
tuple!(A, a, B, b, C, c, D, d, E, e, F, f, G, g, H, h, I, i, J, j);

#[test]
fn tuple() {
    test_bake!((), const, ());
    assert_eq!(BakeSize::borrows_size(&()), 0);
    test_bake!((u8,), const, (0u8,));
    assert_eq!(BakeSize::borrows_size(&("hi",)), 2);
    test_bake!((u8, i8), const, (0u8, 0i8));
    assert_eq!(BakeSize::borrows_size(&("hi", 8u8)), 2);
}

impl<T> Bake for PhantomData<T> {
    fn bake(&self, _ctx: &CrateEnv) -> TokenStream {
        quote! {
            ::core::marker::PhantomData
        }
    }
}

impl<T> BakeSize for PhantomData<T> {
    fn borrows_size(&self) -> usize {
        0
    }
}

#[test]
fn phantom_data() {
    test_bake!(PhantomData<usize>, const, ::core::marker::PhantomData);
}
