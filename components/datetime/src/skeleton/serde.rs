// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::fields;
#[cfg(feature = "provider_serde")]
use alloc::format;
#[cfg(feature = "provider_serde")]
use alloc::string::String;
use core::convert::TryFrom;
use smallvec::SmallVec;

pub mod reference {
    use super::*;
    use crate::skeleton::reference::Skeleton;

    #[cfg(feature = "provider_serde")]
    use serde::{
        de,
        ser::{self, SerializeSeq},
        Deserialize, Deserializer, Serialize,
    };

    /// This is an implementation of the serde deserialization visitor pattern.
    #[cfg(feature = "provider_serde")]
    #[allow(clippy::upper_case_acronyms)]
    struct DeserializeSkeletonFieldsUTS35String;

    #[cfg(feature = "provider_serde")]
    impl<'de> de::Visitor<'de> for DeserializeSkeletonFieldsUTS35String {
        type Value = Skeleton;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(formatter, "Expected to find a valid skeleton.")
        }

        /// A [`Skeleton`] serialized into a string follows UTS-35.
        /// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
        /// This string consists of a symbol that is repeated N times. This string is
        /// deserialized here into the Skeleton format which is used in memory
        /// when working with formatting datetimes.
        fn visit_str<E>(self, skeleton_string: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Skeleton::try_from(skeleton_string).map_err(|err| {
                de::Error::invalid_value(
                    de::Unexpected::Other(&format!("{:?} {}", skeleton_string, err)),
                    &"field symbols representing a skeleton",
                )
            })
        }
    }

    #[cfg(feature = "provider_serde")]
    pub struct DeserializeSkeletonBincode;

    #[cfg(feature = "provider_serde")]
    impl<'de> de::Visitor<'de> for DeserializeSkeletonBincode {
        type Value = Skeleton;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(formatter, "Unable to deserialize a bincode Pattern.")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Skeleton, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut items: SmallVec<[fields::Field; 5]> = SmallVec::new();
            while let Some(item) = seq.next_element()? {
                if let Some(prev_item) = items.last() {
                    if prev_item >= &item {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Other(&format!(
                                "field item out of order or duplicate: {:?}",
                                item
                            )),
                            &"ordered field symbols representing a skeleton",
                        ));
                    }
                }
                items.push(item)
            }
            Ok(Skeleton::from(items))
        }
    }

    #[cfg(feature = "provider_serde")]
    impl<'de> Deserialize<'de> for Skeleton {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(DeserializeSkeletonFieldsUTS35String)
            } else {
                deserializer.deserialize_seq(DeserializeSkeletonBincode)
            }
        }
    }

    #[cfg(feature = "provider_serde")]
    impl Serialize for Skeleton {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                // Serialize into the UTS 35 string representation.
                let mut string = String::new();

                for field in self.fields_iter() {
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        string.push(ch);
                    }
                }

                serializer.serialize_str(&string)
            } else {
                // Serialize into a bincode-friendly representation. This means that pattern parsing
                // will not be needed when deserializing.
                let mut seq = serializer.serialize_seq(Some(self.fields_len()))?;
                for item in self.fields_iter() {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
        }
    }
}

pub mod runtime {
    use super::*;
    use crate::skeleton::runtime::Skeleton;

    #[cfg(feature = "provider_serde")]
    use serde::{
        de,
        ser::{self, SerializeSeq},
        Deserialize, Deserializer, Serialize,
    };

    /// This is an implementation of the serde deserialization visitor pattern.
    #[cfg(feature = "provider_serde")]
    #[allow(clippy::upper_case_acronyms)]
    struct DeserializeSkeletonFieldsUTS35String;

    #[cfg(feature = "provider_serde")]
    impl<'de> de::Visitor<'de> for DeserializeSkeletonFieldsUTS35String {
        type Value = Skeleton<'de>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(formatter, "Expected to find a valid skeleton.")
        }

        /// A [`Skeleton`] serialized into a string follows UTS-35.
        /// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
        /// This string consists of a symbol that is repeated N times. This string is
        /// deserialized here into the Skeleton format which is used in memory
        /// when working with formatting datetimes.
        fn visit_borrowed_str<E>(self, skeleton_string: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            use crate::skeleton::reference::Skeleton;

            let skeleton = Skeleton::try_from(skeleton_string).map_err(|err| {
                de::Error::invalid_value(
                    de::Unexpected::Other(&format!("{:?} {}", skeleton_string, err)),
                    &"field symbols representing a skeleton",
                )
            })?;

            Ok(skeleton.into())
        }
    }

    #[cfg(feature = "provider_serde")]
    struct DeserializeSkeletonBincode;

    #[cfg(feature = "provider_serde")]
    impl<'de> de::Visitor<'de> for DeserializeSkeletonBincode {
        type Value = Skeleton<'de>;

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(formatter, "Unable to deserialize a bincode Pattern.")
        }

        fn visit_seq<V>(self, seq: V) -> Result<Skeleton<'de>, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let reference_deserializer = super::reference::DeserializeSkeletonBincode;
            let skeleton = reference_deserializer.visit_seq(seq)?;
            Ok(skeleton.into())
        }
    }

    #[cfg(feature = "provider_serde")]
    impl<'de> Deserialize<'de> for Skeleton<'de> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                deserializer.deserialize_str(DeserializeSkeletonFieldsUTS35String)
            } else {
                deserializer.deserialize_seq(DeserializeSkeletonBincode)
            }
        }
    }

    #[cfg(feature = "provider_serde")]
    impl Serialize for Skeleton<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            if serializer.is_human_readable() {
                // Serialize into the UTS 35 string representation.
                let mut string = String::new();

                for field in self.fields_iter() {
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        string.push(ch);
                    }
                }

                serializer.serialize_str(&string)
            } else {
                // Serialize into a bincode-friendly representation. This means that pattern parsing
                // will not be needed when deserializing.
                let mut seq = serializer.serialize_seq(Some(self.fields_len()))?;
                for item in self.fields_iter() {
                    seq.serialize_element(&item)?;
                }
                seq.end()
            }
        }
    }
}
