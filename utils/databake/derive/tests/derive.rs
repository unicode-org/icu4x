// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

use databake::*;

#[derive(Bake)]
#[databake(path = test)]
pub struct IntExample {
    x: u8,
}

#[test]
fn test_int_example() {
    test_bake!(IntExample, const, crate::IntExample { x: 17u8 }, test,);
}

#[derive(Bake)]
#[databake(path = test)]
pub struct GenericsExample<T> {
    x: u32,
    y: T,
}

#[test]
fn test_generics_example() {
    test_bake!(
        GenericsExample<isize>,
        const,
        crate::GenericsExample {
            x: 17u32,
            y: 100isize
        },
        test
    );
}

#[derive(Bake)]
#[databake(path = test)]
pub struct CowExample<'a> {
    x: u8,
    y: &'a str,
    z: alloc::borrow::Cow<'a, str>,
    w: alloc::borrow::Cow<'a, [u8]>,
}

#[test]
fn test_cow_example() {
    test_bake!(
        CowExample<'static>,
        const,
        crate::CowExample {
            x: 17u8,
            y: "foo",
            z: alloc::borrow::Cow::Borrowed("bar"),
            w: alloc::borrow::Cow::Borrowed(b"\x01\x02\x03"),
        },
        test
    );
}

#[derive(Bake)]
#[databake(path = test)]
#[databake(custom_bake)]
pub struct CustomBakeExample<'a> {
    x: usize,
    y: alloc::borrow::Cow<'a, str>,
}

impl CustomBake for CustomBakeExample<'_> {
    type BakedType<'a>
        = (usize, &'a str)
    where
        Self: 'a;
    fn to_custom_bake(&self) -> Self::BakedType<'_> {
        (self.x, &*self.y)
    }
}

impl<'a> CustomBakeExample<'a> {
    pub const fn from_custom_bake(baked: <Self as CustomBake>::BakedType<'a>) -> Self {
        Self {
            x: baked.0,
            y: alloc::borrow::Cow::Borrowed(baked.1),
        }
    }
}

#[test]
fn test_custom_bake_example() {
    test_bake!(
        CustomBakeExample<'static>,
        const,
        crate::CustomBakeExample {
            x: 51423usize,
            y: alloc::borrow::Cow::Borrowed("bar"),
        },
        crate::CustomBakeExample::from_custom_bake((51423usize, "bar")),
        test
    );
}

#[derive(Bake)]
#[databake(path = test)]
#[databake(custom_bake_unsafe)]
pub struct CustomBakeUnsafeExample<'a> {
    x: usize,
    y: alloc::borrow::Cow<'a, str>,
}

impl CustomBake for CustomBakeUnsafeExample<'_> {
    type BakedType<'a>
        = (usize, &'a str)
    where
        Self: 'a;
    fn to_custom_bake(&self) -> Self::BakedType<'_> {
        (self.x, &*self.y)
    }
}

// Safety: The type has a `from_custom_bake` fn of the required signature.
unsafe impl CustomBakeUnsafe for CustomBakeExample<'_> {}

impl<'a> CustomBakeUnsafeExample<'a> {
    /// Safety: the argument MUST have been returned from [`Self::to_custom_bake`].
    pub const unsafe fn from_custom_bake(baked: <Self as CustomBake>::BakedType<'a>) -> Self {
        Self {
            x: baked.0,
            y: alloc::borrow::Cow::Borrowed(baked.1),
        }
    }
}

#[test]
fn test_custom_bake_unsafe_example() {
    test_bake!(
        CustomBakeUnsafeExample<'static>,
        const,
        crate::CustomBakeUnsafeExample {
            x: 51423usize,
            y: alloc::borrow::Cow::Borrowed("bar"),
        },
        unsafe { crate::CustomBakeUnsafeExample::from_custom_bake((51423usize, "bar")) },
        test
    );
}
