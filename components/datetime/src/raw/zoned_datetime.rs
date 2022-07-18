// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use icu_decimal::{
    options::{FixedDecimalFormatterOptions, GroupingStrategy},
    provider::DecimalSymbolsV1Marker,
    FixedDecimalFormatter,
};
use icu_locid::{extensions_unicode_key as key, extensions_unicode_value as value, Locale};
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

use crate::{
    date::ExtractedZonedDateTimeInput,
    date::ZonedDateTimeInput,
    format::{
        datetime,
        zoned_datetime::{self, FormattedZonedDateTime},
    },
    options::DateTimeFormatterOptions,
    pattern::runtime::PatternPlurals,
    provider::{
        self,
        calendar::{
            DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker,
            TimePatternsV1Marker, TimeSymbolsV1Marker,
        },
        week_data::WeekDataV1Marker,
    },
    raw,
    time_zone::{TimeZoneFormatter, TimeZoneFormatterOptions},
    DateTimeFormatterError,
};

/// This is the internal "raw" version of [crate::ZonedDateTimeFormatter], i.e. a version of ZonedDateTimeFormatter
/// without the generic parameter. The actual implementation of [crate::ZonedDateTimeFormatter] should live here.
pub(crate) struct ZonedDateTimeFormatter {
    pub datetime_format: raw::DateTimeFormatter,
    pub time_zone_format: TimeZoneFormatter,
}

impl ZonedDateTimeFormatter {
    /// Constructor that takes a selected [`Locale`], a reference to a [`DataProvider`] for
    /// dates, a [`DataProvider`] for time zones, and a list of [`DateTimeFormatterOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// The "calendar" argument should be a Unicode BCP47 calendar identifier
    #[inline(never)]
    pub fn try_new<DP, ZP, PP, DEP>(
        mut locale: Locale,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        decimal_provider: &DEP,
        date_time_format_options: &DateTimeFormatterOptions,
        time_zone_format_options: &TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeFormatterError>
    where
        DP: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<TimeSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<TimePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<WeekDataV1Marker>
            + ?Sized,
        ZP: ResourceProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: ResourceProvider<OrdinalV1Marker> + ?Sized,
        DEP: ResourceProvider<DecimalSymbolsV1Marker> + ?Sized,
    {
        let cal = locale.extensions.unicode.keywords.get(&key!("ca"));
        if cal == Some(&value!("ethioaa")) {
            locale
                .extensions
                .unicode
                .keywords
                .set(key!("ca"), value!("ethiopic"));
        }
        let patterns = provider::date_time::PatternSelector::for_options(
            date_provider,
            &locale,
            date_time_format_options,
        )?;
        let required = datetime::analyze_patterns(&patterns.get().0, true)
            .map_err(|field| DateTimeFormatterError::UnsupportedField(field.symbol))?;

        let week_data = if required.week_data {
            Some(
                date_provider
                    .load_resource(&DataRequest {
                        options: ResourceOptions::from(&locale),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            Some(PluralRules::try_new_ordinal(
                // TODO(#2136): Don't clone here
                locale.clone(),
                plural_provider,
            )?)
        } else {
            None
        };

        let date_symbols_data = if required.date_symbols_data {
            Some(
                date_provider
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
                date_provider
                    .load_resource(&DataRequest {
                        options: ResourceOptions::from(&locale),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let mut fixed_decimal_format_options = FixedDecimalFormatterOptions::default();
        fixed_decimal_format_options.grouping_strategy = GroupingStrategy::Never;

        let fixed_decimal_format = FixedDecimalFormatter::try_new(
            // TODO(#2136): Don't clone here
            locale.clone(),
            decimal_provider,
            fixed_decimal_format_options,
        )
        .map_err(DateTimeFormatterError::FixedDecimalFormatter)?;

        let datetime_format = raw::DateTimeFormatter::new(
            // TODO(#2136): Don't clone here
            locale.clone(),
            patterns,
            date_symbols_data,
            time_symbols_data,
            week_data,
            ordinal_rules,
            fixed_decimal_format,
        );

        let time_zone_format = TimeZoneFormatter::try_new(
            locale,
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
    pub fn format<'l, T>(&'l self, value: &T) -> FormattedZonedDateTime<'l>
    where
        T: ZonedDateTimeInput,
    {
        // Todo: optimize extraction #2143
        FormattedZonedDateTime {
            zoned_datetime_format: self,
            zoned_datetime: ExtractedZonedDateTimeInput::extract_from(value),
        }
    }

    /// Takes a mutable reference to anything that implements the [`Write`](std::fmt::Write) trait
    /// and a [`ZonedDateTimeInput`] implementer, then populates the buffer with a formatted value.
    #[inline(never)]
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl ZonedDateTimeInput,
    ) -> core::fmt::Result {
        zoned_datetime::write_pattern(self, value, w).map_err(|_| core::fmt::Error)
    }

    /// Takes a [`ZonedDateTimeInput`] implementer and returns it formatted as a string.
    #[inline]
    pub fn format_to_string(&self, value: &impl ZonedDateTimeInput) -> String {
        let mut s = String::new();
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }
}
