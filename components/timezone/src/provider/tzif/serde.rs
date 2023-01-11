// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Custom Serialize and Deserialize implementations for tzif types.

#[cfg(feature = "datagen")]
use ::serde::{Serialize, Serializer};

#[cfg(feature = "serde")]
use super::ule::{LocalTimeRecordULE, TransitionDateULE};
#[cfg(feature = "serde")]
use ::serde::{Deserialize, Deserializer};

#[cfg(any(feature = "datagen", feature = "serde"))]
use super::{LocalTimeRecordV1, TransitionDateV1};
#[cfg(any(feature = "datagen", feature = "serde"))]
use zerovec::ule::AsULE;

#[cfg(feature = "serde")]
/// A visitor struct for serde custom deserialization of a [LocalTimeRecordV1].
pub struct LocalTimeRecordVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for LocalTimeRecordVisitor {
    type Value = LocalTimeRecordV1;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a LocalTimeRecordV1")
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let [offset_byte0, offset_byte1, tags_byte, _] = value.to_le_bytes();
        Ok(LocalTimeRecordV1::from_unaligned(LocalTimeRecordULE {
            data: [offset_byte0, offset_byte1, tags_byte],
        }))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        debug_assert!(value <= u32::MAX as u64);
        self.visit_u32(value as u32)
    }
}

#[cfg(feature = "datagen")]
impl Serialize for LocalTimeRecordV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let [offset_byte0, offset_byte1, tags_byte] = self.to_unaligned().data;
        let serialized = u32::from_le_bytes([offset_byte0, offset_byte1, tags_byte, 0]);
        serializer.serialize_u32(serialized)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for LocalTimeRecordV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u32(LocalTimeRecordVisitor)
    }
}

#[cfg(feature = "serde")]
/// A visitor struct for serde custom deserialization of a [TransitionDateVisitor].
pub struct TransitionDateVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for TransitionDateVisitor {
    type Value = TransitionDateV1;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("a TransitionDateV1")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let [day_byte0, day_byte1, day_byte2, time_byte0, time_byte1, time_byte2, pad0, pad1] =
            value.to_le_bytes();
        assert_eq!(pad0, 0);
        assert_eq!(pad1, 0);
        Ok(TransitionDateV1::from_unaligned(TransitionDateULE {
            data: [
                day_byte0, day_byte1, day_byte2, time_byte0, time_byte1, time_byte2,
            ],
        }))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TransitionDateV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(TransitionDateVisitor)
    }
}

#[cfg(feature = "datagen")]
impl Serialize for TransitionDateV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let [day_byte0, day_byte1, day_byte2, time_byte0, time_byte1, time_byte2] =
            self.to_unaligned().data;
        let serialized = u64::from_le_bytes([
            day_byte0, day_byte1, day_byte2, time_byte0, time_byte1, time_byte2, 0, 0,
        ]);
        serializer.serialize_u64(serialized)
    }
}

#[cfg(all(test, feature = "serde"))]
mod test {
    use crate::provider::tzif::{LocalTimeRecordV1, TransitionDateV1, TransitionDayV1};

    #[test]
    fn local_time_record_json_roundtrips() {
        let min_offset = -43200;
        let max_offset = 50400;

        for offset in min_offset..=max_offset {
            let is_dst = offset % 2 == 0;
            let record = LocalTimeRecordV1 { offset, is_dst };
            let json_record = serde_json::to_string(&record).unwrap();
            let record_from_json = serde_json::from_str(&json_record).unwrap();
            assert_eq!(
                record, record_from_json,
                "LocalTimeRecordV1 should roundtrip to and from json, but found two different records\n{record:#?}\n{record_from_json:#?}",
            );
        }
    }

    #[test]
    fn transition_date_json_roundtrips() {
        const H167_M59_S59: i32 = 604799;
        for day in 1..=365 {
            for time_of_day in [-H167_M59_S59, 0, H167_M59_S59] {
                let day_of_year = TransitionDayV1::NoLeap(day);
                let transition_date = TransitionDateV1 {
                    day_of_year,
                    time_of_day,
                };
                let json_transition_date = serde_json::to_string(&transition_date).unwrap();
                let transition_date_from_json =
                    serde_json::from_str(&json_transition_date).unwrap();
                assert_eq!(
                    transition_date, transition_date_from_json,
                    "TransitionDateV1 should roundtrip to and from JSON, but found two different dates\n{transition_date:#?}\n{transition_date_from_json:#?}",
                );
            }
        }
        for day in 0..=365 {
            for time_of_day in [-H167_M59_S59, 0, H167_M59_S59] {
                let day_of_year = TransitionDayV1::WithLeap(day);
                let transition_date = TransitionDateV1 {
                    day_of_year,
                    time_of_day,
                };
                let json_transition_date = serde_json::to_string(&transition_date).unwrap();
                let transition_date_from_json =
                    serde_json::from_str(&json_transition_date).unwrap();
                assert_eq!(
                    transition_date, transition_date_from_json,
                    "TransitionDateV1 should roundtrip to and from JSON, but found two different dates\n{transition_date:#?}\n{transition_date_from_json:#?}",
                );
            }
        }
        for m in 1..=12 {
            for w in 1..=5 {
                for d in 0..=6 {
                    let day_of_year = TransitionDayV1::Mwd(m, w, d);
                    for time_of_day in [-H167_M59_S59, 0, H167_M59_S59] {
                        let transition_date = TransitionDateV1 {
                            day_of_year,
                            time_of_day,
                        };
                        let json_transition_date = serde_json::to_string(&transition_date).unwrap();
                        let transition_date_from_json =
                            serde_json::from_str(&json_transition_date).unwrap();
                        assert_eq!(
                            transition_date, transition_date_from_json,
                            "TransitionDateV1 should roundtrip to and from JSON, but found two different dates\n{transition_date:#?}\n{transition_date_from_json:#?}",
                        );
                    }
                }
            }
        }
        let boundary_days = [
            TransitionDayV1::NoLeap(1),
            TransitionDayV1::NoLeap(365),
            TransitionDayV1::NoLeap(0),
            TransitionDayV1::NoLeap(365),
            TransitionDayV1::Mwd(1, 1, 0),
            TransitionDayV1::Mwd(12, 5, 6),
        ];
        for day_of_year in boundary_days {
            for time_of_day in -H167_M59_S59..=H167_M59_S59 {
                let transition_date = TransitionDateV1 {
                    day_of_year,
                    time_of_day,
                };
                let json_transition_date = serde_json::to_string(&transition_date).unwrap();
                let transition_date_from_json =
                    serde_json::from_str(&json_transition_date).unwrap();
                assert_eq!(
                    transition_date, transition_date_from_json,
                    "TransitionDateV1 should roundtrip to and from JSON, but found two different dates\n{transition_date:#?}\n{transition_date_from_json:#?}",
                );
            }
        }
    }
}
