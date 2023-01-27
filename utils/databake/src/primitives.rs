// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
        )*
    }
}

literal!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, &str, char, bool,
);

#[test]
fn literal() {
    test_bake!(
        u16,
        const: 3849u16
    );
}

impl<'a, T: ?Sized> Bake for &'a T
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        if core::mem::size_of::<Self>() == core::mem::size_of::<&[u8]>()
            && core::any::type_name::<Self>() == core::any::type_name::<&[u8]>()
        {
            let bytes = syn::LitByteStr::new(
                unsafe { core::mem::transmute_copy(self) },
                proc_macro2::Span::call_site(),
            );
            return quote!(#bytes);
        }
        let t = <T as Bake>::bake(*self, ctx);
        quote! {
            &#t
        }
    }
}

#[test]
fn r#ref() {
    test_bake!(
        &f32,
        const: &934.34f32
    );
}

impl<'a, T> Bake for [T]
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        if core::mem::size_of::<T>() == core::mem::size_of::<u8>()
            && core::any::type_name::<T>() == core::any::type_name::<u8>()
        {
            let bytes = syn::LitByteStr::new(
                unsafe { core::mem::transmute(self) },
                proc_macro2::Span::call_site(),
            );
            return quote!(*#bytes);
        }
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            [#(#data,)*]
        }
    }
}

#[test]
fn slice() {
    // Cannot use test_bake! as it's not possible to write a closed slice expression (&[1] has type &[usize; 1])
    let slice: &[bool] = &[true, false];
    assert_eq!(
        Bake::bake(&slice, &Default::default()).to_string(),
        "& [true , false ,]",
    );
}

impl<'a, T, const N: usize> Bake for [T; N]
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        self.as_slice().bake(ctx)
    }
}

#[test]
fn array() {
    // https://github.com/rust-lang/rust/issues/98906
    // test_bake!(
    //     &[bool; 2],
    //     const: &[true, false,]
    // );
    assert_eq!(
        [true, false].bake(&Default::default()).to_string(),
        "[true , false ,]",
    );
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

#[test]
fn option() {
    test_bake!(
        Option<&'static str>,
        const: Some("hello")
    );
    test_bake!(
        Option<&'static str>,
        const: None
    );
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

#[test]
fn result() {
    test_bake!(
        Result<&'static str, ()>,
        const: Ok("hello")
    );
    test_bake!(
        Result<&'static str, ()>,
        const: Err(())
    );
}

macro_rules! tuple {
    ($($ty:ident, $ident:ident),*) => {
        impl<$($ty),*> Bake for ($($ty,)*) where $($ty: Bake),* {
            fn bake(&self, _ctx: &CrateEnv) -> TokenStream {
                let ($($ident,)*) = self;
                $(
                    let $ident = $ident.bake(_ctx);
                )*
                quote! {
                    ($(#$ident,)*)
                }
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
    // https://github.com/rust-lang/rust/issues/98906
    // test_bake!(
    //     (u8, i8),
    //     const: (0u8, 0i8,)
    // );
    assert_eq!(
        (0u8, 0i8).bake(&Default::default()).to_string(),
        "(0u8 , 0i8 ,)",
    );
}
