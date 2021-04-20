// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod kv;
mod vecs;

pub use kv::ZeroMapKV;
pub use vecs::ZeroVecLike;

pub struct ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    keys: K::Container,
    values: V::Container,
}

type NeedleTypeFor<'a, T> = <<T as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, T>>::NeedleType;
type GetTypeFor<'a, T> = <<T as ZeroMapKV<'a>>::Container as ZeroVecLike<'a, T>>::GetType;

impl<'a, K, V> ZeroMap<'a, K, V>
where
    K: ZeroMapKV<'a>,
    V: ZeroMapKV<'a>,
{
    pub fn get(&self, key: &NeedleTypeFor<'a, K>) -> Option<&GetTypeFor<'a, V>> {
        let index = self.keys.binary_search(key).ok()?;
        self.values.get(index)
    }
    // pub fn insert(&self, key: K, value: V) -> Option<V> {
    //     let index = self.keys.binary_search(key).ok()?;
    //     self.values.get(index)
    // }
}
