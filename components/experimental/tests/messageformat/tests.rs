// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Integration-test entry point for the `MessageFormat` 2 submodule.
//!
//! Each phase appends tests that exercise the end-to-end public API from an
//! external-crate perspective.

#[cfg(feature = "compiled_data")]
use fixed_decimal::Decimal;
use icu::locale::locale;
use icu_experimental::messageformat::{
    ast::Message, function::FunctionOptions, BidiIsolation, BidiStrategy, BuildError,
    DefaultBidiStrategy, Direction, FormatError, FormattedPart, FunctionContext, FunctionError,
    FunctionHandler, MessageFormatter, OwnedInputs, ParseError, ResolvedValue,
};

#[test]
fn phase3_parser_round_trips_simple_message() {
    let msg = Message::parse("Hello, {$user}!").expect("valid MF2 source");
    let Message::Pattern { pattern, .. } = msg else {
        panic!("expected pattern message");
    };
    assert_eq!(pattern.len(), 3);
}

#[test]
fn phase3_parser_reports_syntax_error() {
    let err = Message::parse("{$x").expect_err("unterminated expression");
    assert!(matches!(err, ParseError::Syntax { .. }));
}

#[test]
fn phase5_formats_simple_variable_substitution() {
    let formatter = MessageFormatter::builder()
        .source("Hello, {$user}!")
        .locale_undetermined()
        .build()
        .expect("builder succeeds");
    let inputs: &[(&str, &str)] = &[("user", "Ada")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "Hello, Ada!");
    assert!(errs.is_empty());
}

#[test]
fn phase5_formats_with_string_function() {
    let formatter = MessageFormatter::builder()
        .source(".input {$who :string}\n{{Hi, {$who}!}}")
        .locale_undetermined()
        .build()
        .expect("builder succeeds");
    let inputs: &[(&str, &str)] = &[("who", "Grace")];
    let (out, errs) = formatter.format_to_string(&inputs);
    // `:string` leaves direction unset; Default Bidi Strategy wraps with
    // FSI+PDI per spec formatting.md:871-874.
    assert_eq!(out, "Hi, \u{2068}Grace\u{2069}!");
    assert!(errs.is_empty());
}

#[test]
fn phase5_unresolved_variable_yields_fallback() {
    let formatter = MessageFormatter::builder()
        .source("Hello, {$missing}!")
        .locale_undetermined()
        .build()
        .expect("builder succeeds");
    let inputs: &[(&str, &str)] = &[];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "Hello, \u{2068}{$missing}\u{2069}!");
    assert_eq!(errs.len(), 1);
    assert!(
        matches!(&errs[0], FormatError::UnresolvedVariable { name } if name.as_ref() == "missing")
    );
}

#[test]
fn phase5_builder_validation_error() {
    // Missing fallback variant should surface as BuildError::Validation.
    let err = MessageFormatter::builder()
        .source(".input {$x :integer}\n.match $x\n1 {{one}}")
        .locale_undetermined()
        .build()
        .expect_err("validator rejects missing fallback");
    assert!(matches!(err, BuildError::Validation(_)));
}

// ---------------------------------------------------------------------------
// Matcher
// ---------------------------------------------------------------------------

#[cfg(feature = "compiled_data")]
#[test]
fn matcher_picks_exact_numeric_key_over_plural_category() {
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$count :integer}\n\
             .match $count\n\
             0   {{no items}}\n\
             one {{one item}}\n\
             *   {{{$count} items}}",
        )
        .locale(locale!("en"))
        .build()
        .expect("valid");
    // Exact key `0` beats the `one` plural category and the `*` fallback.
    let inputs = OwnedInputs::new().with_number("count", 0_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "no items");
    assert!(errs.is_empty());
}

#[cfg(feature = "compiled_data")]
#[test]
fn matcher_falls_through_to_catchall_when_no_key_matches() {
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$count :integer}\n\
             .match $count\n\
             0 {{zero}}\n\
             * {{many: {$count}}}",
        )
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("count", 42_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "many: 42");
    assert!(errs.is_empty());
}

