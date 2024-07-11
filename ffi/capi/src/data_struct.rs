// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "icu_decimal")]

#[diplomat::bridge]
pub mod ffi {
    use alloc::borrow::Cow;
    use alloc::boxed::Box;

    use icu_provider::any::AnyPayload;
    use icu_provider::DataPayload;

    #[diplomat::opaque]
    /// A generic data struct to be used by ICU4X
    ///
    /// This can be used to construct a StructDataProvider.
    #[diplomat::attr(dart, disable)]
    pub struct DataStruct(pub(crate) AnyPayload);

    impl DataStruct {
        /// Construct a new DecimalSymbolsV1 data struct.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        ///
        /// Digits needs to be a 10-character-long slice of valid Unicode characters, otherwise null is
        /// returned.
        #[diplomat::rust_link(icu::decimal::provider::DecimalSymbolsV1, Struct)]
        #[allow(clippy::too_many_arguments)]
        pub fn create_decimal_symbols_v1(
            plus_sign_prefix: &DiplomatStr,
            plus_sign_suffix: &DiplomatStr,
            minus_sign_prefix: &DiplomatStr,
            minus_sign_suffix: &DiplomatStr,
            decimal_separator: &DiplomatStr,
            grouping_separator: &DiplomatStr,
            primary_group_size: u8,
            secondary_group_size: u8,
            min_group_size: u8,
            digits: &[DiplomatChar],
        ) -> Option<Box<DataStruct>> {
            fn str_to_cow(s: &diplomat_runtime::DiplomatStr) -> Cow<'static, str> {
                if s.is_empty() {
                    Cow::default()
                } else {
                    Cow::Owned(alloc::string::String::from_utf8_lossy(s).into_owned())
                }
            }

            use icu_decimal::provider::{
                AffixesV1, DecimalSymbolsV1, DecimalSymbolsV1Marker, GroupingSizesV1,
            };
            let digits = if digits.len() == 10 {
                let mut new_digits = ['\0'; 10];
                for (old, new) in digits.iter().zip(new_digits.iter_mut()) {
                    *new = char::from_u32(*old)?;
                }
                new_digits
            } else {
                return None;
            };
            let plus_sign_affixes = AffixesV1 {
                prefix: str_to_cow(plus_sign_prefix),
                suffix: str_to_cow(plus_sign_suffix),
            };
            let minus_sign_affixes = AffixesV1 {
                prefix: str_to_cow(minus_sign_prefix),
                suffix: str_to_cow(minus_sign_suffix),
            };
            let grouping_sizes = GroupingSizesV1 {
                primary: primary_group_size,
                secondary: secondary_group_size,
                min_grouping: min_group_size,
            };

            let symbols = DecimalSymbolsV1 {
                plus_sign_affixes,
                minus_sign_affixes,
                decimal_separator: str_to_cow(decimal_separator),
                grouping_separator: str_to_cow(grouping_separator),
                grouping_sizes,
                digits,
            };

            Some(Box::new(DataStruct(
                DataPayload::<DecimalSymbolsV1Marker>::from_owned(symbols).wrap_into_any_payload(),
            )))
        }
    }
}
