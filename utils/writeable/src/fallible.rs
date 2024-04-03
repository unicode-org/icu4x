// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::{convert::Infallible, fmt};

use crate::*;

/// The error returned while writing `Option<T>` when it is the `None` variant.
#[derive(Debug)]
pub struct MissingWriteableError;

impl Writeable for MissingWriteableError {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        // Substitute with the Unicode replacement character
        sink.write_char('\u{FFFD}')
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact(3)
    }
}

impl_display_with_writeable!(MissingWriteableError);

#[test]
fn test_missing_writeable_error() {
    assert_writeable_eq!(MissingWriteableError, "�");
}

#[derive(Debug)]
enum NeverWriteable {}

impl Writeable for NeverWriteable {
    #[inline(always)] // to help the compiler find unreachable code paths
    fn write_to<W: fmt::Write + ?Sized>(&self, _sink: &mut W) -> fmt::Result {
        match *self {}
    }
    #[inline(always)] // to help the compiler find unreachable code paths
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, _sink: &mut S) -> fmt::Result {
        match *self {}
    }
    #[inline(always)] // to help the compiler find unreachable code paths
    fn writeable_length_hint(&self) -> LengthHint {
        match *self {}
    }
    #[inline(always)] // to help the compiler find unreachable code paths
    fn write_to_string(&self) -> Cow<str> {
        match *self {}
    }
    #[inline(always)] // to help the compiler find unreachable code paths
    fn write_cmp_bytes(&self, _other: &[u8]) -> core::cmp::Ordering {
        match *self {}
    }
}

trait UnwrapInfallible {
    type T;
    fn unwrap_infallible(self) -> Self::T;
}

impl<T> UnwrapInfallible for Result<T, Infallible> {
    type T = T;
    #[inline(always)] // to help the compiler find unreachable code paths
    fn unwrap_infallible(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => match e {},
        }
    }
}

/// Lower-level trait for writeables that can fail while writing.
///
/// The methods on this trait should be implemented on custom writeables, but they generally
/// should not called directly. For methods more useful in client code, see [`TryWriteable`].
///
/// # Implementor Notes
///
/// All functions take a `handler` argument. This argument is used by [`TryWriteable`] to
/// customize the error handling behavior.
///
/// When your writeable implementation encounters an error, call the handler with an instance
/// of your error type. The handler will either return `Ok` with a [`Writeable`] that should be
/// written to the sink, or `Err` with an error that should be immediately returned up the stack.
///
/// When in lossy mode, [`FallibleWriteable::Error`] is itself used as the writeable replacement.
///
/// # Examples
///
/// Implementing [`FallibleWriteable`] on a custom type:
///
/// ```
/// use core::fmt;
/// use writeable::assert_writeable_eq;
/// use writeable::FallibleWriteable;
/// use writeable::TryWriteable;
/// use writeable::Writeable;
///
/// struct HelloWriteable<T> {
///     name: Option<T>,
/// }
///
/// impl<T: Writeable> FallibleWriteable for HelloWriteable<T> {
///     type Error = &'static str;
///
///     fn fallible_write_to<
///         W: fmt::Write + ?Sized,
///         L: Writeable,
///         E,
///         F: FnMut(Self::Error) -> Result<L, E>,
///     >(
///         &self,
///         sink: &mut W,
///         mut handler: F,
///     ) -> Result<Result<(), E>, fmt::Error> {
///         sink.write_str("Hello, ")?;
///         match self.name.as_ref() {
///             Some(name) => name.write_to(sink)?,
///             None => match handler("___") {
///                 Ok(replacement) => replacement.write_to(sink)?,
///                 Err(e) => return Ok(Err(e)),
///             },
///         };
///         sink.write_char('!')?;
///         Ok(Ok(()))
///     }
///
///     // TODO: Implement the other methods in a similar way.
/// }
///
/// // See it in action:
/// assert_writeable_eq!(
///     HelloWriteable {
///         name: Some("Alice")
///     }
///     .lossy(),
///     "Hello, Alice!"
/// );
/// assert_writeable_eq!(
///     HelloWriteable { name: None::<&str> }.lossy(),
///     "Hello, ___!"
/// );
/// ```
pub trait FallibleWriteable {
    /// The error type that the writeable may return.
    ///
    /// This type should implement [`Writeable`] so it can be used as a replacement in lossy mode.
    type Error;

