// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) trait MaybeSplitAt<T> {
    /// Like slice::split_at but returns an Option instead of panicking
    /// if the index is out of range.
    fn maybe_split_at(&self, mid: usize) -> Option<(&Self, &Self)>;
    /// Like slice::split_at but debug-panics and returns an empty second slice
    /// if the index is out of range.
    fn debug_split_at(&self, mid: usize) -> (&Self, &Self);
}

impl<T> MaybeSplitAt<T> for [T] {
    #[inline]
    fn maybe_split_at(&self, mid: usize) -> Option<(&Self, &Self)> {
        if mid > self.len() {
            None
        } else {
            // Note: We're trusting the compiler to inline this and remove the assertion
            // hiding on the top of slice::split_at: `assert(mid <= self.len())`
            Some(self.split_at(mid))
        }
    }
    #[inline]
    fn debug_split_at(&self, mid: usize) -> (&Self, &Self) {
        if mid > self.len() {
            debug_assert!(false, "debug_split_at: index expected to be in range");
            (self, &[])
        } else {
            // Note: We're trusting the compiler to inline this and remove the assertion
            // hiding on the top of slice::split_at: `assert(mid <= self.len())`
            self.split_at(mid)
        }
    }
}

pub(crate) trait DebugUnwrapOr<T> {
    /// Unwraps the option or panics in debug mode, returning the `gigo_value`
    fn debug_unwrap_or(self, gigo_value: T) -> T;
}

impl<T> DebugUnwrapOr<T> for Option<T> {
    #[inline]
    fn debug_unwrap_or(self, gigo_value: T) -> T {
        match self {
            Some(x) => x,
            None => {
                debug_assert!(false, "debug_unwrap_or called on a None value");
                gigo_value
            }
        }
    }
}

macro_rules! debug_unwrap {
    ($expr:expr, return $retval:expr, $($arg:tt)+) => {
        match $expr {
            Some(x) => x,
            None => {
                debug_assert!(false, $($arg)*);
                return $retval;
            }
        }
    };
    ($expr:expr, return $retval:expr) => {
        debug_unwrap!($expr, return $retval, "invalid trie")
    };
    ($expr:expr, break, $($arg:tt)+) => {
        match $expr {
            Some(x) => x,
            None => {
                debug_assert!(false, $($arg)*);
                break;
            }
        }
    };
    ($expr:expr, break) => {
        debug_unwrap!($expr, break, "invalid trie")
    };
    ($expr:expr, $($arg:tt)+) => {
        debug_unwrap!($expr, return (), $($arg)*)
    };
    ($expr:expr) => {
        debug_unwrap!($expr, return ())
    };
}

pub(crate) use debug_unwrap;