#[cfg(feature = "compiled_data")]
#[test]
fn matcher_two_selectors_lexicographic_ranking() {
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$n :integer}\n\
             .input {$m :integer}\n\
             .match $n $m\n\
             1 1 {{one-one}}\n\
             1 * {{one-star}}\n\
             * * {{star-star}}",
        )
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new()
        .with_number("n", 1_i64)
        .with_number("m", 1_i64);
    let (out, _errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "one-one");

    let inputs = OwnedInputs::new()
        .with_number("n", 1_i64)
        .with_number("m", 99_i64);
    let (out, _errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "one-star");

    let inputs = OwnedInputs::new()
        .with_number("n", 99_i64)
        .with_number("m", 99_i64);
    let (out, _errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "star-star");
}

#[cfg(feature = "compiled_data")]
#[test]
fn number_notation_compact_short_english() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number notation=compact}}}")
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 1500_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "1.5K");
}

#[cfg(feature = "compiled_data")]
#[test]
fn number_notation_scientific_ascii_exponent() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number notation=scientific}}}")
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 12345_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "1.2345E4");
}

#[cfg(feature = "compiled_data")]
#[test]
fn number_notation_engineering() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number notation=engineering}}}")
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 12345_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "12.345E3");
}

#[cfg(feature = "compiled_data")]
#[test]
fn number_numbering_system_thai_digits() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number numberingSystem=thai}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 42_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "๔๒");
}

#[cfg(feature = "compiled_data")]
#[test]
fn math_alias_matches_offset_subtract() {
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$n :integer}\n\
             .local $m = {$n :math subtract=1}\n\
             .match $n $m\n\
             1 0 {{ok}}\n\
             * * {{other}}",
        )
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 1_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "ok");
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_display_name_uses_long_formatter() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencyDisplay=name}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", Decimal::try_from_str("5.01").unwrap());
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    let lower = out.to_lowercase();
    assert!(
        lower.contains("dollar") || lower.contains("us "),
        "unexpected long currency: {out:?}"
    );
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_display_code_prefixes_iso() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencyDisplay=code}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", Decimal::from(12_i64));
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(out.starts_with("USD"));
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_display_never_hides_currency() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=EUR currencyDisplay=never}}}")
        .locale(locale!("de-DE"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", 999_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(!out.contains("EUR"));
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_display_code_compact_notation_on_amount() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencyDisplay=code notation=compact}}}")
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", 1500_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(out.starts_with("USD "));
    assert!(out.contains("1.5K"), "compact amount in code mode: {out:?}");
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_sign_accounting_parentheses_negative() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencySign=accounting}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", Decimal::try_from_str("-3.50").unwrap());
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(
        out.starts_with('(') && out.ends_with(')'),
        "expected parens: {out:?}"
    );
    assert!(!out.contains("-"));
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_sign_accounting_uses_standard_form_when_locale_has_no_accounting_pattern() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=EUR currencySign=accounting}}}")
        .locale(locale!("de-DE"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", Decimal::try_from_str("-3.50").unwrap());
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(out.contains('-'));
    assert!(!out.starts_with('(') && !out.ends_with(')'));
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_sign_accounting_arabic_uses_accounting_affixes() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencySign=accounting}}}")
        .locale(locale!("ar-EG"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", Decimal::try_from_str("-3.50").unwrap());
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(out.contains('(') && out.contains(')'), "{out:?}");
    assert!(out.contains('\u{061C}'), "{out:?}");
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_symbol_scientific_en_us() {
    let formatter = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD notation=scientific}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("p", 1500_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "$1.5E3");
}

#[cfg(feature = "compiled_data")]
#[test]
fn currency_name_compact_long_and_short_en_us() {
    let f_long = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencyDisplay=name notation=compact compactDisplay=long}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let (out_long, errs) =
        f_long.format_to_string(&OwnedInputs::new().with_number("p", 1_500_000_i64));
    assert!(errs.is_empty(), "{errs:?}");
    let lower = out_long.to_lowercase();
    assert!(
        lower.contains("million") && lower.contains("dollar"),
        "{out_long:?}"
    );

    let f_short = MessageFormatter::builder()
        .source("{{{$p :currency currency=USD currencyDisplay=name notation=compact compactDisplay=short}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let (out_short, errs) =
        f_short.format_to_string(&OwnedInputs::new().with_number("p", 1_500_000_i64));
    assert!(errs.is_empty(), "{errs:?}");
    let lower = out_short.to_lowercase();
    assert!(
        out_short.contains('M') && lower.contains("dollar"),
        "{out_short:?}"
    );
}

#[cfg(feature = "compiled_data")]
#[test]
fn number_scientific_times_superscript() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number notation=scientific scientificNotation=timesSuperscript}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 1500_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(out.contains('\u{00d7}'), "multiplication sign: {out:?}");
    // Superscript digit ³ (U+00B3) for exponent 3
    assert!(out.contains('\u{00b3}'), "{out:?}");
}

