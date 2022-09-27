// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

/// A trait which has to be implemented on any type that will be used to
/// provide replacement values for the placeholder pattern during interpolation.
///
/// # Examples
///
/// Since the trait comes with implementations for [`Vec<E>`] and [`HashMap<String, E>`][`HashMap`],
/// most common cases are already covered and manual implementation of the trait
/// is not needed.
///
/// The consumer may want to implement it in less conventional cases where the replacements
/// are stored in a different data collection.
/// To illustrate such example, we'll use a `HashMap<usize, E>`, a map where keys value and position may
/// be disassociate.
///
/// ```
/// use icu_pattern::{
///     InterpolatedKind, Interpolator, Parser, ParserOptions, Pattern,
///     ReplacementProvider,
/// };
/// use std::{borrow::Cow, collections::HashMap, convert::TryInto};
///
/// #[derive(Debug, PartialEq)]
/// enum Element {
///     TokenZero,
///     TokenFive,
/// }
///
/// impl<'r> ReplacementProvider<'r, Element> for HashMap<usize, Vec<Element>> {
///     type Key = usize;
///     type Iter = std::slice::Iter<'r, Element>;
///
///     fn take_replacement(&'r self, key: &usize) -> Option<Self::Iter> {
///         let replacement = self.get(key)?;
///         Some(replacement.iter())
///     }
/// }
///
/// let mut replacements = HashMap::new();
/// replacements.insert(0, vec![Element::TokenZero]);
/// replacements.insert(5, vec![Element::TokenFive]);
///
/// let pattern: Pattern<_> = Parser::new(
///     "{5}, {0}",
///     ParserOptions {
///         allow_raw_letters: false,
///     },
/// )
/// .try_into()
/// .unwrap();
/// let mut interpolator = Interpolator::new(&pattern, &replacements);
///
/// assert_eq!(
///     Ok(Some(InterpolatedKind::Element(&Element::TokenFive))),
///     interpolator.try_next()
/// );
/// assert_eq!(
///     Ok(Some(InterpolatedKind::Literal(&", ".into()))),
///     interpolator.try_next()
/// );
/// assert_eq!(
///     Ok(Some(InterpolatedKind::Element(&Element::TokenZero))),
///     interpolator.try_next()
/// );
/// assert_eq!(Ok(None), interpolator.try_next());
/// ```
pub trait ReplacementProvider<'r, E: 'r> {
    type Key;
    type Iter: Iterator<Item = &'r E>;

    /// Retrieves a replacement iterator to be used by the [`Interpolator`] in
    /// place of a placeholder.
    ///
    /// # Examples
    /// ```
    /// use icu_pattern::ReplacementProvider;
    /// use std::{collections::HashMap, convert::TryInto};
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum Element {
    ///     TokenFive,
    /// }
    ///
    /// impl<'r> ReplacementProvider<'r, Element> for HashMap<usize, Vec<Element>> {
    ///     type Key = usize;
    ///     type Iter = std::slice::Iter<'r, Element>;
    ///
    ///     fn take_replacement(&'r self, key: &usize) -> Option<Self::Iter> {
    ///         let replacement = self.get(key)?;
    ///         Some(replacement.iter())
    ///     }
    /// }
    ///
    /// let mut replacements = HashMap::new();
    /// replacements.insert(5, vec![Element::TokenFive]);
    ///
    /// assert_eq!(
    ///     replacements.take_replacement(&5).map(|r| r.collect()),
    ///     Some(vec![&Element::TokenFive])
    /// );
    ///
    /// assert_eq!(
    ///     replacements
    ///         .take_replacement(&1)
    ///         .map(|r| r.collect::<Vec<_>>()),
    ///     None
    /// );
    /// ```
    ///
    /// [`Interpolator`]: crate::interpolator::Interpolator
    fn take_replacement(&'r self, key: &Self::Key) -> Option<Self::Iter>;
}

impl<'r, E: 'r> ReplacementProvider<'r, E> for Vec<Vec<E>> {
    type Key = usize;
    type Iter = std::slice::Iter<'r, E>;

    fn take_replacement(&'r self, input: &usize) -> Option<Self::Iter> {
        let replacement = self.get(*input)?;
        Some(replacement.iter())
    }
}

impl<'r, E: 'r> ReplacementProvider<'r, E> for Vec<E> {
    type Key = usize;
    type Iter = std::iter::Once<&'r E>;

    fn take_replacement(&'r self, input: &usize) -> Option<Self::Iter> {
        let replacement = self.get(*input)?;
        Some(std::iter::once(replacement))
    }
}

impl<'r, E: 'r> ReplacementProvider<'r, E> for HashMap<String, Vec<E>> {
    type Key = String;
    type Iter = std::slice::Iter<'r, E>;

    fn take_replacement(&'r self, input: &String) -> Option<Self::Iter> {
        let replacement = self.get(input)?;
        Some(replacement.iter())
    }
}

impl<'r, E: 'r> ReplacementProvider<'r, E> for HashMap<String, E> {
    type Key = String;
    type Iter = std::iter::Once<&'r E>;

    fn take_replacement(&'r self, input: &String) -> Option<Self::Iter> {
        let replacement = self.get(input)?;
        Some(std::iter::once(replacement))
    }
}
