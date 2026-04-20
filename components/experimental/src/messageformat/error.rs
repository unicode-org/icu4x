// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Error types for `MessageFormat` 2.
//!
//! Mirrors the four spec error categories: Syntax ([`ParseError::Syntax`]),
//! Data Model ([`ValidationError`]), Resolution ([`FormatError`]), and
//! Message Function ([`FunctionError`]).

use alloc::boxed::Box;

use displaydoc::Display;

/// Errors produced by the `MessageFormat` 2 parser.
#[non_exhaustive]
#[derive(Debug, Clone, Display, PartialEq, Eq)]
#[ignore_extra_doc_attributes]
pub enum ParseError {
    /// Syntax error at byte offset {offset}: {message}
    Syntax {
        /// Byte offset of the error within the input.
        offset: usize,
        /// Short human-readable description.
        message: &'static str,
    },
    /// Data-model error detected during parsing: {0}
    ///
    /// The parser surfaces the subset of data-model errors that are only
    /// observable in source text (e.g. duplicate option names, which the
    /// JSON interchange form cannot represent). Structural checks that
    /// work on the AST alone live in [`ValidationError`] and are emitted
    /// by the post-parse validator.
    DataModel(ValidationError),
}

impl From<ValidationError> for ParseError {
    fn from(e: ValidationError) -> Self {
        ParseError::DataModel(e)
    }
}

/// Data-model validation errors.
///
/// These are structural invariants required by the `MessageFormat` 2
/// specification that cannot be expressed in the grammar alone. See
/// `spec/errors.md` in the working-group repository.
#[non_exhaustive]
#[derive(Debug, Clone, Display, PartialEq, Eq)]
#[ignore_extra_doc_attributes]
pub enum ValidationError {
    /// Missing fallback variant
    ///
    /// A `.match` message must contain a variant whose keys are all `*`.
    MissingFallbackVariant,
    /// Variant key count {actual} does not match selector count {expected}
    VariantKeyMismatch {
        /// Number of selectors the matcher declares.
        expected: usize,
        /// Number of keys the offending variant declared.
        actual: usize,
    },
    /// Duplicate declaration of `${name}`
    DuplicateDeclaration {
        /// The variable name declared twice (no `$` prefix).
        name: Box<str>,
    },
    /// Duplicate variant
    ///
    /// Two variants in a `.match` message share the same key list.
    DuplicateVariant,
    /// Duplicate option name `{name}`
    DuplicateOptionName {
        /// The option identifier that appears more than once within a single
        /// function or markup element.
        name: Box<str>,
    },
    /// Missing selector annotation for `${name}`
    ///
    /// Selector variables must be declared (directly or transitively) with a
    /// function annotation.
    MissingSelectorAnnotation {
        /// The unannotated selector variable name.
        name: Box<str>,
    },
    /// Empty expression
    ///
    /// The JSON schema requires an `Expression` to be one of
    /// `LiteralExpression` / `VariableExpression` / `FunctionExpression` —
    /// i.e. at least one of `arg` or `function` must be present. The parser
    /// rejects `{}` at the syntax level; this variant covers AST values
    /// constructed programmatically.
    EmptyExpression,
}

/// Errors produced while building a [`super::MessageFormatter`].
#[non_exhaustive]
#[derive(Debug, Clone, Display, PartialEq, Eq)]
#[ignore_extra_doc_attributes]
pub enum BuildError {
    /// No message source or pre-parsed message supplied to the builder
    NoMessage,
    /// No locale supplied to the builder
    ///
    /// The builder requires an explicit locale so that locale-sensitive
    /// functions (`:number`, `:integer`, `:percent`, `:currency`, `:date` /
    /// `:time` / `:datetime`) cannot silently produce root-locale output.
    /// Call [`super::MessageFormatterBuilder::locale`] with a concrete
    /// [`icu_locale_core::Locale`], or
    /// [`super::MessageFormatterBuilder::locale_undetermined`] to opt into
    /// `und` explicitly.
    MissingLocale,
    /// Parse error: {0}
    Parse(ParseError),
    /// Validation error: {0}
    Validation(ValidationError),
}

