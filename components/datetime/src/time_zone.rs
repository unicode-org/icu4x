// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! A formatter specifically for the time zone.

use crate::provider::time_zones::TimeZoneBcp47Id;
use alloc::borrow::Cow;
use alloc::format;
use alloc::string::String;
use core::fmt;
use smallvec::SmallVec;
use tinystr::tinystr;

use crate::{
    error::DateTimeError,
    fields::{FieldSymbol, TimeZone},
    format::time_zone::FormattedTimeZone,
    input::TimeZoneInput,
    pattern::{PatternError, PatternItem},
    provider::{self, calendar::patterns::PatternPluralsFromPatternsV1Marker},
};
use icu_provider::prelude::*;
use writeable::Writeable;

#[cfg(doc)]
use crate::ZonedDateTimeFormatter;

/// Loads a resource into its destination if the destination has not already been filled.
fn load<D, P>(
    locale: &DataLocale,
    destination: &mut Option<DataPayload<D>>,
    provider: &P,
) -> Result<(), DateTimeError>
where
    D: KeyedDataMarker,
    P: DataProvider<D> + ?Sized,
{
    if destination.is_none() {
        *destination = Some(
            provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?,
        );
    }
    Ok(())
}

/// [`TimeZoneFormatter`] is available for users who need to separately control the formatting of time
/// zones.  Note: most users might prefer [`ZonedDateTimeFormatter`], which includes default time zone
/// formatting according to the calendar.
///
/// [`TimeZoneFormatter`] uses data from the [data provider] and the selected locale
/// to format time zones into that locale.
///
/// The various time-zone configs specified in UTS-35 require different sets of data for
/// formatting. As such,[`TimeZoneFormatter`] will pull in only the resources needed to format the
/// config that it is given upon construction.
///
/// For that reason, one should think of the process of formatting a time zone in two steps:
/// first, a computationally heavy construction of [`TimeZoneFormatter`], and then fast formatting
/// of the time-zone data using the instance.
///
/// [`CustomTimeZone`] can be used as formatting input.
///
/// # Examples
///
/// Here, we configure the [`TimeZoneFormatter`] to first look for time zone formatting symbol
/// data for `generic_non_location_short`, and if it does not exist, to subsequently check
/// for `generic_non_location_long` data.
///
/// ```
/// use icu::calendar::DateTime;
/// use icu::timezone::{CustomTimeZone, MetazoneCalculator};
/// use icu::datetime::{DateTimeError, time_zone::TimeZoneFormatter};
/// use icu::locid::locale;
/// use tinystr::tinystr;
/// use writeable::assert_writeable_eq;
///
/// // Set up the time zone. Note: the inputs here are
/// //   1. The GMT offset
/// //   2. The BCP-47 time zone ID
/// //   3. A datetime (for metazone resolution)
/// //   4. Note: we do not need the zone variant because of `load_generic_*()`
///
/// // Set up the Metazone calculator and the DateTime to use in calculation
/// let mzc = MetazoneCalculator::new();
/// let datetime = DateTime::try_new_iso_datetime(2022, 8, 29, 0, 0, 0)
///     .unwrap();
///
/// // Set up the formatter
/// let mut tzf = TimeZoneFormatter::try_new(
///     &locale!("en").into(),
///     Default::default(),
/// )
/// .unwrap();
/// tzf.with_generic_non_location_short()?
///     .with_generic_non_location_long()?;
///
/// // "uschi" - has metazone symbol data for generic_non_location_short
/// let mut time_zone = "-0600".parse::<CustomTimeZone>().unwrap();
/// time_zone.time_zone_id = Some(tinystr!(8, "uschi").into());
/// time_zone.maybe_calculate_metazone(&mzc, &datetime);
/// assert_writeable_eq!(
///     tzf.format(&time_zone),
///     "CT"
/// );
///
/// // "ushnl" - has time zone override symbol data for generic_non_location_short
/// let mut time_zone = "-1000".parse::<CustomTimeZone>().unwrap();
/// time_zone.time_zone_id = Some(tinystr!(8, "ushnl").into());
/// time_zone.maybe_calculate_metazone(&mzc, &datetime);
/// assert_writeable_eq!(
///     tzf.format(&time_zone),
///     "HST"
/// );
///
/// // "frpar" - does not have symbol data for generic_non_location_short, so falls
/// //           back to generic_non_location_long
/// let mut time_zone = "+0100".parse::<CustomTimeZone>().unwrap();
/// time_zone.time_zone_id = Some(tinystr!(8, "frpar").into());
/// time_zone.maybe_calculate_metazone(&mzc, &datetime);
/// assert_writeable_eq!(
///     tzf.format(&time_zone),
///     "Central European Time"
/// );
///
/// // GMT with offset - used when metazone is not available
/// let mut time_zone = "+0530".parse::<CustomTimeZone>().unwrap();
/// assert_writeable_eq!(
///     tzf.format(&time_zone),
///     "GMT+05:30"
/// );
///
/// # Ok::<(), DateTimeError>(())
/// ```
///
/// [data provider]: icu_provider
/// [`CustomTimeZone`]: icu_timezone::CustomTimeZone
#[derive(Debug)]
pub struct TimeZoneFormatter {
    pub(super) locale: DataLocale,
    pub(super) data_payloads: TimeZoneDataPayloads,
    pub(super) format_units: SmallVec<[TimeZoneFormatterUnit; 3]>,
    pub(super) fallback_unit: TimeZoneFormatterUnit,
}

