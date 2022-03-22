// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use alloc::format;
use alloc::string::String;
use core::fmt;
use smallvec::SmallVec;

use crate::{
    date::TimeZoneInput,
    error::DateTimeFormatError,
    fields::{FieldSymbol, TimeZone},
    format::time_zone::FormattedTimeZone,
    pattern::{PatternError, PatternItem},
    provider::{self, calendar::patterns::PatternPluralsFromPatternsV1Marker},
};
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::prelude::*;
use writeable::Writeable;

/// Loads a resource into its destination if the destination has not already been filled.
fn load_resource<D, L, P>(
    locale: &L,
    destination: &mut Option<DataPayload<D>>,
    provider: &P,
) -> Result<(), DateTimeFormatError>
where
    D: ResourceMarker,
    L: Clone + Into<LanguageIdentifier>,
    P: ResourceProvider<D> + ?Sized,
{
    if destination.is_none() {
        *destination = Some(
            provider
                .load_resource(&DataRequest {
                    options: ResourceOptions {
                        langid: Some(locale.clone().into()),
                        variant: None,
                    },
                    metadata: Default::default(),
                })?
                .take_payload()?,
        );
    }
    Ok(())
}

/// [`TimeZoneFormat`] uses data from the [data provider], the selected [`Locale`], and the provided
/// [`TimeZoneFormatConfig`] to collect all data necessary to format time zones into that locale.
///
/// The various time-zone configs specified in UTS-35 require different sets of data for
/// formatting. As such,[`TimeZoneFormat`] will pull in only the resources needed to format the
/// config that it is given upon construction.
///
/// For that reason, one should think of the process of formatting a time zone in two steps:
/// first, a computationally heavy construction of [`TimeZoneFormat`], and then fast formatting
/// of the time-zone data using the instance.
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_datetime::{TimeZoneFormat, TimeZoneFormatConfig};
/// use icu_datetime::date::GmtOffset;
/// use icu_datetime::mock::time_zone::MockTimeZone;
/// use icu_provider::inv::InvariantDataProvider;
///
/// let provider = InvariantDataProvider;
///
/// let tzf = TimeZoneFormat::try_from_config(locale!("en"), TimeZoneFormatConfig::GenericNonLocationLong, &provider, &TimeZoneFormatOptions::default())
///     .expect("Failed to create TimeZoneFormat");
///
/// let time_zone = MockTimeZone::new(
///        GmtOffset::default(),
///        None,
///        None,
///        None,
/// );
///
/// let value = tzf.format_to_string(&time_zone);
/// ```
///
/// [data provider]: icu_provider
pub struct TimeZoneFormat {
    // The kind of time zone format.
    pub(super) kind: TimeZoneFormatKind,
    pub(super) locale: Locale,
    pub(super) data_payloads: TimeZoneDataPayloads,
    pub(super) format_units: SmallVec<[TimeZoneFormatUnit; 3]>,
    pub(super) fallback_unit: Option<TimeZoneFormatUnit>,
}

/// A container contains all data payloads for TimeZone.
pub struct TimeZoneDataPayloads {
    /// The data that contains meta information about how to display content.
    pub(super) zone_formats: DataPayload<provider::time_zones::TimeZoneFormatsV1Marker>,
    /// The exemplar cities for time zones.
    pub(super) exemplar_cities: Option<DataPayload<provider::time_zones::ExemplarCitiesV1Marker>>,
    /// The generic long metazone names, e.g. Pacific Time
    pub(super) mz_generic_long:
        Option<DataPayload<provider::time_zones::MetaZoneGenericNamesLongV1Marker>>,
    /// The generic short metazone names, e.g. PT
    pub(super) mz_generic_short:
        Option<DataPayload<provider::time_zones::MetaZoneGenericNamesShortV1Marker>>,
    /// The specific long metazone names, e.g. Pacific Daylight Time
    pub(super) mz_specific_long:
        Option<DataPayload<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>>,
    /// The specific short metazone names, e.g. Pacific Daylight Time
    pub(super) mz_specific_short:
        Option<DataPayload<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>>,
}

