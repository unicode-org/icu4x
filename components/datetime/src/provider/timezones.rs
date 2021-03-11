use std::borrow::Cow;

pub type LiteMap<K, V> = std::collections::BTreeMap<K, V>;
//use litemap::LiteMap;

macro_rules! map_access {
    ($outer: ty => $inner: ty) => {
        impl $outer {
            pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$inner>
            where
                Q: Ord,
                Cow<'static, str>: std::borrow::Borrow<Q>,
            {
                self.0.get(key)
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
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
pub struct TimeZoneFormatsV1 {
    pub hour_format: Cow<'static, str>,
    pub gmt_format: Cow<'static, str>,
    pub gmt_zero_format: Cow<'static, str>,
    pub region_format: Cow<'static, str>,
    pub region_format_variants: LiteMap<Cow<'static, str>, Cow<'static, str>>,
    pub fallback_format: Cow<'static, str>,
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ExemplarCitiesV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(ExemplarCitiesV1 => Cow<'static, str>);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct LocationV1 {}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneGenericNamesLongV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(MetaZoneGenericNamesLongV1 => Cow<'static, str>);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneGenericNamesShortV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(MetaZoneGenericNamesShortV1 => Cow<'static, str>);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneSpecificNamesLongV1(pub LiteMap<Cow<'static, str>, MetaZoneSpecificNamesV1>);
map_access!(MetaZoneSpecificNamesLongV1 => MetaZoneSpecificNamesV1);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneSpecificNamesShortV1(pub LiteMap<Cow<'static, str>, MetaZoneSpecificNamesV1>);
map_access!(MetaZoneSpecificNamesShortV1 => MetaZoneSpecificNamesV1);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneSpecificNamesV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(MetaZoneSpecificNamesV1 => Cow<'static, str>);
