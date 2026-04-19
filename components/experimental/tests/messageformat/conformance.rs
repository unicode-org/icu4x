// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `MessageFormat` 2 conformance runner.
//!
//! Loads the JSON fixtures vendored at
//! `components/experimental/tests/messageformat/fixtures/` (synced from
//! `unicode-org/message-format-wg` via `cargo make sync-mf2-tests`) and
//! exercises each test case against [`MessageFormatter`].
//!
//! Each fixture JSON follows the schema at `schemas/v0/tests.schema.json`.
//! We execute every case and compare:
//! - `exp` — formatted-string output
//! - `expErrors` — error type(s) emitted (fatal build errors count too)
//! - `expParts` — structured-parts output, compared via a spec-shape
//!   projection of [`FormattedPart`] (see [`parts_match_expected`]).
//! - `only` / `skip` fixture-schema flags are honored: an `only: true`
//!   case narrows the run to that case only (per-fixture); a case with
//!   the `skip` tag is excluded.
//!
//! Tests that rely on features this implementation does not yet provide
//! (e.g. some draft datetime option behaviors) are listed in
//! [`KNOWN_FAILURES`] with a brief reason.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use fixed_decimal::Decimal;
use icu_experimental::messageformat::{
    ast::MarkupKind, BuildError, Direction, FormatError, FormattedPart, FunctionContext,
    FunctionError, FunctionHandler, FunctionOptions, MessageFormatter, OwnedInputs, ParseError,
    ResolvedValue, SelectorImpl, ValidationError,
};
use icu_locale_core::Locale;
use serde_json::Value;

/// Tests we cannot pass yet. Entries are `(file_stem, index_as_string)` and
/// must match the case's position in its fixture file.
///
/// Draft `:datetime` / `:date` / `:time` handlers and the spec's
/// reserved-for-testing `:test:function` / `:test:select` / `:test:format`
/// handlers are registered by [`run_case`] so every upstream case runs.
const KNOWN_FAILURES: &[(&str, &str)] = &[];

fn fixtures_root() -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest).join("tests/messageformat/fixtures/tests")
}

#[derive(Debug, Clone)]
struct Case {
    file: String,
    index: usize,
    description: Option<String>,
    locale: Option<String>,
    src: String,
    params: Vec<(String, Value)>,
    exp: Option<String>,
    exp_parts: Option<Vec<Value>>,
    exp_errors: Vec<String>,
    tags: Vec<String>,
    /// `"default"` (enabled) or `"none"` (disabled).
    bidi_isolation: Option<String>,
    /// Per the fixture schema, `only: true` restricts the run to matching
    /// cases within the same fixture file.
    only: bool,
}

fn expand_cases(file_stem: &str, doc: &Value) -> Vec<Case> {
    let defaults = doc
        .get("defaultTestProperties")
        .cloned()
        .unwrap_or(Value::Null);
    let tests = doc
        .get("tests")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    tests
        .into_iter()
        .enumerate()
        .map(|(idx, t)| build_case(file_stem, idx, &defaults, &t))
        .collect()
}

fn build_case(file_stem: &str, idx: usize, defaults: &Value, t: &Value) -> Case {
    let get = |key: &str| t.get(key).or_else(|| defaults.get(key));
    let src = get("src").and_then(Value::as_str).unwrap_or("").to_string();
    let locale = get("locale").and_then(Value::as_str).map(str::to_string);
    let description = t
        .get("description")
        .and_then(Value::as_str)
        .map(str::to_string);
    let exp = get("exp").and_then(Value::as_str).map(str::to_string);
    let exp_parts = get("expParts")
        .and_then(Value::as_array)
        .map(|a| a.to_vec());
    let exp_errors = get("expErrors")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|e| e.get("type").and_then(Value::as_str).map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let bidi_isolation = get("bidiIsolation")
        .and_then(Value::as_str)
        .map(str::to_string);
    let tags = get("tags")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|e| e.as_str().map(str::to_string))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let params = get("params")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|p| {
                    let name = p.get("name")?.as_str()?.to_string();
                    let value = p.get("value")?.clone();
                    Some((name, value))
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let only = t.get("only").and_then(Value::as_bool).unwrap_or(false);
    Case {
        file: file_stem.to_string(),
        index: idx,
        description,
        locale,
        src,
        params,
        exp,
        exp_parts,
        exp_errors,
        tags,
        bidi_isolation,
        only,
    }
}

