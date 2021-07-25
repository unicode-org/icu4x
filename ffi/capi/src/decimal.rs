// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use icu_decimal::{FixedDecimalFormat, options::{FixedDecimalFormatOptions, GroupingStrategy, SignDisplay}};
    use alloc::boxed::Box;
    use writeable::Writeable;

    use crate::{fixed_decimal::ffi::ICU4XFixedDecimal, locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider};

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html) for more information.
    pub struct ICU4XFixedDecimalFormat(pub FixedDecimalFormat<'static, 'static>);

    pub struct ICU4XFixedDecimalFormatResult {
        /// The [`ICU4XFixedDecimalFormat`], exists if creation was successful.
        pub fdf: Option<Box<ICU4XFixedDecimalFormat>>,
        /// Whether creating the [`ICU4XFixedDecimalFormat`] was successful.
        pub success: bool,
    }

    pub enum ICU4XFixedDecimalGroupingStrategy {
        Auto,
        Never,
        Always,
        Min2,
    }

    pub enum ICU4XFixedDecimalSignDisplay {
        Auto,
        Never,
        Always,
        ExceptZero,
        Negative,
    }

    pub struct ICU4XFixedDecimalFormatOptions {
        pub grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        pub sign_display: ICU4XFixedDecimalSignDisplay,
    }

    impl ICU4XFixedDecimalFormatOptions {
        pub fn default() -> ICU4XFixedDecimalFormatOptions {
            ICU4XFixedDecimalFormatOptions {
                grouping_strategy: ICU4XFixedDecimalGroupingStrategy::Auto,
                sign_display: ICU4XFixedDecimalSignDisplay::Auto,
            }
        }
    }

    impl ICU4XFixedDecimalFormat {
        /// Creates a new [`ICU4XFixedDecimalFormat`] from locale data. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new) for more information.
        fn try_new(
            locale: &ICU4XLocale,
            provider: &ICU4XDataProvider,
            options: ICU4XFixedDecimalFormatOptions,
        ) -> ICU4XFixedDecimalFormatResult {
            let langid = locale.0.as_ref().clone();
            let provider = provider.0.as_ref();

            if let Result::Ok(fdf) = FixedDecimalFormat::try_new(
                langid,
                provider,
                FixedDecimalFormatOptions {
                    grouping_strategy: match options.grouping_strategy {
                        ICU4XFixedDecimalGroupingStrategy::Auto => GroupingStrategy::Auto,
                        ICU4XFixedDecimalGroupingStrategy::Never => GroupingStrategy::Never,
                        ICU4XFixedDecimalGroupingStrategy::Always => GroupingStrategy::Always,
                        ICU4XFixedDecimalGroupingStrategy::Min2 => GroupingStrategy::Min2,
                    },
                    sign_display: match options.sign_display {
                        ICU4XFixedDecimalSignDisplay::Auto => SignDisplay::Auto,
                        ICU4XFixedDecimalSignDisplay::Never => SignDisplay::Never,
                        ICU4XFixedDecimalSignDisplay::Always => SignDisplay::Always,
                        ICU4XFixedDecimalSignDisplay::ExceptZero => SignDisplay::ExceptZero,
                        ICU4XFixedDecimalSignDisplay::Negative => SignDisplay::Negative,
                    },
                },
            ) {
                ICU4XFixedDecimalFormatResult {
                    fdf: Some(Box::new(ICU4XFixedDecimalFormat(fdf))),
                    success: true,
                }
            } else {
                ICU4XFixedDecimalFormatResult {
                    fdf: None,
                    success: false,
                }
            }
        }

        /// Formats a [`ICU4XFixedDecimal`] to a string. See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format) for more information.
        fn format_write(
            &self,
            value: &ICU4XFixedDecimal,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> bool {
            let result = self.0.format(&value.0).write_to(write).is_ok();
            write.flush();
            result
        }
    }
}