/// A container contains all data payloads for CustomTimeZone.
#[derive(Debug)]
pub(super) struct TimeZoneDataPayloads {
    /// The data that contains meta information about how to display content.
    pub(super) zone_formats: DataPayload<provider::time_zones::TimeZoneFormatsV1Marker>,
    /// The exemplar cities for time zones.
    pub(super) exemplar_cities: Option<DataPayload<provider::time_zones::ExemplarCitiesV1Marker>>,
    /// The generic long metazone names, e.g. Pacific Time
    pub(super) mz_generic_long:
        Option<DataPayload<provider::time_zones::MetazoneGenericNamesLongV1Marker>>,
    /// The generic short metazone names, e.g. PT
    pub(super) mz_generic_short:
        Option<DataPayload<provider::time_zones::MetazoneGenericNamesShortV1Marker>>,
    /// The specific long metazone names, e.g. Pacific Daylight Time
    pub(super) mz_specific_long:
        Option<DataPayload<provider::time_zones::MetazoneSpecificNamesLongV1Marker>>,
    /// The specific short metazone names, e.g. Pacific Daylight Time
    pub(super) mz_specific_short:
        Option<DataPayload<provider::time_zones::MetazoneSpecificNamesShortV1Marker>>,
}

impl TimeZoneFormatter {
    /// Constructor that selectively loads data based on what is required to
    /// format the given pattern into the given locale.
    pub(super) fn try_new_for_pattern<ZP>(
        zone_provider: &ZP,
        locale: &DataLocale,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        options: &TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + DataProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneGenericNamesShortV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesLongV1Marker>
            + DataProvider<provider::time_zones::MetazoneSpecificNamesShortV1Marker>
            + ?Sized,
    {
        let format_units = SmallVec::<[TimeZoneFormatterUnit; 3]>::new();
        let data_payloads = TimeZoneDataPayloads {
            zone_formats: zone_provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?,
            exemplar_cities: None,
            mz_generic_long: None,
            mz_generic_short: None,
            mz_specific_long: None,
            mz_specific_short: None,
        };

        let zone_symbols = patterns
            .get()
            .0
            .patterns_iter()
            .flat_map(|pattern| pattern.items.iter())
            .filter_map(|item| match item {
                PatternItem::Field(field) => Some(field),
                _ => None,
            })
            .filter_map(|field| match field.symbol {
                FieldSymbol::TimeZone(zone) => Some((field.length.idx(), zone)),
                _ => None,
            });

        let mut tz_format: TimeZoneFormatter = Self {
            data_payloads,
            // TODO(#2237): Determine whether we need to save the locale in the formatter
            locale: locale.clone(),
            format_units,
            fallback_unit: TimeZoneFormatter::get_fallback_unit(options.fallback_format),
        };
        let mut prev_length = None;
        let mut prev_symbol = None;
        for (length, symbol) in zone_symbols {
            if prev_length.is_none() && prev_symbol.is_none() {
                prev_length = Some(length);
                prev_symbol = Some(symbol);
            } else if prev_length != Some(length) && prev_symbol != Some(symbol) {
                // We don't support the pattern that has multiple different timezone fields of different types.
                return Err(DateTimeError::Pattern(PatternError::UnsupportedPluralPivot));
            }

            match symbol {
                TimeZone::LowerZ => match length {
                    1..=3 => {
                        tz_format.load_specific_non_location_short(zone_provider)?;
                    }
                    4 => {
                        tz_format.load_specific_non_location_long(zone_provider)?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::LowerV => match length {
                    1 => {
                        tz_format.load_generic_non_location_short(zone_provider)?;
                    }
                    4 => {
                        tz_format.load_generic_non_location_long(zone_provider)?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::UpperV => match length {
                    1 => (), // BCP-47 identifier, no CLDR-data necessary.
                    2 => (), // IANA time-zone ID, no CLDR data necessary.
                    3 => {
                        tz_format.load_exemplar_city_format(zone_provider)?;
                    }
                    4 => {
                        tz_format.load_generic_location_format(zone_provider)?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::UpperZ => match length {
                    1..=3 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::Basic,
                            IsoMinutes::Required,
                            IsoSeconds::Optional,
                        )?;
                    }
                    4 => {
                        tz_format.load_localized_gmt_format()?;
                    }
                    5 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::UtcExtended,
                            IsoMinutes::Required,
                            IsoSeconds::Optional,
                        )?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::LowerX => match length {
                    1 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::UtcBasic,
                            IsoMinutes::Optional,
                            IsoSeconds::Never,
                        )?;
                    }
                    2 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::UtcBasic,
                            IsoMinutes::Required,
                            IsoSeconds::Never,
                        )?;
                    }
                    3 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::UtcExtended,
                            IsoMinutes::Required,
                            IsoSeconds::Never,
                        )?;
                    }
                    4 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::UtcBasic,
                            IsoMinutes::Required,
                            IsoSeconds::Optional,
                        )?;
                    }
                    5 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::UtcExtended,
                            IsoMinutes::Required,
                            IsoSeconds::Optional,
                        )?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::UpperX => match length {
                    1 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::Basic,
                            IsoMinutes::Optional,
                            IsoSeconds::Never,
                        )?;
                    }
                    2 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::Basic,
                            IsoMinutes::Required,
                            IsoSeconds::Never,
                        )?;
                    }
                    3 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::Extended,
                            IsoMinutes::Required,
                            IsoSeconds::Never,
                        )?;
                    }
                    4 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::Basic,
                            IsoMinutes::Required,
                            IsoSeconds::Optional,
                        )?;
                    }
                    5 => {
                        tz_format.load_iso_8601_format(
                            IsoFormat::Extended,
                            IsoMinutes::Required,
                            IsoSeconds::Optional,
                        )?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::UpperO => match length {
                    1..=4 => {
                        tz_format.load_localized_gmt_format()?;
                    }
                    _ => {
                        return Err(DateTimeError::Pattern(PatternError::FieldLengthInvalid(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
            }
        }
        Ok(tz_format)
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: TimeZoneFormatterOptions,
        error: DateTimeError,
        /// Creates a new [`TimeZoneFormatter`] with a GMT or ISO format using compiled data.
        ///
        /// To enable other time zone styles, use one of the `with` (compiled data) or `load` (runtime
        /// data provider) methods.
        ///
        /// # Examples
        ///
        /// Default format is Localized GMT:
        ///
        /// ```
        /// use icu::datetime::time_zone::{
        ///     TimeZoneFormatter, TimeZoneFormatterOptions,
        /// };
        /// use icu::locid::locale;
        /// use icu::timezone::CustomTimeZone;
        /// use writeable::assert_writeable_eq;
        ///
        /// let tzf = TimeZoneFormatter::try_new(
        ///     &locale!("es").into(),
        ///     TimeZoneFormatterOptions::default(),
        /// )
        /// .unwrap();
        ///
        /// let time_zone = "-0700".parse::<CustomTimeZone>().unwrap();
        ///
        /// assert_writeable_eq!(tzf.format(&time_zone), "GMT-07:00");
        /// ```
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
        options: TimeZoneFormatterOptions,
    ) -> Result<Self, DateTimeError>
    where
        P: DataProvider<provider::time_zones::TimeZoneFormatsV1Marker> + ?Sized,
    {
        let format_units = SmallVec::<[TimeZoneFormatterUnit; 3]>::new();
        let data_payloads = TimeZoneDataPayloads {
            zone_formats: provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?,
            exemplar_cities: None,
            mz_generic_long: None,
            mz_generic_short: None,
            mz_specific_long: None,
            mz_specific_short: None,
        };
        Ok(Self {
            data_payloads,
            locale: locale.clone(),
            format_units,
            fallback_unit: TimeZoneFormatter::get_fallback_unit(options.fallback_format),
        })
    }

    /// Load generic non location long format for timezone from compiled data. For example, Pacific Time.
    #[cfg(feature = "data")]
    pub fn with_generic_non_location_long(
        &mut self,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.load_generic_non_location_long(&crate::provider::Baked)
    }

    /// Load generic non location short format for timezone from compiled data. For example, PT.
    #[cfg(feature = "data")]
    pub fn with_generic_non_location_short(
        &mut self,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.load_generic_non_location_short(&crate::provider::Baked)
    }

    /// Load specific non location long format for timezone from compiled data. For example, Pacific Standard Time.
    #[cfg(feature = "data")]
    pub fn with_specific_non_location_long(
        &mut self,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.load_specific_non_location_long(&crate::provider::Baked)
    }

    /// Load specific non location short format for timezone from compiled data. For example, PDT.
    #[cfg(feature = "data")]
    pub fn with_specific_non_location_short(
        &mut self,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.load_specific_non_location_short(&crate::provider::Baked)
    }

    /// Load generic location format for timezone. For example, Los Angeles Time.
    #[cfg(feature = "data")]
    pub fn with_generic_location_format(
        &mut self,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.load_generic_location_format(&crate::provider::Baked)
    }

    /// Load generic non location long format for timezone. For example, Pacific Time.
    pub fn load_generic_non_location_long<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::MetazoneGenericNamesLongV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_generic_long.is_none() {
            load(
                &self.locale,
                &mut self.data_payloads.mz_generic_long,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatterUnit::GenericNonLocationLong(
                GenericNonLocationLongFormat {},
            ));
        Ok(self)
    }

    /// Load generic non location short format for timezone. For example, PT.
    pub fn load_generic_non_location_short<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::MetazoneGenericNamesShortV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_generic_short.is_none() {
            load(
                &self.locale,
                &mut self.data_payloads.mz_generic_short,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatterUnit::GenericNonLocationShort(
                GenericNonLocationShortFormat {},
            ));
        Ok(self)
    }

    /// Load specific non location long format for timezone. For example, Pacific Standard Time.
    pub fn load_specific_non_location_long<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::MetazoneSpecificNamesLongV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_specific_long.is_none() {
            load(
                &self.locale,
                &mut self.data_payloads.mz_specific_long,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatterUnit::SpecificNonLocationLong(
                SpecificNonLocationLongFormat {},
            ));
        Ok(self)
    }

    /// Load specific non location short format for timezone. For example, PDT.
    pub fn load_specific_non_location_short<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::MetazoneSpecificNamesShortV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_specific_short.is_none() {
            load(
                &self.locale,
                &mut self.data_payloads.mz_specific_short,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatterUnit::SpecificNonLocationShort(
                SpecificNonLocationShortFormat {},
            ));
        Ok(self)
    }

    /// Load generic location format for timezone. For example, Los Angeles Time.
    pub fn load_generic_location_format<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::ExemplarCitiesV1Marker> + ?Sized,
    {
        if self.data_payloads.exemplar_cities.is_none() {
            load(
                &self.locale,
                &mut self.data_payloads.exemplar_cities,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatterUnit::GenericLocation(
                GenericLocationFormat {},
            ));
        Ok(self)
    }

    /// Load exemplar city format for timezone. For example, Los Angeles.
    fn load_exemplar_city_format<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError>
    where
        ZP: DataProvider<provider::time_zones::ExemplarCitiesV1Marker> + ?Sized,
    {
        if self.data_payloads.exemplar_cities.is_none() {
            load(
                &self.locale,
                &mut self.data_payloads.exemplar_cities,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatterUnit::ExemplarCity(ExemplarCityFormat {}));
        Ok(self)
    }

    /// Load localized GMT format for timezone. For example, GMT-07:00.
    pub fn load_localized_gmt_format(&mut self) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.format_units
            .push(TimeZoneFormatterUnit::LocalizedGmt(LocalizedGmtFormat {}));
        Ok(self)
    }

    /// Load Iso8601 format for timezone. For example, -07:00.
    pub fn load_iso_8601_format(
        &mut self,
        format: IsoFormat,
        minutes: IsoMinutes,
        seconds: IsoSeconds,
    ) -> Result<&mut TimeZoneFormatter, DateTimeError> {
        self.format_units
            .push(TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                format,
                minutes,
                seconds,
            }));
        Ok(self)
    }

    /// Load a fallback format for timezone. The fallback format will be executed if there are no
    /// matching format results.
    pub(super) fn get_fallback_unit(fallback_format: FallbackFormat) -> TimeZoneFormatterUnit {
        match fallback_format {
            FallbackFormat::LocalizedGmt => {
                TimeZoneFormatterUnit::LocalizedGmt(LocalizedGmtFormat {})
            }
            FallbackFormat::Iso8601(format, minutes, seconds) => {
                TimeZoneFormatterUnit::Iso8601(Iso8601Format {
                    format,
                    minutes,
                    seconds,
                })
            }
        }
    }

    /// Takes a [`TimeZoneInput`] implementer and returns an instance of a [`FormattedTimeZone`]
    /// that contains all information necessary to display a formatted time zone and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::datetime::time_zone::{
    ///     TimeZoneFormatter, TimeZoneFormatterOptions,
    /// };
    /// use icu::locid::locale;
    /// use icu::timezone::CustomTimeZone;
    /// use writeable::assert_writeable_eq;
    ///
    /// let tzf = TimeZoneFormatter::try_new(
    ///     &locale!("en").into(),
    ///     TimeZoneFormatterOptions::default(),
    /// )
    /// .expect("Failed to create TimeZoneFormatter");
    ///
    /// let time_zone = CustomTimeZone::utc();
    ///
    /// assert_writeable_eq!(tzf.format(&time_zone), "GMT");
    /// ```
    pub fn format<'l, T>(&'l self, value: &'l T) -> FormattedTimeZone<'l, T>
    where
        T: TimeZoneInput,
    {
        FormattedTimeZone {
            time_zone_format: self,
            time_zone: value,
        }
    }

    /// Takes a [`TimeZoneInput`] implementer and returns a string with the formatted value.
    pub fn format_to_string(&self, value: &impl TimeZoneInput) -> String {
        self.format(value).write_to_string().into_owned()
    }

    /// Formats a time segment with optional zero-padding.
    fn format_time_segment(n: u8, padding: ZeroPadding) -> String {
        debug_assert!((0..60).contains(&n));
        match padding {
            ZeroPadding::On => format!("{n:>02}"),
            ZeroPadding::Off => format!("{n}"),
        }
    }

    /// Formats the hours as a [`String`] with optional zero-padding.
    fn format_offset_hours(
        time_zone: &impl TimeZoneInput,
        padding: ZeroPadding,
    ) -> Result<String, DateTimeError> {
        if let Some(gmt_offset) = time_zone.gmt_offset() {
            Ok(TimeZoneFormatter::format_time_segment(
                (gmt_offset.offset_seconds() / 3600).unsigned_abs() as u8,
                padding,
            ))
        } else {
            Err(DateTimeError::MissingInputField(Some("gmt_offset")))
        }
    }

    /// Formats the minutes as a [`String`] with zero-padding.
    fn format_offset_minutes(time_zone: &impl TimeZoneInput) -> Result<String, DateTimeError> {
        if let Some(gmt_offset) = time_zone.gmt_offset() {
            Ok(TimeZoneFormatter::format_time_segment(
                (gmt_offset.offset_seconds() % 3600 / 60).unsigned_abs() as u8,
                ZeroPadding::On,
            ))
        } else {
            Err(DateTimeError::MissingInputField(Some("gmt_offset")))
        }
    }

    /// Formats the seconds as a [`String`] with zero-padding.
    fn format_offset_seconds<W: fmt::Write + ?Sized>(
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
    ) -> Result<fmt::Result, DateTimeError> {
        if let Some(gmt_offset) = time_zone.gmt_offset() {
            Ok(sink.write_str(&TimeZoneFormatter::format_time_segment(
                (gmt_offset.offset_seconds() % 3600 % 60).unsigned_abs() as u8,
                ZeroPadding::On,
            )))
        } else {
            Err(DateTimeError::MissingInputField(Some("gmt_offset")))
        }
    }
}

/// Determines which ISO-8601 format should be used to format a [`GmtOffset`](icu_timezone::GmtOffset).
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum IsoFormat {
    /// ISO-8601 Basic Format.
    /// Formats zero-offset numerically.
    /// e.g. +0500, +0000
    Basic,

    /// ISO-8601 Extended Format.
    /// Formats zero-offset numerically.
    /// e.g. +05:00, +00:00
    Extended,

    /// ISO-8601 Basic Format.
    /// Formats zero-offset with the ISO-8601 UTC indicator: "Z"
    /// e.g. +0500, Z
    UtcBasic,

    /// ISO-8601 Extended Format.
    /// Formats zero-offset with the ISO-8601 UTC indicator: "Z"
    /// e.g. +05:00, Z
    UtcExtended,
}

/// Whether the minutes field should be optional or required in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum IsoMinutes {
    /// Minutes are always displayed.
    Required,

    /// Minutes are displayed only if they are non-zero.
    Optional,
}

