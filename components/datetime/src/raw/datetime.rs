// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormatter`].

use crate::{
    input::{DateTimeInput, ExtractedDateTimeInput, IsoTimeInput},
    format::datetime,
    options::components,
    options::{length, preferences, DateTimeFormatterOptions},
    pattern::runtime::PatternPlurals,
    provider,
    provider::calendar::patterns::PatternPluralsFromPatternsV1Marker,
    provider::calendar::{
        patterns::GenericPatternV1Marker, DatePatternsV1Marker, DateSkeletonPatternsV1Marker,
        DateSymbolsV1Marker, TimeLengthsV1Marker, TimeSymbolsV1Marker,
    },
    provider::week_data::WeekDataV1Marker,
    DateTimeFormatterError, FormattedDateTime,
};
use alloc::string::String;

use icu_decimal::{
    options::{FixedDecimalFormatterOptions, GroupingStrategy},
    provider::DecimalSymbolsV1Marker,
    FixedDecimalFormatter,
};
use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value};
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

pub(crate) struct TimeFormatter {
    pub locale: DataLocale,
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
        locale: DataLocale,
        data_provider: &D,
        length: length::Time,
        preferences: Option<preferences::Bag>,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<TimeLengthsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let patterns = provider::date_time::pattern_for_time_length(
            data_provider,
            &locale,
            length,
            preferences,
        )?;

        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeFormatterError::UnsupportedField(field.symbol))?;

        let symbols_data = if required.time_symbols_data {
            Some(
                data_provider
                    .load(DataRequest {
                        locale: &locale,
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format =
            FixedDecimalFormatter::try_new(&locale, data_provider, fixed_decimal_format_options)
                .map_err(DateTimeFormatterError::FixedDecimalFormatter)?;

        Ok(Self::new(
            locale,
            patterns,
            symbols_data,
            fixed_decimal_format,
        ))
    }

    /// Creates a new [`TimeFormatter`] regardless of whether there are time-zone symbols in the pattern.
    pub fn new(
        locale: DataLocale,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
        fixed_decimal_format: FixedDecimalFormatter,
    ) -> Self {
        Self {
            locale,
            patterns,
            symbols,
            fixed_decimal_format,
        }
    }

    /// Takes a [`IsoTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l>
    where
        T: IsoTimeInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            date_symbols: None,
            time_symbols: self.symbols.as_ref().map(|s| s.get()),
            datetime: ExtractedDateTimeInput::extract_from_time(value),
            week_data: None,
            locale: &self.locale,
            ordinal_rules: None,
            fixed_decimal_format: &self.fixed_decimal_format,
        }
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`IsoTimeInput`] implementer and populates the buffer with a formatted value.
    #[inline(never)]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl IsoTimeInput,
    ) -> core::fmt::Result {
        let extracted = ExtractedDateTimeInput::extract_from_time(value);
        datetime::write_pattern_plurals(
            &self.patterns.get().0,
            None,
            self.symbols.as_ref().map(|s| s.get()),
            &extracted,
            None,
            None,
            &self.fixed_decimal_format,
            &self.locale,
            w,
        )
        .map_err(|_| core::fmt::Error)
    }

    /// Takes a [`IsoTimeInput`] implementer and returns it formatted as a string.
    #[inline]
    pub fn format_to_string(&self, value: &impl IsoTimeInput) -> String {
        let mut s = String::new();
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}

pub(crate) struct DateFormatter {
    pub locale: DataLocale,
    pub generic_pattern: DataPayload<GenericPatternV1Marker>,
    pub patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub symbols: Option<DataPayload<DateSymbolsV1Marker>>,
    pub week_data: Option<DataPayload<WeekDataV1Marker>>,
    pub ordinal_rules: Option<PluralRules>,
    pub fixed_decimal_format: FixedDecimalFormatter,
}

impl DateFormatter {
    /// Constructor that takes a selected [`DataLocale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format date values into the given locale.
    #[inline(never)]
    pub fn try_new<D>(
        mut locale: DataLocale,
        data_provider: &D,
        length: length::Date,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<DateSymbolsV1Marker>
            + DataProvider<DatePatternsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        if locale.get_unicode_ext(&key!("ca")) == Some(value!("ethioaa")) {
            locale.set_unicode_ext(key!("ca"), value!("ethiopic"));
        }
        let patterns =
            provider::date_time::pattern_for_date_length(data_provider, &locale, length)?;

        let generic_pattern =
            provider::date_time::generic_pattern_for_date_length(data_provider, &locale, length)?;

        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeFormatterError::UnsupportedField(field.symbol))?;

