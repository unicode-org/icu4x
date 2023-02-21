// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    error::DateTimeError,
    fields::{Field, FieldSymbol, Week},
    input::{DateTimeInput, LocalizedDateTimeInput},
    pattern::{runtime::Pattern, PatternError, PatternItem},
};
use either::Either;
use icu_plurals::{PluralCategory, PluralRules};
use icu_provider::{yoke, zerofrom};

/// A collection of plural variants of a pattern.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_datetime::pattern::runtime),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // part of data struct
pub struct PluralPattern<'data> {
    /// The field that 'variants' are predicated on.
    pub pivot_field: Week,

    #[cfg_attr(feature = "serde", serde(borrow))]
    pub zero: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub one: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub two: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub few: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub many: Option<Pattern<'data>>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub other: Pattern<'data>,
}

impl<'data> PluralPattern<'data> {
    pub fn new(pattern: Pattern<'data>) -> Result<Self, PatternError> {
        let pivot_field = pattern
            .items
            .iter()
            .find_map(|pattern_item| match pattern_item {
                PatternItem::Field(Field {
                    symbol: FieldSymbol::Week(w),
                    ..
                }) => Some(w),
                _ => None,
            })
            .ok_or(PatternError::UnsupportedPluralPivot)?;

        Ok(Self {
            pivot_field,
            zero: None,
            one: None,
            two: None,
            few: None,
            many: None,
            other: pattern,
        })
    }

    /// Returns which week field determines the [icu_plurals::PluralCategory] used to select a pattern variant for a given date.
    pub fn pivot_field(&self) -> Week {
        self.pivot_field
    }

    pub fn maybe_set_variant(&mut self, category: PluralCategory, pattern: Pattern<'data>) {
        if pattern == self.other {
            return;
        }
        match category {
            PluralCategory::Zero => self.zero = Some(pattern),
            PluralCategory::One => self.one = Some(pattern),
            PluralCategory::Two => self.two = Some(pattern),
            PluralCategory::Few => self.few = Some(pattern),
            PluralCategory::Many => self.many = Some(pattern),
            PluralCategory::Other => unreachable!("You can't override other"),
        }
    }

    fn variant(&self, category: PluralCategory) -> &Pattern<'data> {
        let variant = match category {
            PluralCategory::Zero => &self.zero,
            PluralCategory::One => &self.one,
            PluralCategory::Two => &self.two,
            PluralCategory::Few => &self.few,
            PluralCategory::Many => &self.many,
            PluralCategory::Other => return &self.other,
        };
        variant.as_ref().unwrap_or(&self.other)
    }

    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern<'data>> {
        PluralCategory::all().filter_map(move |cat| match cat {
            PluralCategory::Zero => self.zero.as_ref(),
            PluralCategory::One => self.one.as_ref(),
            PluralCategory::Two => self.two.as_ref(),
            PluralCategory::Few => self.few.as_ref(),
            PluralCategory::Many => self.many.as_ref(),
            PluralCategory::Other => Some(&self.other),
        })
    }

    pub fn for_each_mut<F>(&mut self, f: &F)
    where
        F: Fn(&mut Pattern<'data>),
    {
        self.zero.iter_mut().for_each(f);
        self.one.iter_mut().for_each(f);
        self.two.iter_mut().for_each(f);
        self.few.iter_mut().for_each(f);
        self.many.iter_mut().for_each(f);
        f(&mut self.other);
    }

    pub fn into_owned(self) -> PluralPattern<'static> {
        PluralPattern {
            pivot_field: self.pivot_field,
            zero: self.zero.map(|p| p.into_owned()),
            one: self.one.map(|p| p.into_owned()),
            two: self.two.map(|p| p.into_owned()),
            few: self.few.map(|p| p.into_owned()),
            many: self.many.map(|p| p.into_owned()),
            other: self.other.into_owned(),
        }
    }
}

