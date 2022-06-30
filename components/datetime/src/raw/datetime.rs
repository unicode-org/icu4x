// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormat`].

use crate::{
    format::datetime,
    options::components,
    options::DateTimeFormatOptions,
    provider::calendar::patterns::PatternPluralsFromPatternsV1Marker,
    provider::calendar::{
        DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
        TimePatternsV1Marker, TimeSymbolsV1Marker,
    },
    provider::week_data::WeekDataV1Marker,
};
use alloc::string::String;

use icu_decimal::{
    options::{FixedDecimalFormatOptions, GroupingStrategy, SignDisplay},
    provider::DecimalSymbolsV1Marker,
    FixedDecimalFormat,
};
use icu_locid::Locale;
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

use crate::{
    date::DateTimeInput, pattern::runtime::PatternPlurals, provider, DateTimeFormatError,
    FormattedDateTime,
};

/// This is the internal "raw" version of [crate::DateTimeFormat], i.e. a version of DateTimeFormat
/// without the generic parameter. The actual implementation of [crate::DateTimeFormat] should live here.
pub(crate) struct DateTimeFormat {
    pub locale: Locale,
    pub patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub date_symbols: Option<DataPayload<DateSymbolsV1Marker>>,
    pub time_symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
    pub week_data: Option<DataPayload<WeekDataV1Marker>>,
    pub ordinal_rules: Option<PluralRules>,
    pub fixed_decimal_format: FixedDecimalFormat,
}

impl DateTimeFormat {
    /// Constructor that takes a selected [`Locale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    ///
    /// The "calendar" argument should be a Unicode BCP47 calendar identifier
    #[inline(never)]
    pub fn try_new<D>(
        locale: Locale,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<TimeSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<TimePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<DecimalSymbolsV1Marker>
            + ResourceProvider<OrdinalV1Marker>
            + ResourceProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let patterns =
            provider::date_time::PatternSelector::for_options(data_provider, &locale, options)?;

        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeFormatError::UnsupportedField(field.symbol))?;

        let week_data = if required.week_data {
            Some(
                data_provider
                    .load_resource(&DataRequest {
                        options: ResourceOptions::temp_for_region(locale.id.region),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        // TODO(#1109): Implement proper vertical fallback
        let mut locale_no_extensions = locale.clone();
        locale_no_extensions.extensions.unicode.clear();

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal(
                locale_no_extensions.clone(),
                data_provider,
            )?)
        } else {
            None
        };

        let date_symbols_data = if required.date_symbols_data {
            Some(
                data_provider
                    .load_resource(&DataRequest {
                        options: ResourceOptions::from(&locale),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let time_symbols_data = if required.time_symbols_data {
            Some(
                data_provider
                    .load_resource(&DataRequest {
                        options: ResourceOptions::from(&locale),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;
        fixed_decimal_format_options.sign_display = SignDisplay::Never;

        let fixed_decimal_format = FixedDecimalFormat::try_new(
            locale_no_extensions,
            data_provider,
            fixed_decimal_format_options,
        )
        .map_err(DateTimeFormatError::FixedDecimalFormat)?;

        Ok(Self::new(
            locale,
            patterns,
            date_symbols_data,
            time_symbols_data,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        ))
    }

    /// Creates a new [`DateTimeFormat`] regardless of whether there are time-zone symbols in the pattern.
    pub fn new(
        locale: Locale,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        date_symbols: Option<DataPayload<DateSymbolsV1Marker>>,
        time_symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
        week_data: Option<DataPayload<WeekDataV1Marker>>,
        ordinal_rules: Option<PluralRules>,
        fixed_decimal_format: FixedDecimalFormat,
    ) -> Self {
        Self {
            locale,
            patterns,
            date_symbols,
            time_symbols,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        }
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l, T>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            date_symbols: self.date_symbols.as_ref().map(|s| s.get()),
            time_symbols: self.time_symbols.as_ref().map(|s| s.get()),
            datetime: value,
            week_data: self.week_data.as_ref().map(|s| s.get()),
            locale: &self.locale,
            ordinal_rules: self.ordinal_rules.as_ref(),
            fixed_decimal_format: &self.fixed_decimal_format,
        }
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    #[inline(never)]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput,
    ) -> core::fmt::Result {
        datetime::write_pattern_plurals(
            &self.patterns.get().0,
            self.date_symbols.as_ref().map(|s| s.get()),
            self.time_symbols.as_ref().map(|s| s.get()),
            value,
            self.week_data.as_ref().map(|s| s.get()),
            self.ordinal_rules.as_ref(),
            &self.fixed_decimal_format,
            &self.locale,
            w,
        )
        .map_err(|_| core::fmt::Error)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    #[inline]
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        let mut s = String::new();
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormat`].
    pub fn resolve_components(&self) -> components::Bag {
        components::Bag::from(&self.patterns.get().0)
    }
}
