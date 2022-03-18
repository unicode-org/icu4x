// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::core::ParseResult;

use super::map::MapValue;

/// A trait for chaining multiple parsed values into a single tuple
/// that contains all of the parsed values in the chain.
pub trait Chain<Tuple, Value, Source>: Sized
where
    Source: Clone,
    Tuple: TupleAppend<Value>,
{
    /// The type of tuple with the next parsed item appended to it.
    type Combined;

    /// Parses the next value in the chain.
    fn chain(
        self,
        f: impl FnOnce(&mut Source) -> ParseResult<Value, Source>,
    ) -> ParseResult<Self::Combined, Source>;

    /// Parses the next value in the chain, given the previously parsed
    /// values as context. This is useful when the way that the next value
    /// in the chain should be parsed is dependent on one or more previous
    /// values in the chain.
    fn chain_with_context(
        self,
        f: impl FnOnce(&Tuple, &mut Source) -> ParseResult<Value, Source>,
    ) -> ParseResult<Self::Combined, Source>;
}

impl<Tuple, Value, Source> Chain<Tuple, Value, Source> for ParseResult<Tuple, Source>
where
    Source: Clone,
    Tuple: TupleAppend<Value>,
{
    type Combined = <Tuple as TupleAppend<Value>>::Output;

    /// Adds the result of the next parse function to the chain.
    /// If `Self` is already [`Err`], then propagates the error.
    fn chain(
        self,
        f: impl FnOnce(&mut Source) -> ParseResult<Value, Source>,
    ) -> ParseResult<Self::Combined, Source> {
        let (tuple, mut source) = self?.split();
        f(&mut source).map_value(|value| tuple.append(value))
    }

    /// Adds the result of the next parse function to the chain.
    /// If `Self` is already [`Err`], then propagates the error.
    fn chain_with_context(
        self,
        f: impl FnOnce(&Tuple, &mut Source) -> ParseResult<Value, Source>,
    ) -> ParseResult<Self::Combined, Source> {
        let (tuple, mut source) = self?.split();
        f(&tuple, &mut source).map_value(|value| tuple.append(value))
    }
}

/// A trait for appending an item to a tuple.
pub trait TupleAppend<T> {
    /// The type of tuple with another element appended to it.
    type Output;
    /// Appends a new item to the tuple.
    fn append(self, other: T) -> Self::Output;
}

impl<Z> TupleAppend<Z> for () {
    type Output = (Z,);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (z,)
    }
}

impl<Y, Z> TupleAppend<Z> for (Y,) {
    type Output = (Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, z)
    }
}

impl<X, Y, Z> TupleAppend<Z> for (X, Y) {
    type Output = (X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, self.1, z)
    }
}

impl<W, X, Y, Z> TupleAppend<Z> for (W, X, Y) {
    type Output = (W, X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, self.1, self.2, z)
    }
}

impl<V, W, X, Y, Z> TupleAppend<Z> for (V, W, X, Y) {
    type Output = (V, W, X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, self.1, self.2, self.3, z)
    }
}

impl<U, V, W, X, Y, Z> TupleAppend<Z> for (U, V, W, X, Y) {
    type Output = (U, V, W, X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, z)
    }
}

impl<T, U, V, W, X, Y, Z> TupleAppend<Z> for (T, U, V, W, X, Y) {
    type Output = (T, U, V, W, X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, z)
    }
}

impl<S, T, U, V, W, X, Y, Z> TupleAppend<Z> for (S, T, U, V, W, X, Y) {
    type Output = (S, T, U, V, W, X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, z)
    }
}

impl<R, S, T, U, V, W, X, Y, Z> TupleAppend<Z> for (R, S, T, U, V, W, X, Y) {
    type Output = (R, S, T, U, V, W, X, Y, Z);
    /// Appends a new item to the tuple.
    fn append(self, z: Z) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, z,
        )
    }
}
