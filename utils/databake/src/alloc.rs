// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
extern crate alloc;

impl<T: ?Sized + ToOwned> Bake for alloc::borrow::Cow<'_, T>
where
    for<'a> &'a T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("alloc");
        let t = <&T as Bake>::bake(&&**self, ctx);
        quote! {
            alloc::borrow::Cow::Borrowed(#t)
        }
    }
}

#[test]
fn cow() {
    test_bake!(
        alloc::borrow::Cow<'static, str>,
        const: alloc::borrow::Cow::Borrowed("hi"),
        alloc
    );
}

impl<T> Bake for Vec<T>
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("alloc");
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            alloc::vec![#(#data,)*]
        }
    }
}

#[test]
fn vec() {
    test_bake!(Vec<u8>, alloc::vec![1u8, 2u8,], alloc);
}

impl Bake for String {
    fn bake(&self, _: &CrateEnv) -> TokenStream {
        quote! {
            #self.to_owned()
        }
    }
}

#[test]
fn string() {
    test_bake!(String, "hello".to_owned());
}

macro_rules! smart_pointer {
    ($type:ty, $constuctor:path) => {
        impl<T> Bake for $type
        where
            T: Bake,
        {
            fn bake(&self, ctx: &CrateEnv) -> TokenStream {
                ctx.insert("alloc");
                let data = std::ops::Deref::deref(self).bake(ctx);
                quote! {
                    $constuctor(#data)
                }
            }
        }
    };
}

smart_pointer!(alloc::boxed::Box<T>, alloc::boxed::Box::new);
smart_pointer!(alloc::rc::Rc<T>, alloc::rc::Rc::new);
smart_pointer!(alloc::sync::Arc<T>, alloc::arc::Arc::new);

#[test]
fn rc() {
    test_bake!(alloc::rc::Rc<char>, alloc::rc::Rc::new('b'), alloc);
}
