// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use core::cmp::Ordering;

/// A writeable object that can fail while writing.
///
/// The default [`Writeable`] trait returns a [`fmt::Error`], which originates from the sink.
/// In contrast, this trait allows the _writeable itself_ to trigger an error as well.
///
/// Implementations are expected to always make a _best attempt_ at writing to the sink
/// and should write replacement values in the error state. Therefore, the returned `Result`
/// can be safely ignored to emulate a "lossy" mode.
///
/// Any error substrings should be annotated with [`Part::ERROR`].
///
/// # Implementer Notes
///
/// This trait requires that implementers make a _best attempt_ at writing to the sink,
/// _even in the error state_, such as with a placeholder or fallback string.
///
/// In [`TryWriteable::try_write_to_parts()`], error substrings should be annotated with
/// [`Part::ERROR`]. Because of this, writing to parts is not default-implemented like
/// it is on [`Writeable`].
///
/// The trait is implemented on [`Result<T, E>`] where `T` and `E` both implement [`Writeable`];
/// In the `Ok` case, `T` is written, and in the `Err` case, `E` is written as a fallback value.
/// This impl, which writes [`Part::ERROR`], can be used as a basis for more advanced impls.
///
/// # Examples
///
/// Implementing on a custom type:
///
/// ```
/// use core::fmt;
/// use writeable::LengthHint;
/// use writeable::PartsWrite;
/// use writeable::TryWriteable;
///
/// #[derive(Debug, PartialEq, Eq)]
/// enum HelloWorldWriteableError {
///     MissingName,
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct HelloWorldWriteable {
///     pub name: Option<&'static str>,
/// }
///
/// impl TryWriteable for HelloWorldWriteable {
///     type Error = HelloWorldWriteableError;
///
///     fn try_write_to_parts<S: PartsWrite + ?Sized>(
///         &self,
///         sink: &mut S,
///     ) -> Result<Result<(), Self::Error>, fmt::Error> {
///         sink.write_str("Hello, ")?;
///         // Use `impl TryWriteable for Result` to generate the error part:
///         let err = self.name.ok_or("nobody").try_write_to_parts(sink)?.err();
///         sink.write_char('!')?;
///         // Return a doubly-wrapped Result.
///         // The outer Result is for fmt::Error, handled by the `?`s above.
///         // The inner Result is for our own Self::Error.
///         if err.is_none() {
///             Ok(Ok(()))
///         } else {
///             Ok(Err(HelloWorldWriteableError::MissingName))
///         }
///     }
///
///     fn writeable_length_hint(&self) -> LengthHint {
///         self.name.ok_or("nobody").writeable_length_hint() + 8
///     }
/// }
///
/// // Success case:
/// writeable::assert_try_writeable_eq!(
///     HelloWorldWriteable {
///         name: Some("Alice")
///     },
///     "Hello, Alice!"
/// );
///
/// // Failure case, including the ERROR part:
/// writeable::assert_try_writeable_parts_eq!(
///     HelloWorldWriteable { name: None },
///     "Hello, nobody!",
///     Err(HelloWorldWriteableError::MissingName),
///     [(7, 13, writeable::Part::ERROR)]
/// );
/// ```
pub trait TryWriteable {
    type Error;

    /// Writes the content of this writeable to a sink.
    ///
    /// If the sink hits an error, writing immediately ends,
    /// `Err(`[`fmt::Error`]`)` is returned, and the sink does not contain valid output.
    ///
    /// If the writeable hits an error, writing is continued with a replacement value,
    /// `Ok(Err(`[`TryWriteable::Error`]`))` is returned, and the caller may continue using the sink.
    ///
    /// # Lossy Mode
    ///
    /// The [`fmt::Error`] should always be handled, but the [`TryWriteable::Error`] can be
    /// ignored if a fallback string is desired instead of an error.
    ///
    /// To handle the sink error, but not the writeable error, write:
    ///
    /// ```
    /// # use writeable::TryWriteable;
    /// # let my_writeable: Result<&str, &str> = Ok("");
    /// # let mut sink = String::new();
    /// let _ = my_writeable.try_write_to(&mut sink)?;
    /// # Ok::<(), core::fmt::Error>(())
    /// ```
    ///
    /// # Examples
    ///
    /// The following examples use `Result<&str, usize>`, which implements [`TryWriteable`] because both `&str` and `usize` do.
    ///
    /// Success case:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let w: Result<&str, usize> = Ok("success");
    /// let mut sink = String::new();
    /// let result = w.try_write_to(&mut sink);
    ///
    /// assert_eq!(result, Ok(Ok(())));
    /// assert_eq!(sink, "success");
    /// ```
    ///
    /// Failure case:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let w: Result<&str, usize> = Err(44);
    /// let mut sink = String::new();
    /// let result = w.try_write_to(&mut sink);
    ///
    /// assert_eq!(result, Ok(Err(44)));
    /// assert_eq!(sink, "44");
    /// ```
    fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        self.try_write_to_parts(&mut helpers::CoreWriteAsPartsWrite(sink))
    }

    /// Writes the content of this writeable to a sink with parts (annotations).
    ///
    /// For more information, see:
    ///
    /// - [`TryWriteable::try_write_to()`] for the general behavior.
    /// - [`TryWriteable`] for an example with parts.
    /// - [`Part`] for more about parts.
    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error>;

    /// Returns a hint for the number of UTF-8 bytes that will be written to the sink.
    ///
    /// This function returns the length of the "lossy mode" string; for more information,
    /// see [`TryWriteable::try_write_to()`].
    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::undefined()
    }

    /// Writes the content of this writeable to a string.
    ///
    /// This function does not return a string in the failure case. If you need a replacement
    /// string ("lossy mode"), use [`TryWriteable::try_write_to()`] instead.
    fn try_write_to_string(&self) -> Result<Cow<str>, Self::Error> {
        let hint = self.writeable_length_hint();
        if hint.is_zero() {
            return Ok(Cow::Borrowed(""));
        }
        let mut output = String::with_capacity(hint.capacity());
        let result = self.try_write_to(&mut output).unwrap_or_else(|core::fmt::Error| Ok(()));
        result.map(|_| Cow::Owned(output))
    }

    /// Compares the content of this writeable to a byte slice.
    ///
    /// This function compares the "lossy mode" string; for more information,
    /// see [`TryWriteable::try_write_to()`].
    ///
    /// For more information, see [`Writeable::write_cmp_bytes()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use core::cmp::Ordering;
    /// use core::fmt;
    /// use writeable::TryWriteable;
    /// # use writeable::PartsWrite;
    /// # use writeable::LengthHint;
    ///
    /// #[derive(Debug, PartialEq, Eq)]
    /// enum HelloWorldWriteableError {
    ///     MissingName
    /// }
    ///
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct HelloWorldWriteable {
    ///     pub name: Option<&'static str>
    /// }
    ///
    /// impl TryWriteable for HelloWorldWriteable {
    ///     type Error = HelloWorldWriteableError;
    ///     // see impl in TryWriteable docs
    /// #    fn try_write_to_parts<S: PartsWrite + ?Sized>(
    /// #        &self,
    /// #        sink: &mut S,
    /// #    ) -> Result<Result<(), Self::Error>, fmt::Error> {
    /// #        sink.write_str("Hello, ")?;
    /// #        // Use `impl TryWriteable for Result` to generate the error part:
    /// #        let _ = self.name.ok_or("nobody").try_write_to_parts(sink)?;
    /// #        sink.write_char('!')?;
    /// #        // Return a doubly-wrapped Result.
    /// #        // The outer Result is for fmt::Error, handled by the `?`s above.
    /// #        // The inner Result is for our own Self::Error.
    /// #        if self.name.is_some() {
    /// #            Ok(Ok(()))
    /// #        } else {
    /// #            Ok(Err(HelloWorldWriteableError::MissingName))
    /// #        }
    /// #    }
    /// }
    ///
    /// // Success case:
    /// let writeable = HelloWorldWriteable { name: Some("Alice") };
    /// let writeable_str = writeable.try_write_to_string().expect("name is Some");
    ///
    /// assert_eq!(Ordering::Equal, writeable.write_cmp_bytes(b"Hello, Alice!"));
    ///
    /// assert_eq!(Ordering::Greater, writeable.write_cmp_bytes(b"Alice!"));
    /// assert_eq!(Ordering::Greater, (*writeable_str).cmp("Alice!"));
    ///
    /// assert_eq!(Ordering::Less, writeable.write_cmp_bytes(b"Hello, Bob!"));
    /// assert_eq!(Ordering::Less, (*writeable_str).cmp("Hello, Bob!"));
    ///
    /// // Failure case:
    /// let writeable = HelloWorldWriteable { name: None };
    /// let mut writeable_str = String::new();
    /// let _ = writeable.try_write_to(&mut writeable_str).expect("write to String is infallible");
    ///
    /// assert_eq!(Ordering::Equal, writeable.write_cmp_bytes(b"Hello, nobody!"));
    ///
    /// assert_eq!(Ordering::Greater, writeable.write_cmp_bytes(b"Hello, alice!"));
    /// assert_eq!(Ordering::Greater, (*writeable_str).cmp("Hello, alice!"));
    ///
    /// assert_eq!(Ordering::Less, writeable.write_cmp_bytes(b"Hello, zero!"));
    /// assert_eq!(Ordering::Less, (*writeable_str).cmp("Hello, zero!"));
    /// ```
    fn write_cmp_bytes(&self, other: &[u8]) -> Ordering {
        let mut wc = cmp::WriteComparator::new(other);
        let _ = match self.try_write_to(&mut wc) {
            Ok(result) => result,
            Err(core::fmt::Error) => {
                debug_assert!(false, "WriteComparator infallible");
                Ok(())
            }
        };
        wc.finish().reverse()
    }
}

