// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
extern crate alloc;

impl<T> Bake for alloc::borrow::Cow<'_, T>
where
    T: ?Sized + ToOwned,
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

impl<T> BakeSize for alloc::borrow::Cow<'_, T>
where
    T: ?Sized + ToOwned,
    for<'a> &'a T: BakeSize,
{
    fn borrows_size(&self) -> usize {
        (&**self).borrows_size()
    }
}

#[test]
fn cow() {
    test_bake!(
        alloc::borrow::Cow<'static, str>,
        const,
        alloc::borrow::Cow::Borrowed("hi"),
        alloc
    );
    assert_eq!(
        BakeSize::borrows_size(&alloc::borrow::Cow::Borrowed("hi")),
        2
    );
    assert_eq!(
        Bake::bake(
            &alloc::borrow::Cow::<'static, str>::Borrowed("hi"),
            &Default::default(),
        )
        .to_string(),
        Bake::bake(
            &alloc::borrow::Cow::<'static, str>::Owned("hi".to_owned()),
            &Default::default(),
        )
        .to_string(),
    );
    assert_eq!(
        BakeSize::borrows_size(&alloc::borrow::Cow::<'static, str>::Owned("hi".to_owned())),
        2
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
    test_bake!(Vec<u8>, alloc::vec![1u8, 2u8], alloc);
}

impl<T> Bake for alloc::collections::BTreeSet<T>
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("alloc");
        let data = self.iter().map(|d| d.bake(ctx));
        quote! {
            alloc::collections::BTreeSet::from([#(#data),*])
        }
    }
}

#[test]
fn btree_set() {
    test_bake!(
        alloc::collections::BTreeSet<u8>,
        alloc::collections::BTreeSet::from([1u8, 2u8]),
        alloc
    );
}

impl<K, V> Bake for alloc::collections::BTreeMap<K, V>
where
    K: Bake,
    V: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("alloc");
        let data = self.iter().map(|(k, v)| {
            let k = k.bake(ctx);
            let v = v.bake(ctx);
            quote!((#k, #v))
        });
        quote! {
            alloc::collections::BTreeMap::from([#(#data),*])
        }
    }
}

#[test]
fn btree_map() {
    test_bake!(
        alloc::collections::BTreeMap<u8, u8>,
        alloc::collections::BTreeMap::from([(1u8, 2u8), (2u8, 4u8)]),
        alloc
    );
}

impl<T> Bake for std::collections::HashSet<T>
where
    T: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("std");
        let mut data = self.iter().map(|d| d.bake(ctx)).collect::<Vec<_>>();
        data.sort_unstable_by_key(|data| data.to_string());
        quote! {
            std::collections::HashSet::from([#(#data),*])
        }
    }
}

#[test]
fn hash_set() {
    test_bake!(
        std::collections::HashSet<u8>,
        std::collections::HashSet::from([1u8, 2u8]),
        std
    );
}

impl<K, V> Bake for std::collections::HashMap<K, V>
where
    K: Bake,
    V: Bake,
{
    fn bake(&self, ctx: &CrateEnv) -> TokenStream {
        ctx.insert("std");
        let mut data = self
            .iter()
            .map(|(k, v)| {
                let k = k.bake(ctx);
                let v = v.bake(ctx);
                quote!((#k, #v))
            })
            .collect::<Vec<_>>();
        data.sort_unstable_by_key(|data| data.to_string());
        quote! {
            std::collections::HashMap::from([#(#data),*])
        }
    }
}

#[test]
fn hash_map() {
    test_bake!(
        std::collections::HashMap<u8, u8>,
        std::collections::HashMap::from([(1u8, 2u8), (2u8, 4u8)]),
        std
    );
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
