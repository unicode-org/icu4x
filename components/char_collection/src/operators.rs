// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//! Implementations of standard operators for [CharCollection].
//!
//! `+` and `|` are equivalent. `+` is easier to use with `-`, as they have the same operator
//! precedence.
use crate::{CharCollection, MultiCharRange};
use std::convert::Into;
use std::ops;
impl<V: MultiCharRange> ops::BitOr<V> for CharCollection {
    type Output = CharCollection;
    fn bitor(self, rhs: V) -> Self::Output {
        let result: CharCollection = self.into();
        result.union(&rhs)
    }
}
impl<V: MultiCharRange> ops::Add<V> for CharCollection {
    type Output = CharCollection;
    fn add(self, rhs: V) -> Self::Output {
        let result: CharCollection = self.into();
        result.union(&rhs)
    }
}
impl<V: MultiCharRange> ops::BitOrAssign<V> for CharCollection {
    fn bitor_assign(&mut self, rhs: V) {
        self.insert(&rhs);
    }
}
impl<V: MultiCharRange> ops::AddAssign<V> for CharCollection {
    fn add_assign(&mut self, rhs: V) {
        self.insert(&rhs);
    }
}
impl<V: MultiCharRange> ops::Sub<V> for CharCollection {
    type Output = CharCollection;
    fn sub(self, rhs: V) -> Self::Output {
        self.difference(&rhs)
    }
}
impl<V: MultiCharRange> ops::SubAssign<V> for CharCollection {
    fn sub_assign(&mut self, rhs: V) {
        self.remove(&rhs);
    }
}
impl<V: MultiCharRange> ops::BitAnd<V> for CharCollection {
    type Output = CharCollection;
    fn bitand(self, rhs: V) -> Self::Output {
        self.intersection(&rhs)
    }
}
impl<V: MultiCharRange> ops::BitAndAssign<V> for CharCollection {
    fn bitand_assign(&mut self, rhs: V) {
        *self = self.intersection(&rhs);
    }
}
impl ops::Not for CharCollection {
    type Output = CharCollection;
    fn not(self) -> Self::Output {
        self.complement()
    }
}
