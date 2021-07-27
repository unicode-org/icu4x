// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::yoke::{self, *};
use litemap::LiteMap;
use std::borrow::Cow;
use tinystr::TinyStr8;

/// Provides a few common map accessor methods to new-type structs that wrap a map type.
/// The methods are all pass-through calls to the internal methods of the same name.
macro_rules! map_access {
    ($outer: ty[$key: ty] => $inner: ty: $lt: lifetime) => {
        impl<$lt> $outer {
            pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$inner>
            where
                Q: Ord,
                Cow<'data, $key>: std::borrow::Borrow<Q>,
            {
                self.0.get(key)
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<$lt, Q: ?Sized> std::ops::Index<&Q> for $outer
        where
            Q: Ord,
            Cow<'data, $key>: std::borrow::Borrow<Q>,
        {
            type Output = $inner;
            fn index(&self, key: &Q) -> &Self::Output {
                self.0.get(key).unwrap()
            }
        }
    };
}

/// An ICU4X mapping to the CLDR timeZoneNames format strings.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct TimeZoneFormatsV1<'data> {
    /// The hour format for displaying GMT offsets.
    pub hour_format: (Cow<'data, str>, Cow<'data, str>),
    /// The localized GMT-offset format.
    pub gmt_format: Cow<'data, str>,
    /// The localized GMT format with no offset.
    pub gmt_zero_format: Cow<'data, str>,
    /// The format string for a region.
    pub region_format: Cow<'data, str>,
    /// The format strings for region format variants
    /// e.g. daylight, standard.
    pub region_format_variants: LiteMap<Cow<'data, TinyStr8>, Cow<'data, str>>,
    /// The format string to fall back to if data is unavailable.
    pub fallback_format: Cow<'data, str>,
}

/// An ICU4X mapping to the CLDR timeZoneNames exemplar cities.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct ExemplarCitiesV1<'data>(pub LiteMap<Cow<'data, str>, Cow<'data, str>>);
map_access!(ExemplarCitiesV1<'data>[str] => Cow<'data, str>: 'data);

/// An ICU4X mapping to the long-form generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct MetaZoneGenericNamesLongV1<'data>(pub LiteMap<Cow<'data, str>, Cow<'data, str>>);
map_access!(MetaZoneGenericNamesLongV1<'data>[str] => Cow<'data, str>: 'data);

/// An ICU4X mapping to the short-form generic metazone names.
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct MetaZoneGenericNamesShortV1<'data>(pub LiteMap<Cow<'data, str>, Cow<'data, str>>);
map_access!(MetaZoneGenericNamesShortV1<'data>[str] => Cow<'data, str>: 'data);

/// An ICU4X mapping to the long-form specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct MetaZoneSpecificNamesLongV1<'data>(pub LiteMap<Cow<'data, str>, MetaZoneSpecificNamesV1<'data>>);
map_access!(MetaZoneSpecificNamesLongV1<'data>[str] => MetaZoneSpecificNamesV1<'data>: 'data);

/// An ICU4X mapping to the short-form specific metazone names.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct MetaZoneSpecificNamesShortV1<'data>(pub LiteMap<Cow<'data, str>, MetaZoneSpecificNamesV1<'data>>);
map_access!(MetaZoneSpecificNamesShortV1<'data>[str] => MetaZoneSpecificNamesV1<'data>: 'data);

/// A general struct to hold metazone specific name variants.
/// Specific names include time variants such as "daylight."
/// See CLDR-JSON timeZoneNames.json for more context.
#[icu_provider::data_struct]
#[derive(PartialEq, Debug, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct MetaZoneSpecificNamesV1<'data>(pub LiteMap<Cow<'data, TinyStr8>, Cow<'data, str>>);
map_access!(MetaZoneSpecificNamesV1<'data>[TinyStr8] => Cow<'data, str>: 'data);