/// Whether the seconds field should be optional or excluded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub enum IsoSeconds {
    /// Seconds are displayed only if they are non-zero.
    Optional,

    /// Seconds are not displayed.
    Never,
}

/// Whether a field should be zero-padded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::exhaustive_enums)] // this type is stable
pub(crate) enum ZeroPadding {
    /// Add zero-padding.
    On,

    /// Do not add zero-padding.
    Off,
}

/// An enum for time zone fallback formats.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum FallbackFormat {
    /// The ISO 8601 format for time zone format fallback.
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
    /// The localized GMT format for time zone format fallback.
    ///
    /// See [UTS 35 on Dates](https://unicode.org/reports/tr35/tr35-dates.html#71-time-zone-format-terminology) for more information.
    LocalizedGmt,
}

impl Default for FallbackFormat {
    fn default() -> Self {
        FallbackFormat::LocalizedGmt
    }
}

/// A bag of options to define how time zone will be formatted.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct TimeZoneFormatterOptions {
    /// The time zone format fallback option.
    ///
    /// See [UTS 35 on Dates](https://unicode.org/reports/tr35/tr35-dates.html#71-time-zone-format-terminology) for more information.
    pub fallback_format: FallbackFormat,
}

impl From<FallbackFormat> for TimeZoneFormatterOptions {
    fn from(fallback_format: FallbackFormat) -> Self {
        Self { fallback_format }
    }
}

