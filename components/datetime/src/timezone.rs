// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

use crate::{
    date::{TimeZoneInput, ZeroPadding},
    format::timezone::FormattedTimeZone,
    pattern::Error as PatternError,
    provider, DateTimeFormatError,
};
use crate::{format::timezone, provider::timezones::TimeZoneFormatsV1};
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::{DataProvider, DataRequest, ResourceKey, ResourceOptions, ResourcePath};

use crate::fields::{FieldSymbol, TimeZone};
use crate::pattern::{Pattern, PatternItem};

/// Loads a resource into its destination if the destination has not already been filled.
fn load_resource<'d, D, L, P>(
    locale: &L,
    resource_key: ResourceKey,
    destination: &mut Option<Cow<'d, D>>,
    provider: &P,
) -> Result<(), DateTimeFormatError>
where
    D: std::fmt::Debug + Clone + 'd,
    L: Clone + Into<LanguageIdentifier>,
    P: DataProvider<'d, D> + ?Sized,
{
    if destination.is_none() {
        *destination = Some(
            provider
                .load_payload(&DataRequest {
                    resource_path: ResourcePath {
                        key: resource_key,
                        options: ResourceOptions {
                            variant: None,
                            langid: Some(locale.clone().into()),
                        },
                    },
                })?
                .payload
                .take()?,
        );
    }
    Ok(())
}

pub struct TimeZoneFormat<'d> {
    pub(super) pattern: Pattern,
    pub(super) zone_formats: Cow<'d, provider::timezones::TimeZoneFormatsV1<'d>>,
    pub(super) exemplar_cities: Option<Cow<'d, provider::timezones::ExemplarCitiesV1<'d>>>,
    pub(super) mz_generic_long:
        Option<Cow<'d, provider::timezones::MetaZoneGenericNamesLongV1<'d>>>,
    pub(super) mz_generic_short:
        Option<Cow<'d, provider::timezones::MetaZoneGenericNamesShortV1<'d>>>,
    pub(super) mz_specific_long:
        Option<Cow<'d, provider::timezones::MetaZoneSpecificNamesLongV1<'d>>>,
    pub(super) mz_specific_short:
        Option<Cow<'d, provider::timezones::MetaZoneSpecificNamesShortV1<'d>>>,
}

