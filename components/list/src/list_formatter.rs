// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::options::{ListFormatterOptions, ListLength};
use crate::provider::*;
#[cfg(feature = "alloc")]
use alloc::string::String;
use core::fmt::{self, Write};
use icu_locale_core::preferences::define_preferences;
use icu_provider::marker::ErasedMarker;
use icu_provider::prelude::*;
use regex_automata::dfa::sparse::DFA;
use writeable::*;

#[cfg(doc)]
extern crate writeable;

define_preferences!(
    /// The preferences for list formatting.
    [Copy]
    ListFormatterPreferences,
    {}
);

/// A formatter that renders sequences of items in an i18n-friendly way. See the
/// [crate-level documentation](crate) for more details.
#[derive(Debug)]
pub struct ListFormatter {
    data: DataPayload<ErasedMarker<ListFormatterPatterns<'static>>>,
    // A parsed special case condition: the one of `data.end`, or else the one of `data.pair`.
    // It can only borrow from static data, other payloads parse theirs on each format.
    condition: Option<DFA<&'static [u8]>>,
    // Whether `condition` is the one of `data.pair`, which is parsed on each format otherwise.
    condition_is_pairs: bool,
}

macro_rules! constructor {
    ($name: ident, $name_buffer: ident, $name_unstable: ident, $marker: ty, $doc: literal) => {
        icu_provider::gen_buffer_data_constructors!(
            (prefs: ListFormatterPreferences, options: ListFormatterOptions) ->  error: DataError,
            #[doc = concat!("Creates a new [`ListFormatter`] that produces a ", $doc, "-type list using compiled data.")]
            ///
            /// See the [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns) for
            /// an explanation of the different types.
            functions: [
                $name,
                $name_buffer,
                $name_unstable,
                Self
            ]
        );

        #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::$name)]
        pub fn $name_unstable(
            provider: &(impl DataProvider<$marker> + ?Sized),
            prefs: ListFormatterPreferences,
            options: ListFormatterOptions,
        ) -> Result<Self, DataError> {
            let length = match options.length.unwrap_or_default() {
                ListLength::Narrow => ListFormatterPatterns::NARROW,
                ListLength::Short => ListFormatterPatterns::SHORT,
                ListLength::Wide => ListFormatterPatterns::WIDE,
            };
            let locale = <$marker>::make_locale(prefs.locale_preferences);
            let data = provider
                .load(DataRequest {
                    id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        length,
                        &locale
                    ),
                    ..Default::default()
                })?
                .payload
                .cast();
            Ok(Self::new(data))
        }
    };
}

impl ListFormatter {
    fn new(data: DataPayload<ErasedMarker<ListFormatterPatterns<'static>>>) -> Self {
        fn condition(
            pattern: &'static ConditionalListJoinerPattern<'static>,
        ) -> Option<&'static SerdeDFA<'static>> {
            Some(&pattern.special_case.as_ref()?.condition)
        }
        let patterns = data.get_static();
        let end = patterns.and_then(|patterns| condition(&patterns.end));
        let pair = patterns.and_then(|patterns| condition(patterns.pair.as_ref()?));
        let parsed = end.or(pair);
        Self {
            data,
            condition: parsed.map(SerdeDFA::deref),
            condition_is_pairs: pair.is_some() && pair == parsed,
        }
    }

    constructor!(
        try_new_and,
        try_new_and_with_buffer_provider,
        try_new_and_unstable,
        ListAndV1,
        "and"
    );
    constructor!(
        try_new_or,
        try_new_or_with_buffer_provider,
        try_new_or_unstable,
        ListOrV1,
        "or"
    );
    constructor!(
        try_new_unit,
        try_new_unit_with_buffer_provider,
        try_new_unit_unstable,
        ListUnitV1,
        "unit"
    );