#[cfg(feature = "compiled_data")]
#[test]
fn scientific_notation_option_requires_scientific_or_engineering() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number notation=standard scientificNotation=e}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 1_i64);
    let (_out, errs) = formatter.format_to_string(&inputs);
    assert!(!errs.is_empty(), "expected option validation error, got ok");
}

#[cfg(feature = "compiled_data")]
#[test]
fn number_scientific_exponent_uses_numbering_system() {
    let formatter = MessageFormatter::builder()
        .source("{{{$n :number notation=scientific numberingSystem=thai}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 12345_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert!(out.contains('E'));
    // Exponent 4 uses Thai digit ๔
    assert!(out.contains('๔'), "expected Thai exponent digit: {out:?}");
}

// ---------------------------------------------------------------------------
// u-namespace options
// ---------------------------------------------------------------------------

#[test]
fn u_dir_ltr_wraps_with_lri_when_base_is_rtl() {
    let formatter = MessageFormatter::builder()
        .source("before {$x :string u:dir=ltr} after")
        .direction(Direction::Rtl)
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("x", "LTRtext")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    // LRI (U+2066) prefix, PDI (U+2069) suffix.
    assert!(out.contains("\u{2066}LTRtext\u{2069}"));
}

#[test]
fn u_dir_inherit_suppresses_isolation() {
    let formatter = MessageFormatter::builder()
        .source("a {$x :string u:dir=inherit} b")
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("x", "inner")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    // No isolation control characters expected.
    assert_eq!(out, "a inner b");
}

#[test]
fn u_id_flows_to_parts() {
    let formatter = MessageFormatter::builder()
        .source("{$x :string u:id=label1}")
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("x", "v")];
    let (parts, errs) = formatter.format_to_parts(&inputs);
    assert!(errs.is_empty());
    let mut saw_id = false;
    for p in &parts {
        if let FormattedPart::Expression { id: Some(id), .. } = p {
            assert_eq!(id.as_ref(), "label1");
            saw_id = true;
        }
    }
    assert!(saw_id, "expected an Expression part with u:id=label1");
}

#[cfg(feature = "compiled_data")]
#[test]
fn u_locale_override_reformats_number() {
    // Base locale en — French-style grouping would be thin space, but
    // here we use de-DE to get '.' as a grouping separator.
    let formatter = MessageFormatter::builder()
        .source("{$n :number u:locale=de-DE}")
        .locale(locale!("en"))
        .build()
        .expect("valid");
    let inputs = OwnedInputs::new().with_number("n", 1_234_567_i64);
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    // de-DE uses '.' as a group separator.
    assert!(
        out.contains("1.234.567"),
        "expected de-DE grouping, got: {out}"
    );
}

// ---------------------------------------------------------------------------
// Custom functions
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct UppercaseFn;
impl FunctionHandler for UppercaseFn {
    fn format(
        &self,
        _ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        _options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        let txt = operand.map(|o| o.text()).unwrap_or("");
        Ok(ResolvedValue::new(txt.to_uppercase()))
    }
}

