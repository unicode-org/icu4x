// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use core::cmp::{Ord, PartialOrd};
use core::fmt;
use displaydoc::Display;
use writeable::Writeable;
use zerovec::ule::{AsULE, UleError, ULE};

/// An error relating to the length of a field within a date pattern.
#[derive(Display, Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum LengthError {
    /// The length of the field string within the pattern is invalid, according to
    /// the field type and its supported field patterns in LDML. See [`FieldLength`].
    #[displaydoc("Invalid length")]
    InvalidLength,
}

impl core::error::Error for LengthError {}

/// An enum representing the length of a field within a date or time formatting pattern string.
///
/// Such strings represent fields as a letter occurring 1 or more times in a row, ex:
/// `MMM`, `dd`, `y`.  See the
/// [LDML documentation in UTS 35](https://unicode.org/reports/tr35/tr35-dates.html#Date_Format_Patterns)
/// for more details.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::fields))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // part of data struct
pub enum FieldLength {
    /// Numeric: minimum digits
    ///
    /// Text: same as [`Self::Three`]
    One,
    /// Numeric: pad to 2 digits
    ///
    /// Text: same as [`Self::Three`]
    Two,
    /// Numeric: pad to 3 digits
    ///
    /// Text: Abbreviated format.
    Three,
    /// Numeric: pad to 4 digits
    ///
    /// Text: Wide format.
    Four,
    /// Numeric: pad to 5 digits
    ///
    /// Text: Narrow format.
    Five,
    /// Numeric: pad to 6 digits
    ///
    /// Text: Short format.
    Six,
    /// [`FieldLength::One`] (numeric), but overridden with a different numbering system
    NumericOverride(FieldNumericOverrides),
}

/// First index used for numeric overrides in compact [`FieldLength`] representation
///
/// Currently 17 due to decision in <https://unicode-org.atlassian.net/browse/CLDR-17217>,
/// may become 16 if the `> 16` is updated to a ` >= 16`
const FIRST_NUMERIC_OVERRIDE: u8 = 17;
/// Last index used for numeric overrides
const LAST_NUMERIC_OVERRIDE: u8 = 31;

impl FieldLength {
    #[inline]
    pub(crate) fn idx(self) -> u8 {
        match self {
            FieldLength::One => 1,
            FieldLength::Two => 2,
            FieldLength::Three => 3,
            FieldLength::Four => 4,
            FieldLength::Five => 5,
            FieldLength::Six => 6,
            FieldLength::NumericOverride(o) => FIRST_NUMERIC_OVERRIDE
                .saturating_add(o as u8)
                .min(LAST_NUMERIC_OVERRIDE),
        }
    }

    #[inline]
    pub(crate) fn from_idx(idx: u8) -> Result<Self, LengthError> {
        Ok(match idx {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            idx if (FIRST_NUMERIC_OVERRIDE..=LAST_NUMERIC_OVERRIDE).contains(&idx) => {
                Self::NumericOverride((idx - FIRST_NUMERIC_OVERRIDE).try_into()?)
            }
            _ => return Err(LengthError::InvalidLength),
        })
    }

    #[inline]
    pub(crate) fn to_len(self) -> usize {
        match self {
            FieldLength::One => 1,
            FieldLength::Two => 2,
            FieldLength::Three => 3,
            FieldLength::Four => 4,
            FieldLength::Five => 5,
            FieldLength::Six => 6,
            FieldLength::NumericOverride(o) => FIRST_NUMERIC_OVERRIDE as usize + o as usize,
        }
    }

    /// UTS 35 defines several 1 and 2 symbols to be the same as 3 symbols (abbreviated).
    /// For example, 'a' represents an abbreviated day period, the same as 'aaa'.
    ///
    /// This function maps field lengths 1 and 2 to field length 3.
    pub(crate) fn numeric_to_abbr(self) -> Self {
        match self {
            FieldLength::One | FieldLength::Two => FieldLength::Three,
            other => other,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FieldLengthULE(u8);

impl AsULE for FieldLength {
    type ULE = FieldLengthULE;
    fn to_unaligned(self) -> Self::ULE {
        FieldLengthULE(self.idx())
    }
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        #[expect(clippy::unwrap_used)] // OK because the ULE is pre-validated
        Self::from_idx(unaligned.0).unwrap()
    }
}

impl FieldLengthULE {
    #[inline]
    pub(crate) fn validate_byte(byte: u8) -> Result<(), UleError> {
        FieldLength::from_idx(byte)
            .map(|_| ())
            .map_err(|_| UleError::parse::<FieldLength>())
    }
}

// Safety checklist for ULE:
//
// 1. Must not include any uninitialized or padding bytes (true since transparent over a ULE).
// 2. Must have an alignment of 1 byte (true since transparent over a ULE).
// 3. ULE::validate_bytes() checks that the given byte slice represents a valid slice.
// 4. ULE::validate_bytes() checks that the given byte slice has a valid length
//    (true since transparent over a type of size 1).
// 5. All other methods must be left with their default impl.
// 6. Byte equality is semantic equality.
unsafe impl ULE for FieldLengthULE {
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        for byte in bytes {
            Self::validate_byte(*byte)?;
        }
        Ok(())
    }
}

/// Various numeric overrides for datetime patterns
/// as found in CLDR
#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::fields))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum FieldNumericOverrides {
    /// `hanidec`
    Hanidec = 0,
    /// `hanidays`
    Hanidays = 1,
    /// `hebr`
    Hebr = 2,
    /// `romanlow`
    Romanlow = 3,
    /// `jpnyear`
    Jpnyear = 4,
}

