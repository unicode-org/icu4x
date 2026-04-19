// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Bidirectional-text support for `MessageFormat` 2.
//!
//! Two concerns live here:
//!
//! - The [`Direction`] enum — what `u:dir` resolves to for a placeholder,
//!   and what the message base directionality is.
//! - The [`BidiIsolation`] enum — selects the isolation strategy for
//!   formatter output.
//! - An internal `isolates_for` helper — maps a (base, placeholder)
//!   direction pair to the Unicode isolate characters that the formatter
//!   wraps the placeholder's text in when bidi isolation is enabled.
//!
//! The spec defines four `u:dir` values: `ltr`, `rtl`, `auto`, `inherit`.
//! We represent all four plus a "none" sentinel for placeholders that
//! carry no direction annotation at all (the default for non-markup
//! expressions).

/// Selects a bidi-isolation strategy for formatter output.
///
/// Spec [formatting.md] defines `"default"` and `"none"` today and leaves
/// room for additional named strategies. This enum mirrors that option set
/// so callers can opt into new strategies without source breakage if/when
/// the spec defines them. The builder accepts `bool` for backwards
/// compatibility (`true` → [`BidiIsolation::Default`], `false` →
/// [`BidiIsolation::None`]).
///
/// [formatting.md]: https://github.com/unicode-org/message-format-wg/blob/main/spec/formatting.md
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BidiIsolation {
    /// Wrap every resolved expression in the spec's default isolation
    /// characters (FSI/LRI/RLI + PDI). The active default strategy.
    Default,
    /// Do not emit isolation characters. Used when the caller is producing
    /// output that will be isolated externally or when isolates would
    /// interfere with downstream layout.
    None,
}

impl BidiIsolation {
    /// True when this setting would emit isolation characters. Equivalent
    /// to `matches!(self, BidiIsolation::Default)`; exposed so callers can
    /// query the formatter's configuration without pattern-matching the
    /// enum.
    pub fn is_enabled(self) -> bool {
        matches!(self, BidiIsolation::Default)
    }
}

impl Default for BidiIsolation {
    fn default() -> Self {
        BidiIsolation::Default
    }
}

impl From<bool> for BidiIsolation {
    fn from(on: bool) -> Self {
        if on {
            BidiIsolation::Default
        } else {
            BidiIsolation::None
        }
    }
}

/// Directionality of a resolved expression or the message as a whole.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Left-to-right.
    Ltr,
    /// Right-to-left.
    Rtl,
    /// Auto-detect from the first strong character. Always isolated by FSI+PDI.
    Auto,
    /// Inherit the enclosing direction (from an explicit `u:dir=inherit`).
    /// No per-placeholder isolation is applied for this value.
    Inherit,
}

impl Direction {
    /// Parse a `u:dir` option value per the spec.
    pub(crate) fn from_option(s: &str) -> Option<Self> {
        match s {
            "ltr" => Some(Direction::Ltr),
            "rtl" => Some(Direction::Rtl),
            "auto" => Some(Direction::Auto),
            "inherit" => Some(Direction::Inherit),
            _ => None,
        }
    }
}

/// Returns `(prefix, suffix)` — the bidi-isolation characters to wrap a
/// placeholder's text in given the message base direction, the placeholder's
/// own direction (when it has one), and whether a `u:dir` option was set
/// explicitly.
///
/// Implements the _Default Bidi Strategy_ from formatting.md:839-875:
/// - `dir=Ltr`: LRI+PDI when msgdir≠Ltr or isolate, else nothing.
/// - `dir=Rtl`: RLI+PDI (always).
/// - `dir=Auto`: FSI+PDI (always).
/// - `dir=Inherit`: no wrapping (explicit `u:dir=inherit`).
/// - `dir=None` (unknown, no u:dir): FSI+PDI (spec's "dir is unknown" path).
pub(crate) fn isolates_for(
    base: Direction,
    placeholder: Option<Direction>,
    isolate: bool,
) -> (&'static str, &'static str) {
    const LRI: &str = "\u{2066}";
    const RLI: &str = "\u{2067}";
    const FSI: &str = "\u{2068}";
    const PDI: &str = "\u{2069}";

    match placeholder {
        Some(Direction::Inherit) => ("", ""),
        Some(Direction::Ltr) => {
            if !isolate && matches!(base, Direction::Ltr) {
                ("", "")
            } else {
                (LRI, PDI)
            }
        }
        Some(Direction::Rtl) => (RLI, PDI),
        Some(Direction::Auto) => (FSI, PDI),
        // `dir` is unknown — spec requires FSI+PDI.
        None => (FSI, PDI),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u_dir_values() {
        assert_eq!(Direction::from_option("ltr"), Some(Direction::Ltr));
        assert_eq!(Direction::from_option("rtl"), Some(Direction::Rtl));
        assert_eq!(Direction::from_option("auto"), Some(Direction::Auto));
        assert_eq!(Direction::from_option("inherit"), Some(Direction::Inherit));
        assert_eq!(Direction::from_option("sideways"), None);
    }

    #[test]
    fn inherit_never_isolates() {
        assert_eq!(
            isolates_for(Direction::Ltr, Some(Direction::Inherit), true),
            ("", "")
        );
        assert_eq!(
            isolates_for(Direction::Rtl, Some(Direction::Inherit), true),
            ("", "")
        );
    }

    #[test]
    fn explicit_ltr_isolates() {
        assert_eq!(
            isolates_for(Direction::Ltr, Some(Direction::Ltr), true),
            ("\u{2066}", "\u{2069}")
        );
        assert_eq!(
            isolates_for(Direction::Rtl, Some(Direction::Rtl), true),
            ("\u{2067}", "\u{2069}")
        );
    }

    #[test]
    fn ltr_matching_base_without_isolate_flag_skips_wrap() {
        assert_eq!(
            isolates_for(Direction::Ltr, Some(Direction::Ltr), false),
            ("", "")
        );
    }

    #[test]
    fn unknown_direction_gets_fsi() {
        let (p, s) = isolates_for(Direction::Ltr, None, false);
        assert_eq!(p, "\u{2068}");
        assert_eq!(s, "\u{2069}");
    }

    #[test]
    fn auto_always_fsi() {
        let (p, s) = isolates_for(Direction::Ltr, Some(Direction::Auto), true);
        assert_eq!(p, "\u{2068}");
        assert_eq!(s, "\u{2069}");
    }

    #[test]
    fn mismatch_direction_picks_explicit_isolate() {
        assert_eq!(
            isolates_for(Direction::Ltr, Some(Direction::Rtl), true),
            ("\u{2067}", "\u{2069}")
        );
        assert_eq!(
            isolates_for(Direction::Rtl, Some(Direction::Ltr), true),
            ("\u{2066}", "\u{2069}")
        );
    }
}
