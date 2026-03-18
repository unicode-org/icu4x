// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerofrom::transparent;

transparent!(
    #[repr(transparent)]
    /// hello world
    #[derive(Debug)]
    pub(crate) struct Foo([u8; 3]);
    impl ZeroFrom<&[u8; 3]> for &Foo;
    impl {
        @ref
        /// Cast a transparent ref!
        #[inline]
        fn from_transparent_ref(&[u8; 3]) -> &Self;
        @slice
        /// Cast a transparent slice!
        pub fn from_transparent_slice(&[[u8; 3]]) -> &[Self];
        @box
        /// Cast a transparent box!
        #[cfg(feature = "alloc")]
        fn from_transparent_box(Box<[u8; 3]>) -> Box<Self>;
    }
);

#[test]
fn test() {}
