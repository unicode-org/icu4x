// Copyright 2019 The Fuchsia Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
/// Generate a [CharCollection] from a sequence of `char`s,
/// [CharRanges](unic_char_range::CharRange), or Unicode [Blocks](unic_ucd_block::Block).
///
/// The macro can be used with either a comma-separated list of items, or with an expression
/// representing set operations.
///
/// ```
/// use char_collection::char_collect;
/// use unicode_blocks::UnicodeBlockId;
/// use unic_char_range::CharRange;
///
/// let c1 = char_collect!(
///     'a'..='z',
///     CharRange::closed('D', 'G'),
///     UnicodeBlockId::Cyrillic,
///     0x01..=0x05,
///     '@');
///
/// let c2 = char_collect!({ ('a'..='z') - ('p'..='t') + UnicodeBlockId::Bengali });
/// ```
///
/// *NOTE:* Parenthetical expressions currently aren't supported unless they start with a
/// `CharCollection`.
/// ```
/// use char_collection::char_collect;
///
/// // This works:
/// let c1 = char_collect!({ ('a'..='z') + (char_collect!('A'..='Z') - ('L'..='P')) });
///
/// // This doesn't:
/// let c1 = char_collect!({ ('a'..='z') + (('A'..='Z') - ('L'..='P')) });
/// ```
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