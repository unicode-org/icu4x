// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Structured output for [`super::MessageFormatter::format_to_parts`].
//!
//! Each element of the returned `Vec<FormattedPart>` corresponds to a run
//! of literal text, an expression's formatted value, a markup element, or
//! a bidi-isolation character inserted by the formatter.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;

use super::ast::MarkupKind;
use super::bidi::Direction;

/// One element of structured formatter output.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormattedPart {
    /// A literal run of pattern text.
    Text {
        /// The text itself.
        value: String,
    },
    /// The result of formatting an expression.
    Expression {
        /// Part kind (e.g. `"string"`, `"number"`, `"fallback"`).
        kind: Box<str>,
        /// The formatted text of the expression.
        value: String,
        /// The expression's `u:id` tag, if any.
        id: Option<Box<str>>,
        /// The resolved directionality of this expression.
        direction: Option<Direction>,
    },
    /// A markup element from the pattern.
    Markup {
        /// Whether this is an open / close / self-closing marker.
        kind: MarkupKind,
        /// The markup identifier (possibly namespaced).
        name: Box<str>,
        /// Resolved option values (variables replaced by their resolved
        /// text). Empty when the markup element declared no options.
        options: BTreeMap<Box<str>, String>,
        /// The markup's `u:id` option, if any.
        id: Option<Box<str>>,
        /// The markup's resolved direction (from a `u:dir` option), if any.
        direction: Option<Direction>,
    },
    /// A bidi isolation character inserted by the formatter.
    ///
    /// Emitted when `bidi_isolation` is enabled and the formatter inserts
    /// isolate controls around an expression value.
    BidiIsolation {
        /// The isolate character: `U+2066` LRI, `U+2067` RLI, `U+2068` FSI,
        /// or `U+2069` PDI.
        value: char,
    },
}

impl FormattedPart {
    /// Convenience: construct a [`FormattedPart::Text`].
    pub fn text(s: impl Into<String>) -> Self {
        FormattedPart::Text { value: s.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn text_constructor_accepts_string_and_str() {
        let from_str = FormattedPart::text("hi");
        let from_string = FormattedPart::text("hi".to_string());
        assert_eq!(from_str, from_string);
        match from_str {
            FormattedPart::Text { value } => assert_eq!(value, "hi"),
            _ => panic!("expected Text"),
        }
    }

    #[test]
    fn parts_equality_across_variants_is_strict() {
        let text = FormattedPart::Text { value: "x".into() };
        let expr = FormattedPart::Expression {
            kind: Box::from("string"),
            value: "x".into(),
            id: None,
            direction: None,
        };
        assert_ne!(text, expr);
    }

    #[test]
    fn expression_preserves_all_metadata() {
        let p = FormattedPart::Expression {
            kind: Box::from("number"),
            value: "42".into(),
            id: Some(Box::from("count-id")),
            direction: Some(Direction::Ltr),
        };
        match &p {
            FormattedPart::Expression {
                kind,
                value,
                id,
                direction,
            } => {
                assert_eq!(kind.as_ref(), "number");
                assert_eq!(value, "42");
                assert_eq!(id.as_deref(), Some("count-id"));
                assert_eq!(*direction, Some(Direction::Ltr));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn markup_carries_options_and_id() {
        let mut opts: BTreeMap<Box<str>, String> = BTreeMap::new();
        opts.insert(Box::from("color"), "red".into());
        opts.insert(Box::from("weight"), "bold".into());
        let p = FormattedPart::Markup {
            kind: MarkupKind::Open,
            name: Box::from("b"),
            options: opts,
            id: Some(Box::from("m1")),
            direction: Some(Direction::Rtl),
        };
        match p {
            FormattedPart::Markup {
                kind,
                name,
                options,
                id,
                direction,
            } => {
                assert_eq!(kind, MarkupKind::Open);
                assert_eq!(name.as_ref(), "b");
                assert_eq!(options.len(), 2);
                assert_eq!(options.get(&Box::from("color")).unwrap(), "red");
                assert_eq!(id.as_deref(), Some("m1"));
                assert_eq!(direction, Some(Direction::Rtl));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn bidi_isolation_carries_char() {
        // Spec-valid isolate characters.
        for c in ['\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}'] {
            let p = FormattedPart::BidiIsolation { value: c };
            match p {
                FormattedPart::BidiIsolation { value } => assert_eq!(value, c),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn part_is_clone_and_eq() {
        let p = FormattedPart::Text { value: "a".into() };
        let clone = p.clone();
        assert_eq!(p, clone);
    }
}