// Pacific Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct GenericNonLocationLongFormat {}

// PT
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct GenericNonLocationShortFormat {}

// Pacific Standard Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SpecificNonLocationLongFormat {}

// PDT
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct SpecificNonLocationShortFormat {}

// Los Angeles Time
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct GenericLocationFormat {}

// GMT-07:00
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct LocalizedGmtFormat {}

// -07:00
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Iso8601Format {
    format: IsoFormat,
    minutes: IsoMinutes,
    seconds: IsoSeconds,
}

// It is only used for pattern in special case and not public to users.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ExemplarCityFormat {}

// An enum for time zone format unit.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum TimeZoneFormatterUnit {
    GenericNonLocationLong(GenericNonLocationLongFormat),
    GenericNonLocationShort(GenericNonLocationShortFormat),
    SpecificNonLocationLong(SpecificNonLocationLongFormat),
    SpecificNonLocationShort(SpecificNonLocationShortFormat),
    GenericLocation(GenericLocationFormat),
    LocalizedGmt(LocalizedGmtFormat),
    Iso8601(Iso8601Format),
    ExemplarCity(ExemplarCityFormat),
}

impl Default for TimeZoneFormatterUnit {
    fn default() -> Self {
        TimeZoneFormatterUnit::LocalizedGmt(LocalizedGmtFormat {})
    }
}

