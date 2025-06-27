// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Tests for the datetime FFI.

use icu_capi::unstable::time_formatter::ffi::*;
use icu_capi::unstable::locale_core::ffi::*;
use icu_capi::unstable::datetime_options::ffi::*;
use icu_capi::unstable::time::ffi::*;

#[test]
fn test_6688_time_alignment() {
    let locale = Locale::from_string(b"de").unwrap();
    let formatter = TimeFormatter::create(&locale, Some(DateTimeLength::Long), Some(TimePrecision::Minute), Some(DateTimeAlignment::Auto)).unwrap();
    let time = Time::create(12, 12, 12, 12).unwrap();
    let buffer = diplomat_runtime::diplomat_buffer_write_create(50);
    formatter.format(&time, unsafe { &mut *buffer });
    assert_eq!(b"12:12", unsafe { core::slice::from_raw_parts(
        diplomat_runtime::diplomat_buffer_write_get_bytes(&*buffer),
        diplomat_runtime::diplomat_buffer_write_len(&*buffer)
    ) });
    unsafe { diplomat_runtime::diplomat_buffer_write_destroy(buffer); }
}
