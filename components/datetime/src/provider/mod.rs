// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The `provider` module contains struct definitions for ICU4X [`DataProvider`].
//!
//! [`DataProvider`]: icu_provider::DataProvider

pub(crate) mod helpers;

pub mod key {
    use icu_provider::{resource::ResourceKey, resource_key};
    pub const GREGORY_V1: ResourceKey = resource_key!(dates, "gregory", 1);
}

pub mod gregory {
    use smallvec::SmallVec;
    use std::borrow::Cow;

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct DatesV1 {
        pub symbols: DateSymbolsV1,

        pub patterns: PatternsV1,
    }

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct DateSymbolsV1 {
        pub months: months::ContextsV1,

        pub weekdays: weekdays::ContextsV1,

        pub day_periods: day_periods::ContextsV1,
    }

    #[derive(Debug, PartialEq, Clone, Default)]
    #[cfg_attr(
        feature = "provider_serde",
        derive(serde::Serialize, serde::Deserialize)
    )]
    pub struct PatternsV1 {
        pub date: patterns::StylePatternsV1,

        pub time: patterns::StylePatternsV1,

        pub date_time: patterns::DateTimeFormatsV1,
    }

    macro_rules! symbols {
        ($name: ident, $expr: ty) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1(pub $expr);

                symbols!();
            }
        };
        ($name: ident { $($tokens: tt)* }) => {
            symbols!($name { $($tokens)* } -> ());
        };
        ($name: ident { $element: ident: Option<$ty: ty>, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty, $($tokens: tt)+ } -> ($($members:tt)*)) => {
            symbols!($name { $($tokens)* } -> (
                $($members)*
                pub $element: $ty,
            ));
        };
        ($name: ident { $element: ident: Option<$ty: ty> $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub $element: Option<$ty>,
            ));
        };
        ($name: ident { $element: ident: $ty: ty $(,)? } -> ($($members:tt)*)) => {
            symbols!($name { } -> (
                $($members)*
                pub $element: $ty,
            ));
        };
        ($name: ident { } -> ($($members: tt)*)) => {
            pub mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone, Default)]
                #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
                pub struct SymbolsV1 {
                    $($members)*
                }
                symbols!();
            }
        };
        () => {
            // UTS 35 specifies that `format` widths are mandatory
            // except of `short`.
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct FormatWidthsV1 {
                pub abbreviated: SymbolsV1,
                pub narrow: SymbolsV1,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub short: Option<SymbolsV1>,
                pub wide: SymbolsV1,
            }

            // UTS 35 specifies that `stand_alone` widths are optional
            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct StandAloneWidthsV1 {
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub abbreviated: Option<SymbolsV1>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub narrow: Option<SymbolsV1>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub short: Option<SymbolsV1>,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub wide: Option<SymbolsV1>,
            }

            #[derive(Debug, PartialEq, Clone, Default)]
            #[cfg_attr(feature="provider_serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct ContextsV1 {
                pub format: FormatWidthsV1,
                #[cfg_attr(
                    all(feature="provider_serde", not(feature="serialize_none")),
                    serde(skip_serializing_if = "Option::is_none"))
                ]
                pub stand_alone: Option<StandAloneWidthsV1>,
            }
        };
    }

    symbols!(months, [Cow<'static, str>; 12]);

    symbols!(weekdays, [Cow<'static, str>; 7]);

    symbols!(
        day_periods {
            am: Cow<'static, str>,
            pm: Cow<'static, str>,
            noon: Option<Cow<'static, str>>,
            midnight: Option<Cow<'static, str>>,
        }
    );

    pub mod patterns {
        use super::*;
        use crate::{
            fields::{self, Field, FieldLength, FieldSymbol, LengthError, SymbolError},
            pattern::{self, Pattern},
        };
        use litemap::LiteMap;
        use std::fmt;
        use std::{cmp::Ordering, convert::TryFrom};

        #[cfg(feature = "provider_serde")]
        use serde::{
            de::{self, Visitor},
            ser, Deserialize, Deserializer, Serialize,
        };

        #[derive(Debug, PartialEq, Clone, Default)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        pub struct StylePatternsV1 {
            pub full: Cow<'static, str>,
            pub long: Cow<'static, str>,
            pub medium: Cow<'static, str>,
            pub short: Cow<'static, str>,
        }

        /// This struct represents the serialization form of the skeleton fields. It contains
        /// a single "exotic type", the fields::Field, which must remain stable between versions.
        /// The skeletons are stored in a LiteMap, which is sorted according to the canonical
        /// sort order.
        #[derive(Debug, Eq, PartialEq, Clone, Default)]
        pub struct SkeletonFieldsV1(pub SmallVec<[fields::Field; 5]>);

        impl SkeletonFieldsV1 {
            /// The LiteMap<SkeletonFieldsV1, PatternV1> structs should be ordered by the
            /// `SkeletonFieldsV1` canonical order. Thishelps  ensure that the skeleton
            /// serialization is sorted deterministically. This order is determined by the order
            /// of the fields listed in UTS 35, which is ordered from most significant, to least
            /// significant.
            ///
            /// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
            pub fn compare_canonical_order(&self, other: &SkeletonFieldsV1) -> Ordering {
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

        impl PartialOrd for SkeletonFieldsV1 {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.compare_canonical_order(other))
            }
        }

        impl Ord for SkeletonFieldsV1 {
            fn cmp(&self, other: &Self) -> Ordering {
                self.compare_canonical_order(other)
            }
        }

        /// This struct is a public wrapper around the internal Pattern struct. This allows
        /// access to the serialization and deserialization capabilities, without exposing the
        /// internals of the pattern machinery.
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
            type Error = pattern::Error;

            fn try_from(pattern_string: &str) -> Result<Self, Self::Error> {
                let pattern = Pattern::from_bytes(pattern_string);
                match pattern {
                    Ok(pattern) => Ok(PatternV1::from(pattern)),
                    Err(err) => Err(err),
                }
            }
        }

        /// The serialization and deserialization errors need to be mapped to a public-facing
        /// error enum.
        #[derive(Debug, PartialEq)]
        pub enum SkeletonFieldsV1Error {
            SymbolUnimplemented(u8),
            SymbolUnknown(u8),
            SymbolInvalid(char),
            FieldTooLong,
        }

        impl From<SymbolError> for SkeletonFieldsV1Error {
            fn from(err: SymbolError) -> Self {
                match err {
                    SymbolError::Invalid(ch) => SkeletonFieldsV1Error::SymbolInvalid(ch),
                    SymbolError::Unknown(byte) => {
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
                            => SkeletonFieldsV1Error::SymbolUnimplemented(byte),
                            _ => SkeletonFieldsV1Error::SymbolUnknown(byte),
                        }
                    }
                }
            }
        }

        impl From<LengthError> for SkeletonFieldsV1Error {
            fn from(err: LengthError) -> Self {
                match err {
                    LengthError::TooLong => SkeletonFieldsV1Error::FieldTooLong,
                }
            }
        }

        impl TryFrom<&str> for SkeletonFieldsV1 {
            type Error = SkeletonFieldsV1Error;
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

                Ok(SkeletonFieldsV1(fields))
            }
        }

        /// This is an implementation of the serde deserialization visitor pattern.
        #[cfg(feature = "provider_serde")]
        struct SkeletonFieldsDeserializeVisitor;

        #[cfg(feature = "provider_serde")]
        impl<'de> Visitor<'de> for SkeletonFieldsDeserializeVisitor {
            type Value = SkeletonFieldsV1;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Expected to find a valid skeleton.")
            }

            /// A skeleton serialized into a string follows UTS 35.
            /// https://unicode.org/reports/tr35/tr35-dates.html#Date_Field_Symbol_Table
            /// This string consists of a symbol that is repeated N times. This string is
            /// deserialized here into the SkeletonFieldsV1 format which is used in memory
            /// when working with formatting date times.
            fn visit_str<E>(self, skeleton_string: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                SkeletonFieldsV1::try_from(skeleton_string).map_err(|err| match err {
                    SkeletonFieldsV1Error::SymbolUnknown(byte) => E::custom(format!(
                        "Skeleton {:?} contained an unknown symbol, {:?}",
                        skeleton_string, byte as char
                    )),
                    SkeletonFieldsV1Error::SymbolUnimplemented(byte) => E::custom(format!(
                        "Skeleton {:?} contained an unimplemented symbol, {:?}",
                        skeleton_string, byte as char
                    )),
                    SkeletonFieldsV1Error::FieldTooLong => E::custom(format!(
                        "Skeleton \"{}\" contained a field that was too long.",
                        skeleton_string
                    )),
                    SkeletonFieldsV1Error::SymbolInvalid(ch) => E::custom(format!(
                        "Skeleton {:?} contained an invalid symbol, {:?}",
                        skeleton_string, ch
                    )),
                })
            }
        }

        #[cfg(feature = "provider_serde")]
        impl<'de> Deserialize<'de> for SkeletonFieldsV1 {
            fn deserialize<D>(deserializer: D) -> Result<SkeletonFieldsV1, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(SkeletonFieldsDeserializeVisitor)
            }
        }

        #[cfg(feature = "provider_serde")]
        impl Serialize for SkeletonFieldsV1 {
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

        #[derive(Debug, PartialEq, Clone, Default)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        pub struct SkeletonsV1(pub LiteMap<SkeletonFieldsV1, PatternV1>);

        #[derive(Debug, PartialEq, Clone, Default)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        pub struct DateTimeFormatsV1 {
            pub style_patterns: StylePatternsV1,
            pub skeletons: SkeletonsV1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::gregory::patterns::*;
    use crate::fields::{Day, Field, FieldLength, Month, Weekday};
    use std::{cmp::Ordering, convert::TryFrom};

    // These were all of the skeletons from the "available formats" in the CLDR as of 2021-01
    // Generated with:
    // https://gist.github.com/gregtatum/1d76bbdb87132f71a969a10f0c1d2d9c

    #[rustfmt::skip]
    const SUPPORTED_STRING_SKELETONS: [&str; 51] = [
        "E", "EEEEd", "EHm", "EHms", "Ed", "Ehm", "Ehms", "H", "HHmm", "HHmmss", "Hm", "Hms", "M",
        "MEEEEd", "MEd", "MMM", "MMMEEEEd", "MMMEd", "MMMM", "MMMMEEEEd", "MMMMEd", "MMMMd",
        "MMMMdd", "MMMd", "MMMdd", "MMd", "MMdd", "Md", "Mdd", "d", "h", "hm", "hms", "mmss", "ms",
        "y", "yM", "yMEEEEd", "yMEd", "yMM", "yMMM", "yMMMEEEEd", "yMMMEd", "yMMMM", "yMMMMEEEEd",
        "yMMMMEd", "yMMMMccccd", "yMMMMd", "yMMMd", "yMMdd", "yMd",
    ];

    #[rustfmt::skip]
    const UNSUPPORTED_STRING_SKELETONS: [&str; 28] = [
        // TODO(#487) - Flexible day periods
        "Bh", "Bhm", "Bhms", "EBhm", "EBhms",
        // TODO(#486) - Era
        "Gy", "GyM", "GyMMM", "GyMMMEEEEd", "GyMMMEd", "GyMMMM", "GyMMMMEd", "GyMMMMd", "GyMMMd",
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
            match SkeletonFieldsV1::try_from(*string_skeleton) {
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
            match SkeletonFieldsV1::try_from(*string_skeleton) {
                Ok(_) => {
                    panic!(
                        "An unsupported field is now supported, consider moving {:?} to the \
                         supported skeletons, and ensure the skeleton is properly implemented.",
                        string_skeleton
                    )
                }
                Err(err) => match err {
                    SkeletonFieldsV1Error::SymbolUnimplemented(_) => {
                        // Every skeleton should return this error.
                    }
                    SkeletonFieldsV1Error::SymbolUnknown(byte) => {
                        panic!(
                            "An unknown symbol {:?} was found in the skeleton {:?}",
                            byte as char, string_skeleton
                        )
                    }
                    SkeletonFieldsV1Error::FieldTooLong => {
                        panic!("An field was too longin the skeleton {:?}", string_skeleton)
                    }
                    SkeletonFieldsV1Error::SymbolInvalid(ch) => {
                        panic!(
                            "An invalid symbol {:?} was found in the skeleton {:?}",
                            ch, string_skeleton
                        )
                    }
                },
            }
        }
    }

    #[test]
    fn test_skeleton_deserialization() {
        assert_eq!(
            SkeletonFieldsV1::try_from("MMMMEEEEd"),
            Ok(SkeletonFieldsV1(
                vec![
                    Field {
                        symbol: Month::Format.into(),
                        length: FieldLength::Wide
                    },
                    Field {
                        symbol: Weekday::Format.into(),
                        length: FieldLength::Wide
                    },
                    Field {
                        symbol: Day::DayOfMonth.into(),
                        length: FieldLength::One
                    }
                ]
                .into()
            ))
        );
    }

    #[test]
    fn test_skeleton_tuple_ordering() {
        let skeletons_strings = Vec::from([
            "y", "yM", "yMEd", "yMEEEEd", "yMMM", "M", "Md", "Mdd", "MMd", "MMdd", "d", "h", "hm",
            "hms", "Hm", "Hms", "ms", "mmss",
        ]);

        let skeleton_fields: Vec<SkeletonFieldsV1> = skeletons_strings
            .iter()
            .map(|skeleton_string| SkeletonFieldsV1::try_from(*skeleton_string).unwrap())
            .collect();

        for (strings, fields) in skeletons_strings.windows(2).zip(skeleton_fields.windows(2)) {
            if fields[0].compare_canonical_order(&fields[1]) != Ordering::Less {
                panic!("Expected {:?} < {:?}", strings[0], strings[1]);
            }
        }
    }
}
