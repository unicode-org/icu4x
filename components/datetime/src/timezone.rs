// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;

use crate::{
    date::TimeZoneInput,
    format::timezone::FormattedTimeZone,
    provider::{
        self,
        timezones::{
            ExemplarCitiesV1, MetaZoneGenericNamesLongV1, MetaZoneGenericNamesShortV1,
            MetaZoneSpecificNamesLongV1, MetaZoneSpecificNamesShortV1, TimeZoneFormatsV1,
        },
    },
    DateTimeFormatError,
};
use crate::{format::timezone, invalid_pattern_symbol, mock::timezone::ZeroPadding};
use icu_locid::{LanguageIdentifier, Locale};
use icu_provider::{DataProvider, DataRequest, ResourceKey, ResourceOptions, ResourcePath};

use crate::fields::{FieldSymbol, TimeZone};
use crate::pattern::{Pattern, PatternItem};

/// Produces an iterator over the time-zone resource keys required to format a pattern.
fn required_time_zone_keys(pattern: &Pattern) -> impl Iterator<Item = ResourceKey> + '_ {
    pattern
        .items()
        .iter()
        .filter_map(|item| match item {
            PatternItem::Field(field) => Some(field),
            _ => None,
        })
        .filter_map(|field| match field.symbol {
            FieldSymbol::TimeZone(zone) => Some((u8::from(field.length), zone)),
            _ => None,
        })
        .filter_map(|(length, zone_symbol)| match zone_symbol {
            TimeZone::LowerZ => match length {
                1..=3 => Some(provider::key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1),
                4 => Some(provider::key::TIMEZONE_SPECIFIC_NAMES_LONG_V1),
                _ => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            TimeZone::LowerV => match length {
                1 => Some(provider::key::TIMEZONE_GENERIC_NAMES_SHORT_V1),
                4 => Some(provider::key::TIMEZONE_GENERIC_NAMES_LONG_V1),
                _ => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            TimeZone::UpperV => match length {
                1 => None, // BCP-47 identifier, no CLDR-data necessary.
                2 => None, // IANA time-zone ID, no CLDR data necessary.
                3 | 4 => Some(provider::key::TIMEZONE_EXEMPLAR_CITIES_V1),
                _ => invalid_pattern_symbol!(TimeZone, zone_symbol, length),
            },
            // ISO-8601 or localized GMT formats. CLDR data is either unneeded or required by default.
            TimeZone::LowerX | TimeZone::UpperX | TimeZone::UpperZ | TimeZone::UpperO => None,
        })
}

/// Loads a resource into its destionation if the destionation has not already been filled.
fn load_resource<'d, D, L, P>(
    locale: &L,
    resource_key: ResourceKey,
    destionation: &mut Option<Cow<'d, D>>,
    provider: &P,
) -> Result<(), DateTimeFormatError>
where
    D: std::fmt::Debug + Clone + 'd,
    L: Clone + Into<LanguageIdentifier>,
    P: DataProvider<'d, D> + ?Sized,
{
    if destionation.is_none() {
        *destionation = Some(
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

        let mut exemplar_cities: Option<Cow<ExemplarCitiesV1>> = None;
        let mut mz_generic_long: Option<Cow<MetaZoneGenericNamesLongV1>> = None;
        let mut mz_generic_short: Option<Cow<MetaZoneGenericNamesShortV1>> = None;
        let mut mz_specific_long: Option<Cow<MetaZoneSpecificNamesLongV1>> = None;
        let mut mz_specific_short: Option<Cow<MetaZoneSpecificNamesShortV1>> = None;

        for resource_key in required_time_zone_keys(&pattern) {
            if resource_key.eq(&provider::key::TIMEZONE_EXEMPLAR_CITIES_V1) {
                load_resource(&locale, resource_key, &mut exemplar_cities, zone_provider)?;
            } else if resource_key.eq(&provider::key::TIMEZONE_GENERIC_NAMES_LONG_V1) {
                load_resource(&locale, resource_key, &mut mz_generic_long, zone_provider)?;
            } else if resource_key.eq(&provider::key::TIMEZONE_GENERIC_NAMES_SHORT_V1) {
                load_resource(&locale, resource_key, &mut mz_generic_short, zone_provider)?;
            } else if resource_key.eq(&provider::key::TIMEZONE_SPECIFIC_NAMES_SHORT_V1) {
                load_resource(&locale, resource_key, &mut mz_specific_short, zone_provider)?;
            } else if resource_key.eq(&provider::key::TIMEZONE_SPECIFIC_NAMES_LONG_V1) {
                load_resource(&locale, resource_key, &mut mz_specific_long, zone_provider)?;
            }
        }

        Ok(Self {
            pattern,
            zone_formats,
            exemplar_cities,
            mz_generic_long,
            mz_generic_short,
            mz_specific_long,
            mz_specific_short,
        })
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

    pub fn generic_location_format(&self, time_zone: &impl TimeZoneInput) -> Option<Cow<str>> {
        self.exemplar_city(time_zone)
            .map(|location| Cow::Owned(self.zone_formats.region_format.replace("{0}", &location)))
    }

    pub fn short_generic_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<str>> {
        self.mz_generic_short
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
            .map(Clone::clone)
    }

    pub fn long_generic_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<str>> {
        self.mz_generic_long
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
            .map(Clone::clone)
    }

    pub fn short_specific_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<str>> {
        self.mz_specific_short
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
            .and_then(|specific_names| {
                time_zone
                    .time_variant()
                    .and_then(|variant| specific_names.get(variant))
            })
            .map(Clone::clone)
    }

    pub fn long_specific_non_location_format(
        &self,
        time_zone: &impl TimeZoneInput,
    ) -> Option<Cow<str>> {
        self.mz_specific_long
            .as_ref()
            .and_then(|metazones| time_zone.metazone_id().and_then(|mz| metazones.get(mz)))
            .and_then(|specific_names| {
                time_zone
                    .time_variant()
                    .and_then(|variant| specific_names.get(variant))
            })
            .map(Clone::clone)
    }

    pub fn localized_gmt_format(&self, time_zone: &impl TimeZoneInput) -> Cow<str> {
        let gmt_offset = time_zone.gmt_offset();
        if gmt_offset.is_zero() {
            self.zone_formats.gmt_zero_format.clone()
        } else {
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

    pub fn exemplar_city(&self, time_zone: &impl TimeZoneInput) -> Option<Cow<str>> {
        self.exemplar_cities
            .as_ref()
            .and_then(|cities| time_zone.time_zone_id().and_then(|id| cities.get(id)))
            .map(Clone::clone)
    }

    pub fn unknown_city(&self) -> Cow<str> {
        self.exemplar_cities
            .as_ref()
            .unwrap()
            .get("Etc/Unknown")
            .map(Clone::clone)
            .unwrap()
    }
}