pub(super) trait FormatTimeZone {
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError>;
}

impl FormatTimeZone for TimeZoneFormatterUnit {
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        match self {
            Self::GenericNonLocationLong(unit) => unit.format(sink, time_zone, data_payloads),
            Self::GenericNonLocationShort(unit) => unit.format(sink, time_zone, data_payloads),
            Self::SpecificNonLocationLong(unit) => unit.format(sink, time_zone, data_payloads),
            Self::SpecificNonLocationShort(unit) => unit.format(sink, time_zone, data_payloads),
            Self::GenericLocation(unit) => unit.format(sink, time_zone, data_payloads),
            Self::LocalizedGmt(unit) => unit.format(sink, time_zone, data_payloads),
            Self::Iso8601(unit) => unit.format(sink, time_zone, data_payloads),
            Self::ExemplarCity(unit) => unit.format(sink, time_zone, data_payloads),
        }
    }
}

impl FormatTimeZone for GenericNonLocationLongFormat {
    /// Writes the time zone in long generic non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_generic_long
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone
                    .time_zone_id()
                    .and_then(|tz| metazones.overrides.get(&tz))
            })
            .or_else(|| {
                data_payloads
                    .mz_generic_long
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|metazones| {
                        time_zone
                            .metazone_id()
                            .and_then(|mz| metazones.defaults.get(&mz))
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeError::UnsupportedOptions),
        }
    }
}

