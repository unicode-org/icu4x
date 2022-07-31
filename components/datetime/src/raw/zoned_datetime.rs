// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use icu_decimal::{
    options::{FixedDecimalFormatterOptions, GroupingStrategy},
    provider::DecimalSymbolsV1Marker,
    FixedDecimalFormatter,
};
use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value};
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

use crate::{
    format::{
        datetime,
        zoned_datetime::{self, FormattedZonedDateTime},
    },
    input::{DateTimeInput, TimeZoneInput},
    input::{ExtractedDateTimeInput, ExtractedTimeZoneInput},
    options::TypedDateTimeFormatterOptions,
    pattern::runtime::PatternPlurals,
    provider::{
        self,
        calendar::{
            DateLengthsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
            TimeLengthsV1Marker, TimeSymbolsV1Marker,
        },
        week_data::WeekDataV1Marker,
    },
    raw,
    time_zone::{TimeZoneFormatter, TimeZoneFormatterOptions},
    TypedDateTimeFormatterError,
};

/// This is the internal "raw" version of [crate::TypedZonedDateTimeFormatter], i.e. a version of TypedZonedDateTimeFormatter
/// without the generic parameter. The actual implementation of [crate::TypedZonedDateTimeFormatter] should live here.
pub(crate) struct TypedZonedDateTimeFormatter {
    pub datetime_format: raw::TypedDateTimeFormatter,
    pub time_zone_format: TimeZoneFormatter,
}

impl TypedZonedDateTimeFormatter {
    /// Constructor that takes a selected [`DataLocale`], a reference to a [`DataProvider`] for
    /// dates, a [`DataProvider`] for time zones, and a list of [`TypedDateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// The "calendar" argument should be a Unicode BCP47 calendar identifier
    #[inline(never)]
    pub fn try_new<DP, ZP, PP, DEP>(
        mut locale: DataLocale,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        decimal_provider: &DEP,
        date_time_format_options: &TypedDateTimeFormatterOptions,
        time_zone_format_options: &TimeZoneFormatterOptions,
    ) -> Result<Self, TypedDateTimeFormatterError>
    where
        DP: DataProvider<DateSymbolsV1Marker>
            + DataProvider<TimeSymbolsV1Marker>
            + DataProvider<DateLengthsV1Marker>
            + DataProvider<TimeLengthsV1Marker>
            + DataProvider<DateSkeletonPatternsV1Marker>
            + DataProvider<WeekDataV1Marker>
            + ?Sized,
        ZP: DataProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + DataProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: DataProvider<OrdinalV1Marker> + ?Sized,
        DEP: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        if locale.get_unicode_ext(&key!("ca")) == Some(value!("ethioaa")) {
            locale.set_unicode_ext(key!("ca"), value!("ethiopic"));
        }

        let patterns = provider::date_time::PatternSelector::for_options(
            date_provider,
            &locale,
            date_time_format_options,
        )?;
        let required = datetime::analyze_patterns(&patterns.get().0, true)
            .map_err(|field| TypedDateTimeFormatterError::UnsupportedField(field.symbol))?;

        let req = DataRequest {
            locale: &locale,
            metadata: Default::default(),
        };

        let week_data = if required.week_data {
            Some(date_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal(plural_provider, &locale)?)
        } else {
            None
        };

        let date_symbols_data = if required.date_symbols_data {
            Some(date_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let time_symbols_data = if required.time_symbols_data {
            Some(date_provider.load(req)?.take_payload()?)
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format =
            FixedDecimalFormatter::try_new(decimal_provider, &locale, fixed_decimal_format_options)
                .map_err(TypedDateTimeFormatterError::FixedDecimalFormatter)?;

        let datetime_format = raw::TypedDateTimeFormatter::new(
            locale,
            patterns,
            date_symbols_data,
            time_symbols_data,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        );

        let time_zone_format = TimeZoneFormatter::try_new(
            &datetime_format.locale,
            datetime_format
                // Only dates have plural variants so we can use any of the patterns for the time segment.
                .patterns
                .clone(),
            zone_provider,
            time_zone_format_options,
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

    /// Takes a mutable reference to anything that implements the [`Write`](std::fmt::Write) trait
    /// and a [`ZonedDateTimeInput`] implementer, then populates the buffer with a formatted value.
    #[inline(never)]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        date: &impl DateTimeInput,
        time_zone: &impl TimeZoneInput,
    ) -> core::fmt::Result {
        zoned_datetime::write_pattern(self, date, time_zone, w).map_err(|_| core::fmt::Error)
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns it formatted as a string.
    #[inline]
    pub fn format_to_string(
        &self,
        date: &impl DateTimeInput,
        time_zone: &impl TimeZoneInput,
    ) -> String {
        let mut s = String::new();
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.format_to_write(&mut s, date, time_zone)
            .expect("Failed to write to a String.");
        s
    }
}
