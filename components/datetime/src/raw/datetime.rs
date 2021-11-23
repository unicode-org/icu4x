// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The collection of code that is needed for handling formatting operations for DateTimes.
//! Central to this is the [`DateTimeFormat`].

use crate::{
    format::datetime,
    options::DateTimeFormatOptions,
    provider::calendar::patterns::PatternPluralsFromPatternsV1Marker,
    provider::calendar::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
};
use alloc::string::String;
use icu_locid::Locale;
use icu_plurals::{provider::PluralRulesV1Marker, PluralRuleType, PluralRules};
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
    pub symbols: Option<DataPayload<DateSymbolsV1Marker>>,
    pub ordinal_rules: Option<PluralRules>,
}

impl DateTimeFormat {
    /// Constructor that takes a selected [`Locale`], reference to a [`DataProvider`] and
    /// a list of options, then collects all data necessary to format date and time values into the given locale.
    pub fn try_new<T: Into<Locale>, D>(
        locale: T,
        data_provider: &D,
        options: &DateTimeFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        D: DataProvider<DateSymbolsV1Marker>
            + DataProvider<DatePatternsV1Marker>
            + DataProvider<DateSkeletonPatternsV1Marker>
            + DataProvider<PluralRulesV1Marker>,
    {
        let locale = locale.into();

        let patterns =
            provider::date_time::PatternSelector::for_options(data_provider, &locale, options)?;

        let requires_data = datetime::analyze_patterns(&patterns.get().0, false)
            .map_err(|field| DateTimeFormatError::UnsupportedField(field.symbol))?;

        let langid: icu_locid::LanguageIdentifier = locale.clone().into();

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new(
                locale.clone(),
                data_provider,
                PluralRuleType::Ordinal,
            )?)
        } else {
            None
        };

        let symbols_data = if requires_data {
            Some(
                data_provider
                    .load_payload(&DataRequest {
                        resource_path: ResourcePath {
                            key: provider::key::DATE_SYMBOLS_V1,
                            options: ResourceOptions {
                                variant: Some("gregory".into()),
                                langid: Some(langid),
                            },
                        },
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        Ok(Self::new(locale, patterns, symbols_data, ordinal_rules))
    }

    /// Creates a new [`DateTimeFormat`] regardless of whether there are time-zone symbols in the pattern.
    pub(super) fn new<T: Into<Locale>>(
        locale: T,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols: Option<DataPayload<DateSymbolsV1Marker>>,
        ordinal_rules: Option<PluralRules>,
    ) -> Self {
        let locale = locale.into();

        Self {
            locale,
            patterns,
            symbols,
            ordinal_rules,
        }
    }

    /// Takes a [`DateTimeInput`] implementer and returns an instance of a [`FormattedDateTime`]
    /// that contains all information necessary to display a formatted date and operate on it.
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedDateTime<'l, T>
    where
        T: DateTimeInput,
    {
        FormattedDateTime {
            patterns: &self.patterns,
            symbols: self.symbols.as_ref().map(|s| s.get()),
            datetime: value,
            locale: &self.locale,
            ordinal_rules: self.ordinal_rules.as_ref(),
        }
    }

    /// Takes a mutable reference to anything that implements [`Write`](std::fmt::Write) trait
    /// and a [`DateTimeInput`] implementer and populates the buffer with a formatted value.
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl DateTimeInput,
    ) -> core::fmt::Result {
        datetime::write_pattern_plurals(
            &self.patterns.get().0,
            self.symbols.as_ref().map(|s| s.get()),
            value,
            self.ordinal_rules.as_ref(),
            &self.locale,
            w,
        )
        .map_err(|_| core::fmt::Error)
    }

    /// Takes a [`DateTimeInput`] implementer and returns it formatted as a string.
    pub fn format_to_string(&self, value: &impl DateTimeInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
