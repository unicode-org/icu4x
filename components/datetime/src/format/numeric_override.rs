// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::FormattedDateTimePatternError;
use crate::provider::fields::FieldNumericOverrides;
use core::fmt;
use writeable::{Part, PartsWrite, Writeable};

pub(crate) const HANIDEC_DIGITS: &[char; 10] =
    &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'];

/// Formats a number according to the override system.
pub(crate) fn format<W: PartsWrite + ?Sized>(
    part: Part,
    w: &mut W,
    number: u32,
    overrides: FieldNumericOverrides,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
    w.with_part(part, |w| match overrides {
        FieldNumericOverrides::Hanidec => format_hanidec(number, w),
        FieldNumericOverrides::Jpnyear => format_jpan(number, w),
        FieldNumericOverrides::Hanidays => format_hanidays(number, w),
        FieldNumericOverrides::Romanlow => format_romanlow(number, w),
        FieldNumericOverrides::Hebr => format_hebrew(number, w),
    })?;
    Ok(Ok(()))
}

/// <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/rbnf/ja.xml#L16>
fn format_jpan<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    if number == 1 {
        w.write_str("元")?;
    } else {
        // <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/supplemental/numberingSystems.xml#L50>
        // <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/rbnf/ja.xml#L16>
        //
        // This rule has `latn` in the name and the RBNF syntax falls back to
        // decimal formatting, so we should use Latin decimal formatting here.
        //
        // Open CLDR issue:
        // <https://unicode-org.atlassian.net/browse/CLDR-19424>
        number.write_to(w)?;
    }
    Ok(())
}

/// <https://github.com/unicode-org/cldr/blob/c6d4b3579d2ee196ad0f9c3a9adb608a55ddf99b/common/supplemental/numberingSystems.xml#L39>
fn format_hanidec<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    if number == 0 {
        w.write_char(HANIDEC_DIGITS[0])?;
        return Ok(());
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

/// <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/rbnf/root.xml#L522>
fn format_hanidays<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
    // Note that the 0th element is han digit 1!
    const HAN_DIGITS: [char; 10] = ['一', '二', '三', '四', '五', '六', '七', '八', '九', '十'];
    const TWENTY: &str = "二十";
    const TWENTY_ABBR: char = '廿';
    const THIRTY: &str = "三十";
    const THIRTY_ABBR: char = '丗';
    const FORTY: &str = "四十";
    const FORTY_ABBR: char = '卌';
    #[allow(
        clippy::indexing_slicing,
        reason = "We are always indexing a 10-element array with a digit"
    )]
    match number {
        0 => w.write_char('〇'),
        1..=10 => {
            w.write_str("初")?;
            w.write_char(HAN_DIGITS[number as usize - 1])
        }
        11..=19 => {
            w.write_char(HAN_DIGITS[9])?;
            w.write_char(HAN_DIGITS[(number - 11) as usize])
        }
        20 => w.write_str(TWENTY),
        21..=29 => {
            w.write_char(TWENTY_ABBR)?;
            w.write_char(HAN_DIGITS[(number - 21) as usize])
        }
        30 => w.write_str(THIRTY),
        31..=39 => {
            w.write_char(THIRTY_ABBR)?;
            w.write_char(HAN_DIGITS[(number - 31) as usize])
        }
        40 => w.write_str(FORTY),
        41..=49 => {
            w.write_char(FORTY_ABBR)?;
            w.write_char(HAN_DIGITS[(number - 41) as usize])
        }
        50.. => {
            // TODO (https://github.com/unicode-org/icu4x/issues/7922)
            // This falls back to spellout numbering
            // This branch should *only* be hit with custom input types, so it's
            // not actually important to implement spellout rules here.
            number.write_to(w)
        }
    }
}

// <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/rbnf/root.xml#L522>
fn format_romanlow<W: fmt::Write + ?Sized>(mut n: u32, w: &mut W) -> fmt::Result {
    if n == 0 {
        return w.write_char('n'); // null
    }
    if n >= 5000 {
        // romanlow falls back to the default past 5000.
        // This does mean 4000 is `mmmm`.
        //
        // <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/rbnf/root.xml#L719>
        //
        // We may wish to fall back to the DecimalFormatter here:
        // <https://unicode-org.atlassian.net/browse/CLDR-19424>
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
        w.write_str("0")?;
        return Ok(());
    }
    if number == 1000 {
        w.write_str("אלף")?;
        return Ok(());
    }
    if number == 2000 {
        w.write_str("אלפיים")?;
        return Ok(());
    }
    if number == 1_000_000 {
        w.write_str("אלף אלפים")?;
        return Ok(());
    }
    if number > 1_000_000 {
        // Fallback to latn numbers in the out-of-bounds case
        // as noted in the spec
        // <https://github.com/unicode-org/cldr/blob/fb0b4f0cb809cac10e8539dcba669c1d27d8e70c/common/rbnf/root.xml#L577>
        //
        // This is not unreachable, but would only be reached
        // for basically irrelevant very-large dates.
        //
        // We may wish to fall back to the DecimalFormatter here:
        // <https://unicode-org.atlassian.net/browse/CLDR-19424>
        number.write_to(w)?;
        return Ok(());
    }

    if number > 1000 {
        let thousands = number / 1000;
        let rest = number % 1000;
        format_hebrew_less_than_1000(thousands, w, rest > 0)?;

        if rest == 0 {
            w.write_str(" אלפים")?;
        } else {
            format_hebrew_less_than_1000(rest, w, false)?;
        }
    } else if number > 0 {
        format_hebrew_less_than_1000(number, w, false)?;
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::fields::FieldNumericOverrides;

    fn format_to_string(o: FieldNumericOverrides, n: u32) -> String {
        let mut s = String::new();
        let mut w = writeable::adapters::CoreWriteAsPartsWrite(&mut s);
        format(Part::ERROR, &mut w, n, o).unwrap().unwrap();
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

        assert_eq!(format_to_string(Romanlow, 0), "n");
        assert_eq!(format_to_string(Romanlow, 1), "i");
        assert_eq!(format_to_string(Romanlow, 4), "iv");
        assert_eq!(format_to_string(Romanlow, 9), "ix");
        assert_eq!(format_to_string(Romanlow, 49), "xlix");
        assert_eq!(format_to_string(Romanlow, 3999), "mmmcmxcix");
        assert_eq!(format_to_string(Romanlow, 4000), "mmmm");
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
        assert_eq!(format_to_string(Hebr, 1000000), "אלף אלפים");
        assert_eq!(format_to_string(Hebr, 1000001), "1000001");
    }

    #[test]
    #[cfg(feature = "compiled_data")]
    fn test_hanidec_digits() {
        use icu_decimal::provider::{Baked, DecimalDigitsV1};
        use icu_provider::prelude::*;
        let response = DataProvider::<DecimalDigitsV1>::load(
            &Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic("hanidec"),
                    &Default::default(),
                ),
                metadata: Default::default(),
            },
        )
        .expect("Loaded baked data for hanidec digits");
        let baked_digits: &[char] = response.payload.get();
        assert_eq!(HANIDEC_DIGITS, baked_digits);
    }
}
