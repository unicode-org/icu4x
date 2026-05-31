// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerofrom::transparent;

transparent!(
    #[repr(transparent)]
    /// hello world
    #[derive(Debug)]
    pub(crate) struct Foo([u8; 3]);
    impl Foo {
        fn zero_from_transparent_ref(&[u8; 3]) -> &Self;
        fn zero_from_transparent_slice(&[[u8; 3]]) -> &[Self];
    }
    #[cfg(feature = "alloc")]
    impl Foo {
        fn zero_from_transparent_box(Box<[u8; 3]>) -> Box<Self>;
    }
);

// Check more permutations:

transparent!(
    #[repr(transparent)]
    pub(crate) struct NoFns(str);
    impl NoFns {
    }
);

transparent!(
    #[repr(transparent)]
    pub(crate) struct Ref(str);
    impl Ref {
        fn zero_from_transparent_ref(&str) -> &Self;
    }
);

transparent!(
    #[repr(transparent)]
    pub(crate) struct Slice(u32);
    impl Slice {
        fn zero_from_transparent_slice(&[u32]) -> &[Self];
    }
);

#[test]
fn test() {}
