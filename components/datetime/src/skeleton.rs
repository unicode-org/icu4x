// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

//! Skeletons are used for pattern matching. See the [`Skeleton`] struct for more information.

use smallvec::SmallVec;
use std::{cmp::Ordering, convert::TryFrom, fmt};

use crate::fields::{self, Field, FieldLength, FieldSymbol};

#[cfg(feature = "provider_serde")]
use serde::{de, ser, Deserialize, Deserializer, Serialize};

#[derive(Debug, PartialEq)]
struct FieldIndex(usize);

/// A `Skeleton` is used to represent what types of `Field`s are present in a `Pattern`. The
/// ordering of the `Skeleton`'s `Field`s have no bearing on the ordering of the `Field`s and
/// `Literal`s in the `Pattern`.
///
/// A `Skeleton` is a `Vec<Field>`, but with the invariant that it is sorted according to the canonical
/// sort order. This order is sorted according to the most significant `Field` to the least significant.
/// For example, a field with a `Minute` symbol would preceed a field with a `Second` symbol.
/// This order is documented as the order of fields as presented in the
/// [UTS 35 Date Field Symbol Table](https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table)
///
/// The `Field`s are only sorted in the `Skeleton` in order to provide a deterministic
/// serialization strategy, and to provide a faster `Skeleton` matching operation.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Skeleton(SmallVec<[fields::Field; 5]>);

impl Skeleton {
    /// The LiteMap<SkeletonV, PatternV1> structs should be ordered by the `Skeleton` canonical
    /// order. This helps ensure that the skeleton serialization is sorted deterministically.
    /// This order is determined by the order of the fields listed in UTS 35, which is ordered
    /// from most significant, to least significant.
    ///
    /// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
    pub fn compare_canonical_order(&self, other: &Skeleton) -> Ordering {
        let fields_a = &self.0;
        let fields_b = &other.0;
        let max_len = fields_a.len().max(fields_b.len());

        for i in 0..max_len {
            let maybe_field_a = fields_a.get(i);
            let maybe_field_b = fields_b.get(i);

            match maybe_field_a {
                Some(field_a) => match maybe_field_b {
                    Some(field_b) => {
                        let order_a = field_a.symbol.get_canonical_order();
                        let order_b = field_b.symbol.get_canonical_order();
                        if order_a < order_b {
                            return Ordering::Less;
                        }
                        if order_a > order_b {
                            return Ordering::Greater;
                        }

                        // If the fields are equivalent, try to sort by field length.
                        let length_a = field_a.length as u8;
                        let length_b = field_b.length as u8;

                        if length_a < length_b {
                            return Ordering::Less;
                        }
                        if length_a > length_b {
                            return Ordering::Greater;
                        }
                    }
                    None => {
                        // Field A has more fields.
                        return Ordering::Greater;
                    }
                },
                None => {
                    // Field B has more fields.
                    return Ordering::Less;
                }
            };
        }

        Ordering::Equal
    }
}

impl PartialOrd for Skeleton {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_canonical_order(other))
    }
}

impl Ord for Skeleton {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_canonical_order(other)
    }
}

/// This is an implementation of the serde deserialization visitor pattern.
#[cfg(feature = "provider_serde")]
struct SkeletonFieldsDeserializeVisitor;

#[cfg(feature = "provider_serde")]
impl<'de> de::Visitor<'de> for SkeletonFieldsDeserializeVisitor {
    type Value = Skeleton;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Expected to find a valid skeleton.")
    }

    /// A skeleton serialized into a string follows UTS 35.
    /// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
    /// This string consists of a symbol that is repeated N times. This string is
    /// deserialized here into the Skeleton format which is used in memory
    /// when working with formatting date times.
    fn visit_str<E>(self, skeleton_string: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Skeleton::try_from(skeleton_string).map_err(|err| match err {
            SkeletonError::SymbolUnknown(byte) => E::custom(format!(
                "Skeleton {:?} contained an unknown symbol, {:?}",
                skeleton_string, byte as char
            )),
            SkeletonError::SymbolUnimplemented(byte) => E::custom(format!(
                "Skeleton {:?} contained an unimplemented symbol, {:?}",
                skeleton_string, byte as char
            )),
            SkeletonError::FieldLengthTooLong => E::custom(format!(
                "Skeleton \"{}\" contained a field that was too long.",
                skeleton_string
            )),
            SkeletonError::SymbolInvalid(ch) => E::custom(format!(
                "Skeleton {:?} contained an invalid symbol, {:?}",
                skeleton_string, ch
            )),
            SkeletonError::FieldOutOfOrder
            | SkeletonError::DuplicateField
            | SkeletonError::UnimplementedField(_)
            | SkeletonError::SkeletonFieldOutOfOrder
            | SkeletonError::Fields(_) => E::custom("TODO - Following commit"),
        })
    }
}

