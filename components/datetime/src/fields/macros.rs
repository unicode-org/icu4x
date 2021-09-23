// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Macro used to generate Field type.
///
/// The macro takes three arguments:
///  * Name of the field
///  * Map of `index: symbol => EnumVariant`
///  * (optional) LengthType
///
/// and generates all traits used by the Field.
///
/// The third argument is optional, since some fields have more
/// complex rules of calculating the length type and will implement the
/// `LengthType` trait manually.
///
/// # Examples
///
/// ```
/// field_type!(DayPeriod, {
///   0: 'a' => AmPm,
///   1: 'b' => NoonMidnight
/// }; Text);
/// ```
macro_rules! field_type {
    ($i:ident; { $($idx:expr; $key:expr => $val:ident),* }; $length_type:ident) => (
        field_type!($i; {$($idx; $key => $val),*});

        impl LengthType for $i {
            fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
                TextOrNumeric::$length_type
            }
        }
    );
    ($i:ident; { $($idx:expr; $key:expr => $val:ident),* }) => (
        #[derive(Debug, Eq, PartialEq, Clone, Copy)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        #[allow(clippy::enum_variant_names)]
        pub enum $i {
            $($val, )*
        }

        impl TryFrom<char> for $i {
            type Error = SymbolError;

            fn try_from(ch: char) -> Result<Self, Self::Error> {
                match ch {
                    $(
                        $key => Ok(Self::$val),
                    )*
                    _ => Err(SymbolError::Unknown(ch)),
                }
            }
        }

        impl From<$i> for FieldSymbol {
            fn from(input: $i) -> Self {
                Self::$i(input)
            }
        }

        impl From<$i> for char {
            fn from(input: $i) -> char {
                match input {
                    $(
                        $i::$val => $key,
                    )*
                }
            }
        }

        // The conversion from/to `u8` is used for serialization/deserialization

        impl TryFrom<u8> for $i {
            type Error = SymbolError;

            fn try_from(b: u8) -> Result<Self, Self::Error> {
                match b {
                    $(
                        $idx => Ok(Self::$val),
                    )*
                    _ => Err(SymbolError::InvalidIndex(b)),
                }
            }
        }


        impl From<$i> for u8 {
            fn from(b: $i) -> u8 {
                match b {
                    $(
                        $i::$val => $idx,
                    )*
                }
            }
        }
    );
}