    /// Writes a string to the given sink.
    ///
    /// This is a low-level function for implementors. For more details, see [`FallibleWriteable`].
    fn fallible_write_to<
        W: fmt::Write + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut W,
        handler: F,
    ) -> Result<Result<(), E>, fmt::Error>;

    /// Writes a string and [`Part`] annotations to the given sink.
    ///
    /// This is a low-level function for implementors. For more details, see [`FallibleWriteable`].
    fn fallible_write_to_parts<
        S: PartsWrite + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut S,
        handler: F,
    ) -> Result<Result<(), E>, fmt::Error> {
        self.fallible_write_to(sink, handler)
    }

    /// Returns a hint for the length of the string to be written.
    ///
    /// This is a low-level function for implementors. For more details, see [`FallibleWriteable`].
    fn fallible_writeable_length_hint<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        _handler: F,
    ) -> Result<LengthHint, E> {
        Ok(LengthHint::undefined())
    }

    /// Writes directly to a string, returning a borrowed [`Cow`] if able.
    ///
    /// This is a low-level function for implementors. For more details, see [`FallibleWriteable`].
    fn fallible_write_to_string<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        mut handler: F,
    ) -> Result<Cow<str>, E> {
        let hint = self.fallible_writeable_length_hint(&mut handler)?;
        if hint.is_zero() {
            return Ok(Cow::Borrowed(""));
        }
        let mut output = String::with_capacity(hint.capacity());
        match self.fallible_write_to(&mut output, handler) {
            Ok(Ok(())) => Ok(Cow::Owned(output)),
            Ok(Err(e)) => Err(e),
            Err(core::fmt::Error) => unreachable!("writing to string is infallible"),
        }
    }

    /// Compares the contents of this `Writeable` to the given bytes
    /// without allocating a String to hold the `Writeable` contents.
    ///
    /// This is a low-level function for implementors. For more details, see [`FallibleWriteable`].
    fn fallible_write_cmp_bytes<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        other: &[u8],
        handler: F,
    ) -> Result<core::cmp::Ordering, E> {
        let mut wc = cmp::WriteComparator::new(other);
        match self.fallible_write_to(&mut wc, handler) {
            Ok(Ok(())) => (),
            Ok(Err(e)) => return Err(e),
            Err(core::fmt::Error) => unreachable!("writing to string is infallible"),
        }
        Ok(wc.finish().reverse())
    }
}

/// A writeable type with its own configurable error handling.
///
/// This trait offers four ways to consume errors:
///
/// - [`TryWriteable::checked`]
/// - [`TryWriteable::lossy`]
/// - [`TryWriteable::panicky`]
/// - [`TryWriteable::gigo`]
///
/// This trait is blanked-implemented on all types implementing [`FallibleWriteable`],
/// which is the lower-level trait that custom writeables should implement.
pub trait TryWriteable: FallibleWriteable {
    /// Handle errors with functions returning [`Result`].
    ///
    /// # Examples
    ///
    /// Successful behavior on `Option<T>`, which implements [`TryWriteable`]:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = Some("example");
    /// assert_eq!(
    ///     try_writeable.checked().try_write_to_string().unwrap(),
    ///     "example"
    /// );
    /// ```
    ///
    /// Failure behavior:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = None::<&str>;
    /// assert!(matches!(
    ///     try_writeable.checked().try_write_to_string(),
    ///     Err(_),
    /// ));
    /// ```
    #[inline]
    fn checked(&self) -> CheckedWriteable<&Self> {
        CheckedWriteable(self)
    }
    /// Handle errors by writing a replacement to the output string.
    ///
    /// # Examples
    ///
    /// Successful behavior on `Option<T>`, which implements [`TryWriteable`]:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = Some("example");
    /// writeable::assert_writeable_eq!(try_writeable.lossy(), "example");
    /// ```
    ///
    /// Failure behavior:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = None::<&str>;
    /// writeable::assert_writeable_eq!(try_writeable.lossy(), "�");
    /// ```
    #[inline]
    fn lossy(&self) -> LossyWriteable<&Self> {
        LossyWriteable(self)
    }
    /// Handle errors by panicking.
    ///
    /// # Examples
    ///
    /// Successful behavior on `Option<T>`, which implements [`TryWriteable`]:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = Some("example");
    /// writeable::assert_writeable_eq!(try_writeable.panicky(), "example");
    /// ```
    ///
    /// Failure behavior:
    ///
    /// ```should_panic
    /// use writeable::TryWriteable;
    /// use writeable::Writeable;
    ///
    /// let try_writeable = None::<&str>;
    /// try_writeable.panicky().write_to_string();
    /// ```
    #[inline]
    fn panicky(&self) -> PanickyWriteable<&Self> {
        PanickyWriteable(self)
    }
    /// Handle errors by panicking in debug mode and writing a replacement in release mode.
    ///
    /// # Examples
    ///
    /// Successful behavior on `Option<T>`, which implements [`TryWriteable`]:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = Some("example");
    /// writeable::assert_writeable_eq!(try_writeable.gigo(), "example");
    /// ```
    ///
    /// Failure behavior:
    ///
    /// ```
    /// use writeable::TryWriteable;
    ///
    /// let try_writeable = None::<&str>;
    /// if cfg!(not(debug_assertions)) {
    ///     writeable::assert_writeable_eq!(try_writeable.gigo(), "�");
    /// }
    /// ```
    #[inline]
    fn gigo(&self) -> GigoWriteable<&Self> {
        GigoWriteable(self)
    }
}