#[cfg(feature = "provider_serde")]
impl<'de> Deserialize<'de> for Skeleton {
    fn deserialize<D>(deserializer: D) -> Result<Skeleton, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SkeletonFieldsDeserializeVisitor)
    }
}

#[cfg(feature = "provider_serde")]
impl Serialize for Skeleton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut string = String::new();

        for field in self.0.iter() {
            let ch: char = field.symbol.into();
            for _ in 0..field.length as usize {
                string.push(ch);
            }
        }

        serializer.serialize_str(&string)
    }
}

/// Parse a string into a list of fields. This trait implementation validates the input string to
/// verify that fields are correct. If the fields are out of order, this returns an error that
/// contains the fields, which gives the callee a chance to sort the fields with the
/// `From<SmallVec<[fields::Field; 5]>> for Skeleton` trait.
impl TryFrom<&str> for Skeleton {
    type Error = SkeletonError;
    fn try_from(skeleton_string: &str) -> Result<Self, Self::Error> {
        // Parse a string into a list of fields.
        let mut fields = SmallVec::new();

        let mut iter = skeleton_string.bytes().peekable();
        while let Some(byte) = iter.next() {
            // Convert the byte to a valid field symbol.
            let field_symbol = FieldSymbol::try_from(byte)?;

            // Go through the bytes to count how often it's repeated.
            let mut field_length: u8 = 1;
            while let Some(next_byte) = iter.peek() {
                if *next_byte != byte {
                    break;
                }
                field_length += 1;
                iter.next();
            }

            // Add the field.
            fields.push(Field::from((
                field_symbol,
                FieldLength::try_from(field_length)?,
            )));
        }

        Ok(Skeleton(fields))
    }
}

#[derive(Debug)]
pub enum SkeletonError {
    FieldLengthTooLong,
    FieldsOutOfOrder(SmallVec<[fields::Field; 5]>),
    DuplicateField,
    SymbolUnknown(char),
    SymbolInvalid(char),
    SymbolUnimplemented(char),
    UnimplementedField(char),
    Fields(fields::Error),
}

/// These strings follow the recommendations for the serde::de::Unexpected::Other type.
/// https://docs.serde.rs/serde/de/enum.Unexpected.html#variant.Other
///
/// Serde will generate an error such as:
/// "invalid value: unclosed literal in pattern, expected a valid UTS 35 pattern string at line 1 column 12"
impl fmt::Display for SkeletonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SkeletonError::FieldLengthTooLong => write!(f, "field too long in skeleton"),
            SkeletonError::DuplicateField => write!(f, "duplicate field in skeleton"),
            SkeletonError::SymbolUnknown(ch) => write!(f, "symbol unknown {} in skeleton", ch),
            SkeletonError::SymbolInvalid(ch) => write!(f, "symbol invalid {} in skeleton", ch),
            SkeletonError::SymbolUnimplemented(ch) => {
                write!(f, "symbol unimplemented {} in skeleton", ch)
            }
            SkeletonError::UnimplementedField(ch) => {
                write!(f, "unimplemented field {} in skeleton", ch)
            }
            SkeletonError::FieldsOutOfOrder(fields) => {
                write!(f, "skeleton fields {:?} out of order", fields)
            }
            SkeletonError::Fields(err) => write!(f, "{} in skeleton", err),
        }
    }
}

impl From<fields::Error> for SkeletonError {
    fn from(fields_error: fields::Error) -> Self {
        SkeletonError::Fields(fields_error)
    }
}

impl From<fields::LengthError> for SkeletonError {
    fn from(_: fields::LengthError) -> Self {
        SkeletonError::FieldLengthTooLong
    }
}

impl From<fields::SymbolError> for SkeletonError {
    fn from(symbol_error: fields::SymbolError) -> Self {
        match symbol_error {
            fields::SymbolError::Invalid(ch) => SkeletonError::SymbolInvalid(ch),
            fields::SymbolError::Unknown(byte) => {
                match byte {
                    // TODO(#487) - Flexible day periods
                    b'B'
                    // TODO(#486) - Era
                    | b'G'
                    // TODO(#418) - Timezones
                    | b'Z' | b'v'
                    // TODO(#502) - Week of month
                    | b'W'
                    // TODO(#501) - Quarters
                    | b'Q'
                    // TODO (#488) - Week of year
                    | b'w'
                    => SkeletonError::SymbolUnimplemented(byte.into()),
                    _ => SkeletonError::SymbolUnknown(byte.into()),
                }
            }
        }
    }
}

