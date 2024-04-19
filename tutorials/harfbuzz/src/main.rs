fn main() {
    use harfbuzz::{sys, Buffer, Direction, UnicodeFuncsBuilder};
    use icu_harfbuzz::AllUnicodeFuncs;
    let mut b = Buffer::with("مساء الخير");
    let mut builder = UnicodeFuncsBuilder::new_with_empty_parent().unwrap();
    //  Note: AllUnicodeFuncs is zero-sized, so these boxes don't allocate memory.
    builder.set_general_category_func(Box::new(AllUnicodeFuncs));
    builder.set_combining_class_func(Box::new(AllUnicodeFuncs));
    builder.set_mirroring_func(Box::new(AllUnicodeFuncs));
    builder.set_script_func(Box::new(AllUnicodeFuncs));
    builder.set_compose_func(Box::new(AllUnicodeFuncs));
    builder.set_decompose_func(Box::new(AllUnicodeFuncs));
    b.set_unicode_funcs(&builder.build());
    b.guess_segment_properties();
    assert_eq!(b.get_direction(), Direction::RTL);
    assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
}
