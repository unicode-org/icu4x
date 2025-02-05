// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::options::FractionalSecondDigits;

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatOption;
    use icu_datetime::fieldsets::enums::CompositeDateTimeFieldSet;
    use writeable::Writeable;

    use crate::{date::ffi::IsoDate, datetime_formatter::ffi::DateTimeLength, time::ffi::Time};

    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{
        errors::ffi::{DateTimeFormatterBuildOrLoadError, DateTimeFormatterLoadError},
        locale_core::ffi::Locale,
    };

    #[diplomat::enum_convert(icu_datetime::options::Alignment, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::Alignment, Enum)]
    pub enum DateTimeAlignment {
        Auto,
        Column,
    }

    #[diplomat::enum_convert(icu_datetime::options::YearStyle, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::YearStyle, Enum)]
    pub enum YearStyle {
        Auto,
        Full,
        WithEra,
    }

    #[diplomat::rust_link(icu::datetime::TimePrecision, Enum)]
    pub enum TimePrecision {
        Hour,
        Minute,
        MinuteOptional,
        Second,
        SecondF1,
        SecondF2,
        SecondF3,
        SecondF4,
        SecondF5,
        SecondF6,
        SecondF7,
        SecondF8,
        SecondF9,
    }

    #[diplomat::enum_convert(icu_datetime::fieldsets::builder::DateFields, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::DateFields, Enum)]
    pub enum DateFields {
        D,
        MD,
        YMD,
        DE,
        MDE,
        YMDE,
        E,
        M,
        YM,
        Y,
    }

    #[diplomat::enum_convert(icu_datetime::fieldsets::builder::ZoneStyle, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::ZoneStyle, Enum)]
    pub enum ZoneStyle {
        Z,
        Zs,
        O,
        Os,
        V,
        Vs,
        L,
    }

    #[diplomat::rust_link(icu::datetime::builder::FieldSetBuilder, Enum)]
    pub struct DateTimeFieldSetBuilder {
        pub length: DiplomatOption<DateTimeLength>,
        pub date_fields: DiplomatOption<DateFields>,
        pub time_precision: DiplomatOption<TimePrecision>,
        pub zone_style: DiplomatOption<ZoneStyle>,
        pub alignment: DiplomatOption<DateTimeAlignment>,
        pub year_style: DiplomatOption<YearStyle>,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Typedef)]
    pub struct DateTimeFormatter(pub icu_datetime::DateTimeFormatter<CompositeDateTimeFieldSet>);

    impl DateTimeFormatter {
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "from_builder")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DateTimeFormatter::try_new, FnInStruct)]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create_from_builder(
            locale: &Locale,
            builder: DateTimeFieldSetBuilder,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterBuildOrLoadError> {
            let prefs = (&locale.0).into();
            let builder = icu_datetime::fieldsets::builder::FieldSetBuilder::from(builder);
            let options = builder.build_composite_datetime()?;
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?,
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "from_builder_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DateTimeFormatter::try_new, FnInStruct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_from_builder_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            builder: DateTimeFieldSetBuilder,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterBuildOrLoadError> {
            let prefs = (&locale.0).into();
            let builder = icu_datetime::fieldsets::builder::FieldSetBuilder::from(builder);
            let options = builder.build_composite_datetime()?;
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?,
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_dt(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "dt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_dt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdt(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdt")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdt(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
            year_style: YearStyle,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into())
                .with_year_style(year_style.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdt_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDT, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdt_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
            year_style: YearStyle,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDT::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into())
                .with_year_style(year_style.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "det")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_det(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "det_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::DET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_det_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::DET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdet")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdet(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "mdet_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::MDET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_mdet_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::MDET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdet")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdet(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
            year_style: YearStyle,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into())
                .with_year_style(year_style.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "ymdet_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::YMDET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_ymdet_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
            year_style: YearStyle,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::YMDET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into())
                .with_year_style(year_style.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "et")]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_et(
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::ET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new(prefs, options)?.with_fset(),
            )))
        }

        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "et_with_provider")]
        #[diplomat::rust_link(icu::datetime::fieldsets::ET, Struct)]
        #[cfg(feature = "compiled_data")]
        pub fn create_et_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: DateTimeLength,
            time_precision: TimePrecision,
            alignment: DateTimeAlignment,
        ) -> Result<Box<DateTimeFormatter>, DateTimeFormatterLoadError> {
            let prefs = (&locale.0).into();
            let options = icu_datetime::fieldsets::ET::with_length(length.into())
                .with_alignment(alignment.into())
                .with_time_precision(time_precision.into());
            Ok(Box::new(DateTimeFormatter(
                icu_datetime::DateTimeFormatter::try_new_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?
                .with_fset(),
            )))
        }

        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::FormattedDateTime, Struct, hidden)]
        pub fn format_iso(
            &self,
            date: &IsoDate,
            time: &Time,
            write: &mut diplomat_runtime::DiplomatWrite,
        ) {
            let value = icu_timezone::DateTime {
                date: date.0,
                time: time.0,
            };
            let _infallible = self.0.format(&value).write_to(write);
        }
    }
}

impl From<ffi::TimePrecision> for icu_datetime::options::TimePrecision {
    fn from(time_precision: ffi::TimePrecision) -> Self {
        use icu_datetime::options::TimePrecision;
        match time_precision {
            ffi::TimePrecision::Hour => TimePrecision::Hour,
            ffi::TimePrecision::Minute => TimePrecision::Minute,
            ffi::TimePrecision::MinuteOptional => TimePrecision::MinuteOptional,
            ffi::TimePrecision::Second => TimePrecision::Second,
            ffi::TimePrecision::SecondF1 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F1)
            }
            ffi::TimePrecision::SecondF2 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F2)
            }
            ffi::TimePrecision::SecondF3 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F3)
            }
            ffi::TimePrecision::SecondF4 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F4)
            }
            ffi::TimePrecision::SecondF5 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F5)
            }
            ffi::TimePrecision::SecondF6 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F6)
            }
            ffi::TimePrecision::SecondF7 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F7)
            }
            ffi::TimePrecision::SecondF8 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F8)
            }
            ffi::TimePrecision::SecondF9 => {
                TimePrecision::FractionalSecond(FractionalSecondDigits::F9)
            }
        }
    }
}

impl From<ffi::DateTimeFieldSetBuilder> for icu_datetime::fieldsets::builder::FieldSetBuilder {
    fn from(other: ffi::DateTimeFieldSetBuilder) -> Self {
        let mut builder = Self::default();
        builder.length = other.length.into_option().map(Into::into);
        builder.date_fields = other.date_fields.into_option().map(Into::into);
        builder.time_precision = other.time_precision.into_option().map(Into::into);
        builder.zone_style = other.zone_style.into_option().map(Into::into);
        builder.alignment = other.alignment.into_option().map(Into::into);
        builder.year_style = other.year_style.into_option().map(Into::into);
        builder
    }
}