impl<T> TryWriteable for T where T: FallibleWriteable {}

/// A writeable that returns errors as results.
///
/// For more details, see [`TryWriteable::checked`].
#[derive(Debug)]
pub struct CheckedWriteable<T>(pub(crate) T);

fn resulty_handler<E>(e: E) -> Result<NeverWriteable, E> {
    Err(e)
}

impl<T> CheckedWriteable<T>
where
    T: FallibleWriteable,
{
    #[inline]
    pub fn try_write_to<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
    ) -> Result<Result<(), T::Error>, fmt::Error> {
        self.0.fallible_write_to(sink, resulty_handler)
    }

    #[inline]
    pub fn try_write_to_parts<S: PartsWrite + ?Sized>(
        &self,
        sink: &mut S,
    ) -> Result<Result<(), T::Error>, fmt::Error> {
        self.0.fallible_write_to_parts(sink, resulty_handler)
    }

    #[inline]
    pub fn try_write_to_string(&self) -> Result<Cow<str>, T::Error> {
        self.0.fallible_write_to_string(resulty_handler)
    }
}

/// A writeable that writes replacements when errors are encountered.
///
/// For more details, see [`TryWriteable::lossy`].
#[derive(Debug)]
pub struct LossyWriteable<T>(pub(crate) T);

fn lossy_handler<E>(e: E) -> Result<E, Infallible> {
    Ok(e)
}

impl<T> Writeable for LossyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: Writeable,
{
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let result = self.0.fallible_write_to(sink, lossy_handler)?;
        result.unwrap_infallible();
        Ok(())
    }

    #[inline]
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        let result = self.0.fallible_write_to_parts(sink, lossy_handler)?;
        result.unwrap_infallible();
        Ok(())
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        let result = self.0.fallible_writeable_length_hint(lossy_handler);
        result.unwrap_infallible()
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        let result = self.0.fallible_write_to_string(lossy_handler);
        result.unwrap_infallible()
    }

    #[inline]
    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        let result = self.0.fallible_write_cmp_bytes(other, lossy_handler);
        result.unwrap_infallible()
    }
}

impl_display_with_writeable!([T] LossyWriteable<T> where T: FallibleWriteable, T::Error: Writeable);

/// A writeable that panics when errors are encountered.
///
/// For more details, see [`TryWriteable::panicky`].
#[derive(Debug)]
pub struct PanickyWriteable<T>(pub(crate) T);

fn panicky_handler<E>(e: E) -> Result<NeverWriteable, E> {
    Err(e)
}

impl<T> Writeable for PanickyWriteable<T>
where
    T: FallibleWriteable,
    T::Error: fmt::Debug,
{
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let result = self.0.fallible_write_to(sink, panicky_handler)?;
        result.unwrap();
        Ok(())
    }

    #[inline]
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        let result = self.0.fallible_write_to_parts(sink, panicky_handler)?;
        result.unwrap();
        Ok(())
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        let result = self.0.fallible_writeable_length_hint(panicky_handler);
        result.unwrap()
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        let result = self.0.fallible_write_to_string(panicky_handler);
        result.unwrap()
    }

    #[inline]
    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        let result = self.0.fallible_write_cmp_bytes(other, panicky_handler);
        result.unwrap()
    }
}

impl_display_with_writeable!([T] PanickyWriteable<T> where T: FallibleWriteable, T::Error: fmt::Debug);

/// A writeable that panics in debug mode and writes replacements in release mode.
///
/// For more details, see [`TryWriteable::gigo`].
#[derive(Debug)]
pub struct GigoWriteable<T>(pub(crate) T);

