// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::borrow::Cow;
    use alloc::boxed::Box;
    use icu_decimal::{
        options::{FixedDecimalFormatterOptions, GroupingStrategy},
        provider::DecimalSymbolsV1Marker,
        FixedDecimalFormatter,
    };
    use icu_provider_adapters::any_payload::AnyPayloadProvider;
    use writeable::Writeable;

    use crate::{
        errors::ffi::ICU4XError, fixed_decimal::ffi::ICU4XFixedDecimal, locale::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    use icu_provider::AnyPayload;
    use icu_provider::DataPayload;

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
    #[diplomat::rust_link(icu::datetime::FormattedFixedDecimal, Struct, hidden)]
    pub struct ICU4XFixedDecimalFormatter(pub FixedDecimalFormatter);

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
        ) -> Result<Box<ICU4XFixedDecimalFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();

            let grouping_strategy = match grouping_strategy {
                ICU4XFixedDecimalGroupingStrategy::Auto => GroupingStrategy::Auto,
                ICU4XFixedDecimalGroupingStrategy::Never => GroupingStrategy::Never,
                ICU4XFixedDecimalGroupingStrategy::Always => GroupingStrategy::Always,
                ICU4XFixedDecimalGroupingStrategy::Min2 => GroupingStrategy::Min2,
            };
            let mut options = FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            Ok(Box::new(ICU4XFixedDecimalFormatter(call_constructor!(
                FixedDecimalFormatter::try_new,
                FixedDecimalFormatter::try_new_with_any_provider,
                FixedDecimalFormatter::try_new_with_buffer_provider,
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
        #[diplomat::attr(*, disable)]
        pub fn create_with_decimal_symbols_v1(
            data_struct: &ICU4XDataStruct,
            grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        ) -> Result<Box<ICU4XFixedDecimalFormatter>, ICU4XError> {
            let grouping_strategy = match grouping_strategy {
                ICU4XFixedDecimalGroupingStrategy::Auto => GroupingStrategy::Auto,
                ICU4XFixedDecimalGroupingStrategy::Never => GroupingStrategy::Never,
                ICU4XFixedDecimalGroupingStrategy::Always => GroupingStrategy::Always,
                ICU4XFixedDecimalGroupingStrategy::Min2 => GroupingStrategy::Min2,
            };
            let mut options = FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            Ok(Box::new(ICU4XFixedDecimalFormatter(
                FixedDecimalFormatter::try_new_with_any_provider(
                    &AnyPayloadProvider::from_any_payload::<DecimalSymbolsV1Marker>(
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
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> Result<(), ICU4XError> {
            self.0.format(&value.0).write_to(write)?;
            Ok(())
        }
    }

    #[diplomat::opaque]
    /// A generic data struct to be used by ICU4X
    ///
    /// This can be used to construct a StructDataProvider.
    #[diplomat::attr(*, disable)]
    pub struct ICU4XDataStruct(pub(crate) AnyPayload);

    impl ICU4XDataStruct {
        /// Construct a new DecimalSymbolsV1 data struct.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
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
        ) -> Result<Box<ICU4XDataStruct>, ICU4XError> {
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
                    *new = char::from_u32(*old).ok_or(ICU4XError::DataStructValidityError)?;
                }
                new_digits
            } else {
                return Err(ICU4XError::DataStructValidityError);
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
            Ok(Box::new(ICU4XDataStruct(payload.wrap_into_any_payload())))
        }
    }
}
