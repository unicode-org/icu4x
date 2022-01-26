// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[repr(transparent)]
pub struct Aligned4(u32);

impl Aligned4 {
    pub const fn is_ascii_alphabetic(self) -> bool {
        let word = self.0;
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let lower = word | 0x2020_2020;
        let alpha = !(lower + 0x1f1f_1f1f) | (lower + 0x0505_0505);
        (alpha & mask) == 0
    }

    pub const fn is_ascii_alphanumeric(self) -> bool {
        let word = self.0;
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let numeric = !(word + 0x5050_5050) | (word + 0x4646_4646);
        let lower = word | 0x2020_2020;
        let alpha = !(lower + 0x1f1f_1f1f) | (lower + 0x0505_0505);
        (alpha & numeric & mask) == 0
    }

    pub const fn is_ascii_numeric(self) -> bool {
        let word = self.0;
        let mask = (word + 0x7f7f_7f7f) & 0x8080_8080;
        let numeric = !(word + 0x5050_5050) | (word + 0x4646_4646);
        (numeric & mask) == 0
    }

    pub const fn to_ascii_lowercase(self) -> Self {
        let word = self.0;
        let result = word | (((word + 0x3f3f_3f3f) & !(word + 0x2525_2525) & 0x8080_8080) >> 2);
        Self(result)
    }

    pub const fn to_ascii_titlecase(self) -> Self {
        let word = self.0.to_le();
        let mask = ((word + 0x3f3f_3f1f) & !(word + 0x2525_2505) & 0x8080_8080) >> 2;
        let result = (word | mask) & !(0x20 & mask);
        Self(u32::from_le(result))
    }

    pub const fn to_ascii_uppercase(self) -> Self {
        let word = self.0;
        let result = word & !(((word + 0x1f1f_1f1f) & !(word + 0x0505_0505) & 0x8080_8080) >> 2);
        Self(result)
    }

    /// # Panics
    /// Panics if N is greater than 4
    #[inline]
    pub fn from_bytes<const N: usize>(src: &[u8; N]) -> Self {
        let mut bytes = [0; 4];
        bytes[0..N].copy_from_slice(src);
        Self(u32::from_ne_bytes(bytes))
    }

    #[inline]
    pub const fn to_bytes(&self) -> [u8; 4] {
        self.0.to_ne_bytes()
    }
}
