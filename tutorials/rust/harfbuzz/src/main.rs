// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    use harfbuzz::{sys, Buffer, Direction, UnicodeFuncsBuilder};
    use icu_harfbuzz::AllUnicodeFuncs;
    let mut builder = UnicodeFuncsBuilder::new_with_empty_parent().unwrap();
    //  Note: AllUnicodeFuncs is zero-sized, so these boxes don't allocate memory.
    builder.set_general_category_func(AllUnicodeFuncs::boxed());
    builder.set_combining_class_func(AllUnicodeFuncs::boxed());
    builder.set_mirroring_func(AllUnicodeFuncs::boxed());
    builder.set_script_func(AllUnicodeFuncs::boxed());
    builder.set_compose_func(AllUnicodeFuncs::boxed());
    builder.set_decompose_func(AllUnicodeFuncs::boxed());

    
    let mut b = Buffer::with("مساء الخير");
    b.set_unicode_funcs(&builder.build());
    b.guess_segment_properties();
    assert_eq!(b.get_direction(), Direction::RTL);
    assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
}