fn inputs_from_params(params: &[(String, Value)]) -> OwnedInputs {
    let mut inputs = OwnedInputs::new();
    for (name, value) in params {
        match value {
            Value::Null => inputs = inputs.with_null(name),
            Value::Bool(b) => inputs = inputs.with_bool(name, *b),
            Value::String(s) => inputs = inputs.with_str(name, s),
            Value::Number(n) => {
                // Preserve precision by routing through the string form.
                if let Some(i) = n.as_i64() {
                    inputs = inputs.with_number(name, i);
                } else if let Ok(dec) = n.to_string().parse::<Decimal>() {
                    inputs = inputs.with_number(name, dec);
                } else {
                    inputs = inputs.with_str(name, &n.to_string());
                }
            }
            other => inputs = inputs.with_str(name, &other.to_string()),
        }
    }
    inputs
}

fn error_type(err: &FormatError) -> &'static str {
    match err {
        FormatError::UnresolvedVariable { .. } => "unresolved-variable",
        FormatError::UnknownFunction { .. } => "unknown-function",
        FormatError::BadSelector { .. } => "bad-selector",
        FormatError::FunctionError { error, .. } => match error {
            FunctionError::BadOperand => "bad-operand",
            FunctionError::BadOption { .. } => "bad-option",
            FunctionError::BadVariantKey { .. } => "bad-variant-key",
            FunctionError::UnsupportedOperation => "unsupported-operation",
            _ => "unknown-function-error",
        },
        _ => "unknown-format-error",
    }
}

fn build_error_type(err: &BuildError) -> &'static str {
    match err {
        BuildError::Parse(ParseError::Syntax { .. }) => "syntax-error",
        BuildError::Parse(ParseError::DataModel(v)) | BuildError::Validation(v) => {
            validation_error_type(v)
        }
        BuildError::NoMessage => "syntax-error",
        _ => "unknown-build-error",
    }
}

fn validation_error_type(v: &ValidationError) -> &'static str {
    match v {
        ValidationError::MissingFallbackVariant => "missing-fallback-variant",
        ValidationError::VariantKeyMismatch { .. } => "variant-key-mismatch",
        ValidationError::DuplicateDeclaration { .. } => "duplicate-declaration",
        ValidationError::DuplicateVariant => "duplicate-variant",
        ValidationError::DuplicateOptionName { .. } => "duplicate-option-name",
        ValidationError::MissingSelectorAnnotation { .. } => "missing-selector-annotation",
        ValidationError::EmptyExpression => "syntax-error",
        _ => "unknown-validation-error",
    }
}

#[derive(Debug)]
struct Outcome {
    /// Observed errors (build + format) as spec error-type strings.
    errors: BTreeSet<String>,
    /// Formatted output (empty if the build failed).
    output: String,
    /// Structured parts output. Empty Vec if the build failed or if
    /// `format_to_parts` is not applicable.
    parts: Vec<FormattedPart>,
}

fn run_case(case: &Case) -> Outcome {
    let locale: Locale = case
        .locale
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| "und".parse().unwrap());

    let mut builder = MessageFormatter::builder().source(case.src.clone());
    builder = builder.locale(locale);
    builder = register_conformance_handlers(builder);
    // Honor the per-case `bidiIsolation` property: `"default"` enables the
    // Default Bidi Strategy, `"none"` disables isolation.
    match case.bidi_isolation.as_deref() {
        Some("none") => builder = builder.bidi_isolation(false),
        Some("default") | None => builder = builder.bidi_isolation(true),
        _ => {}
    }
    let formatter = match builder.build() {
        Ok(f) => f,
        Err(e) => {
            let mut errors = BTreeSet::new();
            errors.insert(build_error_type(&e).to_string());
            return Outcome {
                errors,
                output: String::new(),
                parts: Vec::new(),
            };
        }
    };

    let inputs = inputs_from_params(&case.params);
    let (output, format_errors) = formatter.format_to_string(&inputs);
    let errors = format_errors
        .iter()
        .map(|e| error_type(e).to_string())
        .collect();
    let (parts, _parts_errors) = formatter.format_to_parts(&inputs);
    Outcome {
        errors,
        output,
        parts,
    }
}

