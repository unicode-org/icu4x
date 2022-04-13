// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use icu_locid::Locale;
use icu_plurals::{provider::OrdinalV1Marker, PluralRules};
use icu_provider::prelude::*;

use crate::{
    date::ZonedDateTimeInput,
    format::{
        datetime,
        zoned_datetime::{self, FormattedZonedDateTime},
    },
    options::DateTimeFormatOptions,
    pattern::runtime::PatternPlurals,
    provider::{
        self,
        calendar::{DatePatternsV1Marker, DateSkeletonPatternsV1Marker, DateSymbolsV1Marker},
        week_data::WeekDataV1Marker,
    },
    raw,
    time_zone::{TimeZoneFormat, TimeZoneFormatOptions},
    DateTimeFormatError,
};

/// This is the internal "raw" version of [crate::ZonedDateTimeFormat], i.e. a version of ZonedDateTimeFormat
/// without the generic parameter. The actual implementation of [crate::ZonedDateTimeFormat] should live here.
pub(crate) struct ZonedDateTimeFormat {
    pub datetime_format: raw::DateTimeFormat,
    pub time_zone_format: TimeZoneFormat,
}

impl ZonedDateTimeFormat {
    /// Constructor that takes a selected [`Locale`], a reference to a [`DataProvider`] for
    /// dates, a [`DataProvider`] for time zones, and a list of [`DateTimeFormatOptions`].
    /// It collects all data necessary to format zoned datetime values into the given locale.
    ///
    /// The "calendar" argument should be a Unicode BCP47 calendar identifier
    #[inline(never)]
    pub fn try_new<DP, ZP, PP>(
        locale: Locale,
        date_provider: &DP,
        zone_provider: &ZP,
        plural_provider: &PP,
        date_time_format_options: &DateTimeFormatOptions,
        time_zone_format_options: &TimeZoneFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        DP: ResourceProvider<DateSymbolsV1Marker>
            + ResourceProvider<DatePatternsV1Marker>
            + ResourceProvider<DateSkeletonPatternsV1Marker>
            + ResourceProvider<WeekDataV1Marker>,
        ZP: ResourceProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
        PP: ResourceProvider<OrdinalV1Marker>,
    {
        let patterns = provider::date_time::PatternSelector::for_options(
            date_provider,
            &locale,
            date_time_format_options,
        )?;
        let required = datetime::analyze_patterns(&patterns.get().0, true)
            .map_err(|field| DateTimeFormatError::UnsupportedField(field.symbol))?;

        let week_data = if required.week_data {
            Some(
                date_provider
                    .load_resource(&DataRequest {
                        options: ResourceOptions::temp_for_region(locale.id.region),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let ordinal_rules = if let PatternPlurals::MultipleVariants(_) = &patterns.get().0 {
            // TODO(#1109): Implement proper vertical fallback
            let mut locale_no_extensions = locale.clone();
            locale_no_extensions.extensions.unicode.clear();
            Some(PluralRules::try_new_ordinal(
                locale_no_extensions,
                plural_provider,
            )?)
        } else {
            None
        };

        let symbols_data = if required.symbols_data {
            Some(
                date_provider
                    .load_resource(&DataRequest {
                        options: locale.clone().into(),
                        metadata: Default::default(),
                    })?
                    .take_payload()?,
            )
        } else {
            None
        };

        let datetime_format =
            raw::DateTimeFormat::new(locale, patterns, symbols_data, week_data, ordinal_rules);

        // TODO(#1109): Implement proper vertical fallback
        let mut locale_no_extensions = datetime_format.locale.clone();
        locale_no_extensions.extensions.unicode.clear();
        let time_zone_format = TimeZoneFormat::try_new(
            locale_no_extensions,
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
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedZonedDateTime<'l, T>
    where
        T: ZonedDateTimeInput,
    {
        FormattedZonedDateTime {
            zoned_datetime_format: self,
            zoned_datetime: value,
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
