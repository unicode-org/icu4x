// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is a demo project demonstrating `harfbuzz` integration.
//!
//! For more information, see the [index](..).

use harfbuzz::{sys, Buffer, Direction, UnicodeFuncsBuilder};
use icu_harfbuzz::AllUnicodeFuncs;
    
fn main() {
    let mut builder = UnicodeFuncsBuilder::new_with_empty_parent().unwrap();
    //  Note: AllUnicodeFuncs::boxed() doesn't allocate memory.
    builder.set_general_category_func(AllUnicodeFuncs::boxed());
    builder.set_combining_class_func(AllUnicodeFuncs::boxed());
    builder.set_mirroring_func(AllUnicodeFuncs::boxed());
    builder.set_script_func(AllUnicodeFuncs::boxed());
    builder.set_compose_func(AllUnicodeFuncs::boxed());
    builder.set_decompose_func(AllUnicodeFuncs::boxed());
    let funcs = builder.build();
    
    let mut b = Buffer::with("مساء الخير");
    b.set_unicode_funcs(&funcs);
    b.guess_segment_properties();

    assert_eq!(b.get_direction(), Direction::RTL);
    assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
}
