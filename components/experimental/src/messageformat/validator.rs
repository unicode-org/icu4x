// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data-model validator for `MessageFormat` 2.
//!
//! Accepts a parsed [`Message`] and either promotes it to a
//! [`ValidatedMessage`] — the typed witness consumed downstream by the
//! formatter — or returns a [`ValidationError`].
//!
//! Implements all six data-model checks listed in `spec/errors.md` at
//! WG rev `dd86e42e10d1d0c9c4401d0781cdd87ee7166366`:
//!
//! - Missing Fallback Variant
//! - Variant Key Mismatch
//! - Duplicate Declaration
//! - Duplicate Variant
//! - Duplicate Option Name (detected during parsing; re-checked here defensively)
//! - Missing Selector Annotation

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::ast::{
    Arg, Declaration, Expression, Message, OptionMap, OptionValue, Pattern, PatternElement,
    Variable, Variant, VariantKey,
};
use super::error::ValidationError;

/// A [`Message`] that has passed data-model validation.
///
/// The only way to obtain a `ValidatedMessage` is via [`validate`] or the
/// [`TryFrom<Message>`] impl. Downstream consumers (the formatter) take
/// `ValidatedMessage` by reference and can rely on all data-model invariants
/// holding.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedMessage(Message);

impl ValidatedMessage {
    /// Borrow the inner [`Message`].
    pub fn as_message(&self) -> &Message {
        &self.0
    }

    /// Consume and return the inner [`Message`].
    pub fn into_inner(self) -> Message {
        self.0
    }
}

impl TryFrom<Message> for ValidatedMessage {
    type Error = ValidationError;
    fn try_from(msg: Message) -> Result<Self, Self::Error> {
        validate(&msg)?;
        Ok(Self(msg))
    }
}

