// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::FormattedDateTimePatternError;
use crate::provider::fields::FieldNumericOverrides;
use core::fmt;
use writeable::{Part, PartsWrite, Writeable};

pub(crate) const HANIDEC_DIGITS: &[char] =
    &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'];

/// Formats a number according to the override system.
pub(crate) fn format<W: PartsWrite + ?Sized>(
    part: Part,
    w: &mut W,
    number: u32,
    overrides: FieldNumericOverrides,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
    let mut inner_res = Ok(());
    w.with_part(part, |w| {
        let res = match overrides {
            FieldNumericOverrides::Hanidec => format_hanidec(number, w),
            //
            FieldNumericOverrides::Jpnyear => format_jpan(number, w),
            FieldNumericOverrides::Hanidays => format_hanidays(number, w),
            FieldNumericOverrides::Romanlow => format_romanlow(number, w),
            FieldNumericOverrides::Hebr => format_hebrew(number, w),
        };
        // Unfortunately with_part doesn't allow returning anything, so
        // we need to smuggle out the error
        match res {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                inner_res = Err(e);
                w.with_part(Part::ERROR, |w| number.write_to(w))?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    })?;
    Ok(inner_res)
}

/// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/ja.xml#L16>
fn format_jpan<W: fmt::Write + ?Sized>(
    number: u32,
    w: &mut W,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
    if number == 1 {
        w.write_str("元")?;
    } else {
        // <https://github.com/unicode-org/cldr/blob/main/common/supplemental/numberingSystems.xml#L50>
        // <https://github.com/unicode-org/cldr/blob/main/common/rbnf/ja.xml#L16>
        //
        // This rule has `latn` in the name and the RBNF syntax falls back to
        // decimal formatting, so we should use Latin decimal formatting here.
        //
        // Open CLDR issue:
        // <https://unicode-org.atlassian.net/browse/CLDR-19424>
        number.write_to(w)?;
    }
    Ok(Ok(()))
}

/// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
fn format_hanidec<W: fmt::Write + ?Sized>(
    number: u32,
    w: &mut W,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
    if number == 0 {
        w.write_char(HANIDEC_DIGITS[0])?;
        return Ok(Ok(()));
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
    Ok(Ok(()))
}

/// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
fn format_hanidays<W: fmt::Write + ?Sized>(
    number: u32,
    w: &mut W,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
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
        11..=19 => {
            w.write_str("十")?;
            w.write_str(han_digits[(number % 10) as usize])?;
        }
        20 => w.write_str("二十")?,
        21..=29 => {
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
            return Ok(Err(
                FormattedDateTimePatternError::DecimalFormatterNotLoaded,
            ));
        }
    }
    Ok(Ok(()))
}

// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
fn format_romanlow<W: fmt::Write + ?Sized>(
    mut n: u32,
    w: &mut W,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
    if n == 0 || n >= 4000 {
        return Ok(Err(
            FormattedDateTimePatternError::DecimalFormatterNotLoaded,
        ));
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
    Ok(Ok(()))
}

fn format_hebrew<W: fmt::Write + ?Sized>(
    number: u32,
    w: &mut W,
) -> Result<Result<(), FormattedDateTimePatternError>, fmt::Error> {
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
        return Ok(Ok(()));
    }
    if number == 1000 {
        w.write_str("אלף")?;
        return Ok(Ok(()));
    }
    if number == 2000 {
        w.write_str("אלפיים")?;
        return Ok(Ok(()));
    }
    let thousands = number / 1000;
    let rest = number % 1000;

    if thousands >= 1000 {
        // Fallback to latn numbers in the out-of-bounds case
        //
        // This is not unreachable, but would only be reached
        // for basically irrelevant very-large dates.
        number.write_to(w)?;
        return Ok(Ok(()));
    }

    if thousands > 0 {
        format_hebrew_less_than_1000(thousands, w, rest > 0)?;

        if rest == 0 {
            w.write_str(" אלפים")?;
            return Ok(Ok(()));
        }
    }

    if rest > 0 {
        format_hebrew_less_than_1000(rest, w, false)?;
    }

    Ok(Ok(()))
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

        assert_eq!(format_to_string(Romanlow, 1), "i");
        assert_eq!(format_to_string(Romanlow, 4), "iv");
        assert_eq!(format_to_string(Romanlow, 9), "ix");
        assert_eq!(format_to_string(Romanlow, 49), "xlix");
        assert_eq!(format_to_string(Romanlow, 3999), "mmmcmxcix");
    }

    struct TestWriter {
        string: String,
        parts: Vec<(usize, usize, Part)>,
    }
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.string.write_str(s)
        }
    }
    impl PartsWrite for TestWriter {
        type SubPartsWrite = Self;
        fn with_part(
            &mut self,
            part: Part,
            mut f: impl FnMut(&mut Self::SubPartsWrite) -> fmt::Result,
        ) -> fmt::Result {
            let start = self.string.len();
            f(self)?;
            let end = self.string.len();
            if start < end {
                self.parts.push((start, end, part));
            }
            Ok(())
        }
    }

    const TEST_PART: Part = Part {
        category: "foo",
        value: "bar",
    };

    #[test]
    fn test_hanidays_invalid() {
        let mut w = TestWriter {
            string: String::new(),
            parts: Vec::new(),
        };
        let res = format(TEST_PART, &mut w, 32, FieldNumericOverrides::Hanidays).unwrap();
        assert_eq!(
            res,
            Err(FormattedDateTimePatternError::DecimalFormatterNotLoaded)
        );
        assert_eq!(w.string, "32");
        assert_eq!(w.parts, vec![(0, 2, Part::ERROR), (0, 2, TEST_PART)]);
    }

    #[test]
    fn test_romanlow_invalid() {
        let mut w = TestWriter {
            string: String::new(),
            parts: Vec::new(),
        };
        let res = format(TEST_PART, &mut w, 4000, FieldNumericOverrides::Romanlow).unwrap();
        assert_eq!(
            res,
            Err(FormattedDateTimePatternError::DecimalFormatterNotLoaded)
        );
        assert_eq!(w.string, "4000");
        assert_eq!(w.parts, vec![(0, 4, Part::ERROR), (0, 4, TEST_PART)]);
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