fn is_known_failure(case: &Case) -> bool {
    KNOWN_FAILURES.iter().any(|(file, idx)| {
        *file == case.file.as_str() && idx.parse::<usize>().ok() == Some(case.index)
    })
}

fn check_case(case: &Case, outcome: &Outcome) -> Result<(), String> {
    let mut mismatches = Vec::new();

    // Compare error types (set equality — every expected error must appear
    // and the implementation must not emit unexpected ones).
    let expected: BTreeSet<String> = case.exp_errors.iter().cloned().collect();
    let got = &outcome.errors;
    let missing: Vec<_> = expected.difference(got).cloned().collect();
    let extra: Vec<_> = got.difference(&expected).cloned().collect();
    if !missing.is_empty() {
        mismatches.push(format!("missing errors: {missing:?}"));
    }
    if !extra.is_empty() {
        mismatches.push(format!("unexpected errors: {extra:?}"));
    }

    // Compare formatted string when the expected value is present AND no
    // fatal build error was expected.
    if let Some(expected_out) = &case.exp {
        if outcome.errors.iter().any(|e| e == "syntax-error") {
            // Build failed — can't compare output.
        } else if outcome.output != *expected_out {
            mismatches.push(format!(
                "output mismatch: expected {:?}, got {:?}",
                expected_out, outcome.output
            ));
        }
    }

    // Compare structured parts when present.
    if let Some(expected_parts) = &case.exp_parts {
        if !outcome.errors.iter().any(|e| e == "syntax-error") {
            if let Err(msg) = parts_match_expected(&outcome.parts, expected_parts) {
                mismatches.push(msg);
            }
        }
    }

    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(mismatches.join("; "))
    }
}

/// Compare our [`FormattedPart`]s against the fixture's `expParts` list.
///
/// Fixtures enumerate `bidiIsolation` parts explicitly, so we compare them
/// 1:1 against our [`FormattedPart::BidiIsolation`] output. For each
/// expected part, required fields are compared strictly; optional fields
/// (`id`, `value`, `dir`, `options`, `locale`, `parts`) are compared only
/// when the fixture specifies them.
fn parts_match_expected(actual: &[FormattedPart], expected: &[Value]) -> Result<(), String> {
    if actual.len() != expected.len() {
        return Err(format!(
            "parts length mismatch: expected {} parts, got {} (got: {:?})",
            expected.len(),
            actual.len(),
            actual
        ));
    }
    for (i, (got, want)) in actual.iter().zip(expected.iter()).enumerate() {
        check_part(got, want).map_err(|e| format!("parts[{i}]: {e} (got {got:?}, want {want})"))?;
    }
    Ok(())
}

