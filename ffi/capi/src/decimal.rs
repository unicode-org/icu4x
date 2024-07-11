// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        data_struct::ffi::ICU4XDataStruct, errors::ffi::ICU4XDataError,
        fixed_decimal::ffi::ICU4XFixedDecimal, locale_core::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    use writeable::Writeable;

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
    #[diplomat::rust_link(icu::datetime::FormattedFixedDecimal, Struct, hidden)]
    pub struct ICU4XFixedDecimalFormatter(pub icu_decimal::FixedDecimalFormatter);

    #[diplomat::rust_link(icu::decimal::options::GroupingStrategy, Enum)]
    pub enum ICU4XFixedDecimalGroupingStrategy {
        Auto,
        Never,
        Always,
        Min2,
    }

    impl ICU4XFixedDecimalFormatter {
        /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_grouping_strategy")]
        pub fn create_with_grouping_strategy(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        ) -> Result<Box<ICU4XFixedDecimalFormatter>, ICU4XDataError> {
            let locale = locale.to_datalocale();

            let grouping_strategy = match grouping_strategy {
                ICU4XFixedDecimalGroupingStrategy::Auto => {
                    icu_decimal::options::GroupingStrategy::Auto
                }
                ICU4XFixedDecimalGroupingStrategy::Never => {
                    icu_decimal::options::GroupingStrategy::Never
                }
                ICU4XFixedDecimalGroupingStrategy::Always => {
                    icu_decimal::options::GroupingStrategy::Always
                }
                ICU4XFixedDecimalGroupingStrategy::Min2 => {
                    icu_decimal::options::GroupingStrategy::Min2
                }
            };
            let mut options = icu_decimal::options::FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            Ok(Box::new(ICU4XFixedDecimalFormatter(call_constructor!(
                icu_decimal::FixedDecimalFormatter::try_new,
                icu_decimal::FixedDecimalFormatter::try_new_with_any_provider,
                icu_decimal::FixedDecimalFormatter::try_new_with_buffer_provider,
                provider,
                &locale,
                options,
            )?)))
        }

        /// Creates a new [`ICU4XFixedDecimalFormatter`] from preconstructed locale data in the form of an [`ICU4XDataStruct`]
        /// constructed from `ICU4XDataStruct::create_decimal_symbols()`.
        ///
        /// The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed.
        /// Passing a consumed struct to this method will return an error.
        #[diplomat::attr(dart, disable)]
        pub fn create_with_decimal_symbols_v1(
            data_struct: &ICU4XDataStruct,
            grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        ) -> Result<Box<ICU4XFixedDecimalFormatter>, ICU4XDataError> {
            let grouping_strategy = match grouping_strategy {
                ICU4XFixedDecimalGroupingStrategy::Auto => {
                    icu_decimal::options::GroupingStrategy::Auto
                }
                ICU4XFixedDecimalGroupingStrategy::Never => {
                    icu_decimal::options::GroupingStrategy::Never
                }
                ICU4XFixedDecimalGroupingStrategy::Always => {
                    icu_decimal::options::GroupingStrategy::Always
                }
                ICU4XFixedDecimalGroupingStrategy::Min2 => {
                    icu_decimal::options::GroupingStrategy::Min2
                }
            };
            let mut options = icu_decimal::options::FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            Ok(Box::new(ICU4XFixedDecimalFormatter(
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

        /// Formats a [`ICU4XFixedDecimal`] to a string.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::format, FnInStruct)]
        #[diplomat::rust_link(
            icu::decimal::FixedDecimalFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(icu::decimal::FormattedFixedDecimal, Struct, hidden)]
        #[diplomat::rust_link(icu::decimal::FormattedFixedDecimal::write_to, FnInStruct, hidden)]
        pub fn format(
            &self,
            value: &ICU4XFixedDecimal,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let _infallible = self.0.format(&value.0).write_to(write);
        }
    }
}
