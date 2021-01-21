// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::LengthHint;

impl std::ops::Add<LengthHint> for LengthHint {
    type Output = Self;

    fn add(self, other: LengthHint) -> Self {
        match self {
            LengthHint::Undefined => LengthHint::Undefined,
            LengthHint::Exact(len1) => match other {
                LengthHint::Undefined => LengthHint::Undefined,
                LengthHint::Exact(len2) => LengthHint::Exact(len1 + len2),
            }
        }
    }
}

impl std::ops::Add<usize> for LengthHint {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        match self {
            LengthHint::Undefined => LengthHint::Undefined,
            LengthHint::Exact(len) => LengthHint::Exact(len + other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(LengthHint::Exact(5), LengthHint::Exact(3) + 2);
        assert_eq!(LengthHint::Exact(5), LengthHint::Exact(3) + LengthHint::Exact(2));
        assert_eq!(LengthHint::Undefined, LengthHint::Exact(3) + LengthHint::Undefined);

        assert_eq!(LengthHint::Undefined, LengthHint::Undefined + 2);
        assert_eq!(LengthHint::Undefined, LengthHint::Undefined + LengthHint::Exact(2));
        assert_eq!(LengthHint::Undefined, LengthHint::Undefined + LengthHint::Undefined);
    }
}