impl FormatTimeZone for GenericNonLocationShortFormat {
    /// Writes the time zone in short generic non-location format as defined by the UTS-35 spec.
    /// e.g. PT
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_generic_short
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone
                    .time_zone_id()
                    .and_then(|tz| metazones.overrides.get(&tz))
            })
            .or_else(|| {
                data_payloads
                    .mz_generic_short
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|metazones| {
                        time_zone
                            .metazone_id()
                            .and_then(|mz| metazones.defaults.get(&mz))
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeError::UnsupportedOptions),
        }
    }
}

impl FormatTimeZone for SpecificNonLocationShortFormat {
    /// Writes the time zone in short specific non-location format as defined by the UTS-35 spec.
    /// e.g. PDT
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_specific_short
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone.time_zone_id().and_then(|tz| {
                    time_zone
                        .zone_variant()
                        .and_then(|variant| metazones.overrides.get_2d(&tz, &variant))
                })
            })
            .or_else(|| {
                data_payloads
                    .mz_specific_short
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|metazones| {
                        time_zone.metazone_id().and_then(|mz| {
                            time_zone
                                .zone_variant()
                                .and_then(|variant| metazones.defaults.get_2d(&mz, &variant))
                        })
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeError::UnsupportedOptions),
        }
    }
}

impl FormatTimeZone for SpecificNonLocationLongFormat {
    /// Writes the time zone in long specific non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Daylight Time
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_specific_long
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone.time_zone_id().and_then(|tz| {
                    time_zone
                        .zone_variant()
                        .and_then(|variant| metazones.overrides.get_2d(&tz, &variant))
                })
            })
            .or_else(|| {
                data_payloads
                    .mz_specific_long
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|metazones| {
                        time_zone.metazone_id().and_then(|mz| {
                            time_zone
                                .zone_variant()
                                .and_then(|variant| metazones.defaults.get_2d(&mz, &variant))
                        })
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeError::UnsupportedOptions),
        }
    }
}

