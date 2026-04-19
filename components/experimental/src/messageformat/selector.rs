// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Pattern-selection algorithm and [`SelectorImpl`] trait.
//!
//! A [`ResolvedValue`](super::value::ResolvedValue) may carry a selector
//! (via [`ResolvedValue::with_selector`](super::value::ResolvedValue::with_selector))
//! if its producing function supports `.match`. The matcher asks each
//! selector to rank the literal keys of a
//! variant at that selector's position, then picks the variant with the
//! best lexicographic rank tuple. Catchall keys (`*`) always participate
//! but carry the lowest possible preference ([`usize::MAX`]).

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::ast::{Variant, VariantKey};
use super::error::{FormatError, FunctionError};

/// Interface a selectable [`super::ResolvedValue`] exposes to the matcher.
///
/// Lower `rank` means higher preference. `Ok(None)` means this key does not
/// match. `Err(FunctionError::BadVariantKey)` means the key is unacceptable
/// input to this selector (e.g. a non-numeric key for `:number`); the
/// variant is excluded and the error is surfaced to the caller.
/// `Send + Sync + Debug` lets selectors live behind `Arc` in cached
/// [`super::ResolvedValue`]s.
pub trait SelectorImpl: core::fmt::Debug + Send + Sync {
    /// Rank of `key` against this selector's resolved value.
    fn rank(&self, key: &str) -> Result<Option<usize>, FunctionError>;
}

/// Rank sentinel assigned to catchall keys (`*`). Greater than any
/// preference rank a selector can return.
pub(crate) const CATCHALL_RANK: usize = usize::MAX;

/// A selector paired with the function name that produced it. Used by the
/// formatter to attribute `BadVariantKey` errors to a specific function.
pub(crate) type NamedSelector = (Box<str>, Arc<dyn SelectorImpl>);

/// Run the best-match variant picker.
///
/// `selectors[i]` is `Some(impl)` when the i-th selector expression resolved
/// to a value that supports selection, or `None` otherwise (e.g. the
/// function didn't attach a selector, or resolution failed). When any
/// selector is `None`, only variants whose i-th key is `*` can match that
/// position — which in the well-formed case means the all-catchall variant
/// (guaranteed by the validator) wins.
///
/// Panics (in debug) only when `variants` is empty or does not contain an
/// all-catchall variant — both invariants are enforced by the validator.
pub(crate) fn pick_variant<'m>(
    selectors: &[Option<NamedSelector>],
    variants: &'m [Variant],
    errors: &mut Vec<FormatError>,
) -> &'m Variant {
    debug_assert!(!variants.is_empty(), "validator rejects empty matcher");

    // Compute rank tuple per variant; reject variants that can't match.
    // Selector errors (e.g. Bad Variant Key) are collected without
    // aborting — the variant is skipped and the error is surfaced.
    let mut best: Option<(Vec<usize>, &'m Variant)> = None;
    for v in variants {
        let Some(rank) = rank_variant(selectors, v, errors) else {
            continue;
        };
        match &best {
            Some((best_rank, _)) if rank_cmp(best_rank, &rank).is_le() => {}
            _ => best = Some((rank, v)),
        }
    }

    match best {
        Some((_, v)) => v,
        None => find_all_catchall(variants),
    }
}

fn rank_variant(
    selectors: &[Option<NamedSelector>],
    v: &Variant,
    errors: &mut Vec<FormatError>,
) -> Option<Vec<usize>> {
    let mut tuple = Vec::with_capacity(v.keys.len());
    for (i, key) in v.keys.iter().enumerate() {
        match key {
            VariantKey::Catchall(_) => tuple.push(CATCHALL_RANK),
            VariantKey::Literal(l) => {
                let named = selectors.get(i).and_then(|s| s.as_ref())?;
                match named.1.rank(&l.value) {
                    Ok(Some(r)) => tuple.push(r),
                    Ok(None) => return None,
                    Err(err) => {
                        errors.push(FormatError::FunctionError {
                            function: named.0.clone(),
                            error: err,
                        });
                        return None;
                    }
                }
            }
        }
    }
    Some(tuple)
}

fn rank_cmp(a: &[usize], b: &[usize]) -> core::cmp::Ordering {
    a.cmp(b)
}

