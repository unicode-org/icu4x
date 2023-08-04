// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::marker::PhantomData;

use icu_locid::LanguageIdentifier;
use serde::de::Error;
use serde::de::MapAccess;
use serde::de::Visitor;
use serde::Deserialize;

/// Deserializer for the top layers of most CLDR JSON locale resources.
///
/// Most locale-specific resources have the following structure:
///
/// ```json
/// {
///   "main": {
///     "en-US": {
///       /* resource-specific fields */
///     }
///   }
/// }
/// ```
///
/// This deserializer is generic over `T`, which is the type of the resource-specific fields,
/// and will in effect "strip" the top two layers from the JSON when parsing.
#[derive(Debug, Deserialize)]
pub struct LocaleResource<T> {
    pub main: SingleLocaleMap<T>,
}

/// A map containing a single locale key and value.
#[derive(Debug)]
pub struct SingleLocaleMap<T> {
    pub locale: LanguageIdentifier,
    pub value: T,
}

struct SingleLocaleMapVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for SingleLocaleMapVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = SingleLocaleMap<T>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a value keyed by a locale")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let Some(locale) = access.next_key::<LanguageIdentifier>()? else {
            return Err(M::Error::missing_field("<LOCALE>"));
        };
        let value = access.next_value::<T>()?;
        if access.next_key::<LanguageIdentifier>()?.is_some() {
            return Err(M::Error::duplicate_field("<LOCALE>"));
        }
        Ok(SingleLocaleMap { locale, value })
    }
}

impl<'de, T> Deserialize<'de> for SingleLocaleMap<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = SingleLocaleMapVisitor(PhantomData);
        deserializer.deserialize_map(visitor)
    }
}
