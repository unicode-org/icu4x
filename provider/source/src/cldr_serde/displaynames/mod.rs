// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod language;
pub(crate) mod locale_display_pattern;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

use core::str::FromStr;
use serde::{Deserialize, Deserializer};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct SubtagWithOptionalAltVariant<T> {
    pub(crate) subtag: T,
    pub(crate) alt_variant: Option<String>,
    pub(crate) menu_variant: Option<String>,
}

impl<'de, T> Deserialize<'de> for SubtagWithOptionalAltVariant<T>
where
    T: FromStr,
    T::Err: core::fmt::Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: FromStr,
            T::Err: core::fmt::Display,
        {
            type Value = SubtagWithOptionalAltVariant<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string with optional -alt- or -menu- suffix")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some(index) = v.rfind("-menu-") {
                    let (subtag_str, menu_str) = v.split_at(index);
                    let menu_variant = menu_str.strip_prefix("-menu-").unwrap().to_string();
                    let subtag = T::from_str(subtag_str).map_err(E::custom)?;
                    Ok(SubtagWithOptionalAltVariant {
                        subtag,
                        alt_variant: None,
                        menu_variant: Some(menu_variant),
                    })
                } else if let Some(index) = v.rfind("-alt-") {
                    let (subtag_str, alt_str) = v.split_at(index);
                    let alt_variant = alt_str.strip_prefix("-alt-").unwrap().to_string();
                    let subtag = T::from_str(subtag_str).map_err(E::custom)?;
                    Ok(SubtagWithOptionalAltVariant {
                        subtag,
                        alt_variant: Some(alt_variant),
                        menu_variant: None,
                    })
                } else {
                    let subtag = T::from_str(v).map_err(E::custom)?;
                    Ok(SubtagWithOptionalAltVariant {
                        subtag,
                        alt_variant: None,
                        menu_variant: None,
                    })
                }
            }
        }

        deserializer.deserialize_str(Visitor(std::marker::PhantomData))
    }
}
