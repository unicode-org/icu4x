// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! ULE types and ipmlementations for tzif types.

use icu_provider::yoke;
use zerovec::{
    ule::{AsULE, ULE},
    ZeroSlice, ZeroVec,
};

use crate::provider::tzif::TransitionDayV1;

use super::{LocalTimeRecordV1, TransitionDateV1};

/// [`LocalTimeRecordULE`] is a type optimized for efficent storing and
/// deserialization of [`LocalTimeRecordV1`] elements using
/// [`ZeroVec`] model.
///
/// The serialization model packages the pattern item in three bytes.
///
/// The [`LocalTimeRecordV1`] which contains a GMT offset stored in an [`i32`]
/// and a [`bool`] for whether the offset represent standard or daylight time.
///
/// However, ICU4X expects GMT offsets only within the range of `[-64800 to 64800]` seconds,
/// which correspond to GMT-18 and GMT+18, the minimum and maximum GMT offsets that exist.
///
/// Since the magnitude of `64800` is small enough to fit within a [`u16`] value,
/// we can more efficiently store this 5-byte structure in only 3 bytes by storing the magnitude
/// of the offset in two bytes, and storing one bit of information about whether the magnitude
/// is positive or negative in the byte that contains the [`bool`].
///
/// # Diagram
///
/// ```text
/// ┌───────────────────────────────┬───────────────┐
/// │              u16              │       u8      │
/// ├─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┤
/// ├─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┼─┴─┴─┴─┴─┴─┼─┼─┤
/// │     GMT Offset Magnitude      │   Empty   │X│X│
/// └───────────────────────────────┴───────────┴─┴─┘
///                                              ▲ ▲
///                                              │ │
///                                              │ Positive/Negative
///                                              STD/DST
/// ```
#[repr(packed)]
#[derive(Debug, Clone, Copy, Default, yoke::Yokeable, ULE)]
pub struct LocalTimeRecordULE {
    /// The unaligned data.
    pub data: [u8; 3],
}

impl Eq for LocalTimeRecordULE {}
impl PartialEq for LocalTimeRecordULE {
    fn eq(&self, other: &Self) -> bool {
        LocalTimeRecordV1::from_unaligned(*self).eq(&LocalTimeRecordV1::from_unaligned(*other))
    }
}

impl PartialOrd for LocalTimeRecordULE {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        LocalTimeRecordV1::from_unaligned(*self)
            .partial_cmp(&LocalTimeRecordV1::from_unaligned(*other))
    }
}

impl Ord for LocalTimeRecordULE {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        LocalTimeRecordV1::from_unaligned(*self).cmp(&LocalTimeRecordV1::from_unaligned(*other))
    }
}

impl AsULE for LocalTimeRecordV1 {
    type ULE = LocalTimeRecordULE;

    fn to_unaligned(self) -> Self::ULE {
        let is_dst_bit = (self.is_dst as u8) << 1;
        let is_positive_bit = (self.offset >= 0) as u8;
        let tags_byte = is_dst_bit | is_positive_bit;
        let [offset_byte0, offset_byte1] = (self.offset.unsigned_abs() as u16).to_le_bytes();
        LocalTimeRecordULE {
            data: [offset_byte0, offset_byte1, tags_byte],
        }
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let [offset_byte0, offset_byte1, tags_byte] = unaligned.data;
        let is_dst = tags_byte & 0b0000_0010 != 0;
        let is_positive = tags_byte & 0b0000_0001 != 0;

        let offset = if is_positive {
            u16::from_le_bytes([offset_byte0, offset_byte1]) as i32
        } else {
            -(u16::from_le_bytes([offset_byte0, offset_byte1]) as i32)
        };

        LocalTimeRecordV1 { offset, is_dst }
    }
}

impl<'a> zerovec::maps::ZeroMapKV<'a> for LocalTimeRecordV1 {
    type Container = ZeroVec<'a, LocalTimeRecordV1>;
    type Slice = ZeroSlice<LocalTimeRecordV1>;
    type GetType = <LocalTimeRecordV1 as AsULE>::ULE;
    type OwnedType = LocalTimeRecordV1;
}

