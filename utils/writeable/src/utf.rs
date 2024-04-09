// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{impl_display_with_writeable, LengthHint, Writeable};

use core::fmt;

/// Implements [`Writeable`] for [`&[u8]`] according to the [WHATWG Encoding Standard](
/// https://encoding.spec.whatwg.org/#utf-8-decoder).
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PotentiallyInvalidUtf8<'a>(pub &'a [u8]);

impl Writeable for PotentiallyInvalidUtf8<'_> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut remaining = self.0;
        while !remaining.is_empty() {
            match core::str::from_utf8(remaining) {
                Ok(str) => {
                    return sink.write_str(str);
                }
                Err(e) => {
                    let (str, rest) = remaining.split_at(e.valid_up_to());
                    sink.write_str(unsafe { core::str::from_utf8_unchecked(str) })?;
                    sink.write_char(char::REPLACEMENT_CHARACTER)?;
                    match e.error_len() {
                        None => remaining = &[],
                        #[allow(clippy::indexing_slicing)] // l guaranteed to be in rest
                        Some(l) => remaining = &rest[l..],
                    }
                }
            }
        }

        Ok(())
    }

    fn writeable_length_hint(&self) -> crate::LengthHint {
        // In the worst case, every byte becomes a replacement character
        LengthHint::at_most(self.0.len() * 3)
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
        LengthHint::undefined() // todo
    }
}

impl_display_with_writeable!(PotentiallyInvalidUtf16<'_>);

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_writeable_eq;

    #[test]
    fn test_utf8() {
        assert_writeable_eq!(PotentiallyInvalidUtf8(b"Foo Bar"), "Foo Bar");
        assert_writeable_eq!(PotentiallyInvalidUtf8(b"Foo\xFDBar"), "Foo�Bar");
    }

    #[test]
    fn test_utf16() {
        assert_writeable_eq!(PotentiallyInvalidUtf16(&[0xD83E, 0xDD73]), "🥳");
        assert_writeable_eq!(PotentiallyInvalidUtf16(&[0xD83E, 0x20, 0xDD73]), "� �");
    }
}
