// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_calendar::provider::WeekDataV1Marker;
use icu_decimal::{
    options::{FixedDecimalFormatterOptions, GroupingStrategy},
    provider::DecimalSymbolsV1Marker,
    FixedDecimalFormatter,
};
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

use crate::{
    format::{datetime, zoned_datetime::FormattedZonedDateTime},
    input::{DateTimeInput, TimeZoneInput},
    input::{ExtractedDateTimeInput, ExtractedTimeZoneInput},
    pattern::runtime::PatternPlurals,
    provider::{
        self,
        calendar::{
            patterns::PatternPluralsFromPatternsV1Marker, ErasedDateSymbolsV1Marker,
            TimeLengthsV1Marker, TimeSymbolsV1Marker,
        },
    },
    raw,
    time_zone::{TimeZoneFormatter, TimeZoneFormatterOptions},
    DateTimeError,
};

/// This is the internal "raw" version of [crate::ZonedDateTimeFormatter], i.e. a version of ZonedDateTimeFormatter
/// without the generic parameter. The actual implementation of [crate::ZonedDateTimeFormatter] should live here.
#[derive(Debug)]
pub(crate) struct ZonedDateTimeFormatter {
    pub datetime_format: raw::DateTimeFormatter,
    pub time_zone_format: TimeZoneFormatter,
}

impl ZonedDateTimeFormatter {
    /// Constructor that takes a selected [`DataLocale`], a reference to a [`DataProvider`] for
    /// dates, a [`DataProvider`] for time zones, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// The "calendar" argument should be a Unicode BCP47 calendar identifier
    #[inline(never)]
    pub fn try_new<P>(
        provider: &P,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        symbols_data_fn: impl FnOnce() -> Result<DataPayload<ErasedDateSymbolsV1Marker>, DataError>,
        locale: &DataLocale,
        time_zone_format_options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<TimeSymbolsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<WeekDataV1Marker>
            + DataProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesShortV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesShortV1Marker>
            + DataProvider<OrdinalV1Marker>
            + DataProvider<DecimalSymbolsV1Marker>
            + ?Sized,
    {
        let required = datetime::analyze_patterns(&patterns.get().0, true)
            .map_err(|field| DateTimeError::UnsupportedField(field.symbol))?;

        let req = DataRequest {
            locale,
            metadata: Default::default(),
        };

        let week_data = if required.week_data {
            Some(provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal_unstable(provider, locale)?)
        } else {
            None
        };

        let date_symbols_data = if required.date_symbols_data {
            Some(symbols_data_fn()?)
        } else {
            None
        };

        let time_symbols_data = if required.time_symbols_data {
            Some(provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format =
            FixedDecimalFormatter::try_new_unstable(provider, locale, fixed_decimal_format_options)
                .map_err(DateTimeError::FixedDecimalFormatter)?;

        let datetime_format = raw::DateTimeFormatter::new(
            patterns,
            date_symbols_data,
            time_symbols_data,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        );

        let time_zone_format = TimeZoneFormatter::try_new_for_pattern(
            provider,
            locale,
            datetime_format
                // Only dates have plural variants so we can use any of the patterns for the time segment.
                .patterns
                .clone(),
            &time_zone_format_options,
        )?;

        Ok(Self {
            datetime_format,
            time_zone_format,
        })
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns an instance of a [`FormattedZonedDateTime`]
    /// that contains all information necessary to display a formatted zoned datetime and operate on it.
    #[inline]
    pub fn format<'l>(
        &'l self,
        date: &impl DateTimeInput,
        time_zone: &impl TimeZoneInput,
    ) -> FormattedZonedDateTime<'l> {
        // Todo: optimize extraction #2143
        FormattedZonedDateTime {
            zoned_datetime_format: self,
            datetime: ExtractedDateTimeInput::extract_from(date),
            time_zone: ExtractedTimeZoneInput::extract_from(time_zone),
        }
    }
}