#[cfg(all(test, feature = "provider_serde"))]
mod test {
    use super::*;
    use crate::fields::{Day, Field, FieldLength, Month, Weekday};

    // These were all of the skeletons from the "available formats" in the CLDR as of 2021-01
    // Generated with:
    // https://gist.github.com/gregtatum/1d76bbdb87132f71a969a10f0c1d2d9c

    #[rustfmt::skip]
    const SUPPORTED_STRING_SKELETONS: [&str; 51] = [
        "E", "dEEEE", "EHm", "EHms", "dE", "Ehm", "Ehms", "H", "HHmm", "HHmmss", "Hm", "Hms", "M",
        "MdEEEE", "MdE", "MMM", "MMMdEEEE", "MMMdE", "MMMM", "MMMMdEEEE", "MMMMdE", "MMMMd",
        "MMMMdd", "MMMd", "MMMdd", "MMd", "MMdd", "Md", "Mdd", "d", "h", "hm", "hms", "mmss", "ms",
        "y", "yM", "yMdEEEE", "yMdE", "yMM", "yMMM", "yMMMdEEEE", "yMMMdE", "yMMMM", "yMMMMdEEEE",
        "yMMMMdE", "yMMMMdcccc", "yMMMMd", "yMMMd", "yMMdd", "yMd",
    ];

    #[rustfmt::skip]
    const UNSUPPORTED_STRING_SKELETONS: [&str; 28] = [
        // TODO(#487) - Flexible day periods
        "Bh", "Bhm", "Bhms", "EBhm", "EBhms",
        // TODO(#486) - Era
        "Gy", "GyM", "GyMMM", "GyMMMdEEEE", "GyMMMdE", "GyMMMM", "GyMMMMdE", "GyMMMMd", "GyMMMd",
        // TODO(#418) - Timezones
        "HHmmZ", "Hmsv", "Hmsvvvv", "Hmv", "Hmvvvv", "hmsv", "hmsvvvv", "hmv", "hmvvvv",
        // TODO(#502) - Week of month
        "MMMMW",
        // TODO(#501) - Quarters
        "yQ", "yQQQ", "yQQQQ",
        // TODO (#488) - Week of year
        "yw"
    ];

    #[test]
    fn test_known_skeletons_ok() {
        for string_skeleton in &SUPPORTED_STRING_SKELETONS {
            match Skeleton::try_from(*string_skeleton) {
                Ok(_) => {}
                Err(err) => {
                    panic!(
                        "Unable to parse string_skeleton {:?} with error, {:?}",
                        string_skeleton, err
                    )
                }
            }
        }
    }

    #[test]
    fn test_unsupported_skeletons_skeletons_err() {
        for string_skeleton in &UNSUPPORTED_STRING_SKELETONS {
            match Skeleton::try_from(*string_skeleton) {
                Ok(_) => {
                    panic!(
                        "An unsupported field is now supported, consider moving {:?} to the \
                         supported skeletons, and ensure the skeleton is properly implemented.",
                        string_skeleton
                    )
                }
                Err(err) => match err {
                    SkeletonError::SymbolUnimplemented(_) => {
                        // Every skeleton should return this error.
                    }
                    _ => panic!("{}", err),
                },
            }
        }
    }

    #[test]
    fn test_skeleton_deserialization() {
        assert_eq!(
            Skeleton::try_from("MMMMdEEEE").unwrap(),
            Skeleton(
                vec![
                    Field {
                        symbol: Month::Format.into(),
                        length: FieldLength::Wide
                    },
                    Field {
                        symbol: Day::DayOfMonth.into(),
                        length: FieldLength::One
                    },
                    Field {
                        symbol: Weekday::Format.into(),
                        length: FieldLength::Wide
                    },
                ]
                .into()
            )
        );
    }

    #[test]
    fn test_skeleton_tuple_ordering() {
        let skeletons_strings = Vec::from([
            "y", "yM", "yMdE", "yMdEEEE", "yMMM", "M", "Md", "Mdd", "MMd", "MMdd", "d", "h", "hm",
            "hms", "Hm", "Hms", "ms", "mmss",
        ]);

        let skeleton_fields: Vec<Skeleton> = skeletons_strings
            .iter()
            .map(|skeleton_string| Skeleton::try_from(*skeleton_string).unwrap())
            .collect();

        for (strings, fields) in skeletons_strings.windows(2).zip(skeleton_fields.windows(2)) {
            if fields[0].compare_canonical_order(&fields[1]) != Ordering::Less {
                panic!("Expected {:?} < {:?}", strings[0], strings[1]);
            }
        }
    }
}