impl From<ParseError> for BuildError {
    fn from(e: ParseError) -> Self {
        // Parser-detected data-model errors get unwrapped into a
        // BuildError::Validation so that downstream callers see a uniform
        // classification.
        match e {
            ParseError::DataModel(v) => BuildError::Validation(v),
            other => BuildError::Parse(other),
        }
    }
}

impl From<ValidationError> for BuildError {
    fn from(e: ValidationError) -> Self {
        BuildError::Validation(e)
    }
}

/// Non-fatal errors emitted during formatting.
///
/// Per the spec, resolution and message-function errors never abort a format —
/// they are recorded alongside the best-effort output.
#[non_exhaustive]
#[derive(Debug, Clone, Display, PartialEq, Eq)]
#[ignore_extra_doc_attributes]
pub enum FormatError {
    /// Unresolved variable: `${name}`
    UnresolvedVariable {
        /// The unbound variable name (without the leading `$`).
        name: Box<str>,
    },
    /// Unknown function: `:{name}`
    UnknownFunction {
        /// The function identifier (without the leading `:`).
        name: Box<str>,
    },
    /// Bad selector for `${name}`
    BadSelector {
        /// The selector's variable name.
        name: Box<str>,
    },
    /// Function `:{function}` failed: {error}
    FunctionError {
        /// The function identifier.
        function: Box<str>,
        /// The underlying handler error.
        error: FunctionError,
    },
}

