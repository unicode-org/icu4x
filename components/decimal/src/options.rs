// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for [`FixedDecimalFormatter`](crate::FixedDecimalFormatter).

/// A bag of options defining how numbers will be formatted by
/// [`FixedDecimalFormatter`](crate::FixedDecimalFormatter).
#[derive(Debug, Eq, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct FixedDecimalFormatterOptions {
    /// When to render grouping separators.
    pub grouping_strategy: GroupingStrategy,
}

/// Configuration for how often to render grouping separators.
///
/// # Examples
///
/// ```
/// use icu_decimal::options;
/// use icu_decimal::FixedDecimalFormatter;
/// use icu_decimal::FormattedFixedDecimal;
/// use icu_locid::Locale;
/// use writeable::Writeable;
///
/// let locale = Locale::UND;
/// let provider = icu_provider::inv::InvariantDataProvider;
/// let mut options: options::FixedDecimalFormatterOptions = Default::default();
/// options.grouping_strategy = options::GroupingStrategy::Min2;
/// let fdf = FixedDecimalFormatter::try_new(locale, &provider, options)
///     .expect("Data should load successfully");
///
/// let one_thousand = 1000.into();
/// assert_eq!("1000", fdf.format(&one_thousand).write_to_string());
///
/// let ten_thousand = 10000.into();
/// assert_eq!("10,000", fdf.format(&ten_thousand).write_to_string());
/// ```
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum GroupingStrategy {
    /// Render grouping separators according to locale preferences.
    Auto,

    /// Never render grouping separators.
    Never,

    /// Always render grouping separators.
    ///
    /// For [`FixedDecimalFormatter`](crate::FixedDecimalFormatter), [`GroupingStrategy::Always`]
    /// has the same behavior as [`GroupingStrategy::Auto`].
    Always,

    /// Render grouping separators only if there are at least 2 digits before the final grouping
    /// separator. In most locales, this means that numbers between 1000 and 9999 do not get
    /// grouping separators, but numbers 10,000 and above will.
    Min2,
}

impl Default for GroupingStrategy {
    fn default() -> Self {
        Self::Auto
    }
}
