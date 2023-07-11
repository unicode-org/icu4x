// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]
use core::cmp::Ordering;
use core::convert::TryFrom;
use icu_collator::AlternateHandling;
use icu_collator::CaseFirst;
use icu_collator::Collator;
use icu_collator::CollatorOptions;
use icu_collator::MaxVariable;
use icu_collator::Strength;
use icu_locid::Locale;
use libfuzzer_sys::fuzz_target;
use rust_icu_sys::UColAttribute;
use rust_icu_sys::UColAttributeValue;
use rust_icu_sys::UColReorderCode;
use rust_icu_ucol::UCollator;
use rust_icu_unorm2::UNormalizer;
use rust_icu_ustring::UChar;

const LOCALES: &[&str] = &[
    "und",
    "bn",
    "es",
    "fi",
    "ja",
    "ko",
    "lt",
    "sv",
    "th",
    "tr",
    "zh-u-co-pinyin",
    "zh-u-co-stroke",
    "zh-u-co-zhuyin",
];

fn compare_icu4x(
    locale_str: &str,
    options: CollatorOptions,
    left: &[u16],
    right: &[u16],
) -> Ordering {
    let locale: Locale = locale_str.parse().unwrap();

    let collator: Collator = Collator::try_new(locale, options).unwrap();
    collator.compare_utf16(left, right)
}

fn bool_to_icu4c(b: bool) -> UColAttributeValue {
    if b {
        UColAttributeValue::UCOL_ON
    } else {
        UColAttributeValue::UCOL_OFF
    }
}

// Sadly, UCollator doesn't take slices
fn slice_to_icu4c(slice: &[u16]) -> UChar {
    slice.to_vec().into()
}

fn compare_icu4c(
    locale_str: &str,
    options: CollatorOptions,
    left: &[u16],
    right: &[u16],
) -> Ordering {
    let mut collator = UCollator::try_from(locale_str).unwrap();

    let strength = match options.strength() {
        Strength::Primary => UColAttributeValue::UCOL_PRIMARY,
        Strength::Secondary => UColAttributeValue::UCOL_SECONDARY,
        Strength::Tertiary => UColAttributeValue::UCOL_TERTIARY,
        Strength::Quaternary => UColAttributeValue::UCOL_QUATERNARY,
        _ => unreachable!(),
    };
    collator.set_strength(strength);

    let alternate_handling = match options.alternate_handling() {
        AlternateHandling::NonIgnorable => UColAttributeValue::UCOL_NON_IGNORABLE,
        AlternateHandling::Shifted => UColAttributeValue::UCOL_SHIFTED,
    };
    collator
        .set_attribute(UColAttribute::UCOL_ALTERNATE_HANDLING, alternate_handling)
        .unwrap();

    let max_variable = match options.max_variable() {
        MaxVariable::Space => UColReorderCode::UCOL_REORDER_CODE_SPACE,
        MaxVariable::Punctuation => UColReorderCode::UCOL_REORDER_CODE_PUNCTUATION,
        MaxVariable::Symbol => UColReorderCode::UCOL_REORDER_CODE_SYMBOL,
        MaxVariable::Currency => UColReorderCode::UCOL_REORDER_CODE_CURRENCY,
    };
    collator.set_max_variable(max_variable).unwrap();

    collator
        .set_attribute(
            UColAttribute::UCOL_CASE_LEVEL,
            bool_to_icu4c(options.case_level()),
        )
        .unwrap();

    let case_first = match options.case_first() {
        CaseFirst::UpperFirst => UColAttributeValue::UCOL_UPPER_FIRST,
        CaseFirst::LowerFirst => UColAttributeValue::UCOL_LOWER_FIRST,
        CaseFirst::Off => UColAttributeValue::UCOL_OFF,
    };
    collator
        .set_attribute(UColAttribute::UCOL_CASE_FIRST, case_first)
        .unwrap();

    collator
        .set_attribute(
            UColAttribute::UCOL_FRENCH_COLLATION,
            bool_to_icu4c(options.backward_second_level()),
        )
        .unwrap();

    collator
        .set_attribute(
            UColAttribute::UCOL_NUMERIC_COLLATION,
            bool_to_icu4c(options.numeric()),
        )
        .unwrap();

    // ICU4X always normalizes
    collator
        .set_attribute(
            UColAttribute::UCOL_NORMALIZATION_MODE,
            UColAttributeValue::UCOL_ON,
        )
        .unwrap();

    // Turning on normalization isn't enough for ICU4C to always normalize, so let's
    // normalize explicitly.
    let normalizer = UNormalizer::new_nfd().unwrap();
    let left_normalized = normalizer.normalize_ustring(&slice_to_icu4c(left)).unwrap();
    let right_normalized = normalizer
        .normalize_ustring(&slice_to_icu4c(right))
        .unwrap();

    collator.strcoll(&left_normalized, &right_normalized)
}

fn is_utf16(buffer: &[u16]) -> bool {
    encoding_rs::mem::utf16_valid_up_to(buffer) == buffer.len()
}

fuzz_target!(|data: &[u8]| {
    // Possibly padding
    // Collation
    // Options
    // Options
    // UTF-16 left
    // UTF-16 left
    // UTF-16 right
    // UTF-16 right
    let (_, aligned, _) = unsafe { data.align_to::<u16>() };
    if aligned.len() < 4 {
        return;
    }

    let (meta, tail) = aligned.split_at(2);
    let locale_str = if let Some(locale) = LOCALES.get(meta[0] as usize) {
        locale
    } else {
        return;
    };

    // Take the defined bits, except cap the strength to Quaternary and
    // ignore max variable, because rust_icu_ucol does not appear to
    // have an API for setting it.
    // Set all options as explicitly set.
    let mut option_bits =
        (u32::from(meta[1]) & 0b1111101101011) | 0b11111110000000000000000000000000;
    // When CASE_FIRST is off, UPPER_FIRST must be off too
    if option_bits & (1 << 9) == 0 {
        option_bits &= !(1 << 8);
    }
    let options: CollatorOptions = unsafe { core::mem::transmute_copy(&option_bits) };

    let mid = tail.len() / 2;
    let (left, right) = tail.split_at(mid);
    if !is_utf16(left) || !is_utf16(right) {
        return;
    }

    assert_eq!(
        compare_icu4x(locale_str, options, left, right),
        compare_icu4c(locale_str, options, left, right)
    );
});
