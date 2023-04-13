# icu_harfbuzz [![crates.io](https://img.shields.io/crates/v/icu_harfbuzz)](https://crates.io/crates/icu_harfbuzz)

Using ICU4X as the Unicode Database back end for HarfBuzz.

## Examples

```rust
use harfbuzz::{Buffer, Direction, sys};
use icu_harfbuzz::new_hb_unicode_funcs_unstable;

let mut b = Buffer::with("مساء الخير");

let unicode_funcs = new_hb_unicode_funcs_unstable(&icu_testdata::unstable()).unwrap();

// NOTE: This currently requires `unsafe` code. For progress toward a safe abstraction, see:
// <https://github.com/servo/rust-harfbuzz/pull/197>
unsafe {
    harfbuzz::sys::hb_buffer_set_unicode_funcs(b.as_ptr(), unicode_funcs.as_ptr());
}

b.guess_segment_properties();
assert_eq!(b.get_direction(), Direction::RTL);
assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
```

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
