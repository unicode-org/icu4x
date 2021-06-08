import { FixedDecimal, BufferWritable, StaticDataProvider, Locale, FixedDecimalFormat } from "../lib/high-level.mjs"

const decimal = new FixedDecimal(BigInt(1234));
decimal.multiply_pow10(-2);
decimal.negate();

const outWritable = new BufferWritable();
decimal.write_to(outWritable);
console.log(outWritable.getString());

const dataProvider = new StaticDataProvider();

const locale = new Locale("bn");

const format = new FixedDecimalFormat(locale, dataProvider, {});
const outWritable2 = new BufferWritable();
format.write(decimal, outWritable2);
console.log(outWritable2.getString());