/// [`TransitionDateULE`] is a type optimized for efficent storing and
/// deserialization of [`TransitionDateV1`] elements using
/// [`ZeroVec`] model.
///
/// The serialization model packages the pattern item in six bytes.
///
/// The [`TransitionDateV1`] which contains a four-byte [`TransitionDayV1`] and an [`i32`].
/// However, the [`TransitionDayV1`] can be stored in only only 3 bytes. The [`TransitionDayV1::NoLeap`]
/// and [`TransitionDayV1::WithLeap`] variants take up only 2 bytes of data and the [`TransitionDayV1::Mwd`]
/// variant takes up 3 bytes, but the left-most byte is restricted to the range `[1, 12]`.
///
/// As such, we can use the most-significant bit to hold information about which enum variant is present
/// * If the left-most byte is `0b0000_0000`, then the variant is [`TransitionDayV1::NoLeap`].
/// * If the left-most byte is `0b1000_0000`, then the variant is [`TransitionDayV1::WithLeap`].
/// * If the left-most byte is one of `[1, 12]`, the variant is [`TransitionDayV1::Mwd`].
/// but will never populate the most significant bit because its value is trestricted to from `[1, 12]`.
///
/// The [`i32`] offset is restricted to a range of `[-604799 to 604799]`, which is +/- 167 hours 59 minutes 59 seconds.
/// The magnitude of `2 x 604799 = 1209598` can fit within 3 bytes of unsigned data, so we can shift the value to be
/// non-negative by adding `604799` and reinterpret the bytes as unsigned.
///
/// # Diagram
///
/// ```text
/// ┌───────────────┬───────────────┬───────────────┬───────────────┬───────────────┬───────────────┐
/// │       u8      │       u8      │       u8      │       u8      │       u8      │       u8      │
/// ├─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┼─┬─┬─┬─┬─┬─┬─┬─┤
/// ├─┼─┴─┴─┴─┴─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┼─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┤
/// │0│    Month    │      Week     │      Day      │                  Time of Day                  │
/// ├─┼─────────────┼───────────────┴───────────────┼───────────────────────────────────────────────┘
/// │X│    Empty    │        NoLeap/WithLeap        │
/// └─┴─────────────┴───────────────────────────────┘
///  ▲
///  │
///  Transition Day Variant Indicator
/// ```
#[repr(packed)]
#[derive(Debug, Clone, Copy, Default, yoke::Yokeable, ULE)]
pub struct TransitionDateULE {
    /// The unaligned data.
    pub data: [u8; 6],
}

impl Eq for TransitionDateULE {}
impl PartialEq for TransitionDateULE {
    fn eq(&self, other: &Self) -> bool {
        TransitionDateV1::from_unaligned(*self).eq(&TransitionDateV1::from_unaligned(*other))
    }
}

impl PartialOrd for TransitionDateULE {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        TransitionDateV1::from_unaligned(*self)
            .partial_cmp(&TransitionDateV1::from_unaligned(*other))
    }
}

impl Ord for TransitionDateULE {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        TransitionDateV1::from_unaligned(*self).cmp(&TransitionDateV1::from_unaligned(*other))
    }
}

/// 167 hours 59 minutes 59 seconds in seconds.
const H167_M59_S59: i32 = 604799;

/// A helper function to encode a [`TransitionDateV1`]`::time_of_day` for storage in a [`TransitionDateULE`].
fn encode_time_of_day(time_of_day: i32) -> [u8; 4] {
    debug_assert!((-H167_M59_S59..=H167_M59_S59).contains(&time_of_day));
    ((time_of_day + H167_M59_S59) as u32).to_le_bytes()
}

/// A helper function to decode part of [`TransitionDateULE`] back into [`TransitionDateV1`]`::time_of_day`.
fn decode_time_of_day(time_of_day @ [_, _, _, empty_byte]: [u8; 4]) -> i32 {
    debug_assert_eq!(empty_byte, 0);
    (u32::from_le_bytes(time_of_day) as i32) - H167_M59_S59
}

impl AsULE for TransitionDateV1 {
    type ULE = TransitionDateULE;

