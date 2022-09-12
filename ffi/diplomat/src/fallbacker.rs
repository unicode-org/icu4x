// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider_adapters::fallback::LocaleFallbackConfig;

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_decimal::{
        options::{FixedDecimalFormatterOptions, GroupingStrategy},
        provider::DecimalSymbolsV1Marker,
        FixedDecimalFormatter,
    };
    use icu_locid::Locale;
    use icu_provider::DataProvider;
    use icu_provider::FallbackPriority;
    use icu_provider::KeyedDataMarker;
    use icu_provider_adapters::fallback::LocaleFallbackConfig;
    use icu_provider_adapters::fallback::LocaleFallbackIterator;
    use icu_provider_adapters::fallback::LocaleFallbacker;
    use icu_provider_adapters::fallback::LocaleFallbackerWithConfig;
    use writeable::Writeable;

    use crate::{
        data_struct::ffi::ICU4XDataStruct, errors::ffi::ICU4XError,
        fixed_decimal::ffi::ICU4XFixedDecimal, locale::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    #[diplomat::opaque]
    #[diplomat::rust_link(icu_provider_adapters::fallback::LocaleFallbacker, Struct)]
    pub struct ICU4XLocaleFallbacker(pub LocaleFallbacker);

    #[diplomat::enum_convert(FallbackPriority, needs_wildcard)]
    #[diplomat::rust_link(icu_provider::FallbackPriority, Enum)]
    pub enum ICU4XLocaleFallbackPriority {
        Language,
        Region,
        Collation,
    }

    #[diplomat::rust_link(icu_provider_adapters::fallback::LocaleFallbackConfig, Struct)]
    pub struct ICU4XLocaleFallbackConfig<'a> {
        pub priority: ICU4XLocaleFallbackPriority,
        /// An empty string is considered `None`.
        pub extension_key: &'a str,
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu_provider_adapters::fallback::LocaleFallbackerWithConfig, Struct)]
    pub struct ICU4XLocaleFallbackerWithConfig<'a>(pub LocaleFallbackerWithConfig<'a>);

    #[diplomat::opaque]
    #[diplomat::rust_link(icu_provider_adapters::fallback::LocaleFallbackIterator, Struct)]
    pub struct ICU4XLocaleFallbackIterator<'a>(pub LocaleFallbackIterator<'a, 'a>);

    impl ICU4XLocaleFallbacker {
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbacker::try_new_unstable,
            FnInStruct
        )]
        pub fn create(provider: &ICU4XDataProvider) -> DiplomatResult<Self, ICU4XError> {
            LocaleFallbacker::try_new_unstable(&provider.0)
                .map(ICU4XLocaleFallbacker)
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbacker::new_without_data,
            FnInStruct
        )]
        pub fn create_without_data() -> Self {
            ICU4XLocaleFallbacker(LocaleFallbacker::new_without_data())
        }

        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbacker::for_config,
            FnInStruct
        )]
        pub fn for_config<'a, 'temp>(
            &'a self,
            config: ICU4XLocaleFallbackConfig<'temp>,
        ) -> DiplomatResult<ICU4XLocaleFallbackerWithConfig<'a>, ICU4XError> {
            match LocaleFallbackConfig::try_from(config) {
                Ok(converted) => Ok(ICU4XLocaleFallbackerWithConfig(
                    self.0.for_config(converted),
                )),
                Err(e) => Err(e),
            }
            .into()
        }
    }

    impl<'a> ICU4XLocaleFallbackerWithConfig<'a> {
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackerWithConfig::fallback_for,
            FnInStruct
        )]
        pub fn fallback_for_locale<'b: 'a, 'temp>(
            &'b self,
            locale: &'temp ICU4XLocale,
        ) -> ICU4XLocaleFallbackIterator<'a> {
            ICU4XLocaleFallbackIterator(self.0.fallback_for((&locale.0).into()))
        }
    }

    impl<'a> ICU4XLocaleFallbackIterator<'a> {
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackIterator::get,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackIterator::take,
            FnInStruct,
            hidden
        )]
        pub fn get(&self) -> ICU4XLocale {
            ICU4XLocale(self.0.get().clone().into_locale())
        }

        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackIterator::step,
            FnInStruct
        )]
        pub fn step(&mut self) {
            self.0.step();
        }
    }
}

impl TryFrom<ffi::ICU4XLocaleFallbackConfig<'_>> for LocaleFallbackConfig {
    type Error = crate::errors::ffi::ICU4XError;
    fn try_from(other: ffi::ICU4XLocaleFallbackConfig) -> Result<Self, Self::Error> {
        let mut result = LocaleFallbackConfig::default();
        result.priority = other.priority.into();
        result.extension_key = match other.extension_key {
            "" => None,
            s => Some(s.parse()?),
        };
        Ok(result)
    }
}