impl FormatTimeZone for LocalizedGmtFormat {
    /// Writes the time zone in localized GMT format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// GMT formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        if let Some(gmt_offset) = time_zone.gmt_offset() {
            return if gmt_offset.is_zero() {
                Ok(sink.write_str(&data_payloads.zone_formats.get().gmt_zero_format.clone()))
            } else {
                // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
                Ok(sink.write_str(
                    &data_payloads
                        .zone_formats
                        .get()
                        .gmt_format
                        .replace(
                            "{0}",
                            if gmt_offset.is_positive() {
                                &data_payloads.zone_formats.get().hour_format.0
                            } else {
                                &data_payloads.zone_formats.get().hour_format.1
                            },
                        )
                        // support all combos of "(HH|H):mm" by replacing longest patterns first.
                        .replace(
                            "HH",
                            if let Ok(offset_hours) =
                                &TimeZoneFormatter::format_offset_hours(time_zone, ZeroPadding::On)
                            {
                                offset_hours
                            } else {
                                return Err(DateTimeError::MissingInputField(Some("gmt_offset")));
                            },
                        )
                        .replace(
                            "mm",
                            if let Ok(offset_minutes) =
                                &TimeZoneFormatter::format_offset_minutes(time_zone)
                            {
                                offset_minutes
                            } else {
                                return Err(DateTimeError::MissingInputField(Some("gmt_offset")));
                            },
                        )
                        .replace(
                            'H',
                            if let Ok(offset_hours) =
                                &TimeZoneFormatter::format_offset_hours(time_zone, ZeroPadding::Off)
                            {
                                offset_hours
                            } else {
                                return Err(DateTimeError::MissingInputField(Some("gmt_offset")));
                            },
                        ),
                ))
            };
        };
        Err(DateTimeError::MissingInputField(Some("gmt_offset")))
    }
}

impl FormatTimeZone for GenericLocationFormat {
    /// Writes the time zone in generic location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
        let formatted_time_zone: Option<alloc::string::String> = data_payloads
            .exemplar_cities
            .as_ref()
            .map(|p| p.get())
            .and_then(|cities| time_zone.time_zone_id().and_then(|id| cities.0.get(&id)))
            .map(|location| {
                data_payloads
                    .zone_formats
                    .get()
                    .region_format
                    .replace("{0}", location)
            });
        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(&ftz)),
            None => Err(DateTimeError::UnsupportedOptions),
        }
    }
}

