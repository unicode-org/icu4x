// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! const_expr_count {
    () => (0);
    ($e:expr) => (1);
    ($e:expr; $($other_e:expr);*) => ({
        1 $(+ const_expr_count!($other_e) )*
    });

    ($e:expr; $($other_e:expr);* ; ) => (
        const_expr_count! { $e; $($other_e);* }
    );
}

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
///   'a' => AmPm,
///   'b' => NoonMidnight
/// }; Text);
/// ```
macro_rules! field_type {
    ($i:ident; { $($key:expr => $val:ident),* }; $length_type:ident) => (
        field_type!($i; {$($key => $val),*});

        impl LengthType for $i {
            fn get_length_type(&self, _length: FieldLength) -> TextOrNumeric {
                TextOrNumeric::$length_type
            }
        }
    );
    ($i:ident; { $($key:expr => $val:ident),* }) => (
        #[derive(Debug, Eq, PartialEq, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
        // FIXME: This should be replaced with a custom derive.
        // See: https://github.com/unicode-org/icu4x/issues/1044
        #[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
        #[cfg_attr(
            feature = "provider_serde",
            derive(serde::Serialize, serde::Deserialize)
        )]
        #[allow(clippy::enum_variant_names)]
        #[repr(u8)]
        pub enum $i {
            $($val, )*
        }

        impl $i {
            /// Retrieves an index of the field variant.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use icu::datetime::fields::Month;
            ///
            /// assert_eq!(Month::StandAlone::idx(), 1);
            /// ```
            ///
            /// # Stability
            ///
            /// This is mostly useful for serialization,
            /// and does not guarantee index stability between ICU4X
            /// versions.
            #[inline]
            pub(crate) fn idx(self) -> u8 {
                self.into()
            }

            /// Retrieves a field variant from an index.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use icu::datetime::fields::Month;
            ///
            /// assert_eq!(Month::from_idx(0), Month::Format);
            /// ```
            ///
            /// # Stability
            ///
            /// This is mostly useful for serialization,
            /// and does not guarantee index stability between ICU4X
            /// versions.
            #[inline]
            pub(crate) fn from_idx(idx: u8) -> Result<Self, SymbolError> {
                Self::try_from(idx)
                    .map_err(|_| SymbolError::InvalidIndex(idx))
            }

            #[inline]
            pub(crate) fn idx_in_range(v: &u8) -> bool {
                let count = const_expr_count!($($key);*);
                (0..count).contains(v)
            }
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
    );
}
