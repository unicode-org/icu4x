// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::Writeable;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt;

enum SliceOrString<'a> {
    Slice(&'a [u8]),
    String(String),
}

struct WriteOrRef<'a> {
    // Safety Invariant: the string is valid UTF-8 up to the offset.
    string: SliceOrString<'a>,
    offset: usize,
}

/// This is an infallible impl. Functions always return Ok, not Err.
impl fmt::Write for WriteOrRef<'_> {
    #[inline]
    fn write_str(&mut self, other: &str) -> fmt::Result {
        let owned: &mut String = match &mut self.string {
            SliceOrString::Slice(slice) => {
                if slice.get(self.offset..self.offset + other.len()) == Some(other.as_bytes()) {
                    // Safety Invariant: `other` is valid UTF-8, and we are
                    // incrementing the offset to cover bytes equal to `other`
                    self.offset += other.len();
                    return Ok(());
                }
                let valid_slice = slice.get(..self.offset).unwrap_or_else(|| {
                    debug_assert!(false, "self.offset always in range");
                    &[]
                });
                debug_assert!(core::str::from_utf8(valid_slice).is_ok());
                // Safety: the UTF-8 of slice has been validated up to offset
                let valid_str = unsafe { core::str::from_utf8_unchecked(valid_slice) };
                // Convert to owned, put it in the field, and get it out again
                let owned = String::from(valid_str);
                self.string = SliceOrString::String(owned);
                let SliceOrString::String(owned) = &mut self.string else {
                    unreachable!()
                };
                owned
            }
            SliceOrString::String(owned) => owned,
        };
        owned.write_str(other)
    }
}

impl<'a> WriteOrRef<'a> {
    #[inline]
    fn new(slice: &'a [u8]) -> Self {
        Self {
            string: SliceOrString::Slice(slice),
            offset: 0,
        }
    }

    #[inline]
    fn finish(self) -> Cow<'a, str> {
        match self.string {
            SliceOrString::Slice(slice) => {
                let valid_slice = slice.get(..self.offset).unwrap_or_else(|| {
                    debug_assert!(false, "self.offset always in range");
                    &[]
                });
                debug_assert!(core::str::from_utf8(valid_slice).is_ok());
                // Safety: the UTF-8 of slice has been validated up to offset
                let valid_str = unsafe { core::str::from_utf8_unchecked(valid_slice) };
                Cow::Borrowed(valid_str)
            }
            SliceOrString::String(owned) => Cow::Owned(owned),
        }
    }
}

/// Writes the contents of a `Writeable` to a string, returning a reference
/// to a slice if it matches, and allocating a string otherwise.
///
/// This function is useful if you have borrowed bytes which you expect
/// to be equal to a writeable a high percentage of the time.
///
/// You can also use this function to make a more efficient implementation of
/// [`Writeable::write_to_string`].
///
/// # Examples
///
/// Basic usage and behavior:
///
/// ```
/// use std::fmt;
/// use std::borrow::Cow;
/// use writeable::Writeable;
///
/// struct WelcomeMessage<'s> {
///     pub name: &'s str,
/// }
///
/// impl<'s> Writeable for WelcomeMessage<'s> {
///     // see impl in Writeable docs
/// #    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
/// #        sink.write_str("Hello, ")?;
/// #        sink.write_str(self.name)?;
/// #        sink.write_char('!')?;
/// #        Ok(())
/// #    }
/// }
///
/// let message = WelcomeMessage { name: "Alice" };
///
/// assert!(matches!(
///     writeable::write_or_ref(&message, b""),
///     Cow::Owned(s) if s == "Hello, Alice!"
/// ));
/// assert!(matches!(
///     writeable::write_or_ref(&message, b"Hello"),
///     Cow::Owned(s) if s == "Hello, Alice!"
/// ));
/// assert!(matches!(
///     writeable::write_or_ref(&message, b"Hello, Bob!"),
///     Cow::Owned(s) if s == "Hello, Alice!"
/// ));
/// assert!(matches!(
///     writeable::write_or_ref(&message, b"Hello, Alice!"),
///     Cow::Borrowed("Hello, Alice!")
/// ));
///
/// // Junk at the end is ignored:
/// assert!(matches!(
///     writeable::write_or_ref(&message, b"Hello, Alice!..\xFF\x00\xFF"),
///     Cow::Borrowed("Hello, Alice!")
/// ));
/// ```
///
/// Example use case: a function that transforms a string to lowercase.
/// We are also able to write a more efficient implementation of
/// [`Writeable::write_to_string`] in this situation.
///
/// ```
/// use std::fmt;
/// use std::borrow::Cow;
/// use writeable::Writeable;
///
/// struct MakeAsciiLower<'a>(&'a str);
///
/// impl<'a> Writeable for MakeAsciiLower<'a> {
///     fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
///         for c in self.0.chars() {
///             sink.write_char(c.to_ascii_lowercase())?;
///         }
///         Ok(())
///     }
///     #[inline]
///     fn write_to_string(&self) -> Cow<str> {
///         writeable::write_or_ref(self, self.0.as_bytes())
///     }
/// }
///
/// fn make_lowercase(input: &str) -> Cow<str> {
///     let writeable = MakeAsciiLower(input);
///     writeable::write_or_ref(&writeable, input.as_bytes())
/// }
///
/// assert!(matches!(
///     make_lowercase("this is lowercase"),
///     Cow::Borrowed("this is lowercase")
/// ));
/// assert!(matches!(
///     make_lowercase("this is UPPERCASE"),
///     Cow::Owned(s) if s == "this is uppercase"
/// ));
///
/// assert!(matches!(
///     MakeAsciiLower("this is lowercase").write_to_string(),
///     Cow::Borrowed("this is lowercase")
/// ));
/// assert!(matches!(
///     MakeAsciiLower("this is UPPERCASE").write_to_string(),
///     Cow::Owned(s) if s == "this is uppercase"
/// ));
/// ```
pub fn write_or_ref<'a>(writeable: &impl Writeable, reference_bytes: &'a [u8]) -> Cow<'a, str> {
    let mut sink = WriteOrRef::new(reference_bytes);
    let _ = writeable.write_to(&mut sink);
    sink.finish()
}