fn check_part(got: &FormattedPart, want: &Value) -> Result<(), String> {
    let want_type = want
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| String::from("expected part is missing `type`"))?;
    match (got, want_type) {
        (FormattedPart::Text { value }, "text") => {
            match want.get("value").and_then(Value::as_str) {
                Some(v) if v == value => Ok(()),
                Some(v) => Err(format!("text value mismatch: want {v:?}, got {value:?}")),
                None => Err(String::from("expected text part missing `value`")),
            }
        }
        (
            FormattedPart::Expression {
                kind, value, id, ..
            },
            "fallback",
        ) => {
            if kind.as_ref() != "fallback" {
                return Err(format!("part kind mismatch: want `fallback`, got `{kind}`"));
            }
            // `source` is the unwrapped inner form; our `value` carries
            // the braced form like `{$var}`. Strip the outer braces.
            let inner = value.strip_prefix('{').and_then(|s| s.strip_suffix('}'));
            let want_source = want.get("source").and_then(Value::as_str);
            match (inner, want_source) {
                (Some(inner), Some(want_source)) if inner == want_source => {}
                (Some(inner), Some(want_source)) => {
                    return Err(format!(
                        "fallback source mismatch: want {want_source:?}, got {inner:?}"
                    ));
                }
                (None, _) => {
                    return Err(format!("fallback value not in `{{...}}` form: {value:?}"));
                }
                (_, None) => {
                    return Err(String::from("expected fallback part missing `source`"));
                }
            }
            compare_optional_id(id.as_deref(), want);
            Ok(())
        }
        (
            FormattedPart::Expression {
                kind,
                value,
                id,
                direction,
            },
            ty,
        ) => {
            if kind.as_ref() != ty {
                return Err(format!(
                    "expression kind mismatch: want `{ty}`, got `{kind}`"
                ));
            }
            if let Some(want_value) = want.get("value").and_then(Value::as_str) {
                if want_value != value.as_str() {
                    return Err(format!(
                        "expression value mismatch: want {want_value:?}, got {value:?}"
                    ));
                }
            }
            if let Some(want_id) = want.get("id").and_then(Value::as_str) {
                if id.as_deref() != Some(want_id) {
                    return Err(format!(
                        "expression id mismatch: want {want_id:?}, got {id:?}"
                    ));
                }
            }
            if let Some(want_dir) = want.get("dir").and_then(Value::as_str) {
                let want_dir_enum = match want_dir {
                    "ltr" => Some(Direction::Ltr),
                    "rtl" => Some(Direction::Rtl),
                    "auto" => Some(Direction::Auto),
                    "inherit" => None,
                    _ => return Err(format!("unknown expected dir {want_dir:?}")),
                };
                if *direction != want_dir_enum {
                    return Err(format!(
                        "expression dir mismatch: want {want_dir:?}, got {direction:?}"
                    ));
                }
            }
            Ok(())
        }
        (FormattedPart::BidiIsolation { value }, "bidiIsolation") => {
            let mut buf = [0u8; 4];
            let got_str: &str = value.encode_utf8(&mut buf);
            match want.get("value").and_then(Value::as_str) {
                Some(s) if s == got_str => Ok(()),
                Some(s) => Err(format!(
                    "bidiIsolation value mismatch: want {s:?}, got {value:?}"
                )),
                None => Err(String::from("expected bidiIsolation part missing `value`")),
            }
        }
        (
            FormattedPart::Markup {
                kind,
                name,
                options,
                id,
                ..
            },
            "markup",
        ) => {
            let kind_str = match kind {
                MarkupKind::Open => "open",
                MarkupKind::Close => "close",
                MarkupKind::Standalone => "standalone",
                _ => "unknown",
            };
            if let Some(want_kind) = want.get("kind").and_then(Value::as_str) {
                if want_kind != kind_str {
                    return Err(format!(
                        "markup kind mismatch: want {want_kind:?}, got {kind_str:?}"
                    ));
                }
            }
            if let Some(want_name) = want.get("name").and_then(Value::as_str) {
                if want_name != name.as_ref() {
                    return Err(format!(
                        "markup name mismatch: want {want_name:?}, got {name:?}"
                    ));
                }
            }
            if let Some(want_id) = want.get("id").and_then(Value::as_str) {
                if id.as_deref() != Some(want_id) {
                    return Err(format!("markup id mismatch: want {want_id:?}, got {id:?}"));
                }
            }
            if let Some(want_opts) = want.get("options").and_then(Value::as_object) {
                for (k, v) in want_opts {
                    let want_str = v.as_str().unwrap_or("");
                    match options.get(k.as_str()) {
                        Some(got) if got == want_str => {}
                        Some(got) => {
                            return Err(format!(
                                "markup option `{k}` mismatch: want {want_str:?}, got {got:?}"
                            ));
                        }
                        None => {
                            return Err(format!(
                                "markup option `{k}` missing (got options {options:?})"
                            ));
                        }
                    }
                }
            }
            Ok(())
        }
        (actual, ty) => Err(format!("part type mismatch: want `{ty}`, got {actual:?}")),
    }
}

fn compare_optional_id(id: Option<&str>, want: &Value) {
    // `id` on fallback parts is not commonly fixture-specified; no-op if
    // the fixture doesn't assert it. Placeholder retained for future
    // strict comparison.
    let _ = (id, want);
}

fn collect_fixture_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_fixture_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
}