impl<T, E> TryWriteable for Result<T, E>
where
    T: Writeable,
    E: Writeable + Clone,
{
    type Error = E;

    #[inline]
    fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        match self {
            Ok(t) => t.write_to(sink).map(Ok),
            Err(e) => e.write_to(sink).map(|_| Err(e.clone())),
        }
    }

    #[inline]
    fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), Self::Error>, fmt::Error> {
        match self {
            Ok(t) => t.write_to_parts(sink).map(Ok),
            Err(e) => sink
                .with_part(Part::ERROR, |sink| e.write_to_parts(sink))
                .map(|_| Err(e.clone())),
        }
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        match self {
            Ok(t) => t.writeable_length_hint(),
            Err(e) => e.writeable_length_hint(),
        }
    }

    #[inline]
    fn try_write_to_string(&self) -> Result<Cow<str>, Self::Error> {
        match self {
            Ok(t) => Ok(t.write_to_string()),
            Err(e) => Err(e.clone()),
        }
    }

    #[inline]
    fn write_cmp_bytes(&self, other: &[u8]) -> Ordering {
        match self {
            Ok(t) => t.write_cmp_bytes(other),
            Err(e) => e.write_cmp_bytes(other),
        }
    }
}

/// Testing macros for types implementing [`TryWriteable`].
///
/// Arguments, in order:
///
/// 1. The [`TryWriteable`] under test
/// 2. The expected string value
/// 3. The expected result value, or `Ok(())` if omitted
/// 3. [`*_parts_eq`] only: a list of parts (`[(start, end, Part)]`)
///
/// Any remaining arguments get passed to `format!`
///
/// The macros tests the following:
///
/// - Equality of string content
/// - Equality of parts ([`*_parts_eq`] only)
/// - Validity of size hint
/// - Reflexivity of `cmp_bytes`
///
/// For a usage example, see [`TryWriteable`].
///
/// [`*_parts_eq`]: assert_try_writeable_parts_eq
#[macro_export]
macro_rules! assert_try_writeable_eq {
    ($actual_writeable:expr, $expected_str:expr $(,)?) => {
        $crate::assert_try_writeable_eq!($actual_writeable, $expected_str, Ok(()))
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr $(,)?) => {
        $crate::assert_try_writeable_eq!($actual_writeable, $expected_str, $expected_result, "")
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $($arg:tt)+) => {{
        $crate::assert_try_writeable_eq!(@internal, $actual_writeable, $expected_str, $expected_result, $($arg)*);
    }};
    (@internal, $actual_writeable:expr, $expected_str:expr, $expected_result:expr, $($arg:tt)+) => {{
        use $crate::TryWriteable;
        let actual_writeable = &$actual_writeable;
        let (actual_str, actual_parts, actual_error) = $crate::_internal::try_writeable_to_parts_for_test(actual_writeable);
        assert_eq!(actual_str, $expected_str, $($arg)*);
        assert_eq!(actual_error, Result::<(), _>::from($expected_result).err(), $($arg)*);
        let actual_result = match actual_writeable.try_write_to_string() {
            Ok(actual_cow_str) => {
                assert_eq!(actual_cow_str, $expected_str, $($arg)+);
                Ok(())
            }
            Err(e) => Err(e)
        };
        assert_eq!(actual_result, Result::<(), _>::from($expected_result), $($arg)*);
        let length_hint = actual_writeable.writeable_length_hint();
        assert!(
            length_hint.0 <= actual_str.len(),
            "hint lower bound {} larger than actual length {}: {}",
            length_hint.0, actual_str.len(), format!($($arg)*),
        );
        if let Some(upper) = length_hint.1 {
            assert!(
                actual_str.len() <= upper,
                "hint upper bound {} smaller than actual length {}: {}",
                length_hint.0, actual_str.len(), format!($($arg)*),
            );
        }
        let ordering = actual_writeable.write_cmp_bytes($expected_str.as_bytes());
        assert_eq!(ordering, core::cmp::Ordering::Equal, $($arg)*);
        actual_parts // return for assert_try_writeable_parts_eq
    }};
}

/// See [`assert_try_writeable_eq`].
#[macro_export]
macro_rules! assert_try_writeable_parts_eq {
    ($actual_writeable:expr, $expected_str:expr, $expected_parts:expr $(,)?) => {
        $crate::assert_try_writeable_parts_eq!($actual_writeable, $expected_str, Ok(()), $expected_parts)
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $expected_parts:expr $(,)?) => {
        $crate::assert_try_writeable_parts_eq!($actual_writeable, $expected_str, $expected_result, $expected_parts, "")
    };
    ($actual_writeable:expr, $expected_str:expr, $expected_result:expr, $expected_parts:expr, $($arg:tt)+) => {{
        let actual_parts = $crate::assert_try_writeable_eq!(@internal, $actual_writeable, $expected_str, $expected_result, $($arg)*);
        assert_eq!(actual_parts, $expected_parts, $($arg)+);
    }};
}

#[test]
fn test_result_try_writeable() {
    let mut result: Result<&str, usize> = Ok("success");
    assert_try_writeable_eq!(result, "success");
    result = Err(44);
    assert_try_writeable_eq!(result, "44", Err(44));
    assert_try_writeable_parts_eq!(result, "44", Err(44), [(0, 2, Part::ERROR)])
}