#[test]
fn custom_function_registered_via_builder() {
    let formatter = MessageFormatter::builder()
        .source("shout {$x :upper}")
        .function("upper", UppercaseFn)
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("x", "hello")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    // `:upper` returns a ResolvedValue with no direction → still wrapped
    // with FSI+PDI by the default bidi strategy.
    assert_eq!(out, "shout \u{2068}HELLO\u{2069}");
}

#[test]
fn unknown_function_yields_fallback_and_error() {
    let formatter = MessageFormatter::builder()
        .source("x {$y :nonexistent}")
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("y", "v")];
    let (out, errs) = formatter.format_to_string(&inputs);
    // Fallback source is `{$y}` (the operand) per spec.
    assert!(out.contains("{$y}"), "expected fallback, got: {out}");
    assert_eq!(errs.len(), 1);
    assert!(
        matches!(&errs[0], FormatError::UnknownFunction { name } if name.as_ref() == "nonexistent")
    );
}

// ---------------------------------------------------------------------------
// format_to_parts
// ---------------------------------------------------------------------------

#[test]
fn format_to_parts_emits_text_and_expression() {
    let formatter = MessageFormatter::builder()
        .source("Hi, {$name}!")
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("name", "Ada")];
    let (parts, errs) = formatter.format_to_parts(&inputs);
    assert!(errs.is_empty());
    // At least one Text part and one Expression part must be present.
    assert!(parts
        .iter()
        .any(|p| matches!(p, FormattedPart::Text { .. })));
    assert!(parts
        .iter()
        .any(|p| matches!(p, FormattedPart::Expression { .. })));
}

#[test]
fn format_to_parts_marks_fallback_kind() {
    let formatter = MessageFormatter::builder()
        .source("{$missing}")
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[];
    let (parts, errs) = formatter.format_to_parts(&inputs);
    assert!(!errs.is_empty());
    let kinds: Vec<_> = parts
        .iter()
        .filter_map(|p| match p {
            FormattedPart::Expression { kind, .. } => Some(kind.as_ref()),
            _ => None,
        })
        .collect();
    assert!(
        kinds.contains(&"fallback"),
        "expected fallback kind, got: {kinds:?}"
    );
}

// ---------------------------------------------------------------------------
// bidi isolation toggle
// ---------------------------------------------------------------------------

