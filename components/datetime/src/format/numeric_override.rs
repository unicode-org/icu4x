// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::fields::FieldNumericOverrides;
use core::fmt;
use writeable::Writeable;

/// Formats a number according to the override system.
pub(crate) fn format<W: fmt::Write + ?Sized>(
    overrides: FieldNumericOverrides,
    number: u32,
    w: &mut W,
) -> fmt::Result {
    match overrides {
        FieldNumericOverrides::Hanidec => format_hanidec(number, w),
        // https://github.com/unicode-org/cldr/blob/main/common/rbnf/ja.xml#L16
        FieldNumericOverrides::Jpnyear => {
            if number == 1 {
                w.write_str("元")
            } else {
                number.write_to(w)
            }
        }
        FieldNumericOverrides::Hanidays => format_hanidays(number, w),
        FieldNumericOverrides::Romanlow => format_romanlow(number, w),
        FieldNumericOverrides::Hebr => format_hebrew(number, w),
    }
}

/// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
fn format_hanidec<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    const HANIDEC_DIGITS: &[char] = &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'];
    if number == 0 {
        return w.write_char('〇');
    }
    let mut n = number;
    let mut buf = [0u8; u32::MAX.ilog10() as usize + 1];
    let mut i = 10;
    #[allow(clippy::indexing_slicing, reason = "i is < 10")]
    while n > 0 && i > 0 {
        i -= 1;
        buf[i] = (n % 10) as u8;
        n /= 10;
    }
    #[allow(
        clippy::indexing_slicing,
        reason = "buf is sliced with an index < 10 and digits are within 0-9"
    )]
    for &d in buf[i..].iter() {
        w.write_char(HANIDEC_DIGITS[d as usize])?;
    }
    Ok(())
}

/// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
fn format_hanidays<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    let han_digits = [
        "", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十",
    ];
    #[allow(
        clippy::indexing_slicing,
        reason = "We are always indexing a 10-element array with a digit"
    )]
    match number {
        1..=10 => {
            w.write_str("初")?;
            w.write_str(han_digits[number as usize])?;
        }
        11..20 => {
            w.write_str("十")?;
            w.write_str(han_digits[(number % 10) as usize])?;
        }
        20 => w.write_str("二十")?,
        21..30 => {
            w.write_str("廿")?;
            w.write_str(han_digits[(number % 20) as usize])?;
        }
        30 => {
            w.write_str("三十")?;
        }
        31 => {
            w.write_str("丗一")?;
        }
        0 | 32.. => {
            debug_assert!(
                false,
                "hanidays should only be found in a d context and only supports 1-31"
            );
            return number.write_to(w);
        }
    }
    Ok(())
}

// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
fn format_romanlow<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    let mut n = number;
    if n == 0 || n >= 4000 {
        debug_assert!(
            false,
            "romanlow should only be found in an M context and only supports 1-3999"
        );
        return n.write_to(w);
    }
    let mappings = [
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ];
    for &(value, roman) in mappings.iter() {
        while n >= value {
            w.write_str(roman)?;
            n -= value;
        }
    }
    Ok(())
}

