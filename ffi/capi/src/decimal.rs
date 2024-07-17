// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "ICU4X{0}"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        data_struct::ffi::DataStruct, errors::ffi::DataError, fixed_decimal::ffi::FixedDecimal,
        locale_core::ffi::Locale, provider::ffi::DataProvider,
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

        /// Creates a new [`FixedDecimalFormatter`] from preconstructed locale data in the form of an [`DataStruct`]
        /// constructed from `DataStruct::create_decimal_symbols()`.
        ///
        /// The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed.
        /// Passing a consumed struct to this method will return an error.
        #[diplomat::attr(any(dart, js), disable)]
        pub fn create_with_decimal_symbols_v1(
            data_struct: &DataStruct,
            grouping_strategy: FixedDecimalGroupingStrategy,
        ) -> Result<Box<FixedDecimalFormatter>, DataError> {
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
                        // Note: This clone is free, since cloning AnyPayload is free.
                        data_struct.0.clone(),
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