impl TryFrom<u8> for FieldNumericOverrides {
    type Error = LengthError;
    fn try_from(other: u8) -> Result<Self, LengthError> {
        Ok(match other {
            0 => Self::Hanidec,
            1 => Self::Hanidays,
            2 => Self::Hebr,
            3 => Self::Romanlow,
            4 => Self::Jpnyear,
            _ => return Err(LengthError::InvalidLength),
        })
    }
}

impl FieldNumericOverrides {
    /// Convert this to the corresponding string code
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Hanidec => "hanidec",
            Self::Hanidays => "hanidays",
            Self::Hebr => "hebr",
            Self::Romanlow => "romanlow",
            Self::Jpnyear => "jpnyear",
        }
    }

    /// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
    fn format_hanidec<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
        const HANIDEC_DIGITS: &[char] =
            &['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'];
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

    /// Formats a number using traditional Hebrew numerals (Gematria).
    ///
    /// Hebrew numerals are a quasi-decimal alphabetic numeral system using the letters of the
    /// Hebrew alphabet. The system is described at <https://en.wikipedia.org/wiki/Hebrew_numerals>.
    ///
    /// The system has unique letters for units (1-9), tens (10-90), and hundreds from 100 to 400.
    /// Hundreds greater than 400 are formed by adding letters together (e.g., 500 = 400 + 100 = תק,
    /// 800 = 400 + 400 = תת, 900 = 400 + 400 + 100 = תתק).
    ///
    /// Numbers are formed additively by concatenating the representations for hundreds, tens, and units.
    /// Numbers 15 and 16 are special cases to avoid using letters that form a name of God (יה and יו),
    /// and are instead written as 9+6 (טו) and 9+7 (טז).
    ///
    /// Punctuation rules (Geresh and Gershayim):
    /// - A single-letter number (e.g., 1 = א) receives a geresh (׳) at the end: א׳.
    /// - A multi-letter number (e.g., 11 = יא) receives a gershayim (״) before the last letter: י״א.
    /// - Thousands are separated from the rest with a geresh
    ///
    /// This implementation handles numbers up to 999,999 by splitting them into thousands and the
    /// remainder, formatting each part separately according to the rules for numbers less than 1000.
    ///
    /// Quirk: For round thousands (e.g., 1000, 2000, 5000) without hundreds/tens/units,
    /// it appends the word "thousands" (e.g., "ה׳ אלפים" for 5000) instead of just using the
    /// letter for thousands, since "aleph geresh" would otherwise ambiguously mean both 1 and
    /// 1000.
    ///
    /// This matches the `hebrew` RBNF rule
    /// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>,
    /// <https://github.com/unicode-org/cldr/blob/main/common/rbnf/root.xml#L522>
    fn format_hebrew<W: fmt::Write + ?Sized>(number: u32, w: &mut W) -> fmt::Result {
        const HEBREW_UNITS: [char; 9] = ['א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט'];
        const HEBREW_TENS: [char; 9] = ['י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע', 'פ', 'צ'];
        // Hebrew numerals only have unique letters for hundreds up to 400 (ת).
        // Values from 500 to 900 are represented by combining Tav (ת = 400)
        // with another hundred letter (e.g., 500 = 400 + 100 = תק).
        const HEBREW_HUNDREDS: [&str; 9] = ["ק", "ר", "ש", "ת", "תק", "תר", "תש", "תת", "תתק"];

        fn format_hebrew_less_than_1000(n: u32) -> String {
            let mut s = String::new();
            let hundreds = n / 100;
            let rem = n % 100;

            if let Some(&str) = HEBREW_HUNDREDS.get((hundreds as usize).wrapping_sub(1)) {
                s.push_str(str);
            }

            if rem == 15 {
                s.push_str("טו");
            } else if rem == 16 {
                s.push_str("טז");
            } else {
                let tens = rem / 10;
                let units = rem % 10;

                if let Some(&c) = HEBREW_TENS.get((tens as usize).wrapping_sub(1)) {
                    s.push(c);
                }
                if let Some(&c) = HEBREW_UNITS.get((units as usize).wrapping_sub(1)) {
                    s.push(c);
                }
            }
            s
        }

        /// Applies Hebrew punctuation (geresh or gershayim) to a string of Hebrew numerals.
        ///
        /// The rules are:
        /// - Single-letter numbers (e.g., units like "א", tens like "י", or hundreds like "ק")
        ///   get a geresh (׳) at the end to indicate they are numerals.
        /// - Multi-letter numbers (e.g., "טו" or "קא") get a gershayim (״) before the last letter.
        ///
        /// This follows standard conventions for Hebrew Gematria numerals, ensuring that
        /// every numeral string has either a geresh or a gershayim, and is consistent
        /// with how ICU4C formats dates with Hebrew numbering.
        fn apply_hebrew_punctuation(s: &mut String) {
            let count = s.chars().count();
            if count == 1 {
                s.push('׳');
            } else if count > 1 {
                if let Some((i, _)) = s.char_indices().last() {
                    s.insert(i, '״');
                }
            }
        }

        if number == 0 {
            // Hebrew just uses regular 0s, 0 is not a part of the
            // traditional system. This *is* reachable for 0 years.
            return w.write_str("0");
        }
        if number == 1000 {
            // Separate words for lone thousands
            return w.write_str("אלף");
        }
        if number == 2000 {
            return w.write_str("אלפיים");
        }
        let thousands = number / 1000;
        let rest = number % 1000;

        if thousands >= 1000 {
            // Fall back to decimal for large numbers not supported by this scheme
            return number.write_to(w);
        }

        if thousands > 0 {
            let mut thousands_s = format_hebrew_less_than_1000(thousands);
            apply_hebrew_punctuation(&mut thousands_s);
            w.write_str(&thousands_s)?;

            if rest == 0 {
                // Special case for bare thousands (e.g. 5000 -> ה׳ אלפים)
                // to avoid ambiguity, based on ICU4C behavior.
                return w.write_str(" אלפים");
            // Geresh is a thousands separator, but apply_hebrew_punctuation
            // may already have applied a geresh. Don't duplicate it.
            } else if !thousands_s.ends_with('׳') {
                w.write_str("׳")?;
            }
        }

        let mut rest_s = format_hebrew_less_than_1000(rest);
        apply_hebrew_punctuation(&mut rest_s);
        w.write_str(&rest_s)?;

        Ok(())
    }

    /// Formats a number according to the override system.
    pub fn format_number<W: fmt::Write + ?Sized>(self, number: u32, w: &mut W) -> fmt::Result {
        match self {
            Self::Hanidec => Self::format_hanidec(number, w),
            // https://github.com/unicode-org/cldr/blob/main/common/rbnf/ja.xml#L16
            Self::Jpnyear => {
                if number == 1 {
                    w.write_str("元")
                } else {
                    number.write_to(w)
                }
            }
            Self::Hanidays => Self::format_hanidays(number, w),
            Self::Romanlow => Self::format_romanlow(number, w),
            Self::Hebr => Self::format_hebrew(number, w),
        }
    }
}