    fn to_unaligned(self) -> Self::ULE {
        let day_byte0;
        let day_byte1;
        let day_byte2;
        match self.day_of_year {
            TransitionDayV1::NoLeap(day) => {
                let [byte1, byte2] = day.to_le_bytes();
                day_byte0 = 0b0000_0000;
                day_byte1 = byte1;
                day_byte2 = byte2;
            }
            TransitionDayV1::WithLeap(day) => {
                let [byte1, byte2] = day.to_le_bytes();
                day_byte0 = 0b1000_0000;
                day_byte1 = byte1;
                day_byte2 = byte2;
            }
            TransitionDayV1::Mwd(m, w, d) => {
                debug_assert!(m > 0);
                debug_assert_eq!(m & 0b1000_0000, 0);
                day_byte0 = m;
                day_byte1 = w;
                day_byte2 = d;
            }
        }

        let [time_byte0, time_byte1, time_byte2, empty_byte] = encode_time_of_day(self.time_of_day);
        debug_assert_eq!(empty_byte, 0);

        TransitionDateULE {
            data: [
                day_byte0, day_byte1, day_byte2, time_byte0, time_byte1, time_byte2,
            ],
        }
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        let [day_byte0, day_byte1, day_byte2, time_byte0, time_byte1, time_byte2] = unaligned.data;
        let day = match day_byte0 {
            0b0000_0000 => TransitionDayV1::NoLeap(u16::from_le_bytes([day_byte1, day_byte2])),
            0b1000_0000 => TransitionDayV1::WithLeap(u16::from_le_bytes([day_byte1, day_byte2])),
            _ => TransitionDayV1::Mwd(day_byte0, day_byte1, day_byte2),
        };
        let time = decode_time_of_day([time_byte0, time_byte1, time_byte2, 0]);
        TransitionDateV1 {
            day_of_year: day,
            time_of_day: time,
        }
    }
}

#[cfg(test)]
mod test {
    use super::H167_M59_S59;
    use crate::provider::tzif::{LocalTimeRecordV1, TransitionDateV1, TransitionDayV1};
    use zerovec::ule::AsULE;

    #[test]
    fn local_time_record_ule_roundtrips() {
        let min_offset = -43200;
        let max_offset = 50400;

        for offset in min_offset..=max_offset {
            let is_dst = offset % 2 == 0;
            let record = LocalTimeRecordV1 { offset, is_dst };
            let record_ule = record.to_unaligned();
            let record_from_ule = LocalTimeRecordV1::from_unaligned(record_ule);
            assert_eq!(
                record, record_from_ule,
                "LocalTimeRecordV1 should roundtrip to and from ULE, but found two different records\n{record:#?}\n{record_from_ule:#?}",
            );
        }
    }

    #[test]
    fn transition_date_ule_roundtrips() {
        for day in 1..=365 {
            for time_of_day in [-H167_M59_S59, 0, H167_M59_S59] {
                let day_of_year = TransitionDayV1::NoLeap(day);
                let transition_date = TransitionDateV1 {
                    day_of_year,
                    time_of_day,
                };
                let transition_date_ule = transition_date.to_unaligned();
                let transition_date_from_ule =
                    TransitionDateV1::from_unaligned(transition_date_ule);
                assert_eq!(
                    transition_date, transition_date_from_ule,
                    "TransitionDateV1 should roundtrip to and from ULE, but found two different dates\n{transition_date:#?}\n{transition_date_from_ule:#?}",
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
                let transition_date_ule = transition_date.to_unaligned();
                let transition_date_from_ule =
                    TransitionDateV1::from_unaligned(transition_date_ule);
                assert_eq!(
                    transition_date, transition_date_from_ule,
                    "TransitionDateV1 should roundtrip to and from ULE, but found two different dates\n{transition_date:#?}\n{transition_date_from_ule:#?}",
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
                        let transition_date_ule = transition_date.to_unaligned();
                        let transition_date_from_ule =
                            TransitionDateV1::from_unaligned(transition_date_ule);
                        assert_eq!(
                            transition_date, transition_date_from_ule,
                            "TransitionDateV1 should roundtrip to and from ULE, but found two different dates\n{transition_date:#?}\n{transition_date_from_ule:#?}",
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
                let transition_date_ule = transition_date.to_unaligned();
                let transition_date_from_ule =
                    TransitionDateV1::from_unaligned(transition_date_ule);
                assert_eq!(
                    transition_date, transition_date_from_ule,
                    "TransitionDateV1 should roundtrip to and from ULE, but found two different dates\n{transition_date:#?}\n{transition_date_from_ule:#?}",
                );
            }
        }
    }
}
