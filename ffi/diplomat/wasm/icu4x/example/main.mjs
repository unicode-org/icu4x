import { ICU4XFixedDecimal, ICU4XDataProvider, ICU4XLocale, ICU4XFixedDecimalFormatter, ICU4XFixedDecimalGroupingStrategy } from "../lib/index.js"

const decimal = ICU4XFixedDecimal.create_from_i32(1234);
decimal.multiply_pow10(-2);
decimal.set_sign("Negative");
console.log(decimal.to_string());

const dataProvider = ICU4XDataProvider.create_test();

const locale = ICU4XLocale.create_from_string("bn");

const fdf = ICU4XFixedDecimalFormatter.create(dataProvider, locale, "Auto");
console.log(fdf.format(decimal));