    /// Returns a [`Writeable`] composed of the input [`Writeable`]s and the language-dependent
    /// formatting.
    ///
    /// The [`Writeable`] is annotated with [`parts::ELEMENT`] for input elements,
    /// and [`parts::LITERAL`] for list literals.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::list::options::*;
    /// use icu::list::{ListFormatter, parts};
    /// # use icu::locale::locale;
    /// # use writeable::*;
    /// let formatteur = ListFormatter::try_new_and(
    ///     locale!("fr").into(),
    ///     ListFormatterOptions::default().with_length(ListLength::Wide),
    /// )
    /// .unwrap();
    /// let pays = ["Italie", "France", "Espagne", "Allemagne"];
    ///
    /// assert_writeable_parts_eq!(
    ///     formatteur.format(pays.iter()),
    ///     "Italie, France, Espagne et Allemagne",
    ///     [
    ///         (0, 6, parts::ELEMENT),
    ///         (6, 8, parts::LITERAL),
    ///         (8, 14, parts::ELEMENT),
    ///         (14, 16, parts::LITERAL),
    ///         (16, 23, parts::ELEMENT),
    ///         (23, 27, parts::LITERAL),
    ///         (27, 36, parts::ELEMENT),
    ///     ]
    /// );
    /// ```
    pub fn format<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a>(
        &'a self,
        values: I,
    ) -> FormattedList<'a, W, I> {
        FormattedList {
            formatter: self,
            values,
        }
    }

    /// Returns a [`String`] composed of the input [`Writeable`]s and the language-dependent
    /// formatting.
    ///
    /// ✨ *Enabled with the `alloc` Cargo feature.*
    #[cfg(feature = "alloc")]
    pub fn format_to_string<W: Writeable, I: Iterator<Item = W> + Clone>(
        &self,
        values: I,
    ) -> String {
        self.format(values).write_to_string().into_owned()
    }
}

/// The [`Part`]s used by [`ListFormatter`].
pub mod parts {
    use writeable::Part;

    /// The [`Part`] used by [`FormattedList`](super::FormattedList) to mark the part of the string that is an element.
    ///
    /// * `category`: `"list"`
    /// * `value`: `"element"`
    pub const ELEMENT: Part = Part {
        category: "list",
        value: "element",
    };

    /// The [`Part`] used by [`FormattedList`](super::FormattedList) to mark the part of the string that is a list literal,
    /// such as ", " or " and ".
    ///
    /// * `category`: `"list"`
    /// * `value`: `"literal"`
    pub const LITERAL: Part = Part {
        category: "list",
        value: "literal",
    };
}

/// The [`Writeable`] implementation that is returned by [`ListFormatter::format`]. See
/// the [`writeable`] crate for how to consume this.
#[derive(Debug)]
pub struct FormattedList<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a> {
    formatter: &'a ListFormatter,
    values: I,
}

impl<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a> Writeable
    for FormattedList<'a, W, I>
{
    fn write_to_parts<V: PartsWrite + ?Sized>(&self, sink: &mut V) -> fmt::Result {
        macro_rules! literal {
            ($lit:ident) => {
                sink.with_part(parts::LITERAL, |l| l.write_str($lit))
            };
        }
        macro_rules! value {
            ($val:expr) => {
                sink.with_part(parts::ELEMENT, |e| $val.write_to_parts(e))
            };
        }

        let patterns = self.formatter.data.get();

        let mut values = self.values.clone();

        if let Some(first) = values.next() {
            if let Some(second) = values.next() {
                if let Some(third) = values.next() {
                    // Start(values[0], middle(..., middle(values[n-3], End(values[n-2], values[n-1]))...)) =
                    // start_before + values[0] + start_between + (values[1..n-3] + middle_between)* +
                    // values[n-2] + end_between + values[n-1] + end_after

                    let (start_before, start_between, _) = patterns.start.parts();

                    literal!(start_before)?;
                    value!(first)?;
                    literal!(start_between)?;
                    value!(second)?;

                    let mut next = third;

                    for next_next in values {
                        let between = &*patterns.middle;
                        literal!(between)?;
                        value!(next)?;
                        next = next_next;
                    }

                    let condition = self.formatter.condition.as_ref();
                    let (_, end_between, end_after) = patterns.end.parts(condition, &next);
                    literal!(end_between)?;
                    value!(next)?;
                    literal!(end_after)
                } else {
                    // Pair(values[0], values[1]) = pair_before + values[0] + pair_between + values[1] + pair_after
                    let condition = self.formatter.condition.as_ref();
                    let (before, between, after) = match &patterns.pair {
                        Some(pair) => pair.parts(
                            condition.filter(|_| self.formatter.condition_is_pairs),
                            &second,
                        ),
                        None => patterns.end.parts(condition, &second),
                    };
                    literal!(before)?;
                    value!(first)?;
                    literal!(between)?;
                    value!(second)?;
                    literal!(after)
                }
            } else {
                value!(first)
            }
        } else {
            Ok(())
        }
    }

    fn writeable_length_hint(&self) -> LengthHint {
        let mut count = 0;
        let item_length = self
            .values
            .clone()
            .map(|w| {
                count += 1;
                w.writeable_length_hint()
            })
            .sum::<LengthHint>();
        item_length + self.formatter.data.get().length_hint(count)
    }
}