fn find_all_catchall(variants: &[Variant]) -> &Variant {
    variants
        .iter()
        .find(|v| v.keys.iter().all(|k| matches!(k, VariantKey::Catchall(_))))
        .expect("validator guarantees an all-catchall variant")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messageformat::ast::{CatchallKey, Literal, Pattern, PatternElement};
    use alloc::boxed::Box;

    fn lit(s: &str) -> VariantKey {
        VariantKey::Literal(Literal::new(Box::from(s)))
    }
    fn star() -> VariantKey {
        VariantKey::Catchall(CatchallKey::default())
    }
    fn text_pattern(t: &str) -> Pattern {
        alloc::vec![PatternElement::Text(t.into())]
    }
    fn variant(keys: Vec<VariantKey>, label: &str) -> Variant {
        Variant {
            keys,
            value: text_pattern(label),
        }
    }

    /// A deterministic test-only selector: `rank(k)` = position of `k` in the
    /// preference list (lower = better), `None` if absent.
    #[derive(Debug)]
    struct FakeSelector {
        prefs: Vec<Box<str>>,
    }
    impl FakeSelector {
        fn new(prefs: &[&str]) -> Self {
            Self {
                prefs: prefs.iter().map(|s| Box::from(*s)).collect(),
            }
        }
    }
    impl SelectorImpl for FakeSelector {
        fn rank(&self, key: &str) -> Result<Option<usize>, FunctionError> {
            Ok(self.prefs.iter().position(|k| k.as_ref() == key))
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn named(sel: Arc<dyn SelectorImpl>) -> Option<NamedSelector> {
        Some((Box::from("fake"), sel))
    }

    #[test]
    fn picks_exact_match() {
        let sel = Arc::new(FakeSelector::new(&["1", "one"])) as Arc<dyn SelectorImpl>;
        let variants = alloc::vec![
            variant(alloc::vec![lit("1")], "exact-1"),
            variant(alloc::vec![lit("one")], "plural-one"),
            variant(alloc::vec![star()], "fallback"),
        ];
        let mut errors = Vec::new();
        let picked = pick_variant(&[named(sel)], &variants, &mut errors);
        let PatternElement::Text(t) = &picked.value[0] else {
            panic!()
        };
        assert_eq!(t, "exact-1");
        assert!(errors.is_empty());
    }

    #[test]
    fn falls_back_to_catchall() {
        let sel = Arc::new(FakeSelector::new(&["other"])) as Arc<dyn SelectorImpl>;
        let variants = alloc::vec![
            variant(alloc::vec![lit("1")], "one"),
            variant(alloc::vec![lit("2")], "two"),
            variant(alloc::vec![star()], "default"),
        ];
        let mut errors = Vec::new();
        let picked = pick_variant(&[named(sel)], &variants, &mut errors);
        let PatternElement::Text(t) = &picked.value[0] else {
            panic!()
        };
        assert_eq!(t, "default");
        assert!(errors.is_empty());
    }

    #[test]
    fn selector_failure_forces_catchall() {
        let variants = alloc::vec![
            variant(alloc::vec![lit("1")], "literal-1"),
            variant(alloc::vec![star()], "default"),
        ];
        let mut errors = Vec::new();
        let picked = pick_variant(&[None], &variants, &mut errors);
        let PatternElement::Text(t) = &picked.value[0] else {
            panic!()
        };
        assert_eq!(t, "default");
    }

    #[test]
    fn multi_selector_lexicographic() {
        // Two selectors; best match is the variant whose (rank_0, rank_1) tuple
        // sorts first.
        let s0 = Arc::new(FakeSelector::new(&["a"])) as Arc<dyn SelectorImpl>;
        let s1 = Arc::new(FakeSelector::new(&["x", "y"])) as Arc<dyn SelectorImpl>;
        let variants = alloc::vec![
            variant(alloc::vec![lit("a"), lit("y")], "a-y"),
            variant(alloc::vec![lit("a"), lit("x")], "a-x"),
            variant(alloc::vec![lit("a"), star()], "a-*"),
            variant(alloc::vec![star(), star()], "*-*"),
        ];
        let mut errors = Vec::new();
        let picked = pick_variant(&[named(s0), named(s1)], &variants, &mut errors);
        let PatternElement::Text(t) = &picked.value[0] else {
            panic!()
        };
        assert_eq!(t, "a-x", "lower rank_1 (`x` at 0) wins over `y` at 1");
    }

    #[test]
    fn catchall_lower_priority_than_literal() {
        // Two variants match a value; the literal one wins over catchall.
        let sel = Arc::new(FakeSelector::new(&["1"])) as Arc<dyn SelectorImpl>;
        let variants = alloc::vec![
            variant(alloc::vec![star()], "catchall"),
            variant(alloc::vec![lit("1")], "exact"),
        ];
        let mut errors = Vec::new();
        let picked = pick_variant(&[named(sel)], &variants, &mut errors);
        let PatternElement::Text(t) = &picked.value[0] else {
            panic!()
        };
        assert_eq!(t, "exact");
    }

    #[test]
    fn bad_variant_key_error_surfaced_and_variant_skipped() {
        // A selector that rejects "bad" with BadVariantKey — the matcher
        // must skip that variant and record the error.
        #[derive(Debug)]
        struct PickySelector;
        impl SelectorImpl for PickySelector {
            fn rank(&self, key: &str) -> Result<Option<usize>, FunctionError> {
                match key {
                    "good" => Ok(Some(0)),
                    "bad" => Err(FunctionError::BadVariantKey {
                        key: Box::from(key),
                    }),
                    _ => Ok(None),
                }
            }
        }
        let sel = Arc::new(PickySelector) as Arc<dyn SelectorImpl>;
        let variants = alloc::vec![
            variant(alloc::vec![lit("bad")], "never"),
            variant(alloc::vec![lit("good")], "match"),
            variant(alloc::vec![star()], "default"),
        ];
        let mut errors = Vec::new();
        let picked = pick_variant(&[named(sel)], &variants, &mut errors);
        let PatternElement::Text(t) = &picked.value[0] else {
            panic!()
        };
        assert_eq!(t, "match");
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            &errors[0],
            FormatError::FunctionError {
                function,
                error: FunctionError::BadVariantKey { key },
            } if function.as_ref() == "fake" && key.as_ref() == "bad"
        ));
    }
}
