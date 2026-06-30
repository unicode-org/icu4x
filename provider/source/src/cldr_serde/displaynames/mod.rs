// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod language;
pub(crate) mod locale_display_pattern;
pub(crate) mod region;
pub(crate) mod script;
pub(crate) mod variant;

const ALT_SEPARATOR: &str = "-alt-";
const MENU_SEPARATOR: &str = "-menu-";
use core::fmt::{self, Display};
use core::marker::PhantomData;
use core::str::FromStr;
use serde::{Deserialize, Deserializer};
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub(crate) enum Alt {
    Unknown,
    Short,
    Long,
    Variant,
    StandAlone,
    Official,
    /// Secondary name variant, used in languages and scripts.
    Secondary,
    /// Abbreviation for territory code `IO` (British Indian Ocean Territory).
    Biot,
    /// Alternate name for territory code `IO` (British Indian Ocean Territory) mapping to "Chagos Archipelago".
    Chagos,
    /// "menu" variant, which is being replaced by menu=core|extension, but is still in CLDR.
    Menu,
}

impl FromStr for Alt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "short" => Ok(Alt::Short),
            "long" => Ok(Alt::Long),
            "variant" => Ok(Alt::Variant),
            "stand-alone" => Ok(Alt::StandAlone),
            "official" => Ok(Alt::Official),
            "secondary" => Ok(Alt::Secondary),
            "biot" => Ok(Alt::Biot),
            "chagos" => Ok(Alt::Chagos),
            "menu" => Ok(Alt::Menu),
            _ => Err(()),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub(crate) enum Menu {
    Unknown,
    Core,
    Extension,
}

impl FromStr for Menu {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "core" => Ok(Menu::Core),
            "extension" => Ok(Menu::Extension),
            _ => Err(()),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub(crate) struct WithAlt<T> {
    pub(crate) subtag: T,
    pub(crate) alt: Option<Alt>,
    pub(crate) menu: Option<Menu>,
}

impl<'de, T> Deserialize<'de> for WithAlt<T>
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor<T>(PhantomData<T>);

        impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
        where
            T: FromStr,
            T::Err: Display,
        {
            type Value = WithAlt<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string with optional -alt- or -menu- suffix")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Some((subtag_str, menu_str)) = v.split_once(MENU_SEPARATOR) {
                    let subtag = T::from_str(subtag_str).map_err(E::custom)?;
                    let menu = match Menu::from_str(menu_str) {
                        Ok(m) => m,
                        Err(_) => {
                            log::warn!("Unknown menu variant: {}", menu_str);
                            Menu::Unknown
                        }
                    };
                    Ok(WithAlt {
                        subtag,
                        alt: None,
                        menu: Some(menu),
                    })
                } else if let Some((subtag_str, alt_str)) = v.split_once(ALT_SEPARATOR) {
                    let subtag = T::from_str(subtag_str).map_err(E::custom)?;
                    let alt = match Alt::from_str(alt_str) {
                        Ok(a) => a,
                        Err(_) => {
                            log::warn!("Unknown alt variant: {}", alt_str);
                            Alt::Unknown
                        }
                    };
                    Ok(WithAlt {
                        subtag,
                        alt: Some(alt),
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

        deserializer.deserialize_str(Visitor(PhantomData))
    }
}
