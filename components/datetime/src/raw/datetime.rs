// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormatter`].

#[cfg(feature = "experimental")]
use crate::options::components;
use crate::{
    format::datetime,
    input::{DateInput, DateTimeInput, ExtractedDateTimeInput, IsoTimeInput},
    options::{length, preferences},
    pattern::runtime::PatternPlurals,
    provider::{
        self,
        calendar::{
            patterns::GenericPatternV1Marker, patterns::PatternPluralsFromPatternsV1Marker,
            ErasedDateLengthsV1Marker, ErasedDateSymbolsV1Marker, TimeLengthsV1Marker,
            TimeSymbolsV1Marker,
        },
    },
    DateTimeError, FormattedDateTime,
};

use icu_calendar::provider::WeekDataV1Marker;
use icu_decimal::{
    options::{FixedDecimalFormatterOptions, GroupingStrategy},
    provider::DecimalSymbolsV1Marker,
    FixedDecimalFormatter,
};
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

#[derive(Debug)]
pub(crate) struct TimeFormatter {
    pub patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
    pub fixed_decimal_format: FixedDecimalFormatter,
}

impl TimeFormatter {
    /// Constructor that takes a selected [`DataLocale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format time values into the given locale,
    /// using the short style.
    #[inline(never)]
    pub fn try_new<D>(
        data_provider: &D,
        locale: &DataLocale,
        length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<TimeLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let patterns = provider::date_time::pattern_for_time_length(
            data_provider,
            locale,
            length,
            preferences,
        )?;

        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeError::UnsupportedField(field.symbol))?;

