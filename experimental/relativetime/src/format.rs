// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Write;

use fixed_decimal::{FixedDecimal, Sign};
use writeable::Writeable;

use crate::{
    options::{Numeric, RelativeTimeFormatterOptions},
    relativetime::RelativeTimeFormatter,
};

pub mod parts {
    use writeable::Part;

    /// The [`Part`] used by [`FormattedRelativeTime`](crate::FormattedRelativeTime) to mark the
    /// part of the string that is without a placeholder.
    pub const LITERAL: Part = Part {
        category: "relativetime",
        value: "literal",
    };
}

/// An intermediate structure returned by [`RelativeTimeFormatter`](crate::RelativeTimeFormatter).
/// This structure can be consumed via [`Writeable`](Writeable) trait to a string or buffer.
pub struct FormattedRelativeTime<'a> {
    pub(crate) formatter: &'a RelativeTimeFormatter,
    pub(crate) options: &'a RelativeTimeFormatterOptions,
    pub(crate) value: FixedDecimal,
}

impl<'a> Writeable for FormattedRelativeTime<'a> {
    fn write_to_parts<S: writeable::PartsWrite + ?Sized>(&self, sink: &mut S) -> core::fmt::Result {
        if self.options.numeric == Numeric::Auto {
            let _ = &self.formatter.rt.get().relatives;
        }

        let plural_rules_mapping = if self.value.sign() == Sign::Negative
            || (self.value.sign() == Sign::Negative && self.value.is_zero())
        {
            &self.formatter.rt.get().past
        } else {
            &self.formatter.rt.get().future
        };
        let category = self.formatter.plural_rules.category_for(&self.value);
        let pattern = match category {
            icu_plurals::PluralCategory::Zero => &plural_rules_mapping.zero,
            icu_plurals::PluralCategory::One => &plural_rules_mapping.one,
            icu_plurals::PluralCategory::Two => &plural_rules_mapping.two,
            icu_plurals::PluralCategory::Few => &plural_rules_mapping.few,
            icu_plurals::PluralCategory::Many => &plural_rules_mapping.many,
            icu_plurals::PluralCategory::Other => &plural_rules_mapping.other,
        };

        // Default to using PluralCategory::Other mapping.
        let pattern = pattern.as_ref().unwrap_or_else(|| {
            plural_rules_mapping
                .other
                .as_ref()
                .expect("Mapping for PluralCategory::Other must be present.")
        });

        // 255 is used to denote a string without placeholder '{0}'.
        if pattern.index == 255 {
            sink.with_part(parts::LITERAL, |s| s.write_str(&pattern.pattern))?;
        } else {
            sink.with_part(parts::LITERAL, |s| {
                s.write_str(&pattern.pattern[..pattern.index as usize])
            })?;
            self.formatter
                .fixed_decimal_format
                .format(&self.value)
                .write_to_parts(sink)?;
            sink.with_part(parts::LITERAL, |s| {
                s.write_str(&pattern.pattern[pattern.index as usize..])
            })?;
        }

        Ok(())
    }
}

writeable::impl_display_with_writeable!(FormattedRelativeTime<'_>);
