// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A container for 0, 1, or 2 elements that owns its data and supports iteration
#[derive(Copy, Clone)]
pub(crate) enum ZeroOneOrTwo<T> {
    Zero,
    One(T),
    Two(T, T),
}

impl<T: Copy> Iterator for ZeroOneOrTwo<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            ZeroOneOrTwo::Zero => None,
            ZeroOneOrTwo::One(a) => {
                *self = ZeroOneOrTwo::Zero;
                Some(a)
            }
            ZeroOneOrTwo::Two(a, b) => {
                *self = ZeroOneOrTwo::One(b);
                Some(a)
            }
        }
    }
}
