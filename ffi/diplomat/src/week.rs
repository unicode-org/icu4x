// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::locale::ffi::ICU4XLocale;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::week::WeekCalculator;

    #[diplomat::rust_link(icu::calendar::week::RelativeUnit, Enum)]
    pub enum ICU4WeekRelativeUnit {
        Previous,
        Current,
        Next,
    }

    #[diplomat::rust_link(icu::calendar::week::WeekOf, Struct)]
    pub struct ICU4XWeekOf {
        pub week: u16,
        pub unit: RelativeUnit,
    }
    /// A Week calculator, useful for the week_of_month and week_of_year functions
    /// on date types.
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::calendar::week::WeekCalculator, Struct)]
    pub struct ICU4XWeekCalculator(pub WeekCalculator);

    impl ICU4XWeekCalculator {
        /// Creates a new [`ICU4XWeekCalculator`] from locale data.
        #[diplomat::rust_link(icu::calendar::week::WeekCalculator::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
        ) -> DiplomatResult<Box<ICU4XWeekCalculator>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            let locale = locale.to_datalocale();

            WeekCalculator::try_new_unstable(&provider, &locale)
                .map(|wc| Box::new(ICU4XWeekCalculator(wc)))
                .map_err(Into::into)
                .into()
        }
    }
}
