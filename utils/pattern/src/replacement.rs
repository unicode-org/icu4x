// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

/// A trait which has to be implemented on any type that will be used to
/// provide replacement values for the placeholder pattern during interpolation.
///
/// # Examples
///
/// Since the trait comes with implementations for [`Vec`] and [`HashMap<String, E>`][`HashMap`],
/// most common cases are already covered and manual implementation of the trait
/// is not needed.
///
/// The consumer may want to implement it in less conventional cases where the replacements
/// are stored in a different data collection.
/// To illustrate such example, we'll use a `HashMap<usize, E>`, a map where keys value and position may
/// be disassociate.
///
/// ```
/// use icu_pattern::{Parser, Interpolator, ReplacementProvider};
/// use std::{
///     collections::HashMap,
///     convert::TryInto,
///     borrow::Cow,
/// };
///
/// #[derive(Debug, PartialEq)]
/// enum Element {
///     Literal(String),
///     TokenZero,
///     TokenFive,
/// }
///
/// // This is necessary to allow for parser literals to be adopted into the final interpolation
/// // return type.
/// impl From<Cow<'_, str>> for Element {
///     fn from(input: Cow<'_, str>) -> Self {
///        Self::Literal(input.to_string())
///     }
/// }
///
/// impl ReplacementProvider<Element> for HashMap<usize, Vec<Element>> {
///     type Key = usize;
///     type Iter = <Vec<Element> as IntoIterator>::IntoIter;
///
///     fn take_replacement(&mut self, key: &usize) -> Option<Self::Iter> {
///         let r = self.remove(key)?;
///         Some(r.into_iter())
///     }
/// }
///
/// let mut replacements = HashMap::new();
/// replacements.insert(0, vec![
///     Element::TokenZero
/// ]);
/// replacements.insert(5, vec![
///     Element::TokenFive
/// ]);
///
/// let pattern: Vec<_> = Parser::new("{5}, {0}", false).try_into().unwrap();
/// let mut interpolator = Interpolator::new(&pattern, replacements);
///
///
/// assert_eq!(Ok(Some(Element::TokenFive)), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Literal(", ".to_string()))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::TokenZero)), interpolator.try_next());
/// assert_eq!(Ok(None), interpolator.try_next());
/// ```
///
/// # Advanced Replacement Provider
///
/// Another scenario in which a consumer may want to implement their own `ReplacementProvider`
/// is when they can avoid allocation and storing of the elements, and instead can stream the
/// replacements or compute them on the fly.
///
/// To illustrate such an example, we'll create a custom iterator which computes digits from `1`
/// to some `max` value and return such iterator from a custom replacement provider.
///
/// ## Examples
/// ```
/// use icu_pattern::{Parser, Interpolator, ReplacementProvider};
/// use std::{
///     convert::TryInto,
///     borrow::Cow
/// };
///
/// #[derive(Debug, PartialEq)]
/// enum Element {
///     Digit(usize),
///     Literal(String)
/// }
///
/// impl From<Cow<'_, str>> for Element {
///     fn from(input: Cow<'_, str>) -> Self {
///         Self::Literal(input.to_string())
///     }
/// }
///
/// struct MyReplacementProvider;
///
/// struct MyIterator {
///     idx: usize,
///     max: usize,
/// }
///
/// impl MyIterator {
///     pub fn new(max: usize) -> Self {
///         Self {
///             idx: 0,
///             max
///         }
///     }
/// }
///
/// impl Iterator for MyIterator {
///     type Item = Element;
///
///     fn next(&mut self) -> Option<Self::Item> {
///         self.idx += 1;
///         if self.idx <= self.max {
///             Some(Element::Digit(self.idx))
///         } else {
///             None
///         }
///     }
/// }
///
///
/// impl ReplacementProvider<Element> for MyReplacementProvider {
///     type Key = usize;
///     type Iter = MyIterator;
///
///     fn take_replacement(&mut self, key: &usize) -> Option<Self::Iter> {
///         Some(MyIterator { idx: 0, max: *key })
///     }
/// }
///
/// let pattern: Vec<_> = Parser::new("{4}, {2}", false).try_into().unwrap();
/// let mut interpolator = Interpolator::new(&pattern, MyReplacementProvider);
///
/// assert_eq!(Ok(Some(Element::Digit(1))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Digit(2))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Digit(3))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Digit(4))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Literal(", ".to_string()))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Digit(1))), interpolator.try_next());
/// assert_eq!(Ok(Some(Element::Digit(2))), interpolator.try_next());
/// assert_eq!(Ok(None), interpolator.try_next());
/// ```
pub trait ReplacementProvider<E> {
    type Key;
    type Iter: Iterator<Item = E>;

    /// Retrieves a replacement iterator to be used by the [`Interpolator`] in
    /// place of a placeholder.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::ReplacementProvider;
    /// use std::{
    ///     collections::HashMap,
    ///     convert::TryInto
    /// };
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum Element {
    ///     TokenFive,
    /// }
    ///
    /// impl ReplacementProvider<Element> for HashMap<usize, Vec<Element>> {
    ///     type Key = usize;
    ///     type Iter = <Vec<Element> as IntoIterator>::IntoIter;
    ///
    ///     fn take_replacement(&mut self, key: &usize) -> Option<Self::Iter> {
    ///         let r = self.remove(key)?;
    ///         Some(r.into_iter())
    ///     }
    /// }
    ///
    /// let mut replacements = HashMap::new();
    /// replacements.insert(5, vec![
    ///     Element::TokenFive
    /// ]);
    ///
    /// assert_eq!(
    ///     replacements.take_replacement(&5).map(|r| r.collect()),
    ///     Some(vec![Element::TokenFive])
    /// );
    ///
    /// assert_eq!(
    ///     replacements.take_replacement(&1).map(|r| r.collect::<Vec<_>>()),
    ///     None
    /// );
    /// ```
    ///
    /// [`Interpolator`]: crate::interpolator::Interpolator
    fn take_replacement(&mut self, key: &Self::Key) -> Option<Self::Iter>;
}

impl<E> ReplacementProvider<E> for Vec<Vec<E>> {
    type Key = usize;
    type Iter = <Vec<E> as IntoIterator>::IntoIter;

    fn take_replacement(&mut self, input: &usize) -> Option<Self::Iter> {
        let r = self.get_mut(*input)?;
        Some(std::mem::take(r).into_iter())
    }
}

impl<E> ReplacementProvider<E> for Vec<Option<E>> {
    type Key = usize;
    type Iter = std::iter::Once<E>;

    fn take_replacement(&mut self, input: &usize) -> Option<Self::Iter> {
        if let Some(elem) = self.get_mut(*input) {
            if let Some(value) = elem.take() {
                Some(std::iter::once(value))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<E> ReplacementProvider<E> for HashMap<String, Vec<E>> {
    type Key = String;
    type Iter = <Vec<E> as IntoIterator>::IntoIter;

    fn take_replacement(&mut self, input: &String) -> Option<Self::Iter> {
        let r = self.remove(input)?;
        Some(r.into_iter())
    }
}

impl<E> ReplacementProvider<E> for HashMap<String, E> {
    type Key = String;
    type Iter = std::iter::Once<E>;

    fn take_replacement(&mut self, input: &String) -> Option<Self::Iter> {
        let r = self.remove(input)?;
        Some(std::iter::once(r))
    }
}
