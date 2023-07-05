// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]
use libfuzzer_sys::fuzz_target;
use rust_icu_ustring::UChar;
use rust_icu_unorm2::UNormalizer;
use icu_normalizer::DecomposingNormalizer;

// Sadly, UNormalizer doesn't take slices
fn slice_to_icu4c(slice: &[u16]) -> UChar {
    slice.to_vec().into()
}

fn is_utf16(buffer: &[u16]) -> bool {
    encoding_rs::mem::utf16_valid_up_to(buffer) == buffer.len()
}

fn slice_from_icu4c(string: &UChar) -> &[u16] {
    // Can't find this on `UChar` itself.
    unsafe { core::slice::from_raw_parts(string.as_c_ptr(), string.len()) }
}

fn normalize_icu4x(buffer: &[u16]) -> Vec<u16> {
    let normalizer = DecomposingNormalizer::new_nfd();
    normalizer.normalize_utf16(buffer)
}

fn normalize_icu4c(buffer: &[u16]) -> UChar {
    let normalizer = UNormalizer::new_nfd().unwrap();
    normalizer.normalize_ustring(&slice_to_icu4c(buffer)).unwrap()
}

fuzz_target!(|data: &[u8]| {
    let (_, aligned, _) = unsafe { data.align_to::<u16>() };
    if !is_utf16(aligned) {
        return;
    }
    let from_icu4x = normalize_icu4x(aligned);
    let from_icu4c = normalize_icu4c(aligned);
    assert_eq!(from_icu4x.as_slice(), slice_from_icu4c(&from_icu4c));
});