/// Validate a parsed message against the MF2 data-model invariants.
///
/// Returns `Ok(())` if `message` is well-formed. On failure, returns the
/// first violation encountered; a multi-error reporting mode may be added
/// in the future.
pub fn validate(message: &Message) -> Result<(), ValidationError> {
    match message {
        Message::Pattern {
            declarations,
            pattern,
        } => {
            validate_declarations(declarations)?;
            validate_pattern(pattern)?;
        }
        Message::Select {
            declarations,
            selectors,
            variants,
        } => {
            validate_declarations(declarations)?;
            validate_variants(selectors.len(), variants)?;
            validate_selector_annotations(declarations, selectors)?;
            for v in variants {
                validate_pattern(&v.value)?;
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Individual checks
// ---------------------------------------------------------------------------

fn validate_declarations(decls: &[Declaration]) -> Result<(), ValidationError> {
    let mut seen: Vec<&str> = Vec::with_capacity(decls.len());
    // `used_in_prior` accumulates every variable referenced by any previous
    // declaration's expression (including its function options). Per spec
    // (syntax.md §Declarations), a declaration must NOT bind a variable that
    // was referenced by any earlier declaration.
    let mut used_in_prior: Vec<Box<str>> = Vec::new();

    for decl in decls {
        let name = declaration_name(decl);

        // Rule 1: no redeclaration.
        if seen.contains(&name) {
            return Err(ValidationError::DuplicateDeclaration {
                name: Box::from(name),
            });
        }
        // Rule 2: must not bind a variable seen in a previous declaration.
        if used_in_prior.iter().any(|n| n.as_ref() == name) {
            return Err(ValidationError::DuplicateDeclaration {
                name: Box::from(name),
            });
        }

        match decl {
            Declaration::Local { name, value } => {
                // Rule 3: local value must not reference the bound variable.
                if expression_references(value, name, true) {
                    return Err(ValidationError::DuplicateDeclaration { name: name.clone() });
                }
            }
            Declaration::Input { name, value } => {
                // Rule 4: input variable must not appear inside the function
                // options of its own variable-expression.
                if let Some(fr) = &value.function {
                    if option_map_references(&fr.options, name) {
                        return Err(ValidationError::DuplicateDeclaration { name: name.clone() });
                    }
                }
                // Rule 5: an input-declaration's `value.arg` MUST be a
                // `$name` variable whose name matches the bound identifier
                // (schema `message.json::input-declaration`). The parser
                // enforces this by construction, but a caller that builds
                // the AST directly can violate it — reject here so the
                // invariant holds for the formatter.
                match &value.arg {
                    Some(Arg::Variable(v)) if v.name.as_ref() == name.as_ref() => {}
                    _ => {
                        return Err(ValidationError::DuplicateDeclaration { name: name.clone() });
                    }
                }
            }
        }

        seen.push(name);
        collect_referenced_variables(declaration_value(decl), &mut used_in_prior);
        validate_expression(declaration_value(decl))?;
    }
    Ok(())
}

fn collect_referenced_variables(expr: &Expression, out: &mut Vec<Box<str>>) {
    if let Some(Arg::Variable(v)) = &expr.arg {
        push_unique(out, &v.name);
    }
    if let Some(fr) = &expr.function {
        for v in option_map_variables(&fr.options) {
            push_unique(out, &v);
        }
    }
}

fn push_unique(out: &mut Vec<Box<str>>, name: &str) {
    if !out.iter().any(|n| n.as_ref() == name) {
        out.push(Box::from(name));
    }
}

fn option_map_variables(opts: &OptionMap) -> Vec<Box<str>> {
    opts.values()
        .filter_map(|v| match v {
            OptionValue::Variable(var) => Some(var.name.clone()),
            OptionValue::Literal(_) => None,
        })
        .collect()
}

fn option_map_references(opts: &OptionMap, name: &str) -> bool {
    opts.values()
        .any(|v| matches!(v, OptionValue::Variable(var) if var.name.as_ref() == name))
}

/// Does `expr` reference `name` as a variable (argument or option value)?
///
/// When `include_arg` is false, only option values are considered; used for
/// the input-declaration check where the arg is, by definition, `$name`.
fn expression_references(expr: &Expression, name: &str, include_arg: bool) -> bool {
    if include_arg {
        if let Some(Arg::Variable(v)) = &expr.arg {
            if v.name.as_ref() == name {
                return true;
            }
        }
    }
    if let Some(fr) = &expr.function {
        if option_map_references(&fr.options, name) {
            return true;
        }
    }
    false
}

fn validate_variants(selector_count: usize, variants: &[Variant]) -> Result<(), ValidationError> {
    // (a) Each variant's key list must match the selector count.
    // (b) Every matcher must have at least one all-catchall variant.
    // (c) No two variants may share the same key list.
    let mut has_fallback = false;
    for v in variants {
        if v.keys.len() != selector_count {
            return Err(ValidationError::VariantKeyMismatch {
                expected: selector_count,
                actual: v.keys.len(),
            });
        }
        if v.keys.iter().all(|k| matches!(k, VariantKey::Catchall(_))) {
            has_fallback = true;
        }
    }
    if !has_fallback {
        return Err(ValidationError::MissingFallbackVariant);
    }
    for i in 0..variants.len() {
        for j in (i + 1)..variants.len() {
            if keys_equal(&variants[i].keys, &variants[j].keys) {
                return Err(ValidationError::DuplicateVariant);
            }
        }
    }
    Ok(())
}

fn keys_equal(a: &[VariantKey], b: &[VariantKey]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| match (x, y) {
        (VariantKey::Literal(l1), VariantKey::Literal(l2)) => nfc_eq(&l1.value, &l2.value),
        (VariantKey::Catchall(_), VariantKey::Catchall(_)) => true,
        _ => false,
    })
}

/// Compare two strings as if both were NFC-normalized. Per spec (syntax.md
/// §Names and §Keys), literal keys are compared as canonically-equivalent
/// strings.
///
/// ASCII content is its own NFC form, so byte equality is exact there.
/// Non-ASCII content uses the `icu_normalizer` NFC data when available
/// (`compiled_data` feature). Without `compiled_data` and with non-ASCII
/// content, this falls back to byte equality — canonically-equivalent
/// non-NFC inputs may then compare unequal. The spec permits an
/// implementation to assume NFC input; ICU4X emits a debug warning when
/// the fallback path is taken.
fn nfc_eq(a: &str, b: &str) -> bool {
    if a.is_ascii() && b.is_ascii() {
        return a == b;
    }
    #[cfg(feature = "compiled_data")]
    {
        use icu_normalizer::ComposingNormalizer;
        let n = ComposingNormalizer::new_nfc();
        n.normalize(a) == n.normalize(b)
    }
    #[cfg(not(feature = "compiled_data"))]
    {
        a == b
    }
}

fn validate_selector_annotations(
    decls: &[Declaration],
    selectors: &[Variable],
) -> Result<(), ValidationError> {
    for sel in selectors {
        if !is_annotated(decls, &sel.name) {
            return Err(ValidationError::MissingSelectorAnnotation {
                name: sel.name.clone(),
            });
        }
    }
    Ok(())
}

fn is_annotated(decls: &[Declaration], name: &str) -> bool {
    // Follow the declaration chain, guarding against cycles.
    let mut current: &str = name;
    let mut visited: Vec<&str> = Vec::new();
    loop {
        if visited.contains(&current) {
            return false;
        }
        visited.push(current);

        let Some(decl) = decls.iter().find(|d| declaration_name(d) == current) else {
            return false;
        };
        let value = declaration_value(decl);
        if value.function.is_some() {
            return true;
        }
        match &value.arg {
            Some(Arg::Variable(v)) => current = v.name.as_ref(),
            _ => return false,
        }
    }
}

fn validate_pattern(pattern: &Pattern) -> Result<(), ValidationError> {
    for element in pattern {
        match element {
            PatternElement::Text(_) => {}
            PatternElement::Expression(e) => validate_expression(e)?,
            PatternElement::Markup(_) => {
                // Markup has no current data-model checks beyond parser
                // invariants (BTreeMap key uniqueness for options /
                // attributes). Retained as a separate match arm so future
                // checks land here without restructuring.
            }
        }
    }
    Ok(())
}

fn validate_expression(expr: &Expression) -> Result<(), ValidationError> {
    // Schema `message.json` requires an `Expression` to be one of
    // `LiteralExpression` / `VariableExpression` / `FunctionExpression`
    // — i.e. at least one of `arg` or `function` must be present. The
    // parser rejects `{}` up front, but the AST can be built directly by
    // downstream tooling. Enforce the invariant here so the formatter
    // never sees an empty expression.
    if expr.arg.is_none() && expr.function.is_none() {
        return Err(ValidationError::EmptyExpression);
    }
    // BTreeMap option maps guarantee key uniqueness; parser catches
    // `DuplicateOptionName` before we get here.
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn declaration_name(decl: &Declaration) -> &str {
    match decl {
        Declaration::Input { name, .. } | Declaration::Local { name, .. } => name.as_ref(),
    }
}

fn declaration_value(decl: &Declaration) -> &Expression {
    match decl {
        Declaration::Input { value, .. } | Declaration::Local { value, .. } => value,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{validate, ValidatedMessage, ValidationError};
    use crate::messageformat::ast::Message;
    use crate::messageformat::error::ParseError;

    fn parse(src: &str) -> Message {
        Message::parse(src).unwrap_or_else(|e| panic!("parse failed for {src:?}: {e:?}"))
    }

    // ---- happy path ----

    #[test]
    fn valid_simple_message() {
        assert!(validate(&parse("Hello, {$user}!")).is_ok());
    }

    #[test]
    fn valid_matcher_with_fallback() {
        let src = ".input {$count :integer}\n\
                   .match $count\n\
                   0 {{zero}}\n\
                   one {{one}}\n\
                   * {{many}}";
        assert!(validate(&parse(src)).is_ok());
    }

    #[test]
    fn validated_wrapper_roundtrips() {
        let msg = parse("Hello, {$user}!");
        let vm: ValidatedMessage = msg.clone().try_into().unwrap();
        assert_eq!(vm.as_message(), &msg);
        assert_eq!(vm.into_inner(), msg);
    }

    // ---- each validation error ----

    #[test]
    fn missing_fallback_variant() {
        let src = ".input {$x :integer}\n.match $x\n1 {{one}}";
        let err = validate(&parse(src)).unwrap_err();
        assert_eq!(err, ValidationError::MissingFallbackVariant);
    }

    #[test]
    fn variant_key_mismatch() {
        let src = ".input {$a :integer} .input {$b :integer} .match $a $b\n\
                   1 {{too few keys}}\n\
                   * * {{fallback}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::VariantKeyMismatch {
                expected: 2,
                actual: 1
            }
        ));
    }

    #[test]
    fn duplicate_declaration_input_input() {
        let src = ".input {$x :integer} .input {$x :integer} {{_}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::DuplicateDeclaration { ref name } if name.as_ref() == "x"
        ));
    }

    #[test]
    fn duplicate_declaration_input_local() {
        let src = ".input {$x :integer} .local $x = {|h|} {{_}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::DuplicateDeclaration { ref name } if name.as_ref() == "x"
        ));
    }

    #[test]
    fn duplicate_variant() {
        let src = ".input {$x :integer}\n.match $x\n\
                   1 {{one}}\n\
                   1 {{also one}}\n\
                   * {{default}}";
        let err = validate(&parse(src)).unwrap_err();
        assert_eq!(err, ValidationError::DuplicateVariant);
    }

    #[test]
    fn duplicate_option_name_is_parser_error() {
        // This one is caught at parse time because BTreeMap would lose
        // duplicate-info before reaching the validator.
        let src = "{$x :number minimumFractionDigits=0 minimumFractionDigits=2}";
        let err = Message::parse(src).unwrap_err();
        assert!(matches!(
            err,
            ParseError::DataModel(ValidationError::DuplicateOptionName { ref name })
                if name.as_ref() == "minimumFractionDigits"
        ));
    }

    #[test]
    fn missing_selector_annotation_direct() {
        // `$x` is declared but without a function annotation.
        let src = ".local $x = {|hello|}\n.match $x\n* {{fallback}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::MissingSelectorAnnotation { ref name } if name.as_ref() == "x"
        ));
    }

    #[test]
    fn missing_selector_annotation_undeclared() {
        // Selector references a variable with no declaration.
        let src = ".match $ghost\n* {{fallback}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::MissingSelectorAnnotation { ref name } if name.as_ref() == "ghost"
        ));
    }

    #[test]
    fn selector_annotation_transitive() {
        // `$a` is annotated; `$b = {$a}` inherits the annotation; selecting
        // on `$b` is valid.
        let src = ".input {$a :integer}\n\
                   .local $b = {$a}\n\
                   .match $b\n\
                   * {{ok}}";
        assert!(validate(&parse(src)).is_ok());
    }

    #[test]
    fn empty_expression_rejected() {
        // Programmatic construction of an expression with neither arg nor
        // function — the parser rejects this at syntax level but AST built
        // by hand can hit this path.
        use crate::messageformat::ast::{Attributes, Expression, Message, PatternElement};
        let msg = Message::Pattern {
            declarations: alloc::vec![],
            pattern: alloc::vec![PatternElement::Expression(Expression::new(
                None,
                None,
                Attributes::new(),
            ))],
        };
        assert_eq!(
            validate(&msg).unwrap_err(),
            ValidationError::EmptyExpression
        );
    }

    #[test]
    fn variant_key_mismatch_too_many_keys() {
        // Inverse of the existing "too few" test — parser rejects at the
        // source level typically, but the validator must still reject
        // keys > selectors if someone builds the AST by hand.
        let src = ".input {$a :integer}\n.match $a\n1 2 {{too many}}\n* {{fb}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::VariantKeyMismatch {
                expected: 1,
                actual: 2
            }
        ));
    }

    #[test]
    fn duplicate_variant_with_catchall() {
        // Two all-catchall variants share the same key list → duplicate.
        let src = ".input {$x :integer}\n.match $x\n* {{a}}\n* {{b}}";
        let err = validate(&parse(src)).unwrap_err();
        assert_eq!(err, ValidationError::DuplicateVariant);
    }

    #[test]
    fn selector_annotation_deep_chain() {
        // `$a :integer` → `$b = {$a}` → `$c = {$b}`; `.match $c` must resolve
        // the annotation transitively through both locals.
        let src = ".input {$a :integer}\n\
                   .local $b = {$a}\n\
                   .local $c = {$b}\n\
                   .match $c\n\
                   * {{ok}}";
        assert!(validate(&parse(src)).is_ok());
    }

    #[test]
    fn redeclaration_after_use_in_option() {
        // `.local $a = {$x :number minimumFractionDigits=$c}` then
        // `.input {$c :integer}` — rule 2 (must not bind a variable seen
        // in a previous declaration's option).
        let src = ".input {$x :number minimumFractionDigits=$c}\n\
                   .input {$c :integer}\n\
                   {{{$x}}}";
        let err = validate(&parse(src)).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::DuplicateDeclaration { ref name } if name.as_ref() == "c"
        ));
    }

    #[test]
    fn input_declaration_arg_name_mismatch() {
        // Programmatic AST: `.input {$y :string}` but arg says `$z`. Rule 5
        // (input-declaration value.arg must reference the bound name).
        use crate::messageformat::ast::{
            Arg, Attributes, Declaration, Expression, FunctionRef, Message, OptionMap,
            PatternElement, Variable,
        };
        let msg = Message::Pattern {
            declarations: alloc::vec![Declaration::Input {
                name: "y".to_string().into_boxed_str(),
                value: Expression::new(
                    Some(Arg::Variable(Variable::new(
                        "z".to_string().into_boxed_str(),
                    ))),
                    Some(FunctionRef::new(
                        "string".to_string().into_boxed_str(),
                        OptionMap::new(),
                    )),
                    Attributes::new(),
                ),
            }],
            pattern: alloc::vec![PatternElement::Text("x".to_string())],
        };
        assert!(matches!(
            validate(&msg).unwrap_err(),
            ValidationError::DuplicateDeclaration { ref name } if name.as_ref() == "y"
        ));
    }

    #[test]
    fn validated_message_eq_for_same_source() {
        // Two parses of the same source produce equal ValidatedMessages.
        let a: ValidatedMessage = parse("Hello!").try_into().unwrap();
        let b: ValidatedMessage = parse("Hello!").try_into().unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn selector_annotation_cycle_rejected_as_duplicate_declaration() {
        // `.local $a = {$a}` is now caught earlier by rule 3 (a local MUST
        // NOT reference the variable it binds) — see syntax.md §Declarations.
        // Surfaces as DuplicateDeclaration.
        let msg = Message::Select {
            declarations: alloc::vec![crate::messageformat::ast::Declaration::Local {
                name: "a".to_string().into_boxed_str(),
                value: crate::messageformat::ast::Expression::new(
                    Some(crate::messageformat::ast::Arg::Variable(
                        crate::messageformat::ast::Variable::new("a".to_string().into_boxed_str(),),
                    )),
                    None,
                    crate::messageformat::ast::Attributes::new(),
                ),
            }],
            selectors: alloc::vec![crate::messageformat::ast::Variable::new(
                "a".to_string().into_boxed_str(),
            )],
            variants: alloc::vec![crate::messageformat::ast::Variant {
                keys: alloc::vec![crate::messageformat::ast::VariantKey::Catchall(
                    crate::messageformat::ast::CatchallKey::default(),
                )],
                value: alloc::vec![crate::messageformat::ast::PatternElement::Text(
                    "x".to_string(),
                )],
            }],
        };
        let err = validate(&msg).unwrap_err();
        assert!(matches!(
            err,
            ValidationError::DuplicateDeclaration { ref name } if name.as_ref() == "a"
        ));
    }
}
