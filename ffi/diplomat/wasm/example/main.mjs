import { ICU4XFixedDecimal, ICU4XDataProvider, ICU4XLocale, ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatOptions } from "../lib/api.mjs"

const decimal = ICU4XFixedDecimal.create(1234);
decimal.multiply_pow10(-2);
decimal.negate();
console.log(decimal.to_string());

const dataProvider = ICU4XDataProvider.create_test().provider;

const locale = ICU4XLocale.create("bn");

const format = ICU4XFixedDecimalFormat.try_new(locale, dataProvider, ICU4XFixedDecimalFormatOptions.default()).fdf;
console.log(format.format(decimal));
