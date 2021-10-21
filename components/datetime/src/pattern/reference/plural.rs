// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    date::{DateTimeInput, LocalizedDateTimeInput},
    error::DateTimeFormatError,
    fields::{Field, FieldSymbol, Week},
    pattern::{reference::Pattern, PatternError, PatternItem},
};
use either::Either;
use icu_plurals::{PluralCategory, PluralRules};

/// A collection of plural variants of a pattern.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PluralPattern {
    /// The field that 'variants' are predicated on.
    pivot_field: Week,

    pub(crate) zero: Option<Pattern>,
    pub(crate) one: Option<Pattern>,
    pub(crate) two: Option<Pattern>,
    pub(crate) few: Option<Pattern>,
    pub(crate) many: Option<Pattern>,
    pub(crate) other: Pattern,
}

impl PluralPattern {
    pub fn new(pattern: Pattern) -> Result<PluralPattern, PatternError> {
        let pivot_field = pattern
            .items()
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
            pivot_field: *pivot_field,
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

    pub fn maybe_set_variant(&mut self, category: PluralCategory, pattern: Pattern) {
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

    fn variant(&self, category: PluralCategory) -> &Pattern {
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

    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern> {
        PluralCategory::all().filter_map(move |cat| match cat {
            PluralCategory::Zero => self.zero.as_ref(),
            PluralCategory::One => self.one.as_ref(),
            PluralCategory::Two => self.two.as_ref(),
            PluralCategory::Few => self.few.as_ref(),
            PluralCategory::Many => self.many.as_ref(),
            PluralCategory::Other => Some(&self.other),
        })
    }

    pub fn for_each_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Pattern),
    {
        PluralCategory::all().for_each(|cat| {
            let field = match cat {
                PluralCategory::Zero => &mut self.zero,
                PluralCategory::One => &mut self.one,
                PluralCategory::Two => &mut self.two,
                PluralCategory::Few => &mut self.few,
                PluralCategory::Many => &mut self.many,
                PluralCategory::Other => {
                    f(&mut self.other);
                    return;
                }
            };
            if let Some(p) = field.as_mut() {
                f(p)
            }
        });
    }
}

/// Either a single Pattern or a collection of pattern when there are plural variants.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum PatternPlurals {
    /// A collection of pattern variants for when plurals differ.
    MultipleVariants(PluralPattern),
    /// A single pattern.
    SinglePattern(Pattern),
}

impl PatternPlurals {
    // Returns the pattern to use according to loc_datetime and ordinal_rules.
    pub fn select<T>(
        &self,
        loc_datetime: &impl LocalizedDateTimeInput<T>,
        ordinal_rules: Option<&PluralRules>,
    ) -> Result<&Pattern, DateTimeFormatError>
    where
        T: DateTimeInput,
    {
        match self {
            PatternPlurals::SinglePattern(pattern) => Ok(pattern),
            PatternPlurals::MultipleVariants(plural_pattern) => {
                let week_number = match plural_pattern.pivot_field() {
                    Week::WeekOfMonth => loc_datetime.week_of_month().0,
                    Week::WeekOfYear => loc_datetime.week_of_year()?.0,
                };
                let category = ordinal_rules
                    .expect("ordinal_rules must be set with PatternPlurals::MultipleVariants")
                    .select(week_number);
                Ok(plural_pattern.variant(category))
            }
        }
    }

    pub fn patterns_iter(&self) -> impl Iterator<Item = &Pattern> {
        match self {
            PatternPlurals::SinglePattern(pattern) => Either::Left(core::iter::once(pattern)),
            PatternPlurals::MultipleVariants(plural_pattern) => {
                Either::Right(plural_pattern.patterns_iter())
            }
        }
    }

    pub fn for_each_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Pattern),
    {
        match self {
            Self::SinglePattern(pattern) => f(pattern),
            Self::MultipleVariants(variants) => variants.for_each_mut(f),
        }
    }

    pub fn expect_pattern(self, msg: &str) -> Pattern {
        match self {
            PatternPlurals::SinglePattern(pattern) => pattern,
            _ => panic!("expect_pattern failed: {}", msg),
        }
    }

    // Removes redundant patterns & transforms singleton [PatternPlurals::MultipleVariants] into a [PatternPlurals::SinglePattern].
    pub fn normalize(&mut self) {
        if let PatternPlurals::MultipleVariants(patterns) = self {
            if patterns.patterns_iter().count() == 1 {
                *self = PatternPlurals::SinglePattern(core::mem::take(&mut patterns.other));
            }
        }
    }
}

impl From<Pattern> for PatternPlurals {
    fn from(pattern: Pattern) -> PatternPlurals {
        PatternPlurals::SinglePattern(pattern)
    }
}

impl From<PluralPattern> for PatternPlurals {
    fn from(pattern: PluralPattern) -> PatternPlurals {
        PatternPlurals::MultipleVariants(pattern)
    }
}

impl Default for PatternPlurals {
    fn default() -> Self {
        PatternPlurals::SinglePattern(Pattern::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_plural_pattern() {
        let red_pattern = Pattern::from_bytes("'red' w").unwrap();
        let blue_pattern = Pattern::from_bytes("'blue' w").unwrap();
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
        let pattern = Pattern::from_bytes("'red' w").unwrap();
        let patterns = PluralPattern::new(pattern.clone()).expect("PluralPattern::new failed");
        let mut plural_patterns: PatternPlurals = PatternPlurals::MultipleVariants(patterns);

        plural_patterns.normalize();

        assert_eq!(plural_patterns, PatternPlurals::SinglePattern(pattern));
    }
}