/// Errors produced by a [`super::FunctionHandler`] during formatting.
///
/// These are wrapped in [`FormatError::FunctionError`] and never abort a
/// format; they trigger fallback substitution per the spec.
#[non_exhaustive]
#[derive(Debug, Clone, Display, PartialEq, Eq)]
#[ignore_extra_doc_attributes]
pub enum FunctionError {
    /// Bad operand
    BadOperand,
    /// Bad option `{name}`
    BadOption {
        /// The option identifier.
        name: Box<str>,
    },
    /// Bad variant key `{key}`
    ///
    /// Emitted by a selector when a literal variant key is neither valid
    /// input for this selector's domain (e.g. a non-numeric key for
    /// `:number`) nor equal to the operand. Per spec the variant is
    /// excluded from matching and this error is recorded.
    BadVariantKey {
        /// The offending variant key as written in the message source.
        key: Box<str>,
    },
    /// Unsupported operation
    UnsupportedOperation,
    /// Implementation-defined Message Function error `{kind}`: {message}
    ///
    /// Corresponds to the spec's allowance for implementation-defined
    /// Message Function Error categories (see `spec/errors.md` §
    /// "Message Function Errors"). Third-party [`super::FunctionHandler`]s
    /// use this to surface a named category that is not one of the four
    /// normative variants above. `kind` carries the category name
    /// (typically kebab-case, e.g. `"bad-timezone"`); `message` is an
    /// optional human-readable detail — use an empty string if none.
    Custom {
        /// A short identifier for the error category, chosen by the handler.
        kind: Box<str>,
        /// Free-form detail, or empty string when none.
        message: Box<str>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn parse_error_syntax_display() {
        let e = ParseError::Syntax {
            offset: 7,
            message: "unterminated expression",
        };
        assert_eq!(
            e.to_string(),
            "Syntax error at byte offset 7: unterminated expression"
        );
    }

    #[test]
    fn parse_error_datamodel_wraps_validation() {
        let v = ValidationError::MissingFallbackVariant;
        let p = ParseError::from(v.clone());
        assert_eq!(p, ParseError::DataModel(v));
        assert!(p.to_string().starts_with("Data-model error detected"));
    }

    #[test]
    fn validation_error_display_variants() {
        assert_eq!(
            ValidationError::MissingFallbackVariant.to_string(),
            "Missing fallback variant"
        );
        assert_eq!(
            ValidationError::VariantKeyMismatch {
                expected: 2,
                actual: 1,
            }
            .to_string(),
            "Variant key count 1 does not match selector count 2"
        );
        assert_eq!(
            ValidationError::DuplicateDeclaration {
                name: Box::from("x"),
            }
            .to_string(),
            "Duplicate declaration of `$x`"
        );
        assert_eq!(
            ValidationError::DuplicateVariant.to_string(),
            "Duplicate variant"
        );
        assert_eq!(
            ValidationError::DuplicateOptionName {
                name: Box::from("kind"),
            }
            .to_string(),
            "Duplicate option name `kind`"
        );
        assert_eq!(
            ValidationError::MissingSelectorAnnotation {
                name: Box::from("n"),
            }
            .to_string(),
            "Missing selector annotation for `$n`"
        );
        assert_eq!(
            ValidationError::EmptyExpression.to_string(),
            "Empty expression"
        );
    }

    #[test]
    fn build_error_display_and_conversions() {
        assert_eq!(
            BuildError::NoMessage.to_string(),
            "No message source or pre-parsed message supplied to the builder"
        );
        assert_eq!(
            BuildError::MissingLocale.to_string(),
            "No locale supplied to the builder"
        );
        // ValidationError flows through both From impls and normalizes to
        // BuildError::Validation — never BuildError::Parse — even when
        // routed via ParseError::DataModel.
        let datamodel = ParseError::DataModel(ValidationError::DuplicateVariant);
        assert!(matches!(
            BuildError::from(datamodel),
            BuildError::Validation(ValidationError::DuplicateVariant)
        ));
        let syntax = ParseError::Syntax {
            offset: 0,
            message: "x",
        };
        assert!(matches!(BuildError::from(syntax), BuildError::Parse(_)));
        assert!(matches!(
            BuildError::from(ValidationError::DuplicateVariant),
            BuildError::Validation(_)
        ));
    }

    #[test]
    fn format_error_display_variants() {
        assert_eq!(
            FormatError::UnresolvedVariable {
                name: Box::from("u"),
            }
            .to_string(),
            "Unresolved variable: `$u`"
        );
        assert_eq!(
            FormatError::UnknownFunction {
                name: Box::from("foo"),
            }
            .to_string(),
            "Unknown function: `:foo`"
        );
        assert_eq!(
            FormatError::BadSelector {
                name: Box::from("s"),
            }
            .to_string(),
            "Bad selector for `$s`"
        );
        let nested = FormatError::FunctionError {
            function: Box::from("number"),
            error: FunctionError::BadOperand,
        };
        assert!(nested.to_string().contains("`:number`"));
        assert!(nested.to_string().contains("Bad operand"));
    }

    #[test]
    fn function_error_display_variants() {
        assert_eq!(FunctionError::BadOperand.to_string(), "Bad operand");
        assert_eq!(
            FunctionError::BadOption {
                name: Box::from("style"),
            }
            .to_string(),
            "Bad option `style`"
        );
        assert_eq!(
            FunctionError::BadVariantKey {
                key: Box::from("3.5"),
            }
            .to_string(),
            "Bad variant key `3.5`"
        );
        assert_eq!(
            FunctionError::UnsupportedOperation.to_string(),
            "Unsupported operation"
        );
        assert_eq!(
            FunctionError::Custom {
                kind: Box::from("bad-timezone"),
                message: Box::from("unknown zone `Foo/Bar`"),
            }
            .to_string(),
            "Implementation-defined Message Function error `bad-timezone`: unknown zone `Foo/Bar`"
        );
        // Empty message is allowed.
        assert_eq!(
            FunctionError::Custom {
                kind: Box::from("rate-limited"),
                message: Box::from(""),
            }
            .to_string(),
            "Implementation-defined Message Function error `rate-limited`: "
        );
    }

    #[test]
    fn custom_function_error_is_non_exhaustive_catchall() {
        // The catch-all pattern on FunctionError should capture Custom without
        // naming it explicitly — this is the pattern used in conformance.rs.
        fn categorize(e: &FunctionError) -> &'static str {
            match e {
                FunctionError::BadOperand => "bad-operand",
                FunctionError::BadOption { .. } => "bad-option",
                FunctionError::BadVariantKey { .. } => "bad-variant-key",
                FunctionError::UnsupportedOperation => "unsupported-operation",
                _ => "unknown-function-error",
            }
        }
        assert_eq!(
            categorize(&FunctionError::Custom {
                kind: Box::from("x"),
                message: Box::from(""),
            }),
            "unknown-function-error"
        );
    }

    #[test]
    fn errors_are_clone_and_eq() {
        let a = FormatError::UnresolvedVariable {
            name: Box::from("x"),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
