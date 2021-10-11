// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(missing_docs)] // TODO(#686) - Add missing docs.

mod skeletons;
mod symbols;

use crate::pattern;
use alloc::borrow::Cow;
use icu_provider::yoke::{self, *};
pub use skeletons::*;
pub use symbols::*;

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
#[yoke(cloning_zcf)]
pub struct DatePatternsV1 {
    pub date: patterns::LengthPatternsV1,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h11 or h12.
    pub time_h11_h12: patterns::LengthPatternsV1,

    /// These patterns are common uses of time formatting, broken down by the length of the
    /// pattern. Users can override the hour cycle with a preference, so there are two
    /// pattern groups stored here. Note that the pattern will contain either h23 or h24.
    pub time_h23_h24: patterns::LengthPatternsV1,

    /// By default a locale will prefer one hour cycle type over another.
    pub preferred_hour_cycle: pattern::CoarseHourCycle,

    /// Patterns used to combine date and time length patterns into full date_time patterns.
    pub length_combinations: patterns::LengthPatternsV1,
}

pub mod patterns {
    use super::*;
    use crate::pattern::reference::{Pattern, PatternPlurals, PluralPattern};
    use core::convert::TryFrom;

    #[cfg(feature = "provider_serde")]
    use alloc::string::ToString;
    #[cfg(feature = "provider_serde")]
    use serde::{
        de::{self, IntoDeserializer},
        ser, Deserialize, Deserializer, Serialize,
    };

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct LengthPatternsV1 {
        pub full: Cow<'static, str>,
        pub long: Cow<'static, str>,
        pub medium: Cow<'static, str>,
        pub short: Cow<'static, str>,
    }

    /// This struct is a public wrapper around the internal [`Pattern`] struct. This allows
    /// access to the serialization and deserialization capabilities, without exposing the
    /// internals of the pattern machinery.
    ///
    /// The [`Pattern`] is an "exotic type" in the serialization process, and handles its own
    /// custom serialization practices.
    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct PatternV1(pub Pattern);

    impl From<Pattern> for PatternV1 {
        fn from(pattern: Pattern) -> Self {
            Self(pattern)
        }
    }

    impl TryFrom<&str> for PatternV1 {
        type Error = pattern::PatternError;

        fn try_from(pattern_string: &str) -> Result<Self, Self::Error> {
            let pattern = Pattern::from_bytes(pattern_string);
            match pattern {
                Ok(pattern) => Ok(Self::from(pattern)),
                Err(err) => Err(err),
            }
        }
    }

    /// This struct is a public wrapper around the internal [`PatternPlurals`]
    /// struct. This allows access to the serialization and deserialization
    /// capabilities, without exposing the internals of the pattern machinery.
    #[derive(Debug, PartialEq, Clone)]
    pub struct PatternPluralsV1(pub PatternPlurals);

    impl From<Pattern> for PatternPluralsV1 {
        fn from(pattern: Pattern) -> Self {
            Self(PatternPlurals::SinglePattern(pattern))
        }
    }

    impl From<PluralPattern> for PatternPluralsV1 {
        fn from(plural_pattern: PluralPattern) -> Self {
            Self(PatternPlurals::MultipleVariants(plural_pattern))
        }
    }

    #[cfg(feature = "provider_serde")]
    impl Serialize for PatternPluralsV1 {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                match &self.0 {
                    PatternPlurals::SinglePattern(pattern) => {
                        serializer.serialize_str(&pattern.to_string())
                    }
                    PatternPlurals::MultipleVariants(patterns) => patterns.serialize(serializer),
                }
            } else {
                serializer.serialize_newtype_struct("PatternPluralsV1", &self.0)
            }
        }
    }
    #[cfg(feature = "provider_serde")]
    struct DeserializeHumanReadablePatternPlurals;

    #[cfg(feature = "provider_serde")]
    impl<'de> de::Visitor<'de> for DeserializeHumanReadablePatternPlurals {
        type Value = PatternPlurals;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str("A valid PatternPlurals")
        }

        fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
                .map(PatternPlurals::MultipleVariants)
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            de::Deserialize::deserialize(s.into_deserializer()).map(PatternPlurals::SinglePattern)
        }
    }

    #[cfg(feature = "provider_serde")]
    impl<'de> Deserialize<'de> for PatternPluralsV1 {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer
                    .deserialize_any(DeserializeHumanReadablePatternPlurals)
                    .map(PatternPluralsV1)
            } else {
                Deserialize::deserialize(deserializer).map(PatternPluralsV1)
            }
        }
    }
}