impl fmt::Display for FieldNumericOverrides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_to_string(o: FieldNumericOverrides, n: u32) -> String {
        let mut s = String::new();
        o.format_number(n, &mut s).unwrap();
        s
    }

    #[test]
    fn test_format_number() {
        use FieldNumericOverrides::*;
        // hanidec
        assert_eq!(format_to_string(Hanidec, 2024), "二〇二四");
        assert_eq!(format_to_string(Hanidec, 0), "〇");
        assert_eq!(format_to_string(Hanidec, 10), "一〇");

        // hanidays
        assert_eq!(format_to_string(Hanidays, 1), "初一");
        assert_eq!(format_to_string(Hanidays, 10), "初十");
        assert_eq!(format_to_string(Hanidays, 11), "十一");
        assert_eq!(format_to_string(Hanidays, 19), "十九");
        assert_eq!(format_to_string(Hanidays, 20), "二十");
        assert_eq!(format_to_string(Hanidays, 21), "廿一");
        assert_eq!(format_to_string(Hanidays, 29), "廿九");
        assert_eq!(format_to_string(Hanidays, 30), "三十");
        assert_eq!(format_to_string(Hanidays, 31), "丗一");

        // jpnyear
        assert_eq!(format_to_string(Jpnyear, 1), "元");
        assert_eq!(format_to_string(Jpnyear, 2), "2");
        assert_eq!(format_to_string(Jpnyear, 2024), "2024");

        // romanlow
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
        // Fallback
        assert_eq!(format_to_string(Hebr, 1000000), "1000000");
    }
}
