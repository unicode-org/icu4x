// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use litemap::LiteMap;
use std::borrow::Cow;

macro_rules! map_access {
    ($outer: ty => $inner: ty: $lt: lifetime) => {
        impl<$lt> $outer {
            pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&$inner>
            where
                Q: Ord,
                Cow<'s, str>: std::borrow::Borrow<Q>,
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
            Cow<'s, str>: std::borrow::Borrow<Q>,
        {
            type Output = $inner;
            fn index(&self, key: &Q) -> &Self::Output {
                &self.0.get(key).unwrap()
            }
        }
    };
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TimeZoneFormatsV1<'s> {
    pub hour_format: Cow<'s, str>,
    pub gmt_format: Cow<'s, str>,
    pub gmt_zero_format: Cow<'s, str>,
    pub region_format: Cow<'s, str>,
    pub region_format_variants: LiteMap<Cow<'s, str>, Cow<'s, str>>,
    pub fallback_format: Cow<'s, str>,
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ExemplarCitiesV1<'s>(pub LiteMap<Cow<'s, str>, Cow<'s, str>>);
map_access!(ExemplarCitiesV1<'s> => Cow<'s, str>: 's);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneGenericNamesLongV1<'s>(pub LiteMap<Cow<'s, str>, Cow<'s, str>>);
map_access!(MetaZoneGenericNamesLongV1<'s> => Cow<'s, str>: 's);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneGenericNamesShortV1<'s>(pub LiteMap<Cow<'s, str>, Cow<'s, str>>);
map_access!(MetaZoneGenericNamesShortV1<'s> => Cow<'s, str>: 's);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneSpecificNamesLongV1<'s>(pub LiteMap<Cow<'s, str>, MetaZoneSpecificNamesV1<'s>>);
map_access!(MetaZoneSpecificNamesLongV1<'s> => MetaZoneSpecificNamesV1<'s>: 's);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneSpecificNamesShortV1<'s>(pub LiteMap<Cow<'s, str>, MetaZoneSpecificNamesV1<'s>>);
map_access!(MetaZoneSpecificNamesShortV1<'s> => MetaZoneSpecificNamesV1<'s>: 's);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct MetaZoneSpecificNamesV1<'s>(pub LiteMap<Cow<'s, str>, Cow<'s, str>>);
map_access!(MetaZoneSpecificNamesV1<'s> => Cow<'s, str>: 's);