#[test]
fn bidi_isolation_disabled_suppresses_isolates() {
    let formatter = MessageFormatter::builder()
        .source(".input {$who :string}\n{{Hi, {$who}!}}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("who", "Grace")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "Hi, Grace!");
}

// ---------------------------------------------------------------------------
// Builder error paths
// ---------------------------------------------------------------------------

#[test]
fn builder_without_source_errors() {
    let err = MessageFormatter::builder()
        .locale_undetermined()
        .build()
        .expect_err("no source supplied");
    assert!(matches!(err, BuildError::NoMessage));
}

#[test]
fn builder_syntax_error_propagates() {
    let err = MessageFormatter::builder()
        .source("{$x")
        .locale_undetermined()
        .build()
        .expect_err("syntax error");
    assert!(matches!(err, BuildError::Parse(ParseError::Syntax { .. })));
}

#[test]
fn duplicate_declaration_is_validation_error() {
    let err = MessageFormatter::builder()
        .source(".input {$x :string}\n.input {$x :string}\n{{{$x}}}")
        .locale_undetermined()
        .build()
        .expect_err("duplicate declaration");
    assert!(matches!(err, BuildError::Validation(_)));
}

// ---------------------------------------------------------------------------
// Custom FunctionError (implementation-defined)
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct FailingFn;
impl FunctionHandler for FailingFn {
    fn format(
        &self,
        _ctx: &FunctionContext<'_>,
        _operand: Option<&ResolvedValue>,
        _options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        Err(FunctionError::Custom {
            kind: Box::from("my-custom-error"),
            message: Box::from("handler said no"),
        })
    }
}

#[test]
fn custom_function_error_surfaces_through_format() {
    let formatter = MessageFormatter::builder()
        .source("x {$y :failing}")
        .function("failing", FailingFn)
        .locale_undetermined()
        .build()
        .expect("valid");
    let inputs: &[(&str, &str)] = &[("y", "v")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(out.contains("{$y}"), "expected fallback output, got: {out}");
    assert_eq!(errs.len(), 1);
    let FormatError::FunctionError { function, error } = &errs[0] else {
        panic!("expected FunctionError, got {:?}", errs[0]);
    };
    assert_eq!(function.as_ref(), "failing");
    let FunctionError::Custom { kind, message } = error else {
        panic!("expected Custom, got {error:?}");
    };
    assert_eq!(kind.as_ref(), "my-custom-error");
    assert_eq!(message.as_ref(), "handler said no");
}

// ---------------------------------------------------------------------------
// Message::to_source — round-trip via parse
// ---------------------------------------------------------------------------

#[test]
fn to_source_round_trip_pattern() {
    let sources = [
        "Hello",
        "Hi, {$name}!",
        "{$count :integer minimumFractionDigits=0}",
        ".input {$n :number}\n{{You have {$n}.}}",
        ".local $m = {$n :number}\n{{value is {$m}}}",
    ];
    for src in sources {
        let a = Message::parse(src).unwrap_or_else(|e| panic!("parse {src:?}: {e:?}"));
        let emitted = a.to_source();
        let b = Message::parse(&emitted).unwrap_or_else(|e| panic!("reparse {emitted:?}: {e:?}"));
        assert_eq!(
            a, b,
            "round-trip mismatch\n  src: {src:?}\n  emit: {emitted:?}"
        );
    }
}

#[test]
fn to_source_round_trip_select() {
    let src = ".input {$count :integer}\n\
               .match $count\n\
               0 {{No messages.}}\n\
               one {{One message.}}\n\
               * {{{$count} messages.}}";
    let a = Message::parse(src).unwrap();
    let emitted = a.to_source();
    let b = Message::parse(&emitted).unwrap();
    assert_eq!(a, b);
}

#[test]
fn to_source_round_trip_markup() {
    let src = "Click {#a href=|/docs|}here{/a}!";
    let a = Message::parse(src).unwrap();
    let emitted = a.to_source();
    let b = Message::parse(&emitted).unwrap();
    assert_eq!(a, b);
}

#[test]
fn to_source_end_to_end_equivalent_format() {
    let src = "Hi, {$name}!";
    let emitted = Message::parse(src).unwrap().to_source();
    let f1 = MessageFormatter::builder()
        .source(src)
        .locale(locale!("en"))
        .bidi_isolation(false)
        .build()
        .unwrap();
    let f2 = MessageFormatter::builder()
        .source(&emitted)
        .locale(locale!("en"))
        .bidi_isolation(false)
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("name", "Ada")];
    assert_eq!(
        f1.format_to_string(&inputs).0,
        f2.format_to_string(&inputs).0
    );
}

// ---------------------------------------------------------------------------
// Miscellaneous coverage additions
// ---------------------------------------------------------------------------

#[test]
fn empty_expression_is_validation_error() {
    // `{}` is a syntax error at the lexer, not a ValidationError::EmptyExpression,
    // but the programmatic builder path surfaces EmptyExpression when an AST is
    // constructed with neither arg nor function. Test the parser-side.
    let err = Message::parse("{}").expect_err("empty expression");
    assert!(matches!(err, ParseError::Syntax { .. }));
}

#[test]
fn missing_fallback_variant_validation_error() {
    let err = MessageFormatter::builder()
        .source(".input {$c :integer}\n.match $c\n1 {{one}}")
        .locale_undetermined()
        .build()
        .expect_err("no catchall");
    assert!(matches!(
        err,
        BuildError::Validation(
            icu_experimental::messageformat::ValidationError::MissingFallbackVariant
        )
    ));
}

#[test]
fn unresolved_variable_emits_fallback() {
    let formatter = MessageFormatter::builder()
        .source("Value: {$missing}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "Value: {$missing}");
    assert!(
        matches!(&errs[0], FormatError::UnresolvedVariable { name } if name.as_ref() == "missing")
    );
}

#[test]
fn attributes_do_not_affect_output() {
    let formatter = MessageFormatter::builder()
        .source("x {$y :string @note=|internal| @flag}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("y", "v")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "x v");
}

#[test]
fn direction_enum_roundtrips_via_bidi_strategy() {
    // Reach into the Direction type to confirm the exported enum is usable
    // and the formatter applies it as expected.
    let formatter = MessageFormatter::builder()
        .source("{$x :string}")
        .locale(locale!("ar"))
        .direction(Direction::Rtl)
        .bidi_isolation(true)
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("x", "ABC")];
    let (out, _errs) = formatter.format_to_string(&inputs);
    // RTL base with an unmarked LTR-looking value must still isolate.
    assert!(out.contains('\u{2068}') || out.contains('\u{2067}'));
}

// ---------------------------------------------------------------------------
// BidiIsolation enum API
// ---------------------------------------------------------------------------

#[test]
fn bidi_isolation_enum_default_matches_bool_true() {
    let a = MessageFormatter::builder()
        .source("{$x :string}")
        .bidi_isolation(BidiIsolation::Default)
        .locale_undetermined()
        .build()
        .unwrap();
    let b = MessageFormatter::builder()
        .source("{$x :string}")
        .bidi_isolation(true)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("x", "v")];
    assert_eq!(a.format_to_string(&inputs).0, b.format_to_string(&inputs).0);
    assert!(matches!(a.bidi_isolation(), BidiIsolation::Default));
    assert!(a.bidi_isolation().is_enabled());
}

#[test]
fn bidi_isolation_enum_none_matches_bool_false() {
    let a = MessageFormatter::builder()
        .source("{$x :string}")
        .bidi_isolation(BidiIsolation::None)
        .locale_undetermined()
        .build()
        .unwrap();
    let b = MessageFormatter::builder()
        .source("{$x :string}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("x", "v")];
    assert_eq!(a.format_to_string(&inputs).0, b.format_to_string(&inputs).0);
    assert!(matches!(a.bidi_isolation(), BidiIsolation::None));
    assert!(matches!(b.bidi_isolation(), BidiIsolation::None));
    assert!(!a.bidi_isolation().is_enabled());
}

#[test]
fn bidi_isolation_defaults_to_enabled_without_call() {
    let fmt = MessageFormatter::builder()
        .source("{$x :string}")
        .locale_undetermined()
        .build()
        .unwrap();
    assert!(matches!(fmt.bidi_isolation(), BidiIsolation::Default));
    assert!(fmt.bidi_isolation().is_enabled());
}

// ---------------------------------------------------------------------------
// Declaration resolution
// ---------------------------------------------------------------------------

#[test]
fn local_declaration_references_prior_input() {
    // `.local $greeting` depends on `$name` from `.input`. Format resolves
    // the local and substitutes its text into the pattern.
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$name :string}\n\
             .local $greeting = {$name :string}\n\
             {{Hello, {$greeting}!}}",
        )
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("name", "Ada")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "Hello, Ada!");
}

