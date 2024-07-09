// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;

use core::fmt::Write;
use writeable::{LengthHint, Part, TryWriteable, Writeable};

#[allow(dead_code)]
pub(crate) struct LossyWrap<T>(pub T);

impl<T: TryWriteable> Writeable for LossyWrap<T> {
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let _ = self.0.try_write_to(sink)?;
        Ok(())
    }

    fn writeable_length_hint(&self) -> LengthHint {
        self.0.writeable_length_hint()
    }
}

use core::{char::DecodeUtf16Error, fmt, str::Utf8Error};

/// Implements [`Writeable`] for [`&[u8]`] according to the [WHATWG Encoding Standard](
/// https://encoding.spec.whatwg.org/#utf-8-decoder).
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PotentiallyInvalidUtf8<'a>(pub &'a [u8]);

impl TryWriteable for PotentiallyInvalidUtf8<'_> {
    type Error = Utf8Error;

    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        let mut remaining = self.0;
        let mut r = Ok(());
        loop {
            match core::str::from_utf8(remaining) {
                Ok(valid) => {
                    sink.write_str(valid)?;
                    return Ok(r);
                }
                Err(e) => {
                    // SAFETY: By Utf8Error invariants
                    let valid = unsafe {
                        core::str::from_utf8_unchecked(remaining.get_unchecked(..e.valid_up_to()))
                    };
                    sink.write_str(valid)?;
                    sink.with_part(Part::ERROR, |s| s.write_char(char::REPLACEMENT_CHARACTER))?;
                    if r.is_ok() {
                        r = Err(e);
                    }
                    let Some(error_len) = e.error_len() else {
                        return Ok(r); // end of string
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

    fn try_write_to_string(&self) -> Result<Cow<str>, (Self::Error, Cow<str>)> {
        match core::str::from_utf8(self.0) {
            Ok(valid) => Ok(Cow::Borrowed(valid)),
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
                out.push(char::REPLACEMENT_CHARACTER);

                // If there's more, we can use `try_write_to`
                if let Some(error_len) = e.error_len() {
                    // SAFETY: By Utf8Error invariants
                    let remaining = unsafe { self.0.get_unchecked(e.valid_up_to() + error_len..) };
                    let _discard = Self(remaining).try_write_to(&mut out);
                }

                Err((e, Cow::Owned(out)))
            }
        }
    }
}

/// Implements [`Writeable`] for [`&[u16]`] according to the [WHATWG Encoding Standard](
/// https://encoding.spec.whatwg.org/#shared-utf-16-decoder).
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // newtype
pub struct PotentiallyInvalidUtf16<'a>(pub &'a [u16]);

impl TryWriteable for PotentiallyInvalidUtf16<'_> {
    type Error = DecodeUtf16Error;

    fn try_write_to_parts<S: writeable::PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        let mut r = Ok(());
        for c in core::char::decode_utf16(self.0.iter().copied()) {
            match c {
                Ok(c) => sink.write_char(c)?,
                Err(e) => {
                    if r.is_ok() {
                        r = Err(e);
                    }
                    sink.with_part(Part::ERROR, |s| s.write_char(char::REPLACEMENT_CHARACTER))?;
                }
            }
        }
        Ok(r)
    }

    fn writeable_length_hint(&self) -> LengthHint {
        // Lower bound is all ASCII, upper bound is all 3-byte code points (including replacement character)
        LengthHint::between(self.0.len(), self.0.len() * 3)
    }
}

#[cfg(test)]
mod test {
    #![allow(invalid_from_utf8)] // only way to construct the error
    use super::*;
    use writeable::assert_try_writeable_parts_eq;

    #[test]
    fn test_utf8() {
        assert_try_writeable_parts_eq!(PotentiallyInvalidUtf8(b"Foo Bar"), "Foo Bar", Ok(()), []);
        assert_try_writeable_parts_eq!(
            PotentiallyInvalidUtf8(b"Foo\xFDBar"),
            "Foo�Bar",
            Err(core::str::from_utf8(b"Foo\xFDBar").unwrap_err()),
            [(3, 6, Part::ERROR)]
        );
        assert_try_writeable_parts_eq!(
            PotentiallyInvalidUtf8(b"Foo\xFDBar\xff"),
            "Foo�Bar�",
            Err(core::str::from_utf8(b"Foo\xFDBar\xff").unwrap_err()),
            [(3, 6, Part::ERROR), (9, 12, Part::ERROR)],
        );
    }

    #[test]
    fn test_utf16() {
        assert_try_writeable_parts_eq!(
            PotentiallyInvalidUtf16(&[0xD83E, 0xDD73]),
            "🥳",
            Ok(()),
            []
        );
        assert_try_writeable_parts_eq!(
            PotentiallyInvalidUtf16(&[0xD83E, 0x20, 0xDD73]),
            "� �",
            Err(core::char::decode_utf16([0xD83E].into_iter())
                .next()
                .unwrap()
                .unwrap_err()),
            [(0, 3, Part::ERROR), (4, 7, Part::ERROR)]
        );
    }
}
