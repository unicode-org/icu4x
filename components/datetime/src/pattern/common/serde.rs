// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::pattern::{PatternItem, TimeGranularity};
use ::serde::{de, ser, Deserialize, Deserializer, Serialize};
use alloc::{fmt, format, string::ToString, vec::Vec};

mod reference {
    use super::*;
    use crate::pattern::reference::Pattern;

    /// A helper struct that is shaped exactly like `runtime::Pattern`
    /// and is used to aid in quick deserialization.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PatternForSerde {
        pub items: Vec<PatternItem>,
        pub(crate) time_granularity: TimeGranularity,
    }

    impl From<PatternForSerde> for Pattern {
        fn from(pfs: PatternForSerde) -> Self {
            Self {
                items: pfs.items,
                time_granularity: pfs.time_granularity,
            }
        }
    }

    impl From<&Pattern> for PatternForSerde {
        fn from(pfs: &Pattern) -> Self {
            Self {
                items: pfs.items.clone(),
                time_granularity: pfs.time_granularity,
            }
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    pub(crate) struct DeserializePatternUTS35String;

    impl<'de> de::Visitor<'de> for DeserializePatternUTS35String {
        type Value = Pattern;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Expected to find a valid pattern.")
        }

        fn visit_str<E>(self, pattern_string: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Parse a string into a list of fields.
            pattern_string.parse().map_err(|err| {
                de::Error::invalid_value(
                    de::Unexpected::Other(&format!("{}", err)),
                    &"a valid UTS 35 pattern string",
                )
            })
        }
    }

    impl<'de> Deserialize<'de> for Pattern {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(DeserializePatternUTS35String)
            } else {
                let pattern = PatternForSerde::deserialize(deserializer)?;
                Ok(Pattern::from(pattern))
            }
        }
    }

    impl Serialize for Pattern {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                serializer.serialize_str(&self.to_string())
            } else {
                let pfs = PatternForSerde::from(self);
                pfs.serialize(serializer)
            }
        }
    }
}

mod runtime {
    use super::*;
    use crate::pattern::{runtime::Pattern, PatternItem};
    use alloc::string::ToString;
    use zerovec::ZeroVec;

