// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

extern crate alloc;

// Don't import anything, to assert that compiled data compiles without imports.
// In particular, *do not* import `icu_provider::prelude::*`.

struct Baked;

const _: () = {
    #[allow(unused_imports)]
    #[path = "data/mod.rs"]
    mod baked_data;
    baked_data::make_provider!(Baked);
    baked_data::impl_hello_world_v1_marker!(Baked, ITER);
};

#[test]
fn iter() {
    use icu_provider::hello_world::HelloWorldProvider;
    use icu_provider::prelude::*;

    assert_eq!(Baked.iter_requests(), HelloWorldProvider.iter_requests(),)
}

#[test]
fn load() {
    use icu_provider::hello_world::HelloWorldProvider;
    use icu_provider::prelude::*;

    for (locale, marker_attributes) in HelloWorldProvider.iter_requests().unwrap().iter() {
        let req = DataRequest {
            locale,
            marker_attributes,
            ..Default::default()
        };
        let expected = HelloWorldProvider.load(req).unwrap().payload;
        let baked = Baked.load(req).unwrap().payload;
        assert_eq!(baked, expected);
    }
}