impl TimeZoneFormat {
    /// Constructor that selectively loads data based on what is required to
    /// format the given pattern into the given locale.
    pub(super) fn try_new<L, ZP>(
        locale: L,
        patterns: DataPayload<PatternPluralsFromPatternsV1Marker>,
        zone_provider: &ZP,
        options: &TimeZoneFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        ZP: ResourceProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
    {
        let locale = locale.into();
        let format_units = SmallVec::<[TimeZoneFormatUnit; 3]>::new();
        let data_payloads = TimeZoneDataPayloads {
            zone_formats: zone_provider
                .load_resource(&DataRequest {
                    options: locale.clone().into(),
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
            .map(|pattern| pattern.items.iter())
            .flatten()
            .filter_map(|item| match item {
                PatternItem::Field(field) => Some(field),
                _ => None,
            })
            .filter_map(|field| match field.symbol {
                FieldSymbol::TimeZone(zone) => Some((field.length.idx(), zone)),
                _ => None,
            });

        let mut tz_format: TimeZoneFormat = Self {
            kind: TimeZoneFormatKind::Pattern(patterns.clone()),
            data_payloads,
            locale,
            format_units,
            fallback_unit: None,
        };

        let mut count_zone_symbols = 0;
        for (length, symbol) in zone_symbols {
            count_zone_symbols += 1;
            match symbol {
                TimeZone::LowerZ => match length {
                    1..=3 => {
                        tz_format.load_specific_non_location_short(zone_provider)?;
                    }
                    4 => {
                        tz_format.load_specific_non_location_long(zone_provider)?;
                    }
                    _ => {
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
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
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
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
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
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
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
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
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
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
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
                    }
                },
                TimeZone::UpperO => match length {
                    1..=4 => {
                        tz_format.load_localized_gmt_format()?;
                    }
                    _ => {
                        return Err(DateTimeFormatError::Pattern(
                            PatternError::FieldLengthInvalid(FieldSymbol::TimeZone(symbol)),
                        ))
                    }
                },
            }
        }

        if count_zone_symbols > 1 {
            return Err(DateTimeFormatError::UnsupportedOptions);
        }
        tz_format.load_fallback_format(options.fallback_format)?;
        Ok(tz_format)
    }

    /// Constructor that selectively loads data based on what is required to
    /// format the given config into the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_datetime::{TimeZoneFormat, TimeZoneFormatConfig};
    /// use icu_datetime::mock::time_zone::MockTimeZone;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let tzf = TimeZoneFormat::try_from_config(locale!("en"), TimeZoneFormatConfig::LocalizedGMT, &provider, &TimeZoneFormatOptions::default());
    ///
    /// assert!(tzf.is_ok());
    /// ```
    pub fn try_from_config<L, ZP>(
        locale: L,
        config: TimeZoneFormatConfig,
        zone_provider: &ZP,
        options: &TimeZoneFormatOptions,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        ZP: ResourceProvider<provider::time_zones::TimeZoneFormatsV1Marker>
            + ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker>
            + ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker>
            + ?Sized,
    {
        let locale = locale.into();
        let format_units = SmallVec::<[TimeZoneFormatUnit; 3]>::new();
        let data_payloads = TimeZoneDataPayloads {
            zone_formats: zone_provider
                .load_resource(&DataRequest {
                    options: locale.clone().into(),
                    metadata: Default::default(),
                })?
                .take_payload()?,
            exemplar_cities: None,
            mz_generic_long: None,
            mz_generic_short: None,
            mz_specific_long: None,
            mz_specific_short: None,
        };

        let mut tz_format: TimeZoneFormat = Self {
            kind: TimeZoneFormatKind::Config(config),
            data_payloads,
            locale,
            format_units,
            fallback_unit: None,
        };

        match config {
            TimeZoneFormatConfig::GenericNonLocationLong => {
                tz_format.load_generic_non_location_long(zone_provider)?;
            }
            TimeZoneFormatConfig::GenericNonLocationShort => {
                tz_format.load_generic_non_location_short(zone_provider)?;
            }
            TimeZoneFormatConfig::GenericLocation => {
                tz_format.load_generic_location_format(zone_provider)?;
            }
            TimeZoneFormatConfig::SpecificNonLocationLong => {
                tz_format.load_specific_non_location_long(zone_provider)?;
            }
            TimeZoneFormatConfig::SpecificNonLocationShort => {
                tz_format.load_specific_non_location_short(zone_provider)?;
            }
            TimeZoneFormatConfig::LocalizedGMT => {
                tz_format.load_localized_gmt_format()?;
            }
            TimeZoneFormatConfig::Iso8601(format, minutes, seconds) => {
                tz_format.load_iso_8601_format(format, minutes, seconds)?;
            }
        }
        tz_format.load_fallback_format(options.fallback_format)?;
        Ok(tz_format)
    }

    /// Load generic non location long format for timezone. For example, Pacific Time.
    pub fn load_generic_non_location_long<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError>
    where
        ZP: ResourceProvider<provider::time_zones::MetaZoneGenericNamesLongV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_generic_long == None {
            load_resource(
                &self.locale,
                &mut self.data_payloads.mz_generic_long,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatUnit::GenericNonLocationLong(
                GenericNonLocationLongFormat {},
            ));
        Ok(self)
    }

    /// Load generic non location short format for timezone. For example, PT.
    pub fn load_generic_non_location_short<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError>
    where
        ZP: ResourceProvider<provider::time_zones::MetaZoneGenericNamesShortV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_generic_short == None {
            load_resource(
                &self.locale,
                &mut self.data_payloads.mz_generic_short,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatUnit::GenericNonLocationShort(
                GenericNonLocationShortFormat {},
            ));
        Ok(self)
    }

    /// Load specific non location long format for timezone. For example, Pacific Standard Time.
    pub fn load_specific_non_location_long<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError>
    where
        ZP: ResourceProvider<provider::time_zones::MetaZoneSpecificNamesLongV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_specific_long == None {
            load_resource(
                &self.locale,
                &mut self.data_payloads.mz_specific_long,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatUnit::SpecificNonLocationLong(
                SpecificNonLocationLongFormat {},
            ));
        Ok(self)
    }

    /// Load specific non location short format for timezone. For example, PDT.
    pub fn load_specific_non_location_short<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError>
    where
        ZP: ResourceProvider<provider::time_zones::MetaZoneSpecificNamesShortV1Marker> + ?Sized,
    {
        if self.data_payloads.mz_specific_short == None {
            load_resource(
                &self.locale,
                &mut self.data_payloads.mz_specific_short,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatUnit::SpecificNonLocationShort(
                SpecificNonLocationShortFormat {},
            ));
        Ok(self)
    }

    /// Load generic location format for timezone. For example, Los Angeles Time.
    pub fn load_generic_location_format<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError>
    where
        ZP: ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker> + ?Sized,
    {
        if self.data_payloads.exemplar_cities == None {
            load_resource(
                &self.locale,
                &mut self.data_payloads.exemplar_cities,
                zone_provider,
            )?;
        }
        self.format_units.push(TimeZoneFormatUnit::GenericLocation(
            GenericLocationFormat {},
        ));
        Ok(self)
    }

    /// Load exemplar city format for timezone. For example, Los Angeles.
    fn load_exemplar_city_format<ZP>(
        &mut self,
        zone_provider: &ZP,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError>
    where
        ZP: ResourceProvider<provider::time_zones::ExemplarCitiesV1Marker> + ?Sized,
    {
        if self.data_payloads.exemplar_cities == None {
            load_resource(
                &self.locale,
                &mut self.data_payloads.exemplar_cities,
                zone_provider,
            )?;
        }
        self.format_units
            .push(TimeZoneFormatUnit::ExemplarCity(ExemplarCityFormat {}));
        Ok(self)
    }

    /// Load localized GMT format for timezone. For example, GMT-07:00.
    pub fn load_localized_gmt_format(
        &mut self,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError> {
        self.format_units
            .push(TimeZoneFormatUnit::LocalizedGmt(LocalizedGmtFormat {}));
        Ok(self)
    }

    /// Load Iso8601 format for timezone. For example, -07:00.
    pub fn load_iso_8601_format(
        &mut self,
        format: IsoFormat,
        minutes: IsoMinutes,
        seconds: IsoSeconds,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError> {
        self.format_units
            .push(TimeZoneFormatUnit::Iso8601(Iso8601Format {
                format,
                minutes,
                seconds,
            }));
        Ok(self)
    }

    /// Load a fallback format for timezone. The fallback format will be executed if there are no
    /// matching format results.
    pub fn load_fallback_format(
        &mut self,
        fallback_format: Option<FallbackFormat>,
    ) -> Result<&mut TimeZoneFormat, DateTimeFormatError> {
        match fallback_format {
            None => {}
            Some(fallback) => match fallback {
                FallbackFormat::LocalizedGmt => {
                    self.fallback_unit =
                        Some(TimeZoneFormatUnit::LocalizedGmt(LocalizedGmtFormat {}));
                }
                FallbackFormat::Iso8601(format, minutes, seconds) => {
                    self.fallback_unit = Some(TimeZoneFormatUnit::Iso8601(Iso8601Format {
                        format,
                        minutes,
                        seconds,
                    }));
                }
            },
        }
        Ok(self)
    }

    /// Takes a [`TimeZoneInput`] implementer and returns an instance of a [`FormattedTimeZone`]
    /// that contains all information necessary to display a formatted time zone and operate on it.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_datetime::{TimeZoneFormat, TimeZoneFormatConfig};
    /// use icu_datetime::date::GmtOffset;
    /// use icu_datetime::mock::time_zone::MockTimeZone;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let tzf = TimeZoneFormat::try_from_config(locale!("en"), TimeZoneFormatConfig::LocalizedGMT, &provider, &TimeZoneFormatOptions::default())
    ///     .expect("Failed to create TimeZoneFormat");
    ///
    /// let time_zone = MockTimeZone::new(
    ///        GmtOffset::default(),
    ///        None,
    ///        None,
    ///        None,
    /// );
    ///
    /// let _ = tzf.format(&time_zone);
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

    /// Takes a mutable reference to anything that implements the [`Write`](std::fmt::Write)
    /// trait and a [`TimeZoneInput`] implementer that populates the buffer with a formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_datetime::{TimeZoneFormat, TimeZoneFormatConfig};
    /// use icu_datetime::date::GmtOffset;
    /// use icu_datetime::mock::time_zone::MockTimeZone;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let tzf = TimeZoneFormat::try_from_config(locale!("en"), TimeZoneFormatConfig::LocalizedGMT, &provider, &TimeZoneFormatOptions::default())
    ///     .expect("Failed to create TimeZoneFormat");
    ///
    /// let time_zone = MockTimeZone::new(
    ///        GmtOffset::default(),
    ///        None,
    ///        None,
    ///        None,
    /// );
    ///
    /// let mut buffer = String::new();
    /// tzf.format_to_write(&mut buffer, &time_zone)
    ///     .expect("Failed to write to a buffer.");
    ///
    /// let _ = format!("Time Zone: {}", buffer);
    /// ```
    pub fn format_to_write(
        &self,
        w: &mut impl core::fmt::Write,
        value: &impl TimeZoneInput,
    ) -> fmt::Result {
        self.format(value).write_to(w)
    }

    /// Takes a [`TimeZoneInput`] implementer and returns a string with the formatted value.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_datetime::{TimeZoneFormat, TimeZoneFormatConfig};
    /// use icu_datetime::date::GmtOffset;
    /// use icu_datetime::mock::time_zone::MockTimeZone;
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let provider = InvariantDataProvider;
    ///
    /// let tzf = TimeZoneFormat::try_from_config(locale!("en"), TimeZoneFormatConfig::LocalizedGMT, &provider, &TimeZoneFormatOptions::default())
    ///     .expect("Failed to create TimeZoneFormat");
    ///
    /// let time_zone = MockTimeZone::new(
    ///        GmtOffset::default(),
    ///        None,
    ///        None,
    ///        None,
    /// );
    ///
    /// let _ = tzf.format_to_string(&time_zone);
    /// ```
    pub fn format_to_string(&self, value: &impl TimeZoneInput) -> String {
        let mut s = String::new();
        #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }

    /// Formats a time segment with optional zero-padding.
    fn format_time_segment(n: u8, padding: ZeroPadding) -> String {
        debug_assert!((0..60).contains(&n));
        match padding {
            ZeroPadding::On => format!("{:>02}", n),
            ZeroPadding::Off => format!("{}", n),
        }
    }

    /// Formats the hours as a [`String`] with optional zero-padding.
    fn format_offset_hours(time_zone: &impl TimeZoneInput, padding: ZeroPadding) -> String {
        TimeZoneFormat::format_time_segment(
            (time_zone.gmt_offset().raw_offset_seconds() / 3600).abs() as u8,
            padding,
        )
    }

    /// Formats the minutes as a [`String`] with zero-padding.
    fn format_offset_minutes(time_zone: &impl TimeZoneInput) -> String {
        TimeZoneFormat::format_time_segment(
            (time_zone.gmt_offset().raw_offset_seconds() % 3600 / 60).abs() as u8,
            ZeroPadding::On,
        )
    }

    /// Formats the seconds as a [`String`] with zero-padding.
    fn format_offset_seconds(time_zone: &impl TimeZoneInput) -> String {
        TimeZoneFormat::format_time_segment(
            (time_zone.gmt_offset().raw_offset_seconds() % 3600 % 60).abs() as u8,
            ZeroPadding::On,
        )
    }
}

/// Determines which type of formats time zone uses. It can be either config or pattern.
#[allow(clippy::large_enum_variant)]
pub(super) enum TimeZoneFormatKind {
    Config(TimeZoneFormatConfig),
    Pattern(DataPayload<PatternPluralsFromPatternsV1Marker>),
}

/// Determines which ISO-8601 format should be used to format a [`GmtOffset`](crate::date::GmtOffset).
#[derive(Debug, Clone, Copy, PartialEq)]
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
pub enum IsoMinutes {
    /// Minutes are always displayed.
    Required,

    /// Minutes are displayed only if they are non-zero.
    Optional,
}

/// Whether the seconds field should be optional or excluded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsoSeconds {
    /// Seconds are displayed only if they are non-zero.
    Optional,

    /// Seconds are not displayed.
    Never,
}

/// Whether a field should be zero-padded in ISO-8601 format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZeroPadding {
    /// Add zero-padding.
    On,

    /// Do not add zero-padding.
    Off,
}

/// A config enum for initializing TimeZoneFormat.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeZoneFormatConfig {
    GenericNonLocationLong,                     // Pacific Time
    GenericNonLocationShort,                    // PT
    GenericLocation,                            // Los Angeles Time
    SpecificNonLocationLong,                    // Pacific Standard Time
    SpecificNonLocationShort,                   // PDT
    LocalizedGMT,                               // GMT-07:00
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds), // -07:00
}

/// An enum for fallback formats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FallbackFormat {
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
    LocalizedGmt,
}

/// A bag of options to define how time zone will be formatted.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct TimeZoneFormatOptions {
    pub fallback_format: Option<FallbackFormat>,
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
pub(super) enum TimeZoneFormatUnit {
    GenericNonLocationLong(GenericNonLocationLongFormat),
    GenericNonLocationShort(GenericNonLocationShortFormat),
    SpecificNonLocationLong(SpecificNonLocationLongFormat),
    SpecificNonLocationShort(SpecificNonLocationShortFormat),
    GenericLocation(GenericLocationFormat),
    LocalizedGmt(LocalizedGmtFormat),
    Iso8601(Iso8601Format),
    ExemplarCity(ExemplarCityFormat),
}

impl Default for TimeZoneFormatUnit {
    fn default() -> Self {
        TimeZoneFormatUnit::LocalizedGmt(LocalizedGmtFormat {})
    }
}

pub(super) trait FormatTimeZone {
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeFormatError>;
}

impl FormatTimeZone for TimeZoneFormatUnit {
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeFormatError> {
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_generic_long
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone
                    .time_zone_id()
                    .and_then(|tz| metazones.overrides.get(tz))
            })
            .or_else(|| {
                data_payloads
                    .mz_generic_long
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|metazones| {
                        time_zone
                            .metazone_id()
                            .and_then(|mz| metazones.defaults.get(mz))
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeFormatError::UnsupportedOptions),
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_generic_short
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone
                    .time_zone_id()
                    .and_then(|tz| metazones.overrides.get(tz))
            })
            .or_else(|| {
                data_payloads
                    .mz_generic_short
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|metazones| {
                        time_zone
                            .metazone_id()
                            .and_then(|mz| metazones.defaults.get(mz))
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeFormatError::UnsupportedOptions),
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_specific_short
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone.time_zone_id().and_then(|tz| {
                    time_zone
                        .time_variant()
                        .and_then(|variant| metazones.overrides.get(tz, variant).ok())
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
                                .time_variant()
                                .and_then(|variant| metazones.defaults.get(mz, variant).ok())
                        })
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeFormatError::UnsupportedOptions),
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        let formatted_time_zone: Option<&str> = data_payloads
            .mz_specific_long
            .as_ref()
            .map(|p| p.get())
            .and_then(|metazones| {
                time_zone.time_zone_id().and_then(|tz| {
                    time_zone
                        .time_variant()
                        .and_then(|variant| metazones.overrides.get(tz, variant).ok())
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
                                .time_variant()
                                .and_then(|variant| metazones.defaults.get(mz, variant).ok())
                        })
                    })
            });

        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => Err(DateTimeFormatError::UnsupportedOptions),
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        let gmt_offset = time_zone.gmt_offset();
        if gmt_offset.is_zero() {
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
                        &TimeZoneFormat::format_offset_hours(time_zone, ZeroPadding::On),
                    )
                    .replace("mm", &TimeZoneFormat::format_offset_minutes(time_zone))
                    .replace(
                        "H",
                        &TimeZoneFormat::format_offset_hours(time_zone, ZeroPadding::Off),
                    ),
            ))
        }
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
        let formatted_time_zone: Option<alloc::string::String> = data_payloads
            .exemplar_cities
            .as_ref()
            .map(|p| p.get())
            .and_then(|cities| time_zone.time_zone_id().and_then(|id| cities.0.get(id)))
            .map(|location| {
                data_payloads
                    .zone_formats
                    .get()
                    .region_format
                    .replace("{0}", location)
            });
        match formatted_time_zone {
            Some(ftz) => Ok(sink.write_str(&ftz)),
            None => Err(DateTimeFormatError::UnsupportedOptions),
        }
    }
}

