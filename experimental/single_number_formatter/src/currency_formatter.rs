
// FormattingData:
// .  Yes, we could merge them.

struct CurrencyFormatter {
    options: CurrencyFormatterOptions,
    data: DataPayload<FormattingData>,
}


struct CurrencyFormatterOptions {
    /// Determine the formatting width.
    /// For example for meter (FULL: 2 US Dollars) and (NARROW: 2$).
    pub width: Width,

    /// Determine the single unit which can be SingleMeasurementUnit or SingleCurrencyUnit
    pub single_unit: SingleUnit,
}

impl CurrencyFormatter {
    fn try_new_for_options<P>(data_provider: &P, locale: &DataLocale, options: SingleNumberFormatterOptions) -> Self
    where
        P: DataProvider<CurrencyFormattingData>,
    {
        unimplemented!();
    }

    /// Call site: `CurrencyFormatter::try_new_with_static_currency<USD>(...)
    fn try_new_with_static_currency<C, P>(data_provider: &P, locale: &DataLocale, options: SingleNumberFormatterOptions) -> Self
    where
        P: DataProvider<CurrencyFormattingData>,
        C: CurrencyT
    {
        unimplemented!();
    }

    fn format(&self, value: FixedDecimal) -> FormattedCurrency {
        unimplemented!();
    }
}
