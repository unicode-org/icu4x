// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
/// Generate a [CharCollection] from a sequence of `char`s,
/// [CharRanges](unic_char_range::CharRange), or Unicode [Blocks](unic_ucd_block::Block).
///
/// The macro can be used with either a comma-separated list of items, or with an expression
/// representing set operations.
///
#[macro_export]
macro_rules! char_collect {
    ({ $($x:tt)+ }) => {
        {
            $crate::CharCollection::new() + $($x)*
        }
    };
    ( $( $x:expr ),* ) => {
        {
            // Allow unused mut in case the collection is empty.
            #[allow(unused_mut)]
            let mut col = $crate::CharCollection::new();
            $(
                col.insert(& $x);
            )*
            col
        }
    };
}
#[macro_export]
macro_rules! chars {
    ($low:tt .. $high:tt) => {
        $crate::CharRange::open_right($low, $high)
    };
    ($low:tt ..= $high:tt) => {
        $crate::CharRange::closed($low, $high)
    };
    (..) => {
        $crate::CharRange::all()
    }
}