/// Either a [`Pattern`] single pattern or a [`PluralPattern`] collection of
/// patterns when there are plural variants.
///
/// Currently, the plural forms are only based on the week number.
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[allow(clippy::large_enum_variant)]
#[allow(clippy::exhaustive_enums)] // this type is stable
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_datetime::pattern::runtime),
)]
pub enum PatternPlurals<'data> {
    /// A collection of pattern variants for when plurals differ.
    MultipleVariants(PluralPattern<'data>),
    /// A single pattern.
    SinglePattern(Pattern<'data>),
}

impl<'data> PatternPlurals<'data> {
    // Returns the pattern to use according to loc_datetime and ordinal_rules.
    pub fn select<T>(
        &self,
        loc_datetime: &impl LocalizedDateTimeInput<T>,
        ordinal_rules: Option<&PluralRules>,
    ) -> Result<&Pattern, DateTimeError>
    where
        T: DateTimeInput,
    {
        match self {
            Self::SinglePattern(pattern) => Ok(pattern),
            Self::MultipleVariants(plural_pattern) => {
                let week_number = match plural_pattern.pivot_field() {
                    Week::WeekOfMonth => loc_datetime.week_of_month()?.0,
                    Week::WeekOfYear => loc_datetime.week_of_year()?.1 .0,
                };
                let category = ordinal_rules
                    .ok_or(DateTimeError::MissingOrdinalRules)?
                    .category_for(week_number);
                Ok(plural_pattern.variant(category))
            }
        }
    }

    pub fn into_owned(self) -> PatternPlurals<'static> {
        match self {
            Self::SinglePattern(pattern) => PatternPlurals::SinglePattern(pattern.into_owned()),
            Self::MultipleVariants(plural_pattern) => {
                PatternPlurals::MultipleVariants(plural_pattern.into_owned())
            }
        }
    }

    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern<'data>> {
        match self {
            Self::SinglePattern(pattern) => Either::Left(core::iter::once(pattern)),
            Self::MultipleVariants(plural_pattern) => Either::Right(plural_pattern.patterns_iter()),
        }
    }

    pub fn for_each_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Pattern<'data>),
    {
        match self {
            Self::SinglePattern(pattern) => f(pattern),
            Self::MultipleVariants(variants) => variants.for_each_mut(&f),
        }
    }

    pub fn expect_pattern(self, msg: &str) -> Pattern<'data> {
        match self {
            Self::SinglePattern(pattern) => pattern,

            Self::MultipleVariants(patterns) => {
                // Potentially change to log::warn! in #2648
                debug_assert!(
                    false,
                    "expect_pattern called with bad data (falling back to `other` pattern): {msg}"
                );
                patterns.other
            }
        }
    }

    // Removes redundant patterns & transforms singleton [PatternPlurals::MultipleVariants] into a [PatternPlurals::SinglePattern].
    pub fn normalize(&mut self) {
        if let Self::MultipleVariants(patterns) = self {
            if patterns.patterns_iter().count() == 1 {
                *self = Self::SinglePattern(core::mem::take(&mut patterns.other));
            }
        }
    }
}

impl<'data> From<Pattern<'data>> for PatternPlurals<'data> {
    fn from(pattern: Pattern<'data>) -> Self {
        Self::SinglePattern(pattern)
    }
}

impl<'data> From<PluralPattern<'data>> for PatternPlurals<'data> {
    fn from(pattern: PluralPattern<'data>) -> Self {
        Self::MultipleVariants(pattern)
    }
}

impl Default for PatternPlurals<'_> {
    fn default() -> Self {
        PatternPlurals::SinglePattern(Pattern::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_plural_pattern() {
        let red_pattern: Pattern = "'red' w".parse().unwrap();
        let blue_pattern: Pattern = "'blue' w".parse().unwrap();
        let mut patterns =
            PluralPattern::new(blue_pattern.clone()).expect("PluralPattern::new failed");
        patterns.maybe_set_variant(PluralCategory::Zero, red_pattern.clone());
        patterns.maybe_set_variant(PluralCategory::One, blue_pattern.clone());
        patterns.maybe_set_variant(PluralCategory::Two, red_pattern.clone());
        patterns.maybe_set_variant(PluralCategory::Few, red_pattern.clone());
        patterns.maybe_set_variant(PluralCategory::Many, blue_pattern.clone());

        assert_eq!(patterns.pivot_field, Week::WeekOfYear);
        assert_eq!(patterns.zero, Some(red_pattern.clone()));
        assert_eq!(patterns.one, None); // duplicate `other
        assert_eq!(patterns.two, Some(red_pattern));
        assert_eq!(patterns.other, blue_pattern);
    }

    #[test]
    fn normalize_pattern_plurals_switches_singletons_to_single_pattern() {
        let pattern: Pattern = "'red' w".parse().unwrap();
        let patterns = PluralPattern::new(pattern.clone()).expect("PluralPattern::new failed");
        let mut plural_patterns: PatternPlurals = PatternPlurals::MultipleVariants(patterns);

        plural_patterns.normalize();

        assert_eq!(plural_patterns, PatternPlurals::SinglePattern(pattern));
    }
}
