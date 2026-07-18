// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::{concat::Concat, replace::Replace, Writeable};

/// An extension trait that adds convenience functions to anything implementing [`Writeable`].
pub trait WriteableExt {
    /// Returns a [`Writeable`] that performs string replacement without allocating.
    /// For more details, see [`Replace`].
    ///
    /// # Examples
    ///
    /// ```
    /// use writeable::WriteableExt;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_writeable_eq!(
    ///     "Hello, World!".replace_streaming("World", "Earth"),
    ///     "Hello, Earth!",
    /// );
    /// ```
    fn replace_streaming<S, W1>(self, needle: S, replacement: W1) -> Replace<Self, S, W1>
    where
        Self: Sized;

    /// Returns a [`Writeable`] that performs concatenation without allocating.
    /// For more details, see [`Concat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use writeable::WriteableExt;
    /// use writeable::assert_writeable_eq;
    ///
    /// assert_writeable_eq!(
    ///     "Hello, ".concat_streaming("Earth").concat_streaming('!'),
    ///     "Hello, Earth!",
    /// );
    /// ```
    fn concat_streaming<W1>(self, other: W1) -> Concat<Self, W1>
    where
        Self: Sized;

    /// Returns a function closure for integrating with third-party libraries.
    ///
    /// # Example
    ///
    /// Integration with a third-party library that can leverage `writeable`'s benefits
    /// without needing a direct dependency:
    ///
    /// ```
    /// use core::fmt;
    /// use writeable::WriteableExt;
    ///
    /// // Example third-party library:
    /// // Custom sink:
    /// struct ThirdPartySink(String);
    /// impl fmt::Write for ThirdPartySink {
    ///     fn write_str(&mut self, value: &str) -> fmt::Result {
    ///         self.0.write_str(value)
    ///     }
    /// }
    /// // Endpoint:
    /// fn third_party_fn(
    ///     value: impl FnOnce(&mut ThirdPartySink) -> fmt::Result
    /// ) -> Result<String, fmt::Error> {
    ///     let mut sink = ThirdPartySink(String::new());
    ///     value(&mut sink)?;
    ///     Ok(sink.0)
    /// }
    ///
    /// // Example usage:
    /// let s = third_party_fn("Hello, ".concat_streaming("World").into_write_fn()).unwrap();
    /// assert_eq!(s, "Hello, World");
    /// ```
    fn into_write_fn<W1: fmt::Write>(self) -> impl FnMut(&mut W1) -> fmt::Result;
}

impl<W> WriteableExt for W
where
    W: Writeable,
{
    #[inline]
    fn replace_streaming<S, W1>(self, needle: S, replacement: W1) -> Replace<Self, S, W1> {
        Replace {
            source: self,
            needle,
            replacement,
        }
    }

    #[inline]
    fn concat_streaming<W1>(self, other: W1) -> Concat<Self, W1> {
        Concat(self, other)
    }

    #[inline]
    fn into_write_fn<W1: fmt::Write>(self) -> impl FnMut(&mut W1) -> fmt::Result {
        move |sink| self.write_to(sink)
    }
}