#[test]
fn duplicate_variant_is_validation_error() {
    use icu_experimental::messageformat::ValidationError;
    let err = MessageFormatter::builder()
        .source(".input {$c :integer}\n.match $c\n1 {{a}}\n1 {{b}}\n* {{c}}")
        .locale_undetermined()
        .build()
        .expect_err("duplicate variant");
    assert!(matches!(
        err,
        BuildError::Validation(ValidationError::DuplicateVariant)
    ));
}

#[test]
fn missing_selector_annotation_is_validation_error() {
    use icu_experimental::messageformat::ValidationError;
    // `.match $x` with `$x` never annotated triggers MissingSelectorAnnotation.
    let err = MessageFormatter::builder()
        .source(".match $x\n1 {{a}}\n* {{b}}")
        .locale_undetermined()
        .build()
        .expect_err("missing annotation");
    assert!(matches!(
        err,
        BuildError::Validation(ValidationError::MissingSelectorAnnotation { ref name })
            if name.as_ref() == "x"
    ));
}

// ---------------------------------------------------------------------------
// Markup
// ---------------------------------------------------------------------------

#[test]
fn markup_with_attributes_emits_but_ignores_attributes() {
    let formatter = MessageFormatter::builder()
        .source("{#link @role=button}click{/link}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[];
    let (parts, errs) = formatter.format_to_parts(&inputs);
    assert!(errs.is_empty());
    // Markup parts surface; attributes must not change the output string.
    let (out, _) = formatter.format_to_string(&inputs);
    assert_eq!(out, "click");
    let has_markup = parts
        .iter()
        .any(|p| matches!(p, FormattedPart::Markup { name, .. } if name.as_ref() == "link"));
    assert!(has_markup);
}

#[test]
fn markup_u_dir_option_triggers_bad_option_error() {
    // Per u-namespace.md:51-52, `u:dir` on markup is a Bad Option error.
    let formatter = MessageFormatter::builder()
        .source("{#b u:dir=ltr}x{/b}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[];
    let (_out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.iter().any(|e| matches!(
        e,
        FormatError::FunctionError {
            error: FunctionError::BadOption { name },
            ..
        } if name.as_ref() == "u:dir"
    )));
}

// ---------------------------------------------------------------------------
// Formatter re-use and fmt::Write sink
// ---------------------------------------------------------------------------

#[test]
fn formatter_is_reusable_across_calls() {
    let formatter = MessageFormatter::builder()
        .source("Hello, {$name}!")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let a_in: &[(&str, &str)] = &[("name", "Ada")];
    let b_in: &[(&str, &str)] = &[("name", "Grace")];
    assert_eq!(formatter.format_to_string(&a_in).0, "Hello, Ada!");
    assert_eq!(formatter.format_to_string(&b_in).0, "Hello, Grace!");
}

#[test]
fn format_into_arbitrary_fmt_write_sink() {
    use core::fmt::Write as _;
    let formatter = MessageFormatter::builder()
        .source("Hi, {$u}")
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    // Test via a buffered String sink — any `fmt::Write` works.
    let mut sink = String::new();
    write!(sink, "[").unwrap();
    let inputs: &[(&str, &str)] = &[("u", "x")];
    let errs = formatter.format(&inputs, &mut sink).unwrap();
    write!(sink, "]").unwrap();
    assert!(errs.is_empty());
    assert_eq!(sink, "[Hi, x]");
}

// ---------------------------------------------------------------------------
// Selector matching
// ---------------------------------------------------------------------------

#[test]
fn string_selector_exact_match_wins_over_catchall() {
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$kind :string}\n\
             .match $kind\n\
             apple {{fruit}}\n\
             *     {{other}}",
        )
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let apple: &[(&str, &str)] = &[("kind", "apple")];
    let zebra: &[(&str, &str)] = &[("kind", "zebra")];
    assert_eq!(formatter.format_to_string(&apple).0, "fruit");
    assert_eq!(formatter.format_to_string(&zebra).0, "other");
}

