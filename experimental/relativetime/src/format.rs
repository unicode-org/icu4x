// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::fmt::Write;

use fixed_decimal::FixedDecimal;
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
    pub(crate) is_negative: bool,
}

impl<'a> Writeable for FormattedRelativeTime<'a> {
    fn write_to_parts<S: writeable::PartsWrite + ?Sized>(&self, sink: &mut S) -> core::fmt::Result {
        if self.options.numeric == Numeric::Auto {
            let relatives = &self.formatter.rt.get().relatives;
            if self.value.magnitude_range() == (0..=0) {
                // Can be cast without overflow as it is a single digit.
                let i8_value = if self.is_negative {
                    -(self.value.digit_at(0) as i8)
                } else {
                    self.value.digit_at(0) as i8
                };
                if let Some(v) = relatives.get(&i8_value) {
                    sink.with_part(parts::LITERAL, |s| s.write_str(v))?;
                    return Ok(());
                }
            }
        }

        let plural_rules_mapping = if self.is_negative {
            &self.formatter.rt.get().past
        } else {
            &self.formatter.rt.get().future
        };
        let category = self.formatter.plural_rules.category_for(&self.value);
        let singular_sub_pattern = match category {
            icu_plurals::PluralCategory::Zero => &plural_rules_mapping.zero,
            icu_plurals::PluralCategory::One => &plural_rules_mapping.one,
            icu_plurals::PluralCategory::Two => &plural_rules_mapping.two,
            icu_plurals::PluralCategory::Few => &plural_rules_mapping.few,
            icu_plurals::PluralCategory::Many => &plural_rules_mapping.many,
            icu_plurals::PluralCategory::Other => &None,
        };

        // Default to using PluralCategory::Other mapping.
        let singular_sub_pattern = singular_sub_pattern
            .as_ref()
            .unwrap_or(&plural_rules_mapping.other);

        // 255 is used to denote a string without placeholder '{0}'.
        if singular_sub_pattern.index == 255 {
            sink.with_part(parts::LITERAL, |s| {
                s.write_str(&singular_sub_pattern.pattern)
            })?;
        } else {
            let (prefix, suffix) =
                if singular_sub_pattern.index as usize <= singular_sub_pattern.pattern.len() {
                    singular_sub_pattern
                        .pattern
                        .split_at(singular_sub_pattern.index as usize)
                } else {
                    return Err(core::fmt::Error);
                };

            sink.with_part(parts::LITERAL, |s| s.write_str(prefix))?;
            self.formatter
                .fixed_decimal_format
                .format(&self.value)
                .write_to_parts(sink)?;
            sink.with_part(parts::LITERAL, |s| s.write_str(suffix))?;
        }

        Ok(())
    }
}

writeable::impl_display_with_writeable!(FormattedRelativeTime<'_>);