impl FormatTimeZone for Iso8601Format {
    /// Writes a [`GmtOffset`](crate::date::GmtOffset) in ISO-8601 format according to the
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
    ) -> Result<fmt::Result, DateTimeFormatError> {
        let gmt_offset = time_zone.gmt_offset();
        if gmt_offset.is_zero()
            && matches!(self.format, IsoFormat::UtcBasic | IsoFormat::UtcExtended)
        {
            if let Err(e) = sink.write_char('Z') {
                return Ok(Err(e));
            }
        }

        let extended_format = matches!(self.format, IsoFormat::Extended | IsoFormat::UtcExtended);
        if let Err(e) = sink.write_char(if gmt_offset.is_positive() { '+' } else { '-' }) {
            return Ok(Err(e));
        }
        if let Err(e) = sink.write_str(&TimeZoneFormat::format_offset_hours(
            time_zone,
            ZeroPadding::On,
        )) {
            return Ok(Err(e));
        }

        match self.minutes {
            IsoMinutes::Required => {
                if extended_format {
                    if let Err(e) = sink.write_char(':') {
                        return Ok(Err(e));
                    }
                }
                if let Err(e) = sink.write_str(&TimeZoneFormat::format_offset_minutes(time_zone)) {
                    return Ok(Err(e));
                }
            }
            IsoMinutes::Optional => {
                if gmt_offset.has_minutes() {
                    if extended_format {
                        if let Err(e) = sink.write_char(':') {
                            return Ok(Err(e));
                        }
                    }
                    if let Err(e) =
                        sink.write_str(&TimeZoneFormat::format_offset_minutes(time_zone))
                    {
                        return Ok(Err(e));
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
                if let Err(e) = sink.write_str(&TimeZoneFormat::format_offset_seconds(time_zone)) {
                    return Ok(Err(e));
                }
            }
        }
        Ok(Ok(()))
    }
}

impl FormatTimeZone for ExemplarCityFormat {
    fn format<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        time_zone: &impl TimeZoneInput,
        data_payloads: &TimeZoneDataPayloads,
    ) -> Result<fmt::Result, DateTimeFormatError> {
        // Writes the exemplar city associated with this time zone.
        let formatted_exemplar_city = data_payloads
            .exemplar_cities
            .as_ref()
            .map(|p| p.get())
            .and_then(|cities| time_zone.time_zone_id().and_then(|id| cities.0.get(id)));

        match formatted_exemplar_city {
            Some(ftz) => Ok(sink.write_str(ftz)),
            None => {
                // Writes the unknown city "Etc/Unknown" for the current locale.
                //
                // If there is no localized form of "Etc/Unknown" for the current locale,
                // returns the "Etc/Uknown" value of the `und` locale as a hard-coded string.
                //
                // This can be used as a fallback if [`exemplar_city()`](TimeZoneFormat::exemplar_city())
                // is unable to produce a localized form of the time zone's exemplar city in the current locale.
                let formatted_unknown_city = data_payloads
                    .exemplar_cities
                    .as_ref()
                    .map(|p| p.get())
                    .and_then(|cities| cities.0.get("Etc/Unknown"))
                    .unwrap_or(&Cow::Borrowed("Unknown"));
                Ok(sink.write_str(formatted_unknown_city))
            }
        }
    }
}
