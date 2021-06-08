import { FixedDecimal, StaticDataProvider, Locale, FixedDecimalFormat } from "../lib/high-level.mjs"

const decimal = new FixedDecimal(BigInt(1234));
decimal.multiply_pow10(-2);
decimal.negate();
console.log(decimal.toString());

const dataProvider = new StaticDataProvider();

const locale = new Locale("bn");

const format = new FixedDecimalFormat(locale, dataProvider, {});
console.log(format.format(decimal));
