// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options for [`FixedDecimalFormat`](crate::FixedDecimalFormat).

/// A bag of options defining how numbers will be formatted by
/// [`FixedDecimalFormat`](crate::FixedDecimalFormat).
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct FixedDecimalFormatOptions {
    /// When to render grouping separators.
    pub grouping_strategy: GroupingStrategy,
    /// When to render the sign.
    pub sign_display: SignDisplay,
}

/// Configuration for how often to render grouping separators.
///
/// # Examples
///
/// ```
/// use icu_decimal::FixedDecimalFormat;
/// use icu_decimal::FormattedFixedDecimal;
/// use icu_decimal::options;
/// use icu_locid::Locale;
/// use writeable::Writeable;
///
/// let locale: Locale = Locale::und().into();
/// let provider = icu_provider::inv::InvariantDataProvider;
/// let mut options: options::FixedDecimalFormatOptions = Default::default();
/// options.grouping_strategy = options::GroupingStrategy::Min2;
/// let fdf = FixedDecimalFormat::try_new(locale, &provider, options)
///     .expect("Data should load successfully");
///
/// let one_thousand = 1000.into();
/// assert_eq!("1000", fdf.format(&one_thousand).writeable_to_string());
///
/// let ten_thousand = 10000.into();
/// assert_eq!("10,000", fdf.format(&ten_thousand).writeable_to_string());
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
    /// For [`FixedDecimalFormat`](crate::FixedDecimalFormat), [`GroupingStrategy::Always`]
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

/// Configuration for when to render the minus sign or plus sign.
///
/// # Examples
///
/// ```
/// use icu_decimal::FixedDecimalFormat;
/// use icu_decimal::FormattedFixedDecimal;
/// use icu_decimal::options;
/// use icu_locid::Locale;
/// use writeable::Writeable;
///
/// let locale: Locale = Locale::und().into();
/// let provider = icu_provider::inv::InvariantDataProvider;
/// let mut options: options::FixedDecimalFormatOptions = Default::default();
/// options.sign_display = options::SignDisplay::ExceptZero;
/// let fdf = FixedDecimalFormat::try_new(locale, &provider, options)
///     .expect("Data should load successfully");
///
/// let pos_thousand = 1000.into();
/// assert_eq!("+1,000", fdf.format(&pos_thousand).writeable_to_string());
///
/// let zero = 0.into();
/// assert_eq!("0", fdf.format(&zero).writeable_to_string());
///
/// let neg_thousand = (-1000).into();
/// assert_eq!("-1,000", fdf.format(&neg_thousand).writeable_to_string());
/// ```
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SignDisplay {
    /// Render the sign according to locale preferences. In most cases, this means a minus sign
    /// will be shown on negative numbers, and no sign will be shown on positive numbers.
    Auto,

    /// Do not display the sign. Positive and negative numbers are indistinguishable.
    Never,

    /// Show a minus sign on negative numbers and a plus sign on positive numbers, including zero.
    Always,

    /// Show a minus sign on negative numbers and a plus sign on positive numbers, except do not
    /// show any sign on positive or negative zero.
    ExceptZero,

    /// Show a minus sign on strictly negative numbers. Do not show a sign on positive numbers or
    /// on positive or negative zero.
    ///
    /// This differs from [`Auto`](SignDisplay::Auto) in that it does not render a sign on negative zero.
    Negative,
}

impl Default for SignDisplay {
    fn default() -> Self {
        Self::Auto
    }
}