        let symbols_data = if required.time_symbols_data {
            Some(
                data_provider
                    .load(DataRequest {
                        locale,
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format = FixedDecimalFormatter::try_new_unstable(
            data_provider,
            locale,
            fixed_decimal_format_options,
        )
        .map_err(DateTimeError::FixedDecimalFormatter)?;

        Ok(Self::new(patterns, symbols_data, fixed_decimal_format))
    }

    /// Creates a new [`TimeFormatter`] regardless of whether there are time-zone symbols in the pattern.
    pub fn new(
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
        fixed_decimal_format: FixedDecimalFormatter,
    ) -> Self {
        Self {
            patterns,
            symbols,
            fixed_decimal_format,
        }
    }

    /// Takes a [`IsoTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: IsoTimeInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            date_symbols: None,
            time_symbols: self.symbols.as_ref().map(|s| s.get()),
            datetime: ExtractedDateTimeInput::extract_from_time(value),
            week_data: None,
            ordinal_rules: None,
            fixed_decimal_format: &self.fixed_decimal_format,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DateFormatter {
    pub generic_pattern: DataPayload<GenericPatternV1Marker>,
    pub patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub symbols: Option<DataPayload<ErasedDateSymbolsV1Marker>>,
    pub week_data: Option<DataPayload<WeekDataV1Marker>>,
    pub ordinal_rules: Option<PluralRules>,
    pub fixed_decimal_format: FixedDecimalFormatter,
}

impl DateFormatter {
    /// Constructor that takes a selected [`DataLocale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format date values into the given locale.
    #[inline(never)]
    pub fn try_new<D>(
        data_provider: &D,
        patterns_data: DataPayload<ErasedDateLengthsV1Marker>,
        symbols_data_fn: impl FnOnce() -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>,
        locale: &DataLocale,
        length: length::Date,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let patterns = provider::date_time::pattern_for_date_length(length, patterns_data.clone());

        let generic_pattern =
            provider::date_time::generic_pattern_for_date_length(length, patterns_data);

        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeError::UnsupportedField(field.symbol))?;

        let req = DataRequest {
            locale,
            metadata: Default::default(),
        };

        let week_data = if required.week_data {
            Some(data_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal_unstable(
                data_provider,
                locale,
            )?)
        } else {
            None
        };

        let symbols_data = if required.date_symbols_data {
            Some(symbols_data_fn()?)
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format = FixedDecimalFormatter::try_new_unstable(
            data_provider,
            locale,
            fixed_decimal_format_options,
        )
        .map_err(DateTimeError::FixedDecimalFormatter)?;

        Ok(Self::new(
            generic_pattern,
            patterns,
            symbols_data,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        ))
    }

    /// Creates a new [`DateTimeFormatter`] regardless of whether there are time-zone symbols in the pattern.
    pub fn new(
        generic_pattern: DataPayload<GenericPatternV1Marker>,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols: Option<DataPayload<ErasedDateSymbolsV1Marker>>,
        week_data: Option<DataPayload<WeekDataV1Marker>>,
        ordinal_rules: Option<PluralRules>,
        fixed_decimal_format: FixedDecimalFormatter,
    ) -> Self {
        Self {
            generic_pattern,
            patterns,
            symbols,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        }
    }

    /// Takes a [`DateInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            date_symbols: self.symbols.as_ref().map(|s| s.get()),
            time_symbols: None,
            datetime: ExtractedDateTimeInput::extract_from_date(value),
            week_data: None,
            ordinal_rules: None,
            fixed_decimal_format: &self.fixed_decimal_format,
        }
    }
}

/// This is the internal "raw" version of [crate::DateTimeFormatter], i.e. a version of DateTimeFormatter
/// without the generic parameter. The actual implementation of [crate::DateTimeFormatter] should live here.
#[derive(Debug)]
pub(crate) struct DateTimeFormatter {
    pub patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub date_symbols: Option<DataPayload<ErasedDateSymbolsV1Marker>>,
    pub time_symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
    pub week_data: Option<DataPayload<WeekDataV1Marker>>,
    pub ordinal_rules: Option<PluralRules>,
    pub fixed_decimal_format: FixedDecimalFormatter,
}

impl DateTimeFormatter {
    /// Constructor that takes previously constructed [`TimeFormatter`] and [`DateFormatter`] instances and builds a
    /// new [`DateTimeFormatter`] instance from them.
    #[inline(never)]
    pub fn try_from_date_and_time(
        date: DateFormatter,
        time: TimeFormatter,
    ) -> Result<Self, DateTimeError> {
        let generic_pattern = &date.generic_pattern;
        let time_patterns = &time.patterns;
        let patterns = date
            .patterns
            .try_map_project::<PatternPluralsFromPatternsV1Marker, _, DateTimeError>(
                |data, _| {
                    let date_pattern = data.0.expect_pattern("Lengths are single patterns");
                    let time_pattern: crate::pattern::runtime::Pattern = time_patterns
                        .get()
                        .clone()
                        .0
                        .expect_pattern("Lengths are single patterns");

                    Ok(PatternPlurals::from(
                        generic_pattern
                            .get()
                            .clone()
                            .0
                            .combined(date_pattern, time_pattern)?,
                    )
                    .into())
                },
            )?;

        Ok(Self {
            patterns,
            date_symbols: date.symbols,
            time_symbols: time.symbols,
            week_data: date.week_data,
            ordinal_rules: date.ordinal_rules,
            fixed_decimal_format: date.fixed_decimal_format,
        })
    }

    /// Constructor that takes a selected [`Locale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    #[inline(never)]
    pub fn try_new<D>(
        data_provider: &D,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols_data_fn: impl FnOnce() -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>,
        locale: &DataLocale,
    ) -> Result<Self, DateTimeError>
    where
        D: DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeError::UnsupportedField(field.symbol))?;

        let req = DataRequest {
            locale,
            metadata: Default::default(),
        };

        let week_data = if required.week_data {
            Some(data_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal_unstable(
                data_provider,
                locale,
            )?)
        } else {
            None
        };

        let date_symbols_data = if required.date_symbols_data {
            Some(symbols_data_fn()?)
        } else {
            None
        };

        let time_symbols_data = if required.time_symbols_data {
            Some(data_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format = FixedDecimalFormatter::try_new_unstable(
            data_provider,
            locale,
            fixed_decimal_format_options,
        )
        .map_err(DateTimeError::FixedDecimalFormatter)?;

        Ok(Self::new(
            patterns,
            date_symbols_data,
            time_symbols_data,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        ))
    }

    /// Creates a new [`DateTimeFormatter`] regardless of whether there are time-zone symbols in the pattern.
    pub fn new(
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        date_symbols: Option<DataPayload<ErasedDateSymbolsV1Marker>>,
        time_symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
        week_data: Option<DataPayload<WeekDataV1Marker>>,
        ordinal_rules: Option<PluralRules>,
        fixed_decimal_format: FixedDecimalFormatter,
    ) -> Self {
        Self {
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
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput,
    {
        // Todo: optimize extraction #2143
        FormattedDateTime {
            patterns: &self.patterns,
            date_symbols: self.date_symbols.as_ref().map(|s| s.get()),
            time_symbols: self.time_symbols.as_ref().map(|s| s.get()),
            datetime: ExtractedDateTimeInput::extract_from(value),
            week_data: self.week_data.as_ref().map(|s| s.get()),
            ordinal_rules: self.ordinal_rules.as_ref(),
            fixed_decimal_format: &self.fixed_decimal_format,
        }
    }

    /// Returns a [`components::Bag`] that represents the resolved components for the
    /// options that were provided to the [`DateTimeFormatter`].
    #[cfg(feature = "experimental")]
    pub fn resolve_components(&self) -> components::Bag {
        components::Bag::from(&self.patterns.get().0)
    }
}