#[test]
fn unknown_function_in_selector_triggers_bad_selector_and_falls_back() {
    let formatter = MessageFormatter::builder()
        .source(
            ".input {$x :unknown-fn}\n\
             .match $x\n\
             1 {{one}}\n\
             * {{other}}",
        )
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("x", "1")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert_eq!(out, "other");
    // Spec requires both UnknownFunction *and* BadSelector.
    assert!(errs
        .iter()
        .any(|e| matches!(e, FormatError::UnknownFunction { .. })));
    assert!(errs
        .iter()
        .any(|e| matches!(e, FormatError::BadSelector { .. })));
}

// ---------------------------------------------------------------------------
// Validated-message reuse path
// ---------------------------------------------------------------------------

#[test]
fn builder_accepts_pre_validated_message() {
    use icu_experimental::messageformat::ValidatedMessage;
    let validated: ValidatedMessage = Message::parse("Hello, {$u}!").unwrap().try_into().unwrap();
    let formatter = MessageFormatter::builder()
        .message(validated)
        .bidi_isolation(false)
        .locale_undetermined()
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("u", "Ada")];
    assert_eq!(formatter.format_to_string(&inputs).0, "Hello, Ada!");
}

// ---------------------------------------------------------------------------
// Explicit-locale enforcement + pluggable bidi strategies
// ---------------------------------------------------------------------------

