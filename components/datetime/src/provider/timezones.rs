use std::borrow::Borrow;
use std::borrow::Cow;
pub type LiteMap<K, V> = std::collections::BTreeMap<K, V>;
//use litemap::LiteMap;

macro_rules! map_access {
    ($outer: ty => $inner: ty) => {
        impl $outer {
            fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$inner>
            where
                Q: Ord,
                Cow<'static, str>: std::borrow::Borrow<Q>,
            {
                self.0.get(key)
            }
        }

        impl<Q: ?Sized> std::ops::Index<&Q> for $outer
        where
            Q: Ord,
            Cow<'static, str>: std::borrow::Borrow<Q>,
        {
            type Output = $inner;
            fn index(&self, key: &Q) -> &Self::Output {
                &self.0[key]
            }
        }
    };
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TimeZonesV1 {
    pub hour_format: Cow<'static, str>,
    pub gmt_format: Cow<'static, str>,
    pub gmt_zero_format: Cow<'static, str>,
    pub region_format: Cow<'static, str>,
    pub region_format_daylight: Cow<'static, str>,
    pub region_format_standard: Cow<'static, str>,
    pub fallback_format: Cow<'static, str>,
    pub zones: ZonesV1,
    pub metazones: Option<MetaZonesV1>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LocationV1 {
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub long: Option<ZoneFormatV1>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub short: Option<ZoneFormatV1>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub exemplar_city: Option<Cow<'static, str>>,
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ZoneFormatV1 {
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub generic: Option<Cow<'static, str>>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub standard: Option<Cow<'static, str>>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub daylight: Option<Cow<'static, str>>,
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ZonesV1(pub LiteMap<Cow<'static, str>, LocationV1>);
map_access!(ZonesV1 => LocationV1);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneV1 {
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub long: Option<ZoneFormatV1>,
    #[cfg_attr(
        all(feature = "provider_serde", not(feature = "serialize_none")),
        serde(skip_serializing_if = "Option::is_none")
    )]
    pub short: Option<ZoneFormatV1>,
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZonesV1(pub LiteMap<Cow<'static, str>, MetaZoneV1>);
map_access!(MetaZonesV1 => MetaZoneV1);
