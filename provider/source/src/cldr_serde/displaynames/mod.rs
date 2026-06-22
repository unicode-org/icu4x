// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod language;
pub(crate) mod locale_display_pattern;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

use crate::displaynames::{ALT_SEPARATOR, MENU_SEPARATOR};
use core::str::FromStr;
use serde::{Deserialize, Deserializer};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct WithAlt<T> {
    pub(crate) subtag: T,
    pub(crate) alt: Option<String>,
    pub(crate) menu: Option<String>,
}

impl<'de, T> Deserialize<'de> for WithAlt<T>
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
            type Value = WithAlt<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string with optional -alt- or -menu- suffix")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some((subtag_str, menu_str)) = v.split_once(MENU_SEPARATOR) {
                    let subtag = T::from_str(subtag_str).map_err(E::custom)?;
                    Ok(WithAlt {
                        subtag,
                        alt: None,
                        menu: Some(menu_str.to_string()),
                    })
                } else if let Some((subtag_str, alt_str)) = v.split_once(ALT_SEPARATOR) {
                    let subtag = T::from_str(subtag_str).map_err(E::custom)?;
                    Ok(WithAlt {
                        subtag,
                        alt: Some(alt_str.to_string()),
                        menu: None,
                    })
                } else {
                    let subtag = T::from_str(v).map_err(E::custom)?;
                    Ok(WithAlt {
                        subtag,
                        alt: None,
                        menu: None,
                    })
                }
            }
        }

        deserializer.deserialize_str(Visitor(std::marker::PhantomData))
    }
}
