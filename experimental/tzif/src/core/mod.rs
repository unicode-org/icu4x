// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Generic trait extensions to the core parsing functionality.
pub mod ext;

/// A result containing a parsed item and a source for the remaining input.
pub type ParseResult<Item, Source> = eyre::Result<Parsed<Item, Source>>;

/// A parse error.
#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    /// The error value that is left behind when a populated error is
    /// moved to a subsequent parse result.
    #[error("empty error")]
    Empty,
    /// The source has reached the end of its input.
    #[error("end of parse input")]
    EndOfInput,
    /// There is no source from which to draw more input.
    #[error("no source")]
    NoSource,
}

/// A parsed value and a source that contains the remaining parse input.
#[derive(Debug, Clone)]
pub struct Parsed<Value, Source: Clone>(Value, Source);

impl<Value, Source: Clone> Parsed<Value, Source> {
    /// Creates a new [`Parsed`] object from a value and a source.
    pub fn new(value: Value, source: Source) -> Self {
        Self(value, source)
    }

    /// Turns this [`Parsed`] object into its value, dropping the source.
    pub fn into_value(self) -> Value {
        self.0
    }

    /// Returns a reference of this [`Parsed`] object's value.
    pub fn value(&self) -> &Value {
        &self.0
    }

    /// Returns a copy of this [`Parsed`] object's source.
    pub fn source(&mut self) -> Source {
        self.1.clone()
    }

    /// Splits this [`Parsed`] object into a tuple of its value and source.
    pub fn split(self) -> (Value, Source) {
        (self.0, self.1)
    }
}

/// A trait describing a type that is able to provide parse input.
/// `Item` is the type of item returned by [`ParseInput::next()`].
/// `Collection` is the type of collection of multiple items returned by [`ParseInput::take()`]
/// `Source` is a clonable source of parse input.
pub trait ParseInput<Item, Collection, Source = Self>
where
    Source: Clone + ParseInput<Item, Collection, Source>,
    Self: Sized,
{
    /// Returns [`true`] if the source is at the end of the input, otherwise [`false`].
    fn end_of_input(&mut self) -> bool {
        if let Err(err) = self.next() {
            if let Ok(err) = err.downcast::<ParseError>() {
                return matches!(err, ParseError::EndOfInput);
            }
        }
        false
    }

    /// Returns a [`ParseResult`] that is compatible with the [`Chain`]
    /// trait to parse multiple values into a single tuple.
    ///
    /// See the [`Chain`] trait for more information.
    ///
    /// [`Chain`]: ext::chain::Chain
    fn begin_chain(&mut self) -> ParseResult<(), Source> {
        self.source().map(|source| Parsed::new((), source))
    }

    /// Returns the source of the parse input, if present.
    /// Otherwise returns a [`ParseError::NoSource`].
    fn source(&mut self) -> eyre::Result<Source>;

    /// Returns the next item from source along with a source that contains
    /// the remaining parse input.
    fn next(&mut self) -> ParseResult<Item, Source>;

    /// Returns the next `n` items in a collection, along with a source that
    /// contains the remaining parse input.
    fn take(&mut self, n: usize) -> ParseResult<Collection, Source>;
}

impl<Value, Item, Collection, Source> ParseInput<Item, Collection, Source> for Parsed<Value, Source>
where
    Source: Clone + ParseInput<Item, Collection, Source>,
{
    /// Returns the source of the [`Parsed`] object.
    fn source(&mut self) -> eyre::Result<Source> {
        Ok(self.source())
    }

    /// Returns a [`ParseResult`] containing the next item, and a source containing
    /// the remaining items of the [`ParseInput`].
    fn next(&mut self) -> ParseResult<Item, Source> {
        self.source().next()
    }

    /// Returns a [`ParseResult`] a collection of `n` items, and a source containing
    /// the remaining items of the [`ParseInput`].
    fn take(&mut self, n: usize) -> ParseResult<Collection, Source> {
        self.source().take(n)
    }
}

impl<Value, Item, Collection, Source> ParseInput<Item, Collection, Source>
    for ParseResult<Value, Source>
where
    Source: Clone + ParseInput<Item, Collection, Source>,
{
    /// Returns a copy of the source of this [`ParseResult`] object.
    /// If `Self` is already [`Err`], propagates the error.
    fn source(&mut self) -> eyre::Result<Source> {
        self.as_mut()
            .map_err(|err| std::mem::replace(err, ParseError::Empty.into()))
            .map(|parsed| parsed.source())
    }

    /// Returns a [`ParseResult`] containing the next item, and a source containing
    /// the remaining items of the [`ParseInput`].
    /// If `Self` is already [`Err`], propagates the error by taking it from `Self`.
    fn next(&mut self) -> ParseResult<Item, Source> {
        self.as_mut()
            .map_err(|err| std::mem::replace(err, ParseError::Empty.into()))
            .and_then(|parsed| parsed.source().next())
    }

    /// Returns a [`ParseResult`] a collection of `n` items, and a source containing
    /// the remaining items of the [`ParseInput`].
    /// If `Self` is already [`Err`], propagates the error by taking it from `Self`.
    fn take(&mut self, n: usize) -> ParseResult<Collection, Source> {
        self.as_mut()
            .map_err(|err| std::mem::replace(err, ParseError::Empty.into()))
            .and_then(|parsed| parsed.source().take(n))
    }
}