impl<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a> fmt::Display
    for FormattedList<'a, W, I>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

#[cfg(all(test, feature = "datagen"))]
mod tests {
    use super::*;
    use writeable::{assert_writeable_eq, assert_writeable_parts_eq};

    fn formatter(patterns: ListFormatterPatterns<'static>) -> ListFormatter {
        ListFormatter::new(DataPayload::from_owned(patterns))
    }

    #[test]
    fn test_slices() {
        let formatter = formatter(crate::patterns::test::test_patterns_general());
        let values = ["one", "two", "three", "four", "five"];

        assert_writeable_eq!(formatter.format(values[0..0].iter()), "");
        assert_writeable_eq!(formatter.format(values[0..1].iter()), "one");
        assert_writeable_eq!(formatter.format(values[0..2].iter()), "$one;two+");
        assert_writeable_eq!(formatter.format(values[0..3].iter()), "@one:two.three!");
        assert_writeable_eq!(
            formatter.format(values[0..4].iter()),
            "@one:two,three.four!"
        );

        assert_writeable_parts_eq!(
            formatter.format(values.iter()),
            "@one:two,three,four.five!",
            [
                (0, 1, parts::LITERAL),
                (1, 4, parts::ELEMENT),
                (4, 5, parts::LITERAL),
                (5, 8, parts::ELEMENT),
                (8, 9, parts::LITERAL),
                (9, 14, parts::ELEMENT),
                (14, 15, parts::LITERAL),
                (15, 19, parts::ELEMENT),
                (19, 20, parts::LITERAL),
                (20, 24, parts::ELEMENT),
                (24, 25, parts::LITERAL)
            ]
        );
    }

    #[test]
    fn test_into_iterator() {
        let formatter = formatter(crate::patterns::test::test_patterns_general());

        let mut vecdeque = std::collections::vec_deque::VecDeque::<u8>::new();
        vecdeque.push_back(10);
        vecdeque.push_front(48);

        assert_writeable_parts_eq!(
            formatter.format(vecdeque.iter()),
            "$48;10+",
            [
                (0, 1, parts::LITERAL),
                (1, 3, parts::ELEMENT),
                (3, 4, parts::LITERAL),
                (4, 6, parts::ELEMENT),
                (6, 7, parts::LITERAL),
            ]
        );
    }

    #[test]
    fn test_iterator() {
        let formatter = formatter(crate::patterns::test::test_patterns_general());

        assert_writeable_parts_eq!(
            formatter.format(core::iter::repeat_n(5, 2)),
            "$5;5+",
            [
                (0, 1, parts::LITERAL),
                (1, 2, parts::ELEMENT),
                (2, 3, parts::LITERAL),
                (3, 4, parts::ELEMENT),
                (4, 5, parts::LITERAL),
            ]
        );
    }

    #[test]
    fn test_conditional() {
        let formatter = formatter(crate::patterns::test::test_patterns_conditional());

        assert_writeable_eq!(formatter.format(["beta", "alpha"].iter()), "beta :o alpha");
    }