    /// A helper struct that is shaped exactly like `runtime::Pattern`
    /// and is used to aid in quick deserialization.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct PatternForSerde<'data> {
        #[serde(borrow)]
        pub items: ZeroVec<'data, PatternItem>,
        pub(crate) time_granularity: TimeGranularity,
    }

    impl<'data> From<PatternForSerde<'data>> for Pattern<'data> {
        fn from(pfs: PatternForSerde<'data>) -> Self {
            Self {
                items: pfs.items,
                time_granularity: pfs.time_granularity,
            }
        }
    }

    impl<'data> From<&Pattern<'data>> for PatternForSerde<'data> {
        fn from(pfs: &Pattern<'data>) -> Self {
            Self {
                items: pfs.items.clone(),
                time_granularity: pfs.time_granularity,
            }
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    struct DeserializePatternUTS35String;

    impl<'de> de::Visitor<'de> for DeserializePatternUTS35String {
        type Value = Pattern<'de>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "Expected to find a valid pattern.")
        }

        fn visit_str<E>(self, pattern_string: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // Parse a string into a list of fields.
            let reference_deserializer = super::reference::DeserializePatternUTS35String;
            let pattern = reference_deserializer.visit_str(pattern_string)?;

            Ok(Self::Value::from(&pattern))
        }
    }

    impl<'de: 'data, 'data> Deserialize<'de> for Pattern<'data> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(DeserializePatternUTS35String)
            } else {
                let pattern = PatternForSerde::deserialize(deserializer)?;
                Ok(Pattern::from(pattern))
            }
        }
    }

    impl Serialize for Pattern<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                serializer.serialize_str(&self.to_string())
            } else {
                let pfs = PatternForSerde::from(self);
                pfs.serialize(serializer)
            }
        }
    }

    mod plural {
        // Can't use `serde(untagged)` on `PatternPlurals` because
        // Postcard can't handle enums not discriminated at compilation time.
        use super::*;
        use crate::pattern::runtime::{PatternPlurals, PluralPattern};
        use alloc::string::ToString;
        use core::fmt;

        /// A helper struct that is shaped exactly like `runtime::Pattern`
        /// and is used to aid in quick deserialization.
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        enum PatternPluralsForSerde<'data> {
            #[serde(borrow)]
            MultipleVariants(PluralPattern<'data>),
            #[serde(borrow)]
            SinglePattern(Pattern<'data>),
        }

        impl<'data> From<PatternPluralsForSerde<'data>> for PatternPlurals<'data> {
            fn from(pfs: PatternPluralsForSerde<'data>) -> Self {
                match pfs {
                    PatternPluralsForSerde::MultipleVariants(variants) => {
                        Self::MultipleVariants(variants)
                    }
                    PatternPluralsForSerde::SinglePattern(pattern) => Self::SinglePattern(pattern),
                }
            }
        }

        impl<'data> From<&PatternPlurals<'data>> for PatternPluralsForSerde<'data> {
            fn from(pfs: &PatternPlurals<'data>) -> Self {
                match pfs.clone() {
                    PatternPlurals::MultipleVariants(variants) => Self::MultipleVariants(variants),
                    PatternPlurals::SinglePattern(pattern) => Self::SinglePattern(pattern),
                }
            }
        }

        struct DeserializePatternPlurals;

        impl<'de> de::Visitor<'de> for DeserializePatternPlurals {
            type Value = PatternPlurals<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Expected to find a valid pattern.")
            }

            fn visit_str<E>(self, pattern_string: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let reference_deserializer = super::super::reference::DeserializePatternUTS35String;
                let pattern = reference_deserializer.visit_str(pattern_string)?;

                Ok(PatternPlurals::SinglePattern(Pattern::from(&pattern)))
            }

            fn visit_map<V>(self, map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                Ok(PatternPlurals::MultipleVariants(
                    PluralPattern::deserialize(de::value::MapAccessDeserializer::new(map))?,
                ))
            }
        }

        impl<'de: 'data, 'data> Deserialize<'de> for PatternPlurals<'data> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                if deserializer.is_human_readable() {
                    deserializer.deserialize_any(DeserializePatternPlurals)
                } else {
                    let pattern = PatternPluralsForSerde::deserialize(deserializer)?;
                    Ok(Self::from(pattern))
                }
            }
        }

        impl Serialize for PatternPlurals<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                if serializer.is_human_readable() {
                    match self {
                        Self::SinglePattern(pattern) => {
                            serializer.serialize_str(&pattern.to_string())
                        }
                        Self::MultipleVariants(variants) => variants.serialize(serializer),
                    }
                } else {
                    let pfs: PatternPluralsForSerde = self.into();
                    pfs.serialize(serializer)
                }
            }
        }
    }

    mod generic {
        use super::*;
        use crate::pattern::runtime::GenericPattern;

        #[allow(clippy::upper_case_acronyms)]
        struct DeserializeGenericPatternUTS35String;

        impl<'de> de::Visitor<'de> for DeserializeGenericPatternUTS35String {
            type Value = GenericPattern<'de>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Expected to find a valid pattern.")
            }

            fn visit_str<E>(self, pattern_string: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                // Parse a string into a list of fields.
                let pattern = pattern_string.parse().expect("Failed to parse pattern");
                Ok(GenericPattern::from(&pattern))
            }
        }

        impl<'de: 'data, 'data> Deserialize<'de> for GenericPattern<'data> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                if deserializer.is_human_readable() {
                    deserializer.deserialize_str(DeserializeGenericPatternUTS35String)
                } else {
                    let items = ZeroVec::deserialize(deserializer)?;
                    Ok(Self { items })
                }
            }
        }

        impl Serialize for GenericPattern<'_> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                if serializer.is_human_readable() {
                    // Serialize into the UTS 35 string representation.
                    let string = self.to_string();
                    serializer.serialize_str(&string)
                } else {
                    self.items.serialize(serializer)
                }
            }
        }
    }
}