impl<'d> TimeZoneFormat<'d> {
    pub fn try_new<L, ZP>(
        locale: L,
        pattern: Pattern,
        zone_provider: &ZP,
    ) -> Result<Self, DateTimeFormatError>
    where
        L: Into<Locale>,
        ZP: DataProvider<'d, provider::timezones::TimeZoneFormatsV1<'d>>
            + DataProvider<'d, provider::timezones::ExemplarCitiesV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneGenericNamesShortV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesLongV1<'d>>
            + DataProvider<'d, provider::timezones::MetaZoneSpecificNamesShortV1<'d>>
            + ?Sized,
    {
        let locale = locale.into();

        let zone_formats: Cow<TimeZoneFormatsV1> = zone_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: provider::key::TIMEZONE_FORMATS_V1,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.clone().into()),
                    },
                },
            })?
            .payload
            .take()?;

        let mut time_zone_format = Self {
            pattern,
            zone_formats,
            exemplar_cities: None,
            mz_generic_long: None,
            mz_generic_short: None,
            mz_specific_long: None,
            mz_specific_short: None,
        };

        let zone_symbols = time_zone_format
            .pattern
            .items()
            .iter()
            .filter_map(|item| match item {
                PatternItem::Field(field) => Some(field),
                _ => None,
            })
            .filter_map(|field| match field.symbol {
                FieldSymbol::TimeZone(zone) => Some((u8::from(field.length), zone)),
                _ => None,
            });

        for (length, symbol) in zone_symbols {
            match symbol {
                TimeZone::LowerZ => match length {
                    1..=3 => load_resource(
                        &locale,
                        provider::key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1,
                        &mut time_zone_format.mz_specific_short,
                        zone_provider,
                    )?,
                    4 => load_resource(
                        &locale,
                        provider::key::TIMEZONE_SPECIFIC_NAMES_LONG_V1,
                        &mut time_zone_format.mz_specific_long,
                        zone_provider,
                    )?,
                    _ => {
                        return Err(DateTimeFormatError::Pattern(PatternError::FieldTooLong(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::LowerV => match length {
                    1 => {
                        load_resource(
                            &locale,
                            provider::key::TIMEZONE_GENERIC_NAMES_SHORT_V1,
                            &mut time_zone_format.mz_generic_short,
                            zone_provider,
                        )?;
                        load_resource(
                            &locale,
                            provider::key::TIMEZONE_EXEMPLAR_CITIES_V1,
                            &mut time_zone_format.exemplar_cities,
                            zone_provider,
                        )?;
                    }
                    4 => {
                        load_resource(
                            &locale,
                            provider::key::TIMEZONE_GENERIC_NAMES_LONG_V1,
                            &mut time_zone_format.mz_generic_long,
                            zone_provider,
                        )?;
                        load_resource(
                            &locale,
                            provider::key::TIMEZONE_EXEMPLAR_CITIES_V1,
                            &mut time_zone_format.exemplar_cities,
                            zone_provider,
                        )?;
                    }
                    _ => {
                        return Err(DateTimeFormatError::Pattern(PatternError::FieldTooLong(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                TimeZone::UpperV => match length {
                    1 => (), // BCP-47 identifier, no CLDR-data necessary.
                    2 => (), // IANA time-zone ID, no CLDR data necessary.
                    3 | 4 => load_resource(
                        &locale,
                        provider::key::TIMEZONE_EXEMPLAR_CITIES_V1,
                        &mut time_zone_format.exemplar_cities,
                        zone_provider,
                    )?,
                    _ => {
                        return Err(DateTimeFormatError::Pattern(PatternError::FieldTooLong(
                            FieldSymbol::TimeZone(symbol),
                        )))
                    }
                },
                // ISO-8601 or localized GMT formats. CLDR data is either unneeded or required by default.
                TimeZone::LowerX | TimeZone::UpperX | TimeZone::UpperZ | TimeZone::UpperO => (),
            }
        }

        Ok(time_zone_format)
    }

    pub fn format<'s: 'd, T>(&'s self, value: &'s T) -> FormattedTimeZone<'s, T>
    where
        T: TimeZoneInput,
    {
        FormattedTimeZone {
            time_zone_format: self,
            time_zone: value,
        }
    }

    pub fn format_to_write(
        &self,
        w: &mut impl std::fmt::Write,
        value: &impl TimeZoneInput,
    ) -> std::fmt::Result {
        timezone::write_pattern(self, value, w).map_err(|_| std::fmt::Error)
    }

    pub fn format_to_string(&self, value: &impl TimeZoneInput) -> String {
        let mut s = String::new();
        self.format_to_write(&mut s, value)
            .expect("Failed to write to a String.");
        s
    }

    /// Returns the time zone in generic location format as defined by the UTS-35 spec.
    /// e.g. France Time
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    pub(super) fn generic_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<'d, str>> {
        // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
        self.exemplar_city(time_zone)
            .map(|location| Cow::Owned(self.zone_formats.region_format.replace("{0}", &location)))
    }

    /// Returns the time zone in short generic non-location format as defined by the UTS-35 spec.
    /// e.g. PT
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    pub(super) fn short_generic_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<'d, str>> {
        self.mz_generic_short
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
    }

    /// Returns the time zone in long generic non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Time
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    pub(super) fn long_generic_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<'d, str>> {
        self.mz_generic_long
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
    }

    /// Returns the time zone in short specific non-location format as defined by the UTS-35 spec.
    /// e.g. PDT
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    pub(super) fn short_specific_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<'d, str>> {
        self.mz_specific_short
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
            .and_then(|specific_names| {
                time_zone
                    .time_variant()
                    .and_then(|variant| specific_names.get(variant))
            })
    }

    /// Returns the time zone in long specific non-location format as defined by the UTS-35 spec.
    /// e.g. Pacific Daylight Time
    /// https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Format_Terminology
    pub(super) fn long_specific_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<'d, str>> {
        self.mz_specific_long
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
            .and_then(|specific_names| {
                time_zone
                    .time_variant()
                    .and_then(|variant| specific_names.get(variant))
            })
    }

    /// Returns the time zone in localized GMT format according to the CLDR localized hour format.
    /// This goes explicitly against the UTS-35 spec, which specifies long or short localized
    /// GMT formats regardless of locale.
    ///
    /// You can see more information about our decision to resolve this conflict here:
    /// https://docs.google.com/document/d/16GAqaDRS6hzL8jNYjus5MglSevGBflISM-BrIS7bd4A/edit?usp=sharing
    pub(super) fn localized_gmt_format(&self, time_zone: &impl TimeZoneInput) -> Cow<str> {
        let gmt_offset = time_zone.gmt_offset();
        if gmt_offset.is_zero() {
            self.zone_formats.gmt_zero_format.clone()
        } else {
            // TODO(blocked on #277) Use formatter utility instead of replacing "{0}".
            Cow::Owned(
                self.zone_formats
                    .gmt_format
                    .replace(
                        "{0}",
                        if gmt_offset.is_positive() {
                            &self.zone_formats.hour_format.0
                        } else {
                            &self.zone_formats.hour_format.1
                        },
                    )
                    // support all combos of "(HH|H):mm" by replacing longest patterns first.
                    .replace("HH", &gmt_offset.format_hours(ZeroPadding::On))
                    .replace("mm", &gmt_offset.format_minutes())
                    .replace("H", &gmt_offset.format_hours(ZeroPadding::Off)),
            )
        }
    }

    /// Returns the exemplar city associated with this time zone.
    pub(super) fn exemplar_city(&self, time_zone: &impl TimeZoneInput) -> Option<Cow<str>> {
        self.exemplar_cities
            .as_ref()
            .and_then(|cities| time_zone.time_zone_id().and_then(|id| cities.get(id)))
    }

    /// Returns the unknown city. This is used as a fallback in case the time-zone's exemplar city
    /// had no localized form in the current locale.
    ///
    /// This function is only invoked as a fallback, which means that we can guarantee that the
    /// exemplar-city data is present, and all locales should have an entry for "Etc/Unknown".
    pub(super) fn unknown_city(&self) -> Cow<str> {
        self.exemplar_cities
            .as_ref()
            .expect("Exemplar City data was not present.")
            .get("Etc/Unknown")
            .expect("Etc/Unknown is not contained in exemplar city data.")
    }
}