impl FormatTimeZone for Iso8601Format {
    /// Writes a [`GmtOffset`](crate::input::GmtOffset) in ISO-8601 format according to the
    /// given formatting options.
    ///
    /// [`IsoFormat`] determines whether the format should be Basic or Extended,
    /// and whether a zero-offset should be formatted numerically or with
    /// The UTC indicator: "Z"
    /// - Basic    e.g. +0800
    /// - Extended e.g. +08:00
    ///
    /// [`IsoMinutes`] can be required or optional.
    /// [`IsoSeconds`] can be optional or never.
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        _data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        if let Some(gmt_offset) = time_zone.gmt_offset() {
            if gmt_offset.is_zero()
                && matches!(self.format, IsoFormat::UtcBasic | IsoFormat::UtcExtended)
            {
                if let Err(e) = sink.write_char('Z') {
                    return Ok(Err(e));
                }
            }

            let extended_format =
                matches!(self.format, IsoFormat::Extended | IsoFormat::UtcExtended);
            if let Err(e) = sink.write_char(if gmt_offset.is_positive() { '+' } else { '-' }) {
                return Ok(Err(e));
            }
            if let Ok(offset_hours) =
                &TimeZoneFormatter::format_offset_hours(time_zone, ZeroPadding::On)
            {
                if let Err(e) = sink.write_str(offset_hours) {
                    return Ok(Err(e));
                }
            } else {
                return Err(DateTimeError::MissingInputField(Some("gmt_offset")));
            }

            match self.minutes {
                IsoMinutes::Required => {
                    if extended_format {
                        if let Err(e) = sink.write_char(':') {
                            return Ok(Err(e));
                        }
                    }
                    if let Ok(offset_minutes) = &TimeZoneFormatter::format_offset_minutes(time_zone)
                    {
                        if let Err(e) = sink.write_str(offset_minutes) {
                            return Ok(Err(e));
                        }
                    } else {
                        return Err(DateTimeError::MissingInputField(Some("gmt_offset")));
                    }
                }
                IsoMinutes::Optional => {
                    if gmt_offset.has_minutes() {
                        if extended_format {
                            if let Err(e) = sink.write_char(':') {
                                return Ok(Err(e));
                            }
                        }
                        if let Ok(offset_minutes) =
                            &TimeZoneFormatter::format_offset_minutes(time_zone)
                        {
                            if let Err(e) = sink.write_str(offset_minutes) {
                                return Ok(Err(e));
                            }
                        } else {
                            return Err(DateTimeError::MissingInputField(Some("gmt_offset")));
                        }
                    }
                }
            }

            if let IsoSeconds::Optional = self.seconds {
                if gmt_offset.has_seconds() {
                    if extended_format {
                        if let Err(e) = sink.write_char(':') {
                            return Ok(Err(e));
                        }
                    }
                    return TimeZoneFormatter::format_offset_seconds(sink, time_zone);
                }
            }
            return Ok(Ok(()));
        };
        Err(DateTimeError::MissingInputField(Some("gmt_offset")))
    }
}

impl FormatTimeZone for ExemplarCityFormat {
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeError> {
        // Writes the exemplar city associated with this time zone.
        let formatted_exemplar_city = data_payloads
            .exemplar_cities
            .as_ref()
            .map(|p| p.get())
            .and_then(|cities| time_zone.time_zone_id().and_then(|id| cities.0.get(&id)));

        match formatted_exemplar_city {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => {
                // Writes the unknown city "Etc/Unknown" for the current locale.
                //
                // If there is no localized form of "Etc/Unknown" for the current locale,
                // returns the "Etc/Unknown" value of the `und` locale as a hard-coded string.
                //
                // This can be used as a fallback if [`exemplar_city()`](TimeZoneFormatter::exemplar_city())
                // is unable to produce a localized form of the time zone's exemplar city in the current locale.
                let formatted_unknown_city = data_payloads
                    .exemplar_cities
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|cities| cities.0.get(&TimeZoneBcp47Id(tinystr!(8, "unk"))))
                    .unwrap_or(&Cow::Borrowed("Unknown"));
                Ok(sink.write_str(formatted_unknown_city))
            }
        }
    }
}
