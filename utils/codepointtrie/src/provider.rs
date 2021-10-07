// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{mem, ptr};

use crate::codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::yoke::*;

// Note:
//   T: TrieValue is T: Copy + zerovec::ule::AsULE + 'static

unsafe impl<'a, T: TrieValue> Yokeable<'a> for CodePointTrie<'static, T> {
    type Output = CodePointTrie<'a, T>;
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    fn transform_owned(self) -> Self::Output {
        self
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

pub struct UnicodePropertyMapV1<'data, T: TrieValue> {
    pub codepoint_trie: CodePointTrie<'data, T>,
}

unsafe impl<'a, T: TrieValue> Yokeable<'a> for UnicodePropertyMapV1<'static, T> {
    type Output = UnicodePropertyMapV1<'a, T>;
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    fn transform_owned(self) -> Self::Output {
        self
    }
    unsafe fn make(from: Self::Output) -> Self {
        debug_assert!(mem::size_of::<Self::Output>() == mem::size_of::<Self>());
        let ptr: *const Self = (&from as *const Self::Output).cast();
        mem::forget(from);
        ptr::read(ptr)
    }
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe { f(mem::transmute::<&mut Self, &mut Self::Output>(self)) }
    }
}

impl<'a, T: TrieValue> ZeroCopyFrom<UnicodePropertyMapV1<'a, T>>
    for UnicodePropertyMapV1<'static, T>
{
    fn zero_copy_from<'b>(cart: &'b UnicodePropertyMapV1<'a, T>) -> UnicodePropertyMapV1<'b, T> {
        UnicodePropertyMapV1 {
            codepoint_trie: CodePointTrie::<'static, T>::zero_copy_from(&cart.codepoint_trie),
        }
    }
}

pub struct UnicodePropertyMapV1Marker<T: TrieValue> {
    _phantom: core::marker::PhantomData<T>,
}

impl<'data, T: TrieValue> icu_provider::DataMarker<'data> for UnicodePropertyMapV1Marker<T> {
    type Yokeable = UnicodePropertyMapV1<'static, T>;
    type Cart = UnicodePropertyMapV1<'data, T>;
}
