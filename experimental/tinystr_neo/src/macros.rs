// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
macro_rules! tinystr {
    ($n:literal, $s:literal) => {
        ({
            // Force it into a const context; otherwise it may get evaluated at runtime instead.
            const TINYSTR_MACRO_CONST: Result<$crate::TinyAsciiStr<$n>, $crate::TinyStrError> = $crate::TinyAsciiStr::from_str($s);
            TINYSTR_MACRO_CONST
        // Note: Rust 1.57 allows panics in consts, until we update to that version we have to panic outside of the const and hope it optimizes away
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_construction() {
        let s1 = tinystr!(8, "foobar");
        assert_eq!(&*s1, "foobar");

        let s1 = tinystr!(12, "foobarbaz");
        assert_eq!(&*s1, "foobarbaz");
    }
}