        let req = DataRequest {
            locale: &locale,
            metadata: Default::default(),
        };

        let week_data = if required.week_data {
            Some(data_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal(&locale, data_provider)?)
        } else {
            None
        };

        let symbols_data = if required.date_symbols_data {
            Some(data_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format =
            FixedDecimalFormatter::try_new(&locale, data_provider, fixed_decimal_format_options)
                .map_err(DateTimeFormatterError::FixedDecimalFormatter)?;

        Ok(Self::new(
            locale,
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
        locale: DataLocale,
        generic_pattern: DataPayload<GenericPatternV1Marker>,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols: Option<DataPayload<DateSymbolsV1Marker>>,
        week_data: Option<DataPayload<WeekDataV1Marker>>,
        ordinal_rules: Option<PluralRules>,
        fixed_decimal_format: FixedDecimalFormatter,
    ) -> Self {
        Self {
            locale,
            generic_pattern,
            patterns,
            symbols,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        }
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    #[inline]
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            date_symbols: self.symbols.as_ref().map(|s| s.get()),
            time_symbols: None,
            datetime: ExtractedDateTimeInput::extract_from(value),
            week_data: None,
            locale: &self.locale,
            ordinal_rules: None,
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
            self.symbols.as_ref().map(|s| s.get()),
            None,
            value,
            None,
            None,
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
}

/// This is the internal "raw" version of [crate::DateTimeFormatter], i.e. a version of DateTimeFormatter
/// without the generic parameter. The actual implementation of [crate::DateTimeFormatter] should live here.
pub(crate) struct DateTimeFormatter {
    pub locale: DataLocale,
    pub patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
    pub date_symbols: Option<DataPayload<DateSymbolsV1Marker>>,
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
    ) -> Result<Self, DateTimeFormatterError> {
        let generic_pattern = &date.generic_pattern;
        let time_patterns = &time.patterns;
        let patterns = date
            .patterns
            .try_map_project::<PatternPluralsFromPatternsV1Marker, _, DateTimeFormatterError>(
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
            locale: date.locale,
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
        mut locale: DataLocale,
        data_provider: &D,
        options: &DateTimeFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        D: DataProvider<DateSymbolsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DatePatternsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DateSkeletonPatternsV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
    {
        let cal = locale.get_unicode_ext(&key!("ca"));
        if cal == Some(value!("ethioaa")) {
            locale.set_unicode_ext(key!("ca"), value!("ethiopic"));
        }
        let patterns =
            provider::date_time::PatternSelector::for_options(data_provider, &locale, options)?;

        let required = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeFormatterError::UnsupportedField(field.symbol))?;

        let req = DataRequest {
            locale: &locale,
            metadata: Default::default(),
        };

        let week_data = if required.week_data {
            Some(data_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal(&locale, data_provider)?)
        } else {
            None
        };

        let date_symbols_data = if required.date_symbols_data {
            Some(data_provider.load(req)?.take_payload()?)
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

        let fixed_decimal_format =
            FixedDecimalFormatter::try_new(&locale, data_provider, fixed_decimal_format_options)
                .map_err(DateTimeFormatterError::FixedDecimalFormatter)?;

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

    /// Creates a new [`DateTimeFormatter`] regardless of whether there are time-zone symbols in the pattern.
    pub fn new(
        locale: DataLocale,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        date_symbols: Option<DataPayload<DateSymbolsV1Marker>>,
        time_symbols: Option<DataPayload<TimeSymbolsV1Marker>>,
        week_data: Option<DataPayload<WeekDataV1Marker>>,
        ordinal_rules: Option<PluralRules>,
        fixed_decimal_format: FixedDecimalFormatter,
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
    /// options that were provided to the [`DateTimeFormatter`].
    pub fn resolve_components(&self) -> components::Bag {
        components::Bag::from(&self.patterns.get().0)
    }
}