fn gigo_handler<E: fmt::Debug>(e: E) -> Result<E, Infallible> {
    debug_assert!(false, "{e:?}");
    Ok(e)
}

impl<T> Writeable for GigoWriteable<T>
where
    T: FallibleWriteable,
    T::Error: Writeable + fmt::Debug,
{
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let result = self.0.fallible_write_to(sink, gigo_handler)?;
        result.unwrap_infallible();
        Ok(())
    }

    #[inline]
    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        let result = self.0.fallible_write_to_parts(sink, gigo_handler)?;
        result.unwrap_infallible();
        Ok(())
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        let result = self.0.fallible_writeable_length_hint(gigo_handler);
        result.unwrap_infallible()
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        let result = self.0.fallible_write_to_string(gigo_handler);
        result.unwrap_infallible()
    }

    #[inline]
    fn write_cmp_bytes(&self, other: &[u8]) -> core::cmp::Ordering {
        let result = self.0.fallible_write_cmp_bytes(other, gigo_handler);
        result.unwrap_infallible()
    }
}

impl_display_with_writeable!([T] GigoWriteable<T> where T: FallibleWriteable, T::Error: Writeable, T::Error: fmt::Debug);

impl<T> FallibleWriteable for Option<T>
where
    T: Writeable,
{
    type Error = MissingWriteableError;

    fn fallible_write_to<
        W: fmt::Write + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut W,
        mut handler: F,
    ) -> Result<Result<(), E>, fmt::Error> {
        match self {
            Some(writeable) => writeable.write_to(sink).map(Ok),
            None => match handler(MissingWriteableError) {
                Ok(l) => l.write_to(sink).map(Ok),
                Err(e) => Ok(Err(e)),
            },
        }
    }

    fn fallible_write_to_parts<
        S: PartsWrite + ?Sized,
        L: Writeable,
        E,
        F: FnMut(Self::Error) -> Result<L, E>,
    >(
        &self,
        sink: &mut S,
        mut handler: F,
    ) -> Result<Result<(), E>, fmt::Error> {
        match self {
            Some(writeable) => writeable.write_to_parts(sink).map(Ok),
            None => match handler(MissingWriteableError) {
                Ok(l) => l.write_to_parts(sink).map(Ok),
                Err(e) => Ok(Err(e)),
            },
        }
    }

    fn fallible_writeable_length_hint<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        mut handler: F,
    ) -> Result<LengthHint, E> {
        match self {
            Some(writeable) => Ok(writeable.writeable_length_hint()),
            None => match handler(MissingWriteableError) {
                Ok(l) => Ok(l.writeable_length_hint()),
                Err(e) => Err(e),
            },
        }
    }

    fn fallible_write_to_string<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        mut handler: F,
    ) -> Result<Cow<str>, E> {
        match self {
            Some(writeable) => Ok(writeable.write_to_string()),
            None => match handler(MissingWriteableError) {
                Ok(l) => Ok(Cow::Owned(l.write_to_string().into_owned())),
                Err(e) => Err(e),
            },
        }
    }

    fn fallible_write_cmp_bytes<E, L: Writeable, F: FnMut(Self::Error) -> Result<L, E>>(
        &self,
        other: &[u8],
        mut handler: F,
    ) -> Result<core::cmp::Ordering, E> {
        match self {
            Some(writeable) => Ok(writeable.write_cmp_bytes(other)),
            None => match handler(MissingWriteableError) {
                Ok(l) => Ok(l.write_cmp_bytes(other)),
                Err(e) => Err(e),
            },
        }
    }
}

#[test]
fn test_basic() {
    let mut sink = String::new();

    Some("abcdefg")
        .checked()
        .try_write_to(&mut sink)
        .unwrap()
        .unwrap();
    assert_eq!(sink, "abcdefg");
    assert!(matches!(
        None::<&str>.checked().try_write_to(&mut sink),
        Ok(Err(MissingWriteableError))
    ));

    assert_writeable_eq!(Some("abcdefg").lossy(), "abcdefg");
    assert_eq!(
        Some("abcdefg").lossy().writeable_length_hint(),
        LengthHint::exact(7)
    );

    assert_writeable_eq!(None::<&str>.lossy(), "�");
    assert_eq!(
        None::<&str>.lossy().writeable_length_hint(),
        LengthHint::exact(3)
    );

    assert_writeable_eq!(Some("abcdefg").panicky(), "abcdefg");
    assert_eq!(
        Some("abcdefg").panicky().writeable_length_hint(),
        LengthHint::exact(7)
    );
}
