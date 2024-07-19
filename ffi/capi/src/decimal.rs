// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        errors::ffi::DataError, fixed_decimal::ffi::FixedDecimal, locale_core::ffi::Locale,
        provider::ffi::DataProvider,
    };

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`FixedDecimal`] as a string.
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
    #[diplomat::rust_link(icu::datetime::FormattedFixedDecimal, Struct, hidden)]
    pub struct FixedDecimalFormatter(pub icu_decimal::FixedDecimalFormatter);

    #[diplomat::rust_link(icu::decimal::options::GroupingStrategy, Enum)]
    pub enum FixedDecimalGroupingStrategy {
        Auto,
        Never,
        Always,
        Min2,
    }

    impl FixedDecimalFormatter {
        /// Creates a new [`FixedDecimalFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_grouping_strategy")]
        pub fn create_with_grouping_strategy(
            provider: &DataProvider,
            locale: &Locale,
            grouping_strategy: FixedDecimalGroupingStrategy,
        ) -> Result<Box<FixedDecimalFormatter>, DataError> {
            let locale = locale.to_datalocale();

            let grouping_strategy = match grouping_strategy {
                FixedDecimalGroupingStrategy::Auto => icu_decimal::options::GroupingStrategy::Auto,
                FixedDecimalGroupingStrategy::Never => {
                    icu_decimal::options::GroupingStrategy::Never
                }
                FixedDecimalGroupingStrategy::Always => {
                    icu_decimal::options::GroupingStrategy::Always
                }
                FixedDecimalGroupingStrategy::Min2 => icu_decimal::options::GroupingStrategy::Min2,
            };
            let mut options = icu_decimal::options::FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            Ok(Box::new(FixedDecimalFormatter(call_constructor!(
                icu_decimal::FixedDecimalFormatter::try_new,
                icu_decimal::FixedDecimalFormatter::try_new_with_any_provider,
                icu_decimal::FixedDecimalFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options,
            )?)))
        }

        /// Creates a new [`FixedDecimalFormatter`] from preconstructed locale data.
        #[diplomat::rust_link(icu::decimal::provider::DecimalSymbolsV1, Struct)]
        #[allow(clippy::too_many_arguments)]
        pub fn create_with_manual_data(
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
            grouping_strategy: FixedDecimalGroupingStrategy,
        ) -> Result<Box<FixedDecimalFormatter>, DataError> {
            use alloc::borrow::Cow;
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
            let mut new_digits = ['\0'; 10];
            for (old, new) in digits
                .iter()
                .copied()
                .chain(core::iter::repeat(char::REPLACEMENT_CHARACTER as u32))
                .zip(new_digits.iter_mut())
            {
                *new = char::from_u32(old).unwrap_or(char::REPLACEMENT_CHARACTER);
            }
            let digits = new_digits;
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

            let grouping_strategy = match grouping_strategy {
                FixedDecimalGroupingStrategy::Auto => icu_decimal::options::GroupingStrategy::Auto,
                FixedDecimalGroupingStrategy::Never => {
                    icu_decimal::options::GroupingStrategy::Never
                }
                FixedDecimalGroupingStrategy::Always => {
                    icu_decimal::options::GroupingStrategy::Always
                }
                FixedDecimalGroupingStrategy::Min2 => icu_decimal::options::GroupingStrategy::Min2,
            };
            let mut options = icu_decimal::options::FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            Ok(Box::new(FixedDecimalFormatter(
                icu_decimal::FixedDecimalFormatter::try_new_with_any_provider(
                    &icu_provider_adapters::any_payload::AnyPayloadProvider::from_any_payload::<
                        icu_decimal::provider::DecimalSymbolsV1Marker,
                    >(
                        icu_provider::DataPayload::<DecimalSymbolsV1Marker>::from_owned(
                            DecimalSymbolsV1 {
                                plus_sign_affixes,
                                minus_sign_affixes,
                                decimal_separator: str_to_cow(decimal_separator),
                                grouping_separator: str_to_cow(grouping_separator),
                                grouping_sizes,
                                digits,
                            },
                        )
                        .wrap_into_any_payload(),
                    ),
                    &Default::default(),
                    options,
                )?,
            )))
        }

        /// Formats a [`FixedDecimal`] to a string.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::decimal::FixedDecimalFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(icu::decimal::FormattedFixedDecimal, Struct, hidden)]
        #[diplomat::rust_link(icu::decimal::FormattedFixedDecimal::write_to, FnInStruct, hidden)]
        pub fn format(&self, value: &FixedDecimal, write: &mut diplomat_runtime::DiplomatWrite) {
            let _infallible = self.0.format(&value.0).write_to(write);
        }
    }
}
