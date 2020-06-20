use std::convert::TryInto;

/// Converts between chars and u8 decimal digits using a zero char as a reference point.
pub(crate) struct DigitConverter {
    /// The char corresponding to the digit 0.
    pub zero_char: char,
}

impl DigitConverter {
    /// Converts from a decimal digit (which must be between 0 and 9) to a char.
    pub(crate) fn char_for(&self, d: u8) -> char {
        debug_assert_ge!(d, 0);
        debug_assert_le!(d, 9);
        let cp: u32 = self.zero_char as u32 + d as u32;
        // TODO: Is there an unchecked version we can use here, if we check zero_char earlier?
        cp.try_into().unwrap_or('\u{FFFD}')
    }

    /// Converts from a char to a decimal digit, or returns None if out of range.
    #[allow(dead_code)]
    pub(crate) fn checked_digit_for(&self, c: char) -> Option<u8> {
        match (c as u32).checked_sub(self.zero_char as u32) {
            Some(v) => {
                if v <= 9 {
                    Some(v as u8)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[test]
fn test_ascii() {
    let cnv = DigitConverter { zero_char: '0' };
    assert_eq!(cnv.zero_char.len_utf8(), 1);

    assert_eq!('0', cnv.char_for(0));
    assert_eq!('1', cnv.char_for(1));
    assert_eq!('2', cnv.char_for(2));
    assert_eq!('3', cnv.char_for(3));
    assert_eq!('4', cnv.char_for(4));
    assert_eq!('5', cnv.char_for(5));
    assert_eq!('6', cnv.char_for(6));
    assert_eq!('7', cnv.char_for(7));
    assert_eq!('8', cnv.char_for(8));
    assert_eq!('9', cnv.char_for(9));

    assert_eq!(Some(0), cnv.checked_digit_for('0'));
    assert_eq!(Some(1), cnv.checked_digit_for('1'));
    assert_eq!(Some(2), cnv.checked_digit_for('2'));
    assert_eq!(Some(3), cnv.checked_digit_for('3'));
    assert_eq!(Some(4), cnv.checked_digit_for('4'));
    assert_eq!(Some(5), cnv.checked_digit_for('5'));
    assert_eq!(Some(6), cnv.checked_digit_for('6'));
    assert_eq!(Some(7), cnv.checked_digit_for('7'));
    assert_eq!(Some(8), cnv.checked_digit_for('8'));
    assert_eq!(Some(9), cnv.checked_digit_for('9'));

    assert_lt!('!', cnv.zero_char);
    assert_eq!(None, cnv.checked_digit_for('!'));

    assert_gt!('A', cnv.zero_char);
    assert_eq!(None, cnv.checked_digit_for('A'));
}

#[test]
fn test_multibyte() {
    let cnv = DigitConverter { zero_char: '০' };
    assert_gt!(cnv.zero_char.len_utf8(), 1);

    assert_eq!('০', cnv.char_for(0));
    assert_eq!('১', cnv.char_for(1));
    assert_eq!('২', cnv.char_for(2));
    assert_eq!('৩', cnv.char_for(3));
    assert_eq!('৪', cnv.char_for(4));
    assert_eq!('৫', cnv.char_for(5));
    assert_eq!('৬', cnv.char_for(6));
    assert_eq!('৭', cnv.char_for(7));
    assert_eq!('৮', cnv.char_for(8));
    assert_eq!('৯', cnv.char_for(9));

    assert_eq!(Some(0), cnv.checked_digit_for('০'));
    assert_eq!(Some(1), cnv.checked_digit_for('১'));
    assert_eq!(Some(2), cnv.checked_digit_for('২'));
    assert_eq!(Some(3), cnv.checked_digit_for('৩'));
    assert_eq!(Some(4), cnv.checked_digit_for('৪'));
    assert_eq!(Some(5), cnv.checked_digit_for('৫'));
    assert_eq!(Some(6), cnv.checked_digit_for('৬'));
    assert_eq!(Some(7), cnv.checked_digit_for('৭'));
    assert_eq!(Some(8), cnv.checked_digit_for('৮'));
    assert_eq!(Some(9), cnv.checked_digit_for('৯'));

    assert_lt!('ক', cnv.zero_char);
    assert_eq!(None, cnv.checked_digit_for('ক'));

    assert_gt!('৳', cnv.zero_char);
    assert_eq!(None, cnv.checked_digit_for('৳'));
}
