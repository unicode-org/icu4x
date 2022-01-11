// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::borrow::Cow;
use alloc::string::String;

#[diplomat::bridge]
pub mod ffi {
    use super::str_to_cow;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_decimal::provider::{
        AffixesV1, DecimalSymbolsV1, DecimalSymbolsV1Marker, GroupingSizesV1,
    };
    use icu_provider::dynutil::UpcastDataPayload;
    use icu_provider::erased::ErasedDataStructMarker;
    use icu_provider::prelude::DataPayload;

    #[diplomat::opaque]
    /// A generic data struct to be used by ICU4X
    ///
    /// This can be used to construct a StructDataProvider.
    pub struct ICU4XDataStruct(pub(crate) Option<DataPayload<ErasedDataStructMarker>>);

    impl ICU4XDataStruct {
        /// Construct a new DecimalSymbolsV1 data struct.
        ///
        /// See the [rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/provider/struct.DecimalSymbolsV1.html) for more details.
        #[allow(clippy::too_many_arguments)]
        pub fn create_decimal_symbols(
            plus_sign_prefix: &str,
            plus_sign_suffix: &str,
            minus_sign_prefix: &str,
            minus_sign_suffix: &str,
            decimal_separator: &str,
            grouping_separator: &str,
            primary_group_size: u8,
            secondary_group_size: u8,
            min_group_size: u8,
            digits: &[char],
        ) -> DiplomatResult<Box<ICU4XDataStruct>, ()> {
            let digits = if digits.len() == 10 {
                let mut new_digits = ['\0'; 10];
                new_digits.copy_from_slice(digits);
                new_digits
            } else {
                return Err(()).into();
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

            let payload: DataPayload<DecimalSymbolsV1Marker> = DataPayload::from_owned(symbols);
            Ok(Box::new(ICU4XDataStruct(Some(UpcastDataPayload::upcast(
                payload,
            )))))
            .into()
        }
    }
}

fn str_to_cow(s: &str) -> Cow<'static, str> {
    if s.is_empty() {
        Cow::default()
    } else {
        Cow::from(String::from(s))
    }
}