#[test]
fn missing_locale_surfaces_as_build_error() {
    let err = MessageFormatter::builder()
        .source("Hello")
        .build()
        .expect_err("locale is required");
    assert!(matches!(err, BuildError::MissingLocale));
}

#[test]
fn custom_bidi_strategy_runs() {
    use std::borrow::Cow;
    use std::sync::Arc;

    #[derive(Debug)]
    struct Bracket;
    impl BidiStrategy for Bracket {
        fn isolate<'a>(
            &'a self,
            _base: Direction,
            _placeholder: Option<Direction>,
            _explicit: bool,
        ) -> (Cow<'a, str>, Cow<'a, str>) {
            (Cow::Borrowed("["), Cow::Borrowed("]"))
        }
    }

    let formatter = MessageFormatter::builder()
        .source("Hi, {$u :string}!")
        .locale(locale!("en"))
        .bidi_isolation(BidiIsolation::Custom(Arc::new(Bracket)))
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("u", "Ada")];
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty());
    assert_eq!(out, "Hi, [Ada]!");
}

#[test]
fn custom_strategy_can_delegate_to_default() {
    use std::borrow::Cow;
    use std::sync::Arc;

    // Wraps the default isolate pair in extra marker characters to prove
    // both delegation and dynamic (owned) returns work.
    #[derive(Debug, Default)]
    struct WrapMarker;
    impl BidiStrategy for WrapMarker {
        fn isolate<'a>(
            &'a self,
            base: Direction,
            placeholder: Option<Direction>,
            explicit: bool,
        ) -> (Cow<'a, str>, Cow<'a, str>) {
            let (pre, suf) = DefaultBidiStrategy.isolate(base, placeholder, explicit);
            (
                Cow::Owned(format!("<{}", pre)),
                Cow::Owned(format!("{}>", suf)),
            )
        }
    }

    let formatter = MessageFormatter::builder()
        .source("{$u :string}")
        .locale(locale!("en"))
        .bidi_isolation(BidiIsolation::Custom(Arc::new(WrapMarker)))
        .build()
        .unwrap();
    let inputs: &[(&str, &str)] = &[("u", "x")];
    let (out, _errs) = formatter.format_to_string(&inputs);
    // DefaultBidiStrategy emits FSI (U+2068)+PDI (U+2069) when direction
    // is unknown; the marker strategy brackets them.
    assert_eq!(out, "<\u{2068}x\u{2069}>");
}

/// End-to-end coverage for draft `:unit` (JSON cases live in
/// `tests/messageformat/fixtures/tests/functions/unit.json`; see
/// `messageformat-tr35-spec-tracking.md` §2).
#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[test]
fn messageformatter_end_to_end_unit_without_usage() {
    let formatter = MessageFormatter::builder()
        .source("{{{$len :unit unit=meter}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("build");
    let inputs = OwnedInputs::new().with_unit("len", 2_i64, "meter");
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty(), "{errs:?}");
    assert!(
        out.contains('2') && (out.contains("meter") || out.contains('m')),
        "unexpected output: {out:?}"
    );
}

#[cfg(all(feature = "unstable", feature = "compiled_data"))]
#[test]
fn messageformatter_unit_usage_converts_to_preferred_region_unit() {
    let formatter = MessageFormatter::builder()
        .source("{{{$len :unit unit=meter usage=road}}}")
        .locale(locale!("en-US"))
        .build()
        .expect("build");
    let inputs = OwnedInputs::new().with_unit("len", 2_i64, "meter");
    let (out, errs) = formatter.format_to_string(&inputs);
    assert!(errs.is_empty(), "{errs:?}");
    assert!(
        out.contains("ft") || out.contains("foot"),
        "expected road usage to pick feet in en-US, got {out:?}"
    );
}