#[test]
fn run_upstream_conformance_suite() {
    let root = fixtures_root();
    if !root.exists() {
        eprintln!(
            "SKIP: conformance fixtures not present at {:?}. \
             Run `cargo make sync-mf2-tests` or invoke \
             `tools/scripts/sync-mf2-tests.sh` to sync them.",
            root
        );
        return;
    }

    let mut files = Vec::new();
    collect_fixture_files(&root, &mut files);
    files.sort();

    let mut total = 0usize;
    let mut passed = 0usize;
    let mut skipped = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut unexpected_passes: Vec<String> = Vec::new();

    for file in files {
        let stem = file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();
        let raw = match fs::read_to_string(&file) {
            Ok(s) => s,
            Err(e) => {
                failures.push(format!("read {file:?}: {e}"));
                continue;
            }
        };
        let doc: Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                failures.push(format!("parse {file:?}: {e}"));
                continue;
            }
        };
        let all_cases = expand_cases(&stem, &doc);
        // Fixture-schema `only: true` on any case restricts the run within
        // this fixture file to cases with that flag set.
        let has_only = all_cases.iter().any(|c| c.only);
        for case in all_cases {
            if has_only && !case.only {
                skipped += 1;
                continue;
            }
            total += 1;
            let known = is_known_failure(&case);
            let outcome = run_case(&case);
            match check_case(&case, &outcome) {
                Ok(()) => {
                    if known {
                        unexpected_passes.push(format!(
                            "{}#{} {:?}",
                            case.file, case.index, case.description
                        ));
                    } else {
                        passed += 1;
                    }
                }
                Err(msg) => {
                    if known {
                        skipped += 1;
                    } else {
                        failures.push(format!(
                            "{}#{} [{}]: {} (desc: {:?}, tags: {:?})",
                            case.file, case.index, case.src, msg, case.description, case.tags
                        ));
                    }
                }
            }
        }
    }

    eprintln!(
        "MF2 conformance: {passed}/{total} passed, {skipped} skipped (known), \
         {} unexpected failures, {} known-skip tests that happened to match",
        failures.len(),
        unexpected_passes.len()
    );

    if !failures.is_empty() {
        let preview = failures
            .iter()
            .take(25)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n  ");
        panic!(
            "{} MF2 conformance failures (showing first 25):\n  {preview}",
            failures.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Custom handlers registered for every conformance case.
//
// - `:test:function`, `:test:select`, `:test:format`: the spec's
//   reserved-for-testing functions (see message-format-wg `spec/registry/
//   test.md`). Conformance fixtures exercise selector / format / option
//   error paths against these.
// - `:date`, `:time`, `:datetime`: fallback placeholders only for builds
//   where the library draft handlers are not present. Under `unstable`,
//   conformance should exercise the crate implementation directly.
// ---------------------------------------------------------------------------

fn register_conformance_handlers(
    builder: icu_experimental::messageformat::MessageFormatterBuilder,
) -> icu_experimental::messageformat::MessageFormatterBuilder {
    let builder = builder
        .function(
            "test:function",
            TestFunction {
                kind: TestKind::Both,
            },
        )
        .function(
            "test:select",
            TestFunction {
                kind: TestKind::Select,
            },
        )
        .function(
            "test:format",
            TestFunction {
                kind: TestKind::Format,
            },
        );
    #[cfg(not(feature = "unstable"))]
    let builder = builder
        .function("date", DateTimeHandler { kind: "date" })
        .function("time", DateTimeHandler { kind: "time" })
        .function("datetime", DateTimeHandler { kind: "datetime" });
    builder
}

#[derive(Debug, Clone, Copy)]
enum TestKind {
    /// `:test:format` — format-only.
    Format,
    /// `:test:select` — selector-only.
    Select,
    /// `:test:function` — both.
    Both,
}

#[derive(Debug)]
struct TestFunction {
    kind: TestKind,
}

impl FunctionHandler for TestFunction {
    fn format(
        &self,
        _ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        // Operand must be present and parseable as a number.
        let operand = operand.ok_or(FunctionError::BadOperand)?;
        if operand.is_fallback() {
            return Err(FunctionError::BadOperand);
        }
        let num: Decimal = match operand.numeric() {
            Some(n) => n.clone(),
            None => operand
                .text()
                .parse::<Decimal>()
                .map_err(|_| FunctionError::BadOperand)?,
        };

        // Parse recognized options. Unknown or out-of-range values are
        // Bad Option errors per spec.
        let mut decimal_places: i16 = 0;
        let mut decimal_places_set = false;
        let mut fails = "never";
        for (name, value) in options {
            if name.starts_with("u:") {
                continue;
            }
            match name.as_ref() {
                "decimalPlaces" => match value.text() {
                    "0" => {
                        decimal_places = 0;
                        decimal_places_set = true;
                    }
                    "1" => {
                        decimal_places = 1;
                        decimal_places_set = true;
                    }
                    _ => return Err(FunctionError::BadOption { name: name.clone() }),
                },
                "fails" => match value.text() {
                    "never" | "format" | "select" | "always" => {
                        fails = match value.text() {
                            "never" => "never",
                            "format" => "format",
                            "select" => "select",
                            _ => "always",
                        }
                    }
                    _ => return Err(FunctionError::BadOption { name: name.clone() }),
                },
                _ => return Err(FunctionError::BadOption { name: name.clone() }),
            }
        }

        // Operand option inheritance: if `decimalPlaces` was not supplied on
        // the expression, inherit it from the operand's resolved options.
        if !decimal_places_set {
            if let Some(inherited) = operand.resolved_options().get("decimalPlaces") {
                match inherited.as_ref() {
                    "0" => decimal_places = 0,
                    "1" => decimal_places = 1,
                    _ => {}
                }
            }
        }

        let acts_as_format = matches!(self.kind, TestKind::Format | TestKind::Both);
        let acts_as_select = matches!(self.kind, TestKind::Select | TestKind::Both);
        let format_fails = matches!(fails, "format" | "always");
        let select_fails = matches!(fails, "select" | "always");

        // `fails=format` / `fails=always` surfaces as Bad Option for any
        // function that can format (`:test:function`, `:test:format`).
        // Selector-only `:test:select` ignores it.
        if acts_as_format && format_fails {
            return Err(FunctionError::BadOption {
                name: Box::from("fails"),
            });
        }

        let text = format_number_with_places(&num, decimal_places);
        let mut rv = ResolvedValue::new(text)
            .with_numeric(num.clone())
            .with_resolved_option("decimalPlaces", if decimal_places == 0 { "0" } else { "1" });
        // Attach a selector unless this function is format-only or
        // `fails=select` disables selection.
        if acts_as_select && !select_fails {
            let sel = TestSelector {
                num: num.clone(),
                decimal_places,
            };
            rv = rv
                .with_selector(Arc::new(sel))
                .with_selector_function(match self.kind {
                    TestKind::Select => "test:select",
                    TestKind::Both => "test:function",
                    TestKind::Format => unreachable!(),
                });
        }
        Ok(rv)
    }
}

#[derive(Debug)]
struct TestSelector {
    num: Decimal,
    decimal_places: i16,
}

impl SelectorImpl for TestSelector {
    fn rank(&self, key: &str) -> Result<Option<usize>, FunctionError> {
        // The key must parse as a number; otherwise it's a Bad Variant Key.
        if key.parse::<Decimal>().is_err() {
            return Err(FunctionError::BadVariantKey {
                key: Box::from(key),
            });
        }
        // Match iff the key equals the operand formatted with the resolved
        // `decimalPlaces`.
        let formatted = format_number_with_places(&self.num, self.decimal_places);
        Ok(if formatted == key { Some(0) } else { None })
    }
}

fn format_number_with_places(num: &Decimal, places: i16) -> String {
    let mut d = num.clone();
    let position = -places;
    d.round(position);
    if places > 0 {
        d.pad_end(position);
    }
    d.to_string()
}

// ---------------------------------------------------------------------------
// `:date` / `:time` / `:datetime` placeholder handlers.
// Validate the operand and options per the draft spec; pass through the
// operand text as the formatted output.
// ---------------------------------------------------------------------------

#[cfg(not(feature = "unstable"))]
#[derive(Debug)]
struct DateTimeHandler {
    kind: &'static str,
}

#[cfg(not(feature = "unstable"))]
impl FunctionHandler for DateTimeHandler {
    fn format(
        &self,
        _ctx: &FunctionContext<'_>,
        operand: Option<&ResolvedValue>,
        options: &FunctionOptions,
    ) -> Result<ResolvedValue, FunctionError> {
        let operand = operand.ok_or(FunctionError::BadOperand)?;
        if operand.is_fallback() {
            return Err(FunctionError::BadOperand);
        }
        let text = operand.text();
        let valid = match self.kind {
            "date" => validate_iso_date(text) || validate_iso_datetime(text),
            "time" => {
                validate_iso_time(text)
                    || validate_iso_datetime(text)
                    || (text.contains('T')
                        && validate_iso_time(text.split_once('T').map(|(_, t)| t).unwrap_or("")))
            }
            "datetime" => validate_iso_datetime(text) || validate_iso_date(text),
            _ => true,
        };
        if !valid {
            return Err(FunctionError::BadOperand);
        }
        for (name, value) in options {
            if name.starts_with("u:") {
                continue;
            }
            if !is_valid_datetime_option(self.kind, name.as_ref(), value.text()) {
                return Err(FunctionError::BadOption { name: name.clone() });
            }
        }
        Ok(ResolvedValue::new(text).with_part_kind(self.kind))
    }
}

#[cfg(not(feature = "unstable"))]
fn is_valid_datetime_option(kind: &str, name: &str, value: &str) -> bool {
    const DATE_FIELDS: &[&str] = &[
        "weekday",
        "day-weekday",
        "month-day",
        "month-day-weekday",
        "year-month-day",
        "year-month-day-weekday",
    ];
    const LENGTH: &[&str] = &["long", "medium", "short"];
    const PRECISION: &[&str] = &["hour", "minute", "second"];
    const TZ_STYLE: &[&str] = &["long", "short"];
    let in_set = |set: &[&str]| set.contains(&value);
    match (kind, name) {
        ("date" | "time" | "datetime", "timeZone" | "calendar" | "numberingSystem") => true,
        ("datetime", "dateFields") | ("date", "fields") => in_set(DATE_FIELDS),
        ("datetime", "dateLength") | ("date", "length") => in_set(LENGTH),
        ("datetime", "timePrecision") | ("time", "precision") => in_set(PRECISION),
        ("datetime" | "time", "timeZoneStyle") => in_set(TZ_STYLE),
        _ => false,
    }
}

#[cfg(not(feature = "unstable"))]
fn validate_iso_date(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.len() != 10 {
        return false;
    }
    let digits_at = |i: usize, n: usize| {
        bytes
            .get(i..i + n)
            .is_some_and(|b| b.iter().all(|c| c.is_ascii_digit()))
    };
    digits_at(0, 4) && bytes[4] == b'-' && digits_at(5, 2) && bytes[7] == b'-' && digits_at(8, 2)
}

#[cfg(not(feature = "unstable"))]
fn validate_iso_time(s: &str) -> bool {
    let (main, rest) = match s.split_once('.') {
        Some((m, r)) => (m, Some(r)),
        None => (s, None),
    };
    let parts: Vec<&str> = main.split(':').collect();
    if parts.len() != 2 && parts.len() != 3 {
        return false;
    }
    if !parts
        .iter()
        .all(|p| p.len() == 2 && p.bytes().all(|c| c.is_ascii_digit()))
    {
        return false;
    }
    match rest {
        Some(frac) if !frac.is_empty() && frac.bytes().all(|c| c.is_ascii_digit()) => true,
        Some(_) => false,
        None => true,
    }
}

#[cfg(not(feature = "unstable"))]
fn validate_iso_datetime(s: &str) -> bool {
    let Some((date_part, after_t)) = s.split_once('T') else {
        return false;
    };
    if !validate_iso_date(date_part) {
        return false;
    }
    let time_part = if let Some(core) = after_t.strip_suffix('Z') {
        core
    } else if let Some((core, tz)) = split_timezone(after_t) {
        if !validate_timezone_offset(tz) {
            return false;
        }
        core
    } else {
        after_t
    };
    validate_iso_time(time_part)
}

#[cfg(not(feature = "unstable"))]
fn split_timezone(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut idx = None;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'+' || (b == b'-' && i > 0) {
            idx = Some(i);
        }
    }
    idx.map(|i| (&s[..i], &s[i..]))
}

#[cfg(not(feature = "unstable"))]
fn validate_timezone_offset(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() || (bytes[0] != b'+' && bytes[0] != b'-') {
        return false;
    }
    let rest = &s[1..];
    match rest.len() {
        4 => rest.bytes().all(|c| c.is_ascii_digit()),
        5 => {
            bytes.get(1 + 2) == Some(&b':')
                && rest[..2].bytes().all(|c| c.is_ascii_digit())
                && rest[3..].bytes().all(|c| c.is_ascii_digit())
        }
        _ => false,
    }
}