    #[test]
    fn test_parsed_condition() {
        fn special_case(condition: &'static str, pattern: &str) -> SpecialCasePattern<'static> {
            SpecialCasePattern {
                condition: SerdeDFA::new(condition.into()).unwrap(),
                pattern: ListJoinerPattern::try_from_str(pattern, true, true).unwrap(),
            }
        }

        // Formats with the conditions parsed by the constructor, which needs static data,
        // and with the conditions parsed on each format.
        fn check(
            patterns: &ListFormatterPatterns<'static>,
            parsed_for_pair: bool,
            cases: &[(&[&str], &str)],
        ) {
            let parsed = ListFormatter::new(DataPayload::from_static_ref(Box::leak(Box::new(
                patterns.clone(),
            ))));
            assert!(parsed.condition.is_some());
            assert_eq!(parsed.condition_is_pairs, parsed_for_pair);

            let unparsed = formatter(patterns.clone());
            assert!(unparsed.condition.is_none());
            assert!(!unparsed.condition_is_pairs);

            for formatter in [&parsed, &unparsed] {
                for (values, expected) in cases {
                    assert_writeable_eq!(formatter.format(values.iter()), *expected);
                }
            }
        }

        let end_only = crate::patterns::test::test_patterns_conditional();
        let pair = ConditionalListJoinerPattern {
            default: ListJoinerPattern::try_from_str("{0}; {1}", true, true).unwrap(),
            special_case: None,
        };

        // No pair pattern, so two items use the end pattern and its condition
        check(
            &end_only,
            false,
            &[
                (&["beta", "alpha"], "beta :o alpha"),
                (&["alpha", "beta"], "alpha. beta"),
                (&["x", "y", "alpha"], "x: y :o alpha"),
                (&["x", "y", "beta"], "x: y. beta"),
                (&["w", "x", "y", "alpha"], "w: x, y :o alpha"),
            ],
        );

        // A pair pattern without a condition
        let mut patterns = end_only.clone();
        patterns.pair = Some(pair.clone());
        check(
            &patterns,
            false,
            &[
                (&["x", "alpha"], "x; alpha"),
                (&["x", "y", "alpha"], "x: y :o alpha"),
            ],
        );

        // The same condition on both patterns
        patterns.pair = Some(ConditionalListJoinerPattern {
            special_case: Some(special_case("^a", "{0} :a {1}")),
            ..pair.clone()
        });
        check(
            &patterns,
            true,
            &[
                (&["x", "alpha"], "x :a alpha"),
                (&["x", "beta"], "x; beta"),
                (&["x", "y", "alpha"], "x: y :o alpha"),
                (&["x", "y", "beta"], "x: y. beta"),
            ],
        );

        // Different conditions, the pair pattern must not use the end pattern's
        patterns.pair = Some(ConditionalListJoinerPattern {
            special_case: Some(special_case("^b", "{0} :b {1}")),
            ..pair.clone()
        });
        check(
            &patterns,
            false,
            &[
                (&["x", "alpha"], "x; alpha"),
                (&["x", "beta"], "x :b beta"),
                (&["x", "y", "alpha"], "x: y :o alpha"),
                (&["x", "y", "beta"], "x: y. beta"),
            ],
        );

        // A condition on the pair pattern only
        patterns.end.special_case = None;
        check(
            &patterns,
            true,
            &[
                (&["x", "alpha"], "x; alpha"),
                (&["x", "beta"], "x :b beta"),
                (&["x", "y", "alpha"], "x: y. alpha"),
                (&["x", "y", "beta"], "x: y. beta"),
            ],
        );

        // Compiled data is static
        let es =
            ListFormatter::try_new_or(icu_locale_core::locale!("es").into(), Default::default())
                .unwrap();
        assert!(es.condition.is_some());
        let en =
            ListFormatter::try_new_or(icu_locale_core::locale!("en").into(), Default::default())
                .unwrap();
        assert!(en.condition.is_none());
    }

    macro_rules! test {
        ($locale:literal, $type:ident, $(($input:expr, $output:literal),)+) => {
            let f = ListFormatter::$type(
                icu_locale_core::locale!($locale).into(),
                Default::default(),
            ).unwrap();
            $(
                assert_writeable_eq!(f.format($input.iter()), $output);
            )+
        };
    }

    #[test]
    fn test_basic() {
        test!("fr", try_new_or, (["A", "B"], "A ou B"),);
    }

    #[test]
    fn test_spanish() {
        test!(
            "es",
            try_new_and,
            (["x", "Mallorca"], "x y Mallorca"),
            (["x", "Ibiza"], "x e Ibiza"),
            (["x", "Hidalgo"], "x e Hidalgo"),
            (["x", "Hierva"], "x y Hierva"),
        );

        test!(
            "es",
            try_new_or,
            (["x", "Ibiza"], "x o Ibiza"),
            (["x", "Okinawa"], "x u Okinawa"),
            (["x", "8 más"], "x u 8 más"),
            (["x", "8"], "x u 8"),
            (["x", "87 más"], "x u 87 más"),
            (["x", "87"], "x u 87"),
            (["x", "11 más"], "x u 11 más"),
            (["x", "11"], "x u 11"),
            (["x", "110 más"], "x o 110 más"),
            (["x", "110"], "x o 110"),
            (["x", "11.000 más"], "x u 11.000 más"),
            (["x", "11.000"], "x u 11.000"),
            (["x", "11.000,92 más"], "x u 11.000,92 más"),
            (["x", "11.000,92"], "x u 11.000,92"),
        );

        test!("es-AR", try_new_and, (["x", "Ibiza"], "x e Ibiza"),);
    }

    #[test]
    fn test_hebrew() {
        test!(
            "he",
            try_new_and,
            (["x", "יפו"], "x ויפו"),
            (["x", "Ibiza"], "x ו‑Ibiza"),
        );
    }
}
