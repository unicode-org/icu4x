// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{reference, PatternItem, TimeGranularity};
use crate::pattern::reference::pattern::dump_buffer_into_formatter;
use alloc::fmt::{self, Write};
use alloc::string::String;
use alloc::{vec, vec::Vec};
use zerovec::ZeroVec;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern<'data> {
    pub items: ZeroVec<'data, PatternItem>,
    pub(crate) time_granularity: TimeGranularity,
}

impl<'data> From<ZeroVec<'data, PatternItem>> for Pattern<'data> {
    fn from(items: ZeroVec<'data, PatternItem>) -> Self {
        Self {
            time_granularity: items
                .iter()
                .map(|pi| (&pi).into())
                .max()
                .unwrap_or_default(),
            items,
        }
    }
}

impl From<Vec<PatternItem>> for Pattern<'_> {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            time_granularity: items.iter().map(Into::into).max().unwrap_or_default(),
            items: ZeroVec::clone_from_slice(&items),
        }
    }
}

impl From<reference::Pattern> for Pattern<'_> {
    fn from(input: reference::Pattern) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&input.items),
            time_granularity: input.time_granularity,
        }
    }
}

impl Default for Pattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::Owned(vec![]),
            time_granularity: TimeGranularity::default(),
        }
    }
}

/// This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35
/// pattern string. It could also be implemented as the Writeable trait, but at the time of writing
/// this was not done, as this code would need to implement the [`write_len()`] method, which would
/// need to duplicate the branching logic of the [`fmt`](std::fmt) method here. This code is used in generating
/// the data providers and is not as performance sensitive.
impl fmt::Display for Pattern<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for pattern_item in self.items.iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    dump_buffer_into_formatter(&buffer, formatter)?;
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(ch);
                }
            }
        }
        dump_buffer_into_formatter(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}

#[cfg(feature = "provider_serde")]
mod serde {
    use super::*;
    use ::serde::{de, ser, Deserialize, Deserializer, Serialize};
    use alloc::string::ToString;

    /// A helper struct that is shaped exactly like `runtime::Pattern`
    /// and is used to aid in quick deserialization.
    #[cfg(feature = "provider_serde")]
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

    #[cfg(feature = "provider_serde")]
    #[allow(clippy::upper_case_acronyms)]
    struct DeserializePatternUTS35String;

    #[cfg(feature = "provider_serde")]
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
            let reference_deserializer =
                crate::pattern::reference::pattern::DeserializePatternUTS35String;
            let pattern = reference_deserializer.visit_str(pattern_string)?;

            Ok(pattern.into())
        }
    }

    #[cfg(feature = "provider_serde")]
    impl<'de> Deserialize<'de> for Pattern<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(DeserializePatternUTS35String)
            } else {
                let pattern = PatternForSerde::deserialize(deserializer)?;
                Ok(pattern.into())
            }
        }
    }

    #[cfg(feature = "provider_serde")]
    impl Serialize for Pattern<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                // Serialize into the UTS 35 string representation.
                let string: String = self.to_string();
                serializer.serialize_str(&string)
            } else {
                let pfs: PatternForSerde = self.into();
                pfs.serialize(serializer)
            }
        }
    }
}