fn format_hebrew<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    const HEBREW_UNITS: [char; 9] = ['א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט'];
    const HEBREW_TENS: [char; 9] = ['י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע', 'פ', 'צ'];
    const HEBREW_HUNDREDS: [&str; 9] = ["ק", "ר", "ש", "ת", "תק", "תר", "תש", "תת", "תתק"];

    fn format_hebrew_less_than_1000<W: fmt::Write + ?Sized>(
        n: u32,
        w: &mut W,
        force_geresh: bool,
    ) -> fmt::Result {
        let hundreds = n / 100;
        let rem = n % 100;

        let hundreds_str = HEBREW_HUNDREDS
            .get((hundreds as usize).wrapping_sub(1))
            .copied()
            .unwrap_or_default();

        let mut wrote_gershayim = false;

        if rem == 15 {
            w.write_str(hundreds_str)?;
            w.write_char('ט')?;
            w.write_char('״')?;
            w.write_char('ו')?;
            wrote_gershayim = true;
        } else if rem == 16 {
            w.write_str(hundreds_str)?;
            w.write_char('ט')?;
            w.write_char('״')?;
            w.write_char('ז')?;
            wrote_gershayim = true;
        } else {
            let tens = rem / 10;
            let units = rem % 10;

            let tens_char = HEBREW_TENS.get((tens as usize).wrapping_sub(1));
            let units_char = HEBREW_UNITS.get((units as usize).wrapping_sub(1));

            match (hundreds_str, tens_char, units_char) {
                (h, Some(&t), Some(&u)) => {
                    w.write_str(h)?;
                    w.write_char(t)?;
                    w.write_char('״')?;
                    w.write_char(u)?;
                    wrote_gershayim = true;
                }
                (h, Some(&x), None) | (h, None, Some(&x)) => {
                    if !h.is_empty() {
                        w.write_str(h)?;
                        w.write_char('״')?;
                        w.write_char(x)?;
                        wrote_gershayim = true;
                    } else {
                        w.write_char(x)?;
                        w.write_char('׳')?;
                    }
                }
                (h, None, None) => {
                    let mut chars = h.chars();
                    if let Some(last) = chars.next_back() {
                        if chars.as_str().is_empty() {
                            w.write_char(last)?;
                            w.write_char('׳')?;
                        } else {
                            for c in chars {
                                w.write_char(c)?;
                            }
                            w.write_char('״')?;
                            w.write_char(last)?;
                            wrote_gershayim = true;
                        }
                    }
                }
            }
        }

        if wrote_gershayim && force_geresh {
            w.write_char('׳')?;
        }

        Ok(())
    }

    if number == 0 {
        return w.write_str("0");
    }
    if number == 1000 {
        return w.write_str("אלף");
    }
    if number == 2000 {
        return w.write_str("אלפיים");
    }
    let thousands = number / 1000;
    let rest = number % 1000;

    if thousands >= 1000 {
        return number.write_to(w);
    }

    if thousands > 0 {
        format_hebrew_less_than_1000(thousands, w, rest > 0)?;

        if rest == 0 {
            return w.write_str(" אלפים");
        }
    }

    if rest > 0 {
        format_hebrew_less_than_1000(rest, w, false)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::fields::FieldNumericOverrides;

    fn format_to_string(o: FieldNumericOverrides, n: u32) -> String {
        let mut s = String::new();
        format(o, n, &mut s).unwrap();
        s
    }

    #[test]
    fn test_format_number() {
        use FieldNumericOverrides::*;
        assert_eq!(format_to_string(Hanidec, 2024), "二〇二四");
        assert_eq!(format_to_string(Hanidec, 0), "〇");
        assert_eq!(format_to_string(Hanidec, 10), "一〇");

        assert_eq!(format_to_string(Hanidays, 1), "初一");
        assert_eq!(format_to_string(Hanidays, 10), "初十");
        assert_eq!(format_to_string(Hanidays, 11), "十一");
        assert_eq!(format_to_string(Hanidays, 19), "十九");
        assert_eq!(format_to_string(Hanidays, 20), "二十");
        assert_eq!(format_to_string(Hanidays, 21), "廿一");
        assert_eq!(format_to_string(Hanidays, 29), "廿九");
        assert_eq!(format_to_string(Hanidays, 30), "三十");
        assert_eq!(format_to_string(Hanidays, 31), "丗一");

        assert_eq!(format_to_string(Jpnyear, 1), "元");
        assert_eq!(format_to_string(Jpnyear, 2), "2");
        assert_eq!(format_to_string(Jpnyear, 2024), "2024");

        assert_eq!(format_to_string(Romanlow, 1), "i");
        assert_eq!(format_to_string(Romanlow, 4), "iv");
        assert_eq!(format_to_string(Romanlow, 9), "ix");
        assert_eq!(format_to_string(Romanlow, 49), "xlix");
        assert_eq!(format_to_string(Romanlow, 3999), "mmmcmxcix");
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "hanidays should only be found in a d context")]
    fn test_hanidays_invalid() {
        format_to_string(FieldNumericOverrides::Hanidays, 32);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "romanlow should only be found in an M context")]
    fn test_romanlow_invalid() {
        format_to_string(FieldNumericOverrides::Romanlow, 4000);
    }

    #[test]
    fn test_hebr() {
        use FieldNumericOverrides::Hebr;
        assert_eq!(format_to_string(Hebr, 1), "א׳");
        assert_eq!(format_to_string(Hebr, 10), "י׳");
        assert_eq!(format_to_string(Hebr, 15), "ט״ו");
        assert_eq!(format_to_string(Hebr, 16), "ט״ז");
        assert_eq!(format_to_string(Hebr, 17), "י״ז");
        assert_eq!(format_to_string(Hebr, 21), "כ״א");
        assert_eq!(format_to_string(Hebr, 100), "ק׳");
        assert_eq!(format_to_string(Hebr, 101), "ק״א");
        assert_eq!(format_to_string(Hebr, 115), "קט״ו");
        assert_eq!(format_to_string(Hebr, 400), "ת׳");
        assert_eq!(format_to_string(Hebr, 415), "תט״ו");
        assert_eq!(format_to_string(Hebr, 419), "תי״ט");
        assert_eq!(format_to_string(Hebr, 500), "ת״ק");
        assert_eq!(format_to_string(Hebr, 719), "תשי״ט");
        assert_eq!(format_to_string(Hebr, 784), "תשפ״ד");
        assert_eq!(format_to_string(Hebr, 1000), "אלף");
        assert_eq!(format_to_string(Hebr, 1001), "א׳א׳");
        assert_eq!(format_to_string(Hebr, 1015), "א׳ט״ו");
        assert_eq!(format_to_string(Hebr, 1415), "א׳תט״ו");
        assert_eq!(format_to_string(Hebr, 1419), "א׳תי״ט");
        assert_eq!(format_to_string(Hebr, 1719), "א׳תשי״ט");
        assert_eq!(format_to_string(Hebr, 2000), "אלפיים");
        assert_eq!(format_to_string(Hebr, 3000), "ג׳ אלפים");
        assert_eq!(format_to_string(Hebr, 4000), "ד׳ אלפים");
        assert_eq!(format_to_string(Hebr, 5000), "ה׳ אלפים");
        assert_eq!(format_to_string(Hebr, 5783), "ה׳תשפ״ג");
        assert_eq!(format_to_string(Hebr, 15000), "ט״ו אלפים");
        assert_eq!(format_to_string(Hebr, 15001), "ט״ו׳א׳");
        assert_eq!(format_to_string(Hebr, 15015), "ט״ו׳ט״ו");
        assert_eq!(format_to_string(Hebr, 15400), "ט״ו׳ת׳");
        assert_eq!(format_to_string(Hebr, 15415), "ט״ו׳תט״ו");
        assert_eq!(format_to_string(Hebr, 15419), "ט״ו׳תי״ט");
        assert_eq!(format_to_string(Hebr, 15719), "ט״ו׳תשי״ט");
        assert_eq!(format_to_string(Hebr, 100000), "ק׳ אלפים");
        assert_eq!(format_to_string(Hebr, 1000000), "1000000");
    }
}
