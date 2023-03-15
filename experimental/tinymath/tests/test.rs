// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use tinymath::i8_mul_div_128;
use tinymath::i8_mul_div_128_reference;
use tinymath::saturating_i16_mul_div_1024;
use tinymath::saturating_i16_mul_div_1024_reference;

#[test]
fn test_i8_mul_div_128() {
    assert_eq!(25, i8_mul_div_128(50, 64));
    assert_eq!(25, i8_mul_div_128(64, 50));
    assert_eq!(-25, i8_mul_div_128(-50, 64));
    assert_eq!(-25, i8_mul_div_128(64, -50));
    assert_eq!(-25, i8_mul_div_128(50, -64));
    assert_eq!(-25, i8_mul_div_128(-64, 50));
    assert_eq!(25, i8_mul_div_128(-50, -64));
    assert_eq!(25, i8_mul_div_128(-64, -50));

    let values = [0i8, 1, 2, 3, 4, 5, 10, 16, 20, 32, 40, 63, 64, 65, 90, 127];
    for a in values.iter() {
        for b in values.iter() {
            for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
                let a = a * p;
                let b = b * q;
                let expected = i8_mul_div_128_reference(a, b);
                let actual = i8_mul_div_128(a, b);
                assert_eq!(expected, actual, "{a}*{b}");
            }
        }
    }
}

fn check_i16_mul_div_1024(a: u16, b: u16) {
    for (p, q) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let a = a as i16 * p;
        let b = b as i16 * q;
        let expected = saturating_i16_mul_div_1024_reference(a, b);
        let actual = saturating_i16_mul_div_1024(a, b);
        assert_eq!(expected, actual, "{a}*{b}");
    }
}

#[test]
fn test_i16_mul_div_1024() {
    assert_eq!(200, saturating_i16_mul_div_1024(400, 512));
    assert_eq!(200, saturating_i16_mul_div_1024(512, 400));
    assert_eq!(-200, saturating_i16_mul_div_1024(-400, 512));
    assert_eq!(-200, saturating_i16_mul_div_1024(512, -400));
    assert_eq!(-200, saturating_i16_mul_div_1024(400, -512));
    assert_eq!(-200, saturating_i16_mul_div_1024(-512, 400));
    assert_eq!(200, saturating_i16_mul_div_1024(-400, -512));
    assert_eq!(200, saturating_i16_mul_div_1024(-512, -400));

    assert_eq!(-3200, saturating_i16_mul_div_1024(i16::MIN, 100));
    assert_eq!(-3200, saturating_i16_mul_div_1024(100, i16::MIN));

    let values = [
        0, 1, 2, 3, 10, 16, 20, 63, 64, 65, 90, 128, 511, 512, 1023, 1024, 4000,
    ];
    for a in values.iter() {
        for b in values.iter() {
            check_i16_mul_div_1024(*a, *b);
        }
    }
}

#[test]
fn test_saturating_i16_mul_div_1024() {
    // Note: The non-saturating behavior is tested above

    // These values are barely in bounds:
    check_i16_mul_div_1024(4200, 7500);
    check_i16_mul_div_1024(7500, 4200);
    check_i16_mul_div_1024(1000, 32000);
    check_i16_mul_div_1024(32000, 1000);
    check_i16_mul_div_1024(i16::MAX as u16, 1000);
    check_i16_mul_div_1024(1000, i16::MAX as u16);

    // These values exceed bounds:
    check_i16_mul_div_1024(8000, 8000);
    check_i16_mul_div_1024(2000, 32000);
    check_i16_mul_div_1024(32000, 2000);
    check_i16_mul_div_1024(32000, 32000);
    check_i16_mul_div_1024(24576, 24576);
    check_i16_mul_div_1024(i16::MAX as u16, i16::MAX as u16);

    // Test saturating behavior with i16::MIN:
    assert_eq!(i16::MIN + 1, saturating_i16_mul_div_1024(1024, i16::MIN));
    assert_eq!(i16::MIN + 1, saturating_i16_mul_div_1024(i16::MIN, 1024));
    assert_eq!(i16::MIN + 1, saturating_i16_mul_div_1024(2000, i16::MIN));
    assert_eq!(i16::MIN + 1, saturating_i16_mul_div_1024(i16::MIN, 2000));
    assert_eq!(i16::MAX, saturating_i16_mul_div_1024(i16::MIN, i16::MIN));
}
