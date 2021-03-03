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
pub struct TimeZoneNamesLongV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(TimeZoneNamesLongV1 => Cow<'static, str>);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TimeZoneNamesShortV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(TimeZoneNamesShortV1 => Cow<'static, str>);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TimeZoneNameVariantsLongV1(pub LiteMap<Cow<'static, str>, TimeZoneNameVariantsV1>);
map_access!(TimeZoneNameVariantsLongV1 => TimeZoneNameVariantsV1);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TimeZoneNameVariantsShortV1(pub LiteMap<Cow<'static, str>, TimeZoneNameVariantsV1>);
map_access!(TimeZoneNameVariantsShortV1 => TimeZoneNameVariantsV1);

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct TimeZoneNameVariantsV1(pub LiteMap<Cow<'static, str>, Cow<'static, str>>);
map_access!(TimeZoneNameVariantsV1 => Cow<'static, str>);

