// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;

use writeable::{impl_display_with_writeable, LengthHint, Writeable};

use core::fmt;

/// Implements [`Writeable`] for [`&[u8]`] according to the [WHATWG Encoding Standard](
/// https://encoding.spec.whatwg.org/#utf-8-decoder).
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PotentiallyInvalidUtf8<'a>(pub &'a [u8]);

impl Writeable for PotentiallyInvalidUtf8<'_> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut remaining = self.0;
        loop {
            match core::str::from_utf8(remaining) {
                Ok(valid) => {
                    return sink.write_str(valid);
                }
                Err(e) => {
                    // SAFETY: By Utf8Error invariants
                    let valid = unsafe {
                        core::str::from_utf8_unchecked(remaining.get_unchecked(..e.valid_up_to()))
                    };
                    sink.write_str(valid)?;
                    sink.write_char(char::REPLACEMENT_CHARACTER)?;
                    let Some(error_len) = e.error_len() else {
                        return Ok(()); // end of string
                    };
                    // SAFETY: By Utf8Error invariants
                    remaining = unsafe { remaining.get_unchecked(e.valid_up_to() + error_len..) }
                }
            }
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        // Lower bound is all valid UTF-8, upper bound is all bytes with the high bit, which become replacement characters.
        LengthHint::between(self.0.len(), self.0.len() * 3)
    }

    fn write_to_string(&self) -> Cow<str> {
        match core::str::from_utf8(self.0) {
            Ok(valid) => Cow::Borrowed(valid),
            Err(e) => {
                // SAFETY: By Utf8Error invariants
                let valid = unsafe {
                    core::str::from_utf8_unchecked(self.0.get_unchecked(..e.valid_up_to()))
                };

                // Let's assume this is the only error
                let mut out = alloc::string::String::with_capacity(
                    self.0.len() + char::REPLACEMENT_CHARACTER.len_utf8()
                        - e.error_len().unwrap_or(0),
                );

                out.push_str(valid);
                out.push_str("�");

                // If there's more, we can use `write_to`
                if let Some(error_len) = e.error_len() {
                    // SAFETY: By Utf8Error invariants
                    let remaining = unsafe { self.0.get_unchecked(e.valid_up_to() + error_len..) };
                    let _infallible = Self(remaining).write_to(&mut out);
                }
                out.into()
            }
        }
    }
}

impl_display_with_writeable!(PotentiallyInvalidUtf8<'_>);

/// Implements [`Writeable`] for [`&[u16]`] according to the [WHATWG Encoding Standard](
/// https://encoding.spec.whatwg.org/#shared-utf-16-decoder).
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PotentiallyInvalidUtf16<'a>(pub &'a [u16]);

impl Writeable for PotentiallyInvalidUtf16<'_> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        for c in core::char::decode_utf16(self.0.iter().copied()) {
            if let Ok(c) = c {
                sink.write_char(c)?;
            } else {
                sink.write_char(char::REPLACEMENT_CHARACTER)?;
            }
        }
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        // Lower bound is all ASCII, upper bound is all 3-byte code points (including replacement character)
        LengthHint::between(self.0.len(), self.0.len() * 3)
    }
}

impl_display_with_writeable!(PotentiallyInvalidUtf16<'_>);

#[cfg(test)]
mod test {
    use super::*;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_utf8() {
        assert_writeable_eq!(PotentiallyInvalidUtf8(b"Foo Bar"), "Foo Bar");
        assert_writeable_eq!(PotentiallyInvalidUtf8(b"Foo\xFDBar"), "Foo�Bar");
        assert_writeable_eq!(PotentiallyInvalidUtf8(b"Foo\xFDBar\xff"), "Foo�Bar�");
    }

    #[test]
    fn test_utf16() {
        assert_writeable_eq!(PotentiallyInvalidUtf16(&[0xD83E, 0xDD73]), "🥳");
        assert_writeable_eq!(PotentiallyInvalidUtf16(&[0xD83E, 0x20, 0xDD73]), "� �");
    }
}